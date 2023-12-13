use crate::store::STABLE_DATA;

use super::store::Store;
use candid::Principal;
use ic_cdk::{caller, query, update};
use ic_scalable_canister::ic_scalable_misc::{
    enums::{api_error_type::ApiError, filter_type::FilterType},
    models::paged_response_models::PagedResponse,
};
use shared::report_model::{PostReport, ReportFilter, ReportResponse, ReportSort};

// This method is used to add a report to the canister,
// The method is async because it optionally creates a new canister
#[update]
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

// This method is used to get a report from the canister
#[update]
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
fn get_chunked_data(
    filters: Vec<ReportFilter>,
    filter_type: FilterType,
    chunk: usize,
    max_bytes_per_chunk: usize,
) -> (Vec<u8>, (usize, usize)) {
    if caller() != STABLE_DATA.with(|data| data.borrow().get().parent) {
        return (vec![], (0, 0));
    }

    Store::get_chunked_data(filters, filter_type, chunk, max_bytes_per_chunk)
}
