import React, { useEffect, useState } from "react";
import { PLUG_SUPPORTED, useIdentity } from "./Identity";
import PlugLogo from "./svg/Plug.svg";
import DfinityLogo from "./svg/Dfinity.svg";
import Loader from "./svg/puff-loader.svg";

const PlugConnect = () => {
  const { identity, isLoading, isConnected, connect } = useIdentity();

  return (
    <button
      className={`group p-[2px] text-sm rounded-md font-medium shadow ${
        PLUG_SUPPORTED ? "" : "disabled:opacity-80"
      }`}
      style={{
        background:
          "linear-gradient(120deg, rgba(255, 231, 1, 0.8), rgba(250, 81, 211, 0.8) 29%, rgba(16, 217, 237, 0.8) 65%, rgba(82, 255, 83, 0.8))",
      }}
      onClick={connect}
      disabled={!PLUG_SUPPORTED || isConnected || isLoading}
    >
      <div
        className={`transition inline-block rounded-md bg-stone-900 hover:bg-stone-800 group-disabled:bg-stone-900 
      ${
        isConnected
          ? "text-emerald-200"
          : isLoading
          ? "text-stone-400"
          : "text-stone-200 group-disabled:text-stone-500"
      } px-5 py-2`}
      >
        {" "}
        {isLoading ? (
          <>
            <Loader className="inline h-4 w-auto mr-2 fill-current" /> Loading
          </>
        ) : isConnected ? (
          <>
            <DfinityLogo className="inline h-4 w-auto mr-2" />{" "}
            {identity.balances.get("ICP")?.amount || 0} ICP
          </>
        ) : (
          <>
            <PlugLogo className="inline h-4 w-auto mr-2" /> Connect Plug
          </>
        )}
      </div>
    </button>
  );
};

export default PlugConnect;
