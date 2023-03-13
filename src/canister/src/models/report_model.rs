use candid::Principal;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_scalable_misc::{enums::sort_type::SortDirection, models::date_models::DateRange};
use serde::Serialize;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Report {
    pub reported_by: Principal,
    pub subject: Principal,
    pub group_identifier: Principal,
    pub message: String,
    pub created_on: u64,
}

impl Default for Report {
    fn default() -> Self {
        Self {
            reported_by: Principal::anonymous(),
            subject: Principal::anonymous(),
            group_identifier: Principal::anonymous(),
            message: Default::default(),
            created_on: Default::default(),
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct PostReport {
    pub subject: Principal,
    pub message: String,
    pub group_identifier: Principal,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct ReportResponse {
    pub identifier: Principal,
    pub reported_by: Principal,
    pub group_identifier: Principal,
    pub subject: Principal,
    pub subject_kind: String,
    pub message: String,
    pub created_on: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ReportSort {
    Id(SortDirection),
    Kind(SortDirection),
    CreatedOn(SortDirection),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ReportFilter {
    Kind(String),
    CreatedOn(DateRange),
    ReportedBy(Principal),
}
