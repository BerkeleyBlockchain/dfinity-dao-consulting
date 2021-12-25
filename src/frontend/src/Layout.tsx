import React, { useContext } from "react";
import { Outlet } from "react-router-dom";
import { DataProviderContext } from "./DataProvider";
import Loading from "./Loading";

import BabLogo from "./svg/bab-flat.svg";

const Layout = () => {
  const data = useContext(DataProviderContext);

  return (
    <>
      <header className="py-8 px-12">
        <BabLogo className="h-6" />
      </header>
      <main>{data === null ? <Loading /> : <Outlet />}</main>
    </>
  );
};

export default Layout;
