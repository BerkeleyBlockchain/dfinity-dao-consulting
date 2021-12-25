import React from "react";
import { Outlet } from "react-router-dom";

import BabLogo from "./svg/bab-flat.svg";

const Layout = () => (
  <>
    <header className="py-8 px-12">
      <BabLogo className="h-6" />
    </header>
    <main>
      <Outlet />
    </main>
  </>
);

export default Layout;
