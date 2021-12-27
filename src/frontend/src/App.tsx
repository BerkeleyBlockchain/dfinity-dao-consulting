import React from "react";
import { BrowserRouter, Routes, Route, useLocation } from "react-router-dom";
import { AnimatePresence } from "framer-motion";

import Layout from "./Layout";
import NewProposal from "./NewProposal";
import NotFound from "./NotFound";
import Proposal from "./Proposal";
import Proposals from "./Proposals";

const App = () => {
  const location = useLocation();

  return (
    <AnimatePresence exitBeforeEnter>
      <Routes location={location} key={location.key}>
        <Route path="/" element={<Layout />}>
          <Route index element={<Proposals />} />
          <Route path="proposals" element={<Proposals />} />
          <Route path="proposal/:principal" element={<Proposal />} />
          <Route path="proposal/new" element={<NewProposal />} />

          <Route path="*" element={<NotFound />} />
        </Route>
      </Routes>
    </AnimatePresence>
  );
};

export default App;
