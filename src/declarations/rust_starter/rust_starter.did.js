export const idlFactory = ({ IDL }) => {
  return IDL.Service({ 'print' : IDL.Func([], [], ['query']) });
};
export const init = ({ IDL }) => { return []; };
