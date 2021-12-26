import { ActorSubclass } from "@dfinity/agent";
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
import { _SERVICE } from "../../declarations/tokens/tokens.did";

// Override the host. (if overriden, will use a custom agent that is insecure).
const AGENT_PARAMS = {
  whitelist: [tokensCanister.canisterId],
  host: process.env.NODE_ENV === "production" ? undefined : location.origin,
};

type Balance = {
  amount: number;
  canisterId: string | null;
  name: string;
  symbol: string;
};

export type TokensActor = ActorSubclass<_SERVICE>;

type Identity = {
  principal: string;
  balances: Map<string, Balance>;
  actor: TokensActor;
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

    const [principal, balances, actor] = await Promise.all<
      [Principal, Balance[], TokensActor]
    >([
      window.ic.plug.getPrincipal(),
      window.ic.plug.requestBalance(),
      window.ic.plug.createActor({
        canisterId: tokensCanister.canisterId,
        interfaceFactory: tokensCanister.idlFactory,
      }),
    ]);

    // Convert balances => map
    const balanceMap = new Map<string, Balance>(
      balances.map((balance: Balance) => [balance.symbol, balance])
    );

    console.log(actor);

    setIdentity({
      principal: principal.toString(),
      balances: balanceMap,
      actor: actor,
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
