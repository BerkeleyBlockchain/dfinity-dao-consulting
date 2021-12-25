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

type Balance = {
  amount: number;
  canisterId: string | null;
  name: string;
  symbol: string;
};

type Identity = {
  principal: string;
  balances: Map<string, Balance>;
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
    const [principal, balances] = await Promise.all([
      window.ic.plug.principal,
      window.ic.plug.requestBalance(),
    ]);

    // Convert balances => map
    const balanceMap = new Map<string, Balance>(
      balances.map((balance: Balance) => [balance.symbol, balance])
    );

    setIdentity({
      principal,
      balances: balanceMap,
    });
    setIsLoading(false);
  }, [setIdentity]);

  const connect = useCallback(async () => {
    const allowed = await window.ic.plug.requestConnect({
      whitelist: ["qoctq-giaaa-aaaaa-aaaea-cai"],
    });
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
