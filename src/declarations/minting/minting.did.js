export const idlFactory = ({ IDL }) => {
  const TxError = IDL.Variant({
    'InsufficientAllowance' : IDL.Null,
    'InsufficientBalance' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : TxError });
  const Metadata = IDL.Record({
    'fee' : IDL.Nat64,
    'decimals' : IDL.Nat8,
    'fee_to' : IDL.Principal,
    'owner' : IDL.Principal,
    'logo' : IDL.Text,
    'name' : IDL.Text,
    'total_supply' : IDL.Nat64,
    'symbol' : IDL.Text,
  });
  const TokenInfo = IDL.Record({
    'deploy_time' : IDL.Nat64,
    'holder_number' : IDL.Nat64,
    'fee_to' : IDL.Principal,
    'history_size' : IDL.Nat64,
    'metadata' : Metadata,
    'cycles' : IDL.Nat64,
  });
  const Operation = IDL.Variant({
    'Approve' : IDL.Null,
    'Mint' : IDL.Null,
    'Transfer' : IDL.Null,
    'TransferFrom' : IDL.Null,
  });
  const OpRecord = IDL.Record({
    'op' : Operation,
    'to' : IDL.Principal,
    'fee' : IDL.Nat64,
    'from' : IDL.Principal,
    'timestamp' : IDL.Nat64,
    'caller' : IDL.Opt(IDL.Principal),
    'index' : IDL.Nat64,
    'amount' : IDL.Nat64,
  });
  return IDL.Service({
    'addTotalSupply' : IDL.Func([IDL.Nat64], [IDL.Nat64], ['query']),
    'allowance' : IDL.Func(
        [IDL.Principal, IDL.Principal],
        [IDL.Nat64],
        ['query'],
      ),
    'approve' : IDL.Func([IDL.Principal, IDL.Nat64], [Result], []),
    'balanceOf' : IDL.Func([IDL.Principal], [IDL.Nat64], ['query']),
    'decimals' : IDL.Func([], [IDL.Nat8], ['query']),
    'getAllowanceSize' : IDL.Func([], [IDL.Nat64], ['query']),
    'getHolders' : IDL.Func(
        [IDL.Nat64, IDL.Nat64],
        [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat64))],
        ['query'],
      ),
    'getLogo' : IDL.Func([], [IDL.Text], ['query']),
    'getMetadata' : IDL.Func([], [Metadata], ['query']),
    'getTokenInfo' : IDL.Func([], [TokenInfo], ['query']),
    'getTransaction' : IDL.Func([IDL.Nat64], [OpRecord], ['query']),
    'getTransactions' : IDL.Func(
        [IDL.Nat64, IDL.Nat64],
        [IDL.Vec(OpRecord)],
        ['query'],
      ),
    'getUserApprovals' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat64))],
        ['query'],
      ),
    'getUserTransactionAmount' : IDL.Func(
        [IDL.Principal],
        [IDL.Nat64],
        ['query'],
      ),
    'getUserTransactions' : IDL.Func(
        [IDL.Principal, IDL.Nat64, IDL.Nat64],
        [IDL.Vec(OpRecord)],
        ['query'],
      ),
    'historySize' : IDL.Func([], [IDL.Nat64], ['query']),
    'name' : IDL.Func([], [IDL.Text], ['query']),
    'owner' : IDL.Func([], [IDL.Principal], ['query']),
    'setFee' : IDL.Func([IDL.Nat64], [], []),
    'setFeeTo' : IDL.Func([IDL.Principal], [], []),
    'setLogo' : IDL.Func([IDL.Text], [], []),
    'setMetadata' : IDL.Func(
        [
          IDL.Text,
          IDL.Text,
          IDL.Text,
          IDL.Nat8,
          IDL.Nat64,
          IDL.Principal,
          IDL.Nat64,
          IDL.Principal,
        ],
        [],
        ['query'],
      ),
    'setOwner' : IDL.Func([IDL.Principal], [], []),
    'symbol' : IDL.Func([], [IDL.Text], ['query']),
    'totalSupply' : IDL.Func([], [IDL.Nat64], ['query']),
    'transfer' : IDL.Func([IDL.Principal, IDL.Nat64], [Result], []),
    'transferFrom' : IDL.Func(
        [IDL.Principal, IDL.Principal, IDL.Nat64],
        [Result],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
