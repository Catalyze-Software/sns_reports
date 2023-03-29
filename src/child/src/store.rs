use candid::Principal;
use ic_cdk::api::time;
use ic_scalable_canister::store::Data;
use ic_scalable_misc::{
    enums::{
        api_error_type::{ApiError, ApiErrorType},
        filter_type::FilterType,
        sort_type::SortDirection,
        whitelist_rights_type::WhitelistRights,
    },
    helpers::{
        error_helper::api_error,
        paging_helper::get_paged_data,
        role_helper::{default_roles, get_group_roles, get_member_roles, has_permission},
        serialize_helper::serialize,
    },
    models::{
        identifier_model::Identifier,
        paged_response_models::PagedResponse,
        permissions_models::{PermissionActionType, PermissionType},
    },
};

use std::{cell::RefCell, collections::HashMap};

use shared::report_model::{PostReport, Report, ReportFilter, ReportResponse, ReportSort};

thread_local! {
    pub static DATA: RefCell<Data<Report>> = RefCell::new(Data::default());
}

pub struct Store;

impl Store {
    pub async fn add_report(
        caller: Principal,
        post_report: PostReport,
    ) -> Result<ReportResponse, ApiError> {
        let new_report = Report {
            reported_by: caller,
            subject: post_report.subject,
            message: post_report.message,
            created_on: time(),
            group_identifier: post_report.group_identifier,
        };
        match DATA.with(|data| Data::add_entry(data, new_report.clone(), Some("rpt".to_string()))) {
            Err(err) => match err {
                ApiError::CanisterAtCapacity(message) => {
                    let _data = DATA.with(|v| v.borrow().clone());
                    match Data::spawn_sibling(_data, new_report).await {
                        Ok(_) => Err(ApiError::CanisterAtCapacity(message)),
                        Err(err) => Err(err),
                    }
                }
                _ => Err(err),
            },
            Ok((identifier, report)) => Ok(Self::map_to_report_response(identifier, report)),
        }
    }

    pub fn get_report(
        identifier: Principal,
        group_identifier: Principal,
    ) -> Result<ReportResponse, ApiError> {
        DATA.with(|data| match Data::get_entry(data, identifier) {
            Err(err) => Err(err),
            Ok((_identifier, _result)) => {
                if _result.group_identifier != group_identifier {
                    return Err(api_error(
                        ApiErrorType::Unauthorized,
                        "REPORT_NOT_FOUND",
                        "Report does not belong to the group",
                        DATA.with(|data| Data::get_name(data)).as_str(),
                        "get_report",
                        None,
                    ));
                } else {
                    Ok(Self::map_to_report_response(_identifier, _result))
                }
            }
        })
    }

    pub fn get_reports(
        limit: usize,
        page: usize,
        sort: ReportSort,
        filters: Vec<ReportFilter>,
        filter_type: FilterType,
        group_identifier: Principal,
    ) -> PagedResponse<ReportResponse> {
        DATA.with(|data| {
            let get_result = Data::get_entries(data);
            let reports: Vec<ReportResponse> = get_result
                .iter()
                .filter(|r| r.1.group_identifier == group_identifier)
                .map(|(identifier, report)| {
                    Self::map_to_report_response(identifier.clone(), report.clone())
                })
                .collect();

            let filtered_reports = Self::get_filtered_reports(reports, filters, filter_type);
            let ordered_reports = Self::get_ordered_reports(filtered_reports, sort);

            get_paged_data(ordered_reports, limit, page)
        })
    }

    pub fn get_chunked_data(
        filters: Vec<ReportFilter>,
        filter_type: FilterType,
        chunk: usize,
        max_bytes_per_chunk: usize,
    ) -> (Vec<u8>, (usize, usize)) {
        let reports = DATA.with(|data| Data::get_entries(data));
        let mapped_reports: Vec<ReportResponse> = reports
            .iter()
            .map(|(_identifier, _report_data)| {
                Self::map_to_report_response(_identifier.clone(), _report_data.clone())
            })
            .collect();

        let filtered_groups = Self::get_filtered_reports(mapped_reports, filters, filter_type);
        if let Ok(bytes) = serialize(&filtered_groups) {
            if bytes.len() >= max_bytes_per_chunk {
                let start = chunk * max_bytes_per_chunk;
                let end = (chunk + 1) * (max_bytes_per_chunk);

                let response = if end >= bytes.len() {
                    bytes[start..].to_vec()
                } else {
                    bytes[start..end].to_vec()
                };

                let mut max_chunks: f64 = 0.00;
                if max_bytes_per_chunk < bytes.len() {
                    max_chunks = (bytes.len() / max_bytes_per_chunk) as f64;
                }
                return (response, (chunk, max_chunks.ceil() as usize));
            }
            return (bytes, (0, 0));
        } else {
            return (vec![], (0, 0));
        }
    }

    fn map_to_report_response(identifier: Principal, report: Report) -> ReportResponse {
        ReportResponse {
            identifier,
            reported_by: report.reported_by,
            subject: report.subject,
            subject_kind: Identifier::kind(&report.subject),
            message: report.message,
            created_on: report.created_on,
            group_identifier: report.group_identifier,
        }
    }

