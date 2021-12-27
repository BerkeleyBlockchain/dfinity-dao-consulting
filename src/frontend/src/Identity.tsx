import { ActorSubclass, Agent } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import React, {
  createContext,
  Dispatch,
  FC,
  SetStateAction,
  useCallback,
  useContext,
  useEffect,
  useState,
} from "react";

import * as tokensCanister from "../../declarations/tokens";
import { _SERVICE as TOKENS_SERVICE } from "../../declarations/tokens/tokens.did";

import * as mintingCanister from "../../declarations/minting";
import { _SERVICE as MINTING_SERVICE } from "../../declarations/minting/minting.did";

import { idlFactory as dip20idlFactory } from "../../external/dip20";
import { _SERVICE as DIP20_SERVICE } from "../../external/dip20.did";

const IS_DEV_HOST = process.env.NODE_ENV !== "production";

export const WICP_CANISTER_ID: string = "utozz-siaaa-aaaam-qaaxq-cai";

export const WICP_CANISTER: Principal = Principal.fromText(WICP_CANISTER_ID);
export const TOKENS_CANISTER: Principal = Principal.fromText(
  tokensCanister.canisterId
);

// Override the host. (if overriden, will use a custom agent that is insecure).
const AGENT_PARAMS = {
  whitelist: [
    tokensCanister.canisterId,
    mintingCanister.canisterId,
    WICP_CANISTER_ID,
  ],
  host: IS_DEV_HOST ? location.origin : undefined,
};

export type TokensActor = ActorSubclass<TOKENS_SERVICE>;
export type MintingActor = ActorSubclass<MINTING_SERVICE>;
export type DIP20Actor = ActorSubclass<MINTING_SERVICE>;

type Identity = {
  principal: string;
  balance: bigint;
  tokens: TokensActor;
  minting: MintingActor;
  wipc: DIP20Actor;
} | null;

type IdentityContextData = {
  identity: Identity;
  isConnected: boolean;
  isLoading: boolean;
  setIdentity: Dispatch<SetStateAction<Identity>>;
  loadIdentityData: () => Promise<void>;
  connect: () => Promise<void>;
};

export const IdentityContext = createContext<IdentityContextData>(null);
export const IdentityProvider: FC = ({ children }) => {
  const [identity, setIdentity] = useState<Identity>(null);
  const [isLoading, setIsLoading] = useState<boolean>(false);

  const loadIdentityData = useCallback(async () => {
    setIsLoading(true);

    const [principal, tokens, minting, wipc] = await Promise.all<
      [Principal, TokensActor, MintingActor, DIP20Actor]
    >([
      window.ic.plug.getPrincipal(),
      window.ic.plug.createActor({
        canisterId: tokensCanister.canisterId,
        interfaceFactory: tokensCanister.idlFactory,
      }),
      window.ic.plug.createActor({
        canisterId: mintingCanister.canisterId,
        interfaceFactory: mintingCanister.idlFactory,
      }),
      window.ic.plug.createActor({
        canisterId: WICP_CANISTER_ID,
        interfaceFactory: dip20idlFactory,
      }),
    ]);

    // Get metadata.
    const userBalance = await minting.balanceOf(principal);

    setIdentity({
      principal: principal.toString(),
      balance: userBalance,
      tokens,
      minting,
      wipc,
    });
    setIsLoading(false);
  }, [setIdentity]);

  const connect = useCallback(async () => {
    const allowed = await window.ic.plug.requestConnect(AGENT_PARAMS);
    if (allowed) {
      await loadIdentityData();
    }
  }, [loadIdentityData]);

  const checkIdentity = async () => {
    if (window.ic?.plug) {
      const connected = await window.ic.plug.isConnected();
      if (!connected) {
        setIdentity(null);
      } else {
        await window.ic.plug.createAgent(AGENT_PARAMS);
        if (IS_DEV_HOST) await window.ic.plug.agent.fetchRootKey();
        await loadIdentityData();
      }
    }
  };

  useEffect(() => {
    checkIdentity();
  }, []);

  return (
    <IdentityContext.Provider
      value={{
        identity,
        setIdentity,
        loadIdentityData,
        connect,
        isConnected: identity !== null,
        isLoading,
      }}
    >
      {children}
    </IdentityContext.Provider>
  );
};

/**
 * Only call this within the <Layout> tag because that
 * has a guard to ensure the data is loaded.
 */
export const useIdentity = () => useContext(IdentityContext);

export const PLUG_SUPPORTED = !!window.ic?.plug;
