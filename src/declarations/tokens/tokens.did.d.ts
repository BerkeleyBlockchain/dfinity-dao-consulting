import type { Principal } from '@dfinity/principal';
export interface Application {
  'principal' : Principal,
  'create_timestamp' : bigint,
  'proposal' : Proposal,
}
export interface Proposal {
  'grant_size' : bigint,
  'name' : string,
  'description' : string,
}
export interface _SERVICE {
  'getApps' : (arg_0: number) => Promise<Array<Application>>,
  'joinAsVoter' : (arg_0: bigint, arg_1: bigint) => Promise<undefined>,
  'submitApp' : (arg_0: Proposal) => Promise<undefined>,
}
