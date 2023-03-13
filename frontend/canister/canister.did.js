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
  const Result = IDL.Variant({ 'Ok' : IDL.Principal, 'Err' : ApiError });
  const WasmVersion = IDL.Variant({
    'None' : IDL.Null,
    'Version' : IDL.Nat64,
    'Custom' : IDL.Null,
  });
  const CanisterType = IDL.Variant({
    'Empty' : IDL.Null,
    'Foundation' : IDL.Null,
    'Custom' : IDL.Null,
    'ScalableChild' : IDL.Null,
    'Scalable' : IDL.Null,
  });
  const ScalableCanisterDetails = IDL.Record({
    'entry_range' : IDL.Tuple(IDL.Nat64, IDL.Opt(IDL.Nat64)),
    'principal' : IDL.Principal,
    'wasm_version' : WasmVersion,
    'is_available' : IDL.Bool,
    'canister_type' : CanisterType,
  });
  const Result_1 = IDL.Variant({
    'Ok' : ScalableCanisterDetails,
    'Err' : IDL.Text,
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
  const ReportResponse = IDL.Record({
    'subject' : IDL.Principal,
    'group_identifier' : IDL.Principal,
    'subject_kind' : IDL.Text,
    'created_on' : IDL.Nat64,
    'message' : IDL.Text,
    'reported_by' : IDL.Principal,
    'identifier' : IDL.Principal,
  });
  const PagedResponse = IDL.Record({
    'total' : IDL.Nat64,
    'data' : IDL.Vec(ReportResponse),
    'page' : IDL.Nat64,
    'limit' : IDL.Nat64,
    'number_of_pages' : IDL.Nat64,
  });
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
    'close_child_canister_and_spawn_sibling' : IDL.Func(
        [IDL.Principal, IDL.Nat64, IDL.Vec(IDL.Nat8), IDL.Opt(IDL.Principal)],
        [Result],
        [],
      ),
    'get_available_canister' : IDL.Func([], [Result_1], ['query']),
    'get_canisters' : IDL.Func(
        [],
        [IDL.Vec(ScalableCanisterDetails)],
        ['query'],
      ),
    'get_latest_wasm_version' : IDL.Func([], [WasmVersion], ['query']),
    'get_reports' : IDL.Func(
        [IDL.Nat64, IDL.Nat64, IDL.Vec(ReportFilter), FilterType, ReportSort],
        [PagedResponse],
        ['query'],
      ),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
  });
};
export const init = ({ IDL }) => { return [IDL.Principal]; };
