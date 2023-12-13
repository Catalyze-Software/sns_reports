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
export interface CanisterStatusResponse {
  'status' : CanisterStatusType,
  'memory_size' : bigint,
  'cycles' : bigint,
  'settings' : DefiniteCanisterSettings,
  'idle_cycles_burned_per_day' : bigint,
  'module_hash' : [] | [Uint8Array | number[]],
}
export type CanisterStatusType = { 'stopped' : null } |
  { 'stopping' : null } |
  { 'running' : null };
export interface DateRange { 'end_date' : bigint, 'start_date' : bigint }
export interface DefiniteCanisterSettings {
  'freezing_threshold' : bigint,
  'controllers' : Array<Principal>,
  'memory_allocation' : bigint,
  'compute_allocation' : bigint,
}
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
export interface PostReport {
  'subject' : Principal,
  'group_identifier' : Principal,
  'message' : string,
}
export type RejectionCode = { 'NoError' : null } |
  { 'CanisterError' : null } |
  { 'SysTransient' : null } |
  { 'DestinationInvalid' : null } |
  { 'Unknown' : null } |
  { 'SysFatal' : null } |
  { 'CanisterReject' : null };
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
export type Result = { 'Ok' : null } |
  { 'Err' : ApiError };
export type Result_1 = { 'Ok' : ReportResponse } |
  { 'Err' : ApiError };
export type Result_2 = { 'Ok' : [CanisterStatusResponse] } |
  { 'Err' : [RejectionCode, string] };
export type Result_3 = { 'Ok' : PagedResponse } |
  { 'Err' : ApiError };
export type SortDirection = { 'Asc' : null } |
  { 'Desc' : null };
export interface UpdateMessage {
  'canister_principal' : Principal,
  'message' : string,
}
export interface ValidationResponse { 'field' : string, 'message' : string }
export interface _SERVICE {
  '__get_candid_interface_tmp_hack' : ActorMethod<[], string>,
  'accept_cycles' : ActorMethod<[], bigint>,
  'add_entry_by_parent' : ActorMethod<[Uint8Array | number[]], Result>,
  'add_report' : ActorMethod<[PostReport, Principal, Principal], Result_1>,
  'canister_status' : ActorMethod<[], Result_2>,
  'clear_backup' : ActorMethod<[], undefined>,
  'download_chunk' : ActorMethod<[bigint], [bigint, Uint8Array | number[]]>,
  'finalize_upload' : ActorMethod<[], string>,
  'get_chunked_data' : ActorMethod<
    [Array<ReportFilter>, FilterType, bigint, bigint],
    [Uint8Array | number[], [bigint, bigint]]
  >,
  'get_report' : ActorMethod<[Principal, Principal, Principal], Result_1>,
  'get_reports' : ActorMethod<
    [
      bigint,
      bigint,
      ReportSort,
      Array<ReportFilter>,
      FilterType,
      Principal,
      Principal,
    ],
    Result_3
  >,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'restore_data' : ActorMethod<[], undefined>,
  'total_chunks' : ActorMethod<[], bigint>,
  'upload_chunk' : ActorMethod<[[bigint, Uint8Array | number[]]], undefined>,
}
