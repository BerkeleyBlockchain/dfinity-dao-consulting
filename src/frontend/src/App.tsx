import React from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";

import Layout from "./Layout";
import Proposal from "./Proposal";
import Proposals from "./Proposals";

const App = () => (
  <Routes>
    <Route path="/" element={<Layout />}>
      <Route index element={<Proposals />} />
      <Route path="proposals" element={<Proposals />} />
      <Route path="proposal/:principal" element={<Proposal />} />
    </Route>
  </Routes>
);

export default App;
