import type { Principal } from '@dfinity/principal';
export interface Application {
  'principal' : Principal,
  'create_timestamp' : bigint,
  'proposal' : Proposal,
}
export interface Invoice { 'random' : bigint, 'amount' : bigint }
export interface Proposal {
  'grant_size' : bigint,
  'name' : string,
  'description' : string,
}
export type Result = { 'E' : string } |
  { 'T' : boolean };
export interface _SERVICE {
  'getApps' : () => Promise<Array<Application>>,
  'joinAsVoter' : (arg_0: bigint, arg_1: bigint) => Promise<Invoice>,
  'stake' : (arg_0: Invoice, arg_1: bigint, arg_2: bigint) => Promise<boolean>,
  'submitApp' : (arg_0: Proposal) => Promise<undefined>,
}
