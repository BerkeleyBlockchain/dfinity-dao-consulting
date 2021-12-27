import React, { useContext } from "react";
import { Outlet } from "react-router-dom";
import { DataProviderContext } from "./DataProvider";
import Loading from "./Loading";
import PlugConnect from "./PlugConnect";

import BabLogo from "./svg/bab-flat.svg";

const Layout = () => {
  const data = useContext(DataProviderContext);

  return (
    <>
      <header className="grid grid-cols-3 py-8 px-12">
        <div>
          <BabLogo className="h-6" />
        </div>
        <div />
        <div className="flex items-center justify-end">
          <PlugConnect />
        </div>
      </header>
      <main>{data === null ? <Loading /> : <Outlet />}</main>
    </>
  );
};

export default Layout;
