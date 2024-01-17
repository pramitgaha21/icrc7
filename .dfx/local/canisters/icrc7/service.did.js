export const idlFactory = ({ IDL }) => {
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const ApprovalArgs = IDL.Record({
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'from_subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'token_ids' : IDL.Opt(IDL.Vec(IDL.Nat)),
    'created_at_time' : IDL.Opt(IDL.Nat64),
    'expires_at' : IDL.Opt(IDL.Nat64),
    'spender' : Account,
  });
  const ApprovalError = IDL.Variant({
    'GenericError' : IDL.Record({
      'message' : IDL.Text,
      'error_code' : IDL.Nat,
    }),
    'TemporarilyUnavailable' : IDL.Null,
    'Unauthorized' : IDL.Vec(IDL.Nat),
    'TooOld' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : ApprovalError });
  const Icrc7CollectionMetadata = IDL.Record({
    'icrc7_supply_cap' : IDL.Opt(IDL.Nat),
    'icrc7_description' : IDL.Opt(IDL.Text),
    'icrc7_total_supply' : IDL.Nat,
    'icrc7_royalty_recipient' : IDL.Opt(Account),
    'icrc7_royalties' : IDL.Opt(IDL.Nat16),
    'icrc7_symbol' : IDL.Text,
    'icrc7_image' : IDL.Opt(IDL.Text),
    'icrc7_name' : IDL.Text,
  });
  const MetadataValue = IDL.Variant({
    'Int' : IDL.Int,
    'Nat' : IDL.Nat,
    'Blob' : IDL.Vec(IDL.Nat8),
    'Text' : IDL.Text,
  });
  const MintArgs = IDL.Record({
    'to' : IDL.Opt(Account),
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'is_atomic' : IDL.Opt(IDL.Bool),
    'token_description' : IDL.Opt(IDL.Text),
    'token_ids' : IDL.Vec(IDL.Nat),
    'image' : IDL.Opt(IDL.Text),
    'token_name' : IDL.Text,
  });
  const MintError = IDL.Variant({
    'GenericError' : IDL.Record({
      'message' : IDL.Text,
      'error_code' : IDL.Nat,
    }),
    'SupplyCapReached' : IDL.Record({ 'cap' : IDL.Nat }),
    'TokenIdExist' : IDL.Record({ 'token_ids' : IDL.Vec(IDL.Nat) }),
    'Unauthorized' : IDL.Record({ 'minting_authority' : IDL.Opt(Account) }),
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : MintError });
  const TransferArgs = IDL.Record({
    'to' : Account,
    'spender_subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'from' : Account,
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'is_atomic' : IDL.Opt(IDL.Bool),
    'token_ids' : IDL.Vec(IDL.Nat),
    'created_at_time' : IDL.Opt(IDL.Nat64),
  });
  const TransferError = IDL.Variant({
    'GenericError' : IDL.Record({
      'message' : IDL.Text,
      'error_code' : IDL.Nat,
    }),
    'TemporarilyUnavailable' : IDL.Null,
    'Duplicate' : IDL.Record({ 'duplicate_of' : IDL.Nat }),
    'Unauthorized' : IDL.Record({ 'token_ids' : IDL.Vec(IDL.Nat) }),
    'CreatedInFuture' : IDL.Record({ 'ledger_time' : IDL.Nat64 }),
    'TooOld' : IDL.Null,
  });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : TransferError });
  return IDL.Service({
    'icrc7_approve' : IDL.Func([ApprovalArgs], [Result], []),
    'icrc7_balance_of' : IDL.Func([Account], [IDL.Nat], ['query']),
    'icrc7_collection_metadata' : IDL.Func(
        [],
        [Icrc7CollectionMetadata],
        ['query'],
      ),
    'icrc7_description' : IDL.Func([], [IDL.Opt(IDL.Text)], ['query']),
    'icrc7_image' : IDL.Func([], [IDL.Opt(IDL.Text)], ['query']),
    'icrc7_metadata' : IDL.Func(
        [IDL.Nat],
        [IDL.Vec(IDL.Tuple(IDL.Text, MetadataValue))],
        ['query'],
      ),
    'icrc7_mint' : IDL.Func([MintArgs], [Result_1], []),
    'icrc7_name' : IDL.Func([], [IDL.Text], ['query']),
    'icrc7_owner_of' : IDL.Func([IDL.Nat], [Account], ['query']),
    'icrc7_royalties' : IDL.Func([], [IDL.Opt(IDL.Nat16)], ['query']),
    'icrc7_royalty_recipient' : IDL.Func([], [IDL.Opt(Account)], ['query']),
    'icrc7_supply_cap' : IDL.Func([], [IDL.Opt(IDL.Nat)], ['query']),
    'icrc7_symbol' : IDL.Func([], [IDL.Text], ['query']),
    'icrc7_tokens_of' : IDL.Func([Account], [IDL.Vec(IDL.Nat)], ['query']),
    'icrc7_total_supply' : IDL.Func([], [IDL.Nat], ['query']),
    'icrc7_transfer' : IDL.Func([TransferArgs], [Result_2], []),
  });
};
export const init = ({ IDL }) => { return []; };
