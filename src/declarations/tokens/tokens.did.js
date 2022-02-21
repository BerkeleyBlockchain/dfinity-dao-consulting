export const idlFactory = ({ IDL }) => {
  const Proposal = IDL.Record({
    'grant_size' : IDL.Nat64,
    'name' : IDL.Text,
    'description' : IDL.Text,
  });
  const Application = IDL.Record({
    'principal' : IDL.Principal,
    'create_timestamp' : IDL.Nat64,
    'proposal' : Proposal,
  });
  const Invoice = IDL.Record({ 'random' : IDL.Nat64, 'amount' : IDL.Nat64 });
  return IDL.Service({
    'getApps' : IDL.Func([], [IDL.Vec(Application)], ['query']),
    'joinAsVoter' : IDL.Func([IDL.Nat64, IDL.Nat64], [Invoice], ['query']),
    'stake' : IDL.Func([Invoice, IDL.Nat64, IDL.Nat64], [IDL.Bool], []),
    'submitApp' : IDL.Func([Proposal], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
