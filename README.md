# Report canister

This repository is responsible for handling reports of the Catalyze application. Reports are used to have soft moderation on the application.

## setup

The parent canister is SNS controlled, the child canisters are controlled by their parent. Upgrading the child canister is done through the parent canister as the (gzipped) child wasm is included in the parent canister.

When the parent canister is upgraded it checks if the child wasm has changed (currently it generates a new wasm hash every time you run the script). if changed it upgrades the child canisters automatically.

## Project structure

**|- candid**
Contains the candid files for the `parent` and `child` canister.

**|- frontend**
Contains all declarations that are needed for the frontend

**|- scripts**
Contains a single script that generates the following files for the parent and child canisters;

- candid files
- frontend declarations
- wasms (gzipped and regular)

**|- src/child**
Contains codebase related to the child canisters
**|- src/parent**
Contains codebase related to the child canisters
**|- src/shared**
Contains data used by both codebases

**|- wasm**
Contains

- child wasm
- child wasm (gzipped)
- parent wasm
- parent wasm (gzipped)

## Parent canister

The parent canister manages all underlying child canisters.

#### This canister is responsible for;

- keeping track of all report child canisters
- spinning up a new child canisters
- composite query call to the children (preperation)

#### methods

Described methods can be found below, for more details you can check out the code which is inline commented

###### DEFAULT

```
// Stores the data in stable storage before upgrading the canister.
pub fn pre_upgrade() {}

// Restores the data from stable- to heap storage after upgrading the canister.
pub fn post_upgrade() {}

// Init methods thats get triggered when the canister is installed
pub fn init() {}
```

##

###### QUERY CALLS

```
// Method to retrieve an available canister to write updates to
fn get_available_canister() -> Result<ScalableCanisterDetails, String> {}

// Method to retrieve all the canisters
fn get_canisters() -> Vec<ScalableCanisterDetails> {}

// Method to retrieve the latest wasm version of the child canister that is currently stored
fn get_latest_wasm_version() -> WasmVersion {}

// HTTP request handler (canister metrics are added to the response)
fn http_request(req: HttpRequest) -> HttpResponse {}

// Method used to get all the reports from the child canisters filtered, sorted and paged
// requires composite queries to be released to mainnet
async fn get_reports(
    limit: usize,
    page: usize,
    filters: Vec<ReportFilter>,
    filter_type: FilterType,
    sort: ReportSort,
) -> PagedResponse<ReportResponse> {}
```

##

###### UPDATE CALLS

```
// Method called by child canister once full (inter-canister call)
// can only be called by a child canister
async fn close_child_canister_and_spawn_sibling(
    last_entry_id: u64,
    entry: Vec<u8>
    ) -> Result<Principal, ApiError> {}

// Method to accept cycles when send to this canister
fn accept_cycles() -> u64 {}
```

## Child canister

The child canister is where the data is stored that the app uses.

This canister is responsible for;

- storing data records
- data validation
- messaging the parent to spin up a new sibling

#### methods

Described methods can be found below, for more details you can check out the code which is inline commented

###### DEFAULT

```
// Stores the data in stable storage before upgrading the canister.
pub fn pre_upgrade() {}

// Restores the data from stable- to heap storage after upgrading the canister.
pub fn post_upgrade() {}

// Init methods thats get triggered when the canister is installed
pub fn init(parent: Principal, name: String, identifier: usize) {}
```

##

###### QUERY CALLS

```
// HTTP request handler, canister metrics are added to the response by default
fn http_request(req: HttpRequest) -> HttpResponse {}

// COMPOSITE_QUERY PREPARATION
// This methods is used by the parent canister to get filtered reports from the (this) child canister
fn get_chunked_data(
    filters: Vec<ReportFilter>,
    filter_type: FilterType,
    chunk: usize,
    max_bytes_per_chunk: usize,
) -> (Vec<u8>, (usize, usize)) {}

```

###

###### UPDATE CALLS

```
// This call get triggered when a new canister is spun up
async fn add_entry_by_parent(entry: Vec<u8>) -> Result<(), ApiError> {}

// Method to accept cycles when send to this canister
fn accept_cycles() -> u64 {}

// This method is used to add a report to the canister,
async fn add_report(
    value: PostReport,
    group_identifier: Principal,
    member_identifier: Principal,
) -> Result<ReportResponse, ApiError> {}

// This method is used to get a report from the canister
async fn get_report(
    identifier: Principal,
    group_identifier: Principal,
    member_identifier: Principal,
) -> Result<ReportResponse, ApiError> {}

// This method is used to get reports filtered and sorted with pagination
async fn get_reports(
    limit: usize,
    page: usize,
    sort: ReportSort,
    filters: Vec<ReportFilter>,
    filter_type: FilterType,
    group_identifier: Principal,
    member_identifier: Principal,
) -> Result<PagedResponse<ReportResponse>, ApiError> {}
```

## SNS controlled

// TBD

## Testing

// TBD