    fn get_filtered_reports(
        mut reports: Vec<ReportResponse>,
        filters: Vec<ReportFilter>,
        filter_type: FilterType,
    ) -> Vec<ReportResponse> {
        use FilterType::*;
        match filter_type {
            And => {
                for filter in filters {
                    use ReportFilter::*;
                    match filter {
                        Kind(value) => {
                            reports = reports
                                .into_iter()
                                .filter(|report| report.subject_kind.contains(&value))
                                .collect()
                        }
                        CreatedOn(value) => {
                            reports = reports
                                .into_iter()
                                .filter(|report| {
                                    if value.end_date > 0 {
                                        return report.created_on >= value.start_date
                                            && report.created_on <= value.end_date;
                                    } else {
                                        return report.created_on >= value.start_date;
                                    }
                                })
                                .collect();
                        }
                        ReportedBy(value) => {
                            reports = reports
                                .into_iter()
                                .filter(|report| report.reported_by == value)
                                .collect()
                        }
                    }
                }
                reports
            }
            Or => {
                let mut hashmap_reports: HashMap<Principal, ReportResponse> = HashMap::new();
                for filter in filters {
                    use ReportFilter::*;
                    match filter {
                        Kind(value) => reports
                            .iter()
                            .filter(|report| report.subject_kind.contains(&value))
                            .for_each(|v| {
                                hashmap_reports.insert(v.identifier.clone(), v.clone());
                            }),
                        CreatedOn(value) => reports
                            .iter()
                            .filter(|report| {
                                if value.end_date > 0 {
                                    return report.created_on >= value.start_date
                                        && report.created_on <= value.end_date;
                                } else {
                                    return report.created_on >= value.start_date;
                                }
                            })
                            .for_each(|v| {
                                hashmap_reports.insert(v.identifier.clone(), v.clone());
                            }),
                        ReportedBy(value) => reports
                            .iter()
                            .filter(|report| report.reported_by == value)
                            .for_each(|v| {
                                hashmap_reports.insert(v.identifier.clone(), v.clone());
                            }),
                    }
                }
                hashmap_reports.into_iter().map(|v| v.1).collect()
            }
        }
    }

    fn get_ordered_reports(
        mut reports: Vec<ReportResponse>,
        sort: ReportSort,
    ) -> Vec<ReportResponse> {
        use ReportSort::*;
        match sort {
            Id(direction) => match direction {
                SortDirection::Asc => reports.sort_by(|a, b| {
                    Identifier::decode(&a.identifier)
                        .0
                        .cmp(&Identifier::decode(&b.identifier).0)
                }),
                SortDirection::Desc => reports.sort_by(|a, b| {
                    Identifier::decode(&b.identifier)
                        .0
                        .cmp(&Identifier::decode(&a.identifier).0)
                }),
            },
            CreatedOn(direction) => match direction {
                SortDirection::Asc => reports.sort_by(|a, b| a.created_on.cmp(&b.created_on)),
                SortDirection::Desc => reports.sort_by(|a, b| b.created_on.cmp(&a.created_on)),
            },
            Kind(direction) => match direction {
                SortDirection::Asc => reports.sort_by(|a, b| a.subject_kind.cmp(&b.subject_kind)),
                SortDirection::Desc => reports.sort_by(|a, b| b.subject_kind.cmp(&a.subject_kind)),
            },
        };
        reports
    }

    pub async fn can_write(
        caller: Principal,
        group_identifier: Principal,
        member_identifier: Principal,
    ) -> Result<Principal, ApiError> {
        Self::check_permission(
            caller,
            group_identifier,
            member_identifier,
            PermissionActionType::Write,
        )
        .await
    }

    pub async fn can_read(
        caller: Principal,
        group_identifier: Principal,
        member_identifier: Principal,
    ) -> Result<Principal, ApiError> {
        Self::check_permission(
            caller,
            group_identifier,
            member_identifier,
            PermissionActionType::Read,
        )
        .await
    }

    pub async fn can_edit(
        caller: Principal,
        group_identifier: Principal,
        member_identifier: Principal,
    ) -> Result<Principal, ApiError> {
        Self::check_permission(
            caller,
            group_identifier,
            member_identifier,
            PermissionActionType::Edit,
        )
        .await
    }

    pub async fn can_delete(
        caller: Principal,
        group_identifier: Principal,
        member_identifier: Principal,
    ) -> Result<Principal, ApiError> {
        Self::check_permission(
            caller,
            group_identifier,
            member_identifier,
            PermissionActionType::Delete,
        )
        .await
    }

    async fn check_permission(
        caller: Principal,
        group_identifier: Principal,
        member_identifier: Principal,
        permission: PermissionActionType,
    ) -> Result<Principal, ApiError> {
        let group_roles = get_group_roles(group_identifier).await;
        let member_roles = get_member_roles(member_identifier, group_identifier).await;

        match member_roles {
            Ok((_principal, _roles)) => {
                if caller != _principal {
                    return Err(api_error(
                        ApiErrorType::Unauthorized,
                        "PRINCIPAL_MISMATCH",
                        "Principal mismatch",
                        DATA.with(|data| Data::get_name(data)).as_str(),
                        "check_permission",
                        None,
                    ));
                }

                match group_roles {
                    Ok(mut _group_roles) => {
                        _group_roles.append(&mut default_roles());
                        let has_permission = has_permission(
                            &_roles,
                            &PermissionType::Group(None),
                            &_group_roles,
                            &permission,
                        );

                        if !has_permission {
                            return Err(api_error(
                                ApiErrorType::Unauthorized,
                                "NO_PERMISSION",
                                "No permission",
                                DATA.with(|data| Data::get_name(data)).as_str(),
                                "check_permission",
                                None,
                            ));
                        }

                        Ok(caller)
                    }
                    Err(err) => Err(api_error(
                        ApiErrorType::Unauthorized,
                        "NO_PERMISSION",
                        err.as_str(),
                        DATA.with(|data| Data::get_name(data)).as_str(),
                        "check_permission",
                        None,
                    )),
                }
            }
            Err(err) => Err(api_error(
                ApiErrorType::Unauthorized,
                "NO_PERMISSION",
                err.as_str(),
                DATA.with(|data| Data::get_name(data)).as_str(),
                "check_permission",
                None,
            )),
        }
    }
}
