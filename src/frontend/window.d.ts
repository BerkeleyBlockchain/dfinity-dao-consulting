import Provider from "@fleekhq/plug-inpage-provider/src/Provider";

declare global {
  interface Window {
    ic?: {
      plug?: any;
    };
  }
}
