use super::store::{Store, DATA};
use candid::{candid_method, Principal};
use ic_cdk::{caller, query};
use ic_cdk_macros::update;
use ic_scalable_misc::{
    enums::{api_error_type::ApiError, filter_type::FilterType},
    models::paged_response_models::PagedResponse,
};
use shared::report_model::{PostReport, ReportFilter, ReportResponse, ReportSort};

#[update]
#[candid_method(update)]
async fn add_report(
    value: PostReport,
    group_identifier: Principal,
    member_identifier: Principal,
) -> Result<ReportResponse, ApiError> {
    match Store::can_read(caller(), group_identifier, member_identifier).await {
        Ok(_caller) => Store::add_report(_caller, value).await,
        Err(err) => Err(err),
    }
}

#[update]
#[candid_method(update)]
async fn get_report(
    identifier: Principal,
    group_identifier: Principal,
    member_identifier: Principal,
) -> Result<ReportResponse, ApiError> {
    match Store::can_write(caller(), group_identifier, member_identifier).await {
        Ok(_caller) => Store::get_report(identifier, group_identifier),
        Err(err) => Err(err),
    }
}

#[update]
#[candid_method(update)]
async fn get_reports(
    limit: usize,
    page: usize,
    sort: ReportSort,
    filters: Vec<ReportFilter>,
    filter_type: FilterType,
    group_identifier: Principal,
    member_identifier: Principal,
) -> Result<PagedResponse<ReportResponse>, ApiError> {
    match Store::can_write(caller(), group_identifier, member_identifier).await {
        Ok(_caller) => Ok(Store::get_reports(
            limit,
            page,
            sort,
            filters,
            filter_type,
            group_identifier,
        )),
        Err(err) => Err(err),
    }
}

#[query]
#[candid_method(query)]
fn get_chunked_data(
    filters: Vec<ReportFilter>,
    filter_type: FilterType,
    chunk: usize,
    max_bytes_per_chunk: usize,
) -> (Vec<u8>, (usize, usize)) {
    if caller() != DATA.with(|data| data.borrow().parent) {
        return (vec![], (0, 0));
    }

    Store::get_chunked_data(filters, filter_type, chunk, max_bytes_per_chunk)
}
