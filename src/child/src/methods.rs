use std::{collections::HashMap, iter::FromIterator};

use super::store::{Store, DATA};
use candid::{candid_method, Principal};
use ic_cdk::{caller, query};
use ic_cdk_macros::update;
use ic_scalable_misc::{
    enums::{api_error_type::ApiError, filter_type::FilterType},
    models::paged_response_models::PagedResponse,
};
use shared::report_model::{PostReport, Report, ReportFilter, ReportResponse, ReportSort};

#[update]
#[candid_method(update)]
pub fn migration_add_reports(reports: Vec<(Principal, Report)>) -> () {
    if caller()
        == Principal::from_text("ledm3-52ncq-rffuv-6ed44-hg5uo-iicyu-pwkzj-syfva-heo4k-p7itq-aqe")
            .unwrap()
    {
        DATA.with(|data| {
            data.borrow_mut().current_entry_id = reports.clone().len() as u64;
            data.borrow_mut().entries = HashMap::from_iter(reports);
        })
    }
}
// This method is used to add a report to the canister,
// The method is async because it optionally creates a new canister
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
async fn add_report_test() -> () {
    Store::add_report_test().await;
}

// This method is used to get a report from the canister
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

// This method is used to get reports filtered and sorted with pagination
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

// COMPOSITE_QUERY PREPARATION
// This methods is used by the parent canister to get filtered reports from the (this) child canister
// Data serialized and send as byte array chunks ` (bytes, (start_chunk, end_chunk)) `
// The parent canister can then deserialize the data and pass it to the frontend
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
