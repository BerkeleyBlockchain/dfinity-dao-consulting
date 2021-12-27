import React, {
  createContext,
  FC,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";
import useSWR from "swr";
import { tokens } from "../../declarations/tokens";
import { Application } from "../../declarations/tokens/tokens.did";

type DataProviderData = { proposals: Map<string, Application> };

type DataProviderType = DataProviderData | null;

export const DataProviderContext = createContext<DataProviderType>(null);
export const DataProvider: FC = ({ children }) => {
  const [proposalData, setProposalData] = useState<Application[]>(null);

  useEffect(() => {
    (async () => {
      const proposals = await tokens.getApps();
      setProposalData(proposals);
    })();
  }, []);

  // Maps principal string => Application
  const proposalMap: Map<string, Application> | null = useMemo(
    () =>
      !proposalData
        ? null
        : new Map(
            proposalData.map((proposal) => [
              proposal.principal.toString(),
              proposal,
            ])
          ),
    [proposalData]
  );

  return (
    <DataProviderContext.Provider
      value={
        proposalData && {
          proposals: proposalMap,
        }
      }
    >
      {children}
    </DataProviderContext.Provider>
  );
};

/**
 * Only call this within the <Layout> tag because that
 * has a guard to ensure the data is loaded.
 */
export const useDataProvider = (): DataProviderData => {
  const ctx = useContext(DataProviderContext);
  if (ctx === null) {
    throw new Error(
      "Attempted to call `useDataProvider` from outside Layout guard."
    );
  }
  return ctx!;
};
