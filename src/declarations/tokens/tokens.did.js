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
  return IDL.Service({
    'getApps' : IDL.Func([IDL.Nat32], [IDL.Vec(Application)], ['query']),
    'joinAsVoter' : IDL.Func([IDL.Nat64, IDL.Nat64], [], []),
    'submitApp' : IDL.Func([Proposal], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
