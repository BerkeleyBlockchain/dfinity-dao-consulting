export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'joinVoter' : IDL.Func([IDL.Nat64, IDL.Nat64], [], []),
    'submitApp' : IDL.Func([IDL.Text, IDL.Nat64], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
