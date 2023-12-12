export const idlFactory = ({ IDL }) => {
  const ErrorMessage = IDL.Record({
    'tag' : IDL.Text,
    'message' : IDL.Text,
    'inputs' : IDL.Opt(IDL.Vec(IDL.Text)),
    'location' : IDL.Text,
  });
  const ValidationResponse = IDL.Record({
    'field' : IDL.Text,
    'message' : IDL.Text,
  });
  const UpdateMessage = IDL.Record({
    'canister_principal' : IDL.Principal,
    'message' : IDL.Text,
  });
  const ApiError = IDL.Variant({
    'SerializeError' : ErrorMessage,
    'DeserializeError' : ErrorMessage,
    'NotFound' : ErrorMessage,
    'ValidationError' : IDL.Vec(ValidationResponse),
    'CanisterAtCapacity' : ErrorMessage,
    'UpdateRequired' : UpdateMessage,
    'Unauthorized' : ErrorMessage,
    'Unexpected' : ErrorMessage,
    'BadRequest' : ErrorMessage,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : ApiError });
  const PostReport = IDL.Record({
    'subject' : IDL.Principal,
    'group_identifier' : IDL.Principal,
    'message' : IDL.Text,
  });
  const ReportResponse = IDL.Record({
    'subject' : IDL.Principal,
    'group_identifier' : IDL.Principal,
    'subject_kind' : IDL.Text,
    'created_on' : IDL.Nat64,
    'message' : IDL.Text,
    'reported_by' : IDL.Principal,
    'identifier' : IDL.Principal,
  });
  const Result_1 = IDL.Variant({ 'Ok' : ReportResponse, 'Err' : ApiError });
  const CanisterStatusType = IDL.Variant({
    'stopped' : IDL.Null,
    'stopping' : IDL.Null,
    'running' : IDL.Null,
  });
  const DefiniteCanisterSettings = IDL.Record({
    'freezing_threshold' : IDL.Nat,
    'controllers' : IDL.Vec(IDL.Principal),
    'memory_allocation' : IDL.Nat,
    'compute_allocation' : IDL.Nat,
  });
  const CanisterStatusResponse = IDL.Record({
    'status' : CanisterStatusType,
    'memory_size' : IDL.Nat,
    'cycles' : IDL.Nat,
    'settings' : DefiniteCanisterSettings,
    'idle_cycles_burned_per_day' : IDL.Nat,
    'module_hash' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const RejectionCode = IDL.Variant({
    'NoError' : IDL.Null,
    'CanisterError' : IDL.Null,
    'SysTransient' : IDL.Null,
    'DestinationInvalid' : IDL.Null,
    'Unknown' : IDL.Null,
    'SysFatal' : IDL.Null,
    'CanisterReject' : IDL.Null,
  });
  const Result_2 = IDL.Variant({
    'Ok' : IDL.Tuple(CanisterStatusResponse),
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const DateRange = IDL.Record({
    'end_date' : IDL.Nat64,
    'start_date' : IDL.Nat64,
  });
  const ReportFilter = IDL.Variant({
    'Kind' : IDL.Text,
    'ReportedBy' : IDL.Principal,
    'CreatedOn' : DateRange,
  });
  const FilterType = IDL.Variant({ 'Or' : IDL.Null, 'And' : IDL.Null });
  const SortDirection = IDL.Variant({ 'Asc' : IDL.Null, 'Desc' : IDL.Null });
  const ReportSort = IDL.Variant({
    'Id' : SortDirection,
    'Kind' : SortDirection,
    'CreatedOn' : SortDirection,
  });
  const PagedResponse = IDL.Record({
    'total' : IDL.Nat64,
    'data' : IDL.Vec(ReportResponse),
    'page' : IDL.Nat64,
    'limit' : IDL.Nat64,
    'number_of_pages' : IDL.Nat64,
  });
  const Result_3 = IDL.Variant({ 'Ok' : PagedResponse, 'Err' : ApiError });
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  const HttpHeader = IDL.Record({ 'value' : IDL.Text, 'name' : IDL.Text });
  const HttpResponse = IDL.Record({
    'status' : IDL.Nat,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HttpHeader),
  });
  return IDL.Service({
    '__get_candid_interface_tmp_hack' : IDL.Func([], [IDL.Text], ['query']),
    'accept_cycles' : IDL.Func([], [IDL.Nat64], []),
    'add_entry_by_parent' : IDL.Func([IDL.Vec(IDL.Nat8)], [Result], []),
    'add_report' : IDL.Func(
        [PostReport, IDL.Principal, IDL.Principal],
        [Result_1],
        [],
      ),
    'canister_status' : IDL.Func([], [Result_2], []),
    'clear_backup' : IDL.Func([], [], []),
    'download_chunk' : IDL.Func(
        [IDL.Nat64],
        [IDL.Tuple(IDL.Nat64, IDL.Vec(IDL.Nat8))],
        ['query'],
      ),
    'finalize_upload' : IDL.Func([], [IDL.Text], []),
    'get_chunked_data' : IDL.Func(
        [IDL.Vec(ReportFilter), FilterType, IDL.Nat64, IDL.Nat64],
        [IDL.Vec(IDL.Nat8), IDL.Tuple(IDL.Nat64, IDL.Nat64)],
        ['query'],
      ),
    'get_report' : IDL.Func(
        [IDL.Principal, IDL.Principal, IDL.Principal],
        [Result_1],
        [],
      ),
    'get_reports' : IDL.Func(
        [
          IDL.Nat64,
          IDL.Nat64,
          ReportSort,
          IDL.Vec(ReportFilter),
          FilterType,
          IDL.Principal,
          IDL.Principal,
        ],
        [Result_3],
        [],
      ),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'restore_data' : IDL.Func([], [], []),
    'total_chunks' : IDL.Func([], [IDL.Nat64], ['query']),
    'upload_chunk' : IDL.Func(
        [IDL.Tuple(IDL.Nat64, IDL.Vec(IDL.Nat8))],
        [],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  return [IDL.Principal, IDL.Text, IDL.Nat64];
};
