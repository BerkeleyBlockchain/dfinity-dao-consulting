import type { ActorSubclass, Agent, HttpAgent } from "@dfinity/agent";
import type { Principal } from "@dfinity/principal";

declare global {
  // namespace IDL {
  //   type InterfaceFactory = any;
  // }

  // export interface Transaction<
  //   SuccessReturn = unknown,
  //   FailReturn = unknown,
  //   SuccessResponse = unknown,
  //   FailResponse = unknown
  // > {
  //   idl: IDL.InterfaceFactory;
  //   canisterId: string;
  //   methodName: string;
  //   args: any[];
  //   onSuccess: (res: SuccessResponse) => Promise<SuccessReturn>;
  //   onFail: (res: FailResponse) => Promise<FailReturn>;
  // }

  // export interface RequestConnectInput {
  //   canisters?: Principal[];
  //   timeout?: number;
  // }

  // export interface TimeStamp {
  //   timestamp_nanos: bigint;
  // }

  // export interface SendOpts {
  //   fee?: bigint;
  //   memo?: bigint;
  //   from_subaccount?: number;
  //   created_at_time?: TimeStamp;
  // }

  // // The amount in e8s (ICPs)
  // interface RequestTransferParams {
  //   to: string;
  //   amount: bigint;
  //   opts?: SendOpts;
  // }

  // interface CreateActor<T> {
  //   agent: HttpAgent;
  //   actor: ActorSubclass<ActorSubclass<T>>;
  //   canisterId: string;
  //   interfaceFactory: IDL.InterfaceFactory;
  // }

  // interface RequestBurnXTCParams {
  //   to: string;
  //   amount: bigint;
  // }

  // export interface CreateAgentParams {
  //   whitelist?: string[];
  //   host?: string;
  // }

  // interface RequestConnectParams extends CreateAgentParams {
  //   timeout?: number;
  // }

  // export interface ProviderInterfaceVersions {
  //   provider: string;
  //   extension: string;
  // }

  // export interface ProviderInterface {
  //   isConnected(): Promise<boolean>;
  //   disconnect(): Promise<void>;
  //   batchTransactions(transactions: Transaction[]): Promise<boolean>;
  //   requestBalance(accountId?: number): Promise<bigint>;
  //   requestTransfer(params: RequestTransferParams): Promise<bigint>;
  //   requestConnect(params: RequestConnectParams): Promise<any>;
  //   createActor<T>({
  //     canisterId,
  //     interfaceFactory,
  //   }: CreateActor<T>): Promise<ActorSubclass<T>>;
  //   agent: Agent | null;
  //   createAgent(params: CreateAgentParams): Promise<boolean>;
  //   requestBurnXTC(params: RequestBurnXTCParams): Promise<any>;
  //   versions: ProviderInterfaceVersions;
  //   getPrincipal: () => Promise<Principal>;
  // }

  interface Window {
    ic?: {
      plug?: any;
    };
  }
}

export {};
