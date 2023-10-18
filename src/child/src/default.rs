use candid::Principal;
use ic_cdk::{caller, init, post_upgrade, pre_upgrade, query, update};

#[allow(unused_imports)]
use ic_scalable_canister::{ic_methods, store::Data};
use ic_scalable_misc::{
    enums::api_error_type::ApiError,
    models::http_models::{HttpRequest, HttpResponse},
};
use ic_stable_structures::memory_manager::MemoryId;

use crate::{
    store::{DATA, ENTRIES, MEMORY_MANAGER, STABLE_DATA},
    IDENTIFIER_KIND,
};

// Stores the data in stable storage before upgrading the canister.
#[pre_upgrade]
pub fn pre_upgrade() {
    let memory = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)));
    DATA.with(|data| ic_methods::deprecated_pre_upgrade(data, memory))
}

// Restores the data from stable- to heap storage after upgrading the canister.
#[post_upgrade]
pub fn post_upgrade() {
    let memory = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)));
    DATA.with(|data| ic_methods::deprecated_post_upgrade(data, memory))
}

// This call get triggered when a new canister is spun up
// the data is passed along to the new canister as a byte array
#[update]
async fn add_entry_by_parent(entry: Vec<u8>) -> Result<(), ApiError> {
    STABLE_DATA.with(|v| {
        ENTRIES.with(|entries| {
            Data::add_entry_by_parent(
                v,
                entries,
                caller(),
                entry,
                Some(IDENTIFIER_KIND.to_string()),
            )
        })
    })
}

// Method to accept cycles when send to this canister
#[update]
fn accept_cycles() -> u64 {
    ic_methods::accept_cycles()
}

// HTTP request handler, canister metrics are added to the response by default
// can be extended by adding `Vec<PathEntry>` as a third parameter
#[query]
fn http_request(req: HttpRequest) -> HttpResponse {
    STABLE_DATA.with(|data| {
        ENTRIES.with(|entries| Data::http_request_with_metrics(data, entries, req, vec![]))
    })
}

// Init methods thats get triggered when the canister is installed
// The parent canister is the canister that spins up this canister
// the name is a simple identification of what the canister stores
// the identifier is a incremented number that is used to create a unique name for the canister combined with the name
#[init]
pub fn init(parent: Principal, name: String, identifier: usize) {
    STABLE_DATA.with(|data| {
        ic_methods::init(data, parent, name, identifier);
    })
}

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    use candid::{export_service, Principal};
    use ic_cdk::api::management_canister::http_request::HttpResponse;
    use ic_scalable_misc::enums::api_error_type::ApiError;
    use ic_scalable_misc::enums::filter_type::FilterType;
    use ic_scalable_misc::models::http_models::HttpRequest;
    use ic_scalable_misc::models::paged_response_models::PagedResponse;
    use shared::report_model::*;
    export_service!();
    __export_service()
}

// Method used to save the candid interface to a file
#[test]
pub fn candid() {
    use ic_scalable_misc::helpers::candid_helper::save_candid;
    save_candid(__export_did_tmp_(), String::from("child"));
}
