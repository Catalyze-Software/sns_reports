use ic_cdk::query;
use ic_scalable_misc::{
    enums::filter_type::FilterType, models::paged_response_models::PagedResponse,
};

use shared::report_model::{ReportFilter, ReportResponse, ReportSort};

use super::store::ScalableData;

// Method used to get all the reports from the child canisters filtered, sorted and paged
// requires composite queries to be released to mainnet
#[query(composite = true)]
async fn get_reports(
    limit: usize,
    page: usize,
    filters: Vec<ReportFilter>,
    filter_type: FilterType,
    sort: ReportSort,
) -> PagedResponse<ReportResponse> {
    ScalableData::get_child_canister_data(limit, page, filters, filter_type, sort).await
}
