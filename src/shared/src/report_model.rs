use std::borrow::Cow;

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_scalable_misc::{
    enums::sort_type::SortDirection, models::date_models::DateRange,
    traits::stable_storage_trait::StableStorableTrait,
};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Serialize;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Report {
    pub reported_by: Principal,
    pub subject: Principal,
    pub group_identifier: Principal,
    pub message: String,
    pub created_on: u64,
}

impl StableStorableTrait for Report {}

impl Storable for Report {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
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
