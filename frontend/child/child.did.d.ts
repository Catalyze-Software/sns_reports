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
export interface PostReport {
  'subject' : Principal,
  'group_identifier' : Principal,
  'message' : string,
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
export type Result = { 'Ok' : null } |
  { 'Err' : ApiError };
export type Result_1 = { 'Ok' : ReportResponse } |
  { 'Err' : ApiError };
export type Result_2 = { 'Ok' : PagedResponse } |
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
  'add_report_test' : ActorMethod<[], undefined>,
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
    Result_2
  >,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
}
