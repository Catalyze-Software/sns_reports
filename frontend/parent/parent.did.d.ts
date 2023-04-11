import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type ApiError = { 'SerializeError' : ErrorMessage } |
  { 'DeserializeError' : ErrorMessage } |
  { 'NotFound' : ErrorMessage } |
  { 'ValidationError' : Array<ValidationResponse> } |
  { 'CanisterAtCapacity' : ErrorMessage } |
  { 'UpdateRequired' : UpdateMessage } |
  { 'Unauthorized' : ErrorMessage } |
  { 'Unexpected' : ErrorMessage } |
  { 'BadRequest' : ErrorMessage };
export type CanisterType = { 'Empty' : null } |
  { 'Foundation' : null } |
  { 'Custom' : null } |
  { 'ScalableChild' : null } |
  { 'Scalable' : null };
export interface DateRange { 'end_date' : bigint, 'start_date' : bigint }
export interface ErrorMessage {
  'tag' : string,
  'message' : string,
  'inputs' : [] | [Array<string>],
  'location' : string,
}
export type FilterType = { 'Or' : null } |
  { 'And' : null };
export interface HttpHeader { 'value' : string, 'name' : string }
export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : Uint8Array | number[],
  'headers' : Array<[string, string]>,
}
export interface HttpResponse {
  'status' : bigint,
  'body' : Uint8Array | number[],
  'headers' : Array<HttpHeader>,
}
export interface PagedResponse {
  'total' : bigint,
  'data' : Array<ReportResponse>,
  'page' : bigint,
  'limit' : bigint,
  'number_of_pages' : bigint,
}
export type ReportFilter = { 'Kind' : string } |
  { 'ReportedBy' : Principal } |
  { 'CreatedOn' : DateRange };
export interface ReportResponse {
  'subject' : Principal,
  'group_identifier' : Principal,
  'subject_kind' : string,
  'created_on' : bigint,
  'message' : string,
  'reported_by' : Principal,
  'identifier' : Principal,
}
export type ReportSort = { 'Id' : SortDirection } |
  { 'Kind' : SortDirection } |
  { 'CreatedOn' : SortDirection };
export type Result = { 'Ok' : Principal } |
  { 'Err' : ApiError };
export type Result_1 = { 'Ok' : ScalableCanisterDetails } |
  { 'Err' : string };
export interface ScalableCanisterDetails {
  'entry_range' : [bigint, [] | [bigint]],
  'principal' : Principal,
  'wasm_version' : WasmVersion,
  'is_available' : boolean,
  'canister_type' : CanisterType,
}
export type SortDirection = { 'Asc' : null } |
  { 'Desc' : null };
export interface UpdateMessage {
  'canister_principal' : Principal,
  'message' : string,
}
export interface ValidationResponse { 'field' : string, 'message' : string }
export type WasmVersion = { 'None' : null } |
  { 'Version' : bigint } |
  { 'Custom' : null };
export interface _SERVICE {
  '__get_candid_interface_tmp_hack' : ActorMethod<[], string>,
  'accept_cycles' : ActorMethod<[], bigint>,
  'close_child_canister_and_spawn_sibling' : ActorMethod<
    [bigint, Uint8Array | number[]],
    Result
  >,
  'get_available_canister' : ActorMethod<[], Result_1>,
  'get_canisters' : ActorMethod<[], Array<ScalableCanisterDetails>>,
  'get_latest_wasm_version' : ActorMethod<[], WasmVersion>,
  'get_reports' : ActorMethod<
    [bigint, bigint, Array<ReportFilter>, FilterType, ReportSort],
    PagedResponse
  >,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
}
