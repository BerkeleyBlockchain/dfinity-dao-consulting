import React, { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";
import useSWR from "swr";
import { tokens } from "../../declarations/tokens";
import { Application } from "../../declarations/tokens/tokens.did";

const Proposal = () => {
  const { principal } = useParams<"principal">();
  return (
    <section>
      <h1>Proposal for {principal}</h1>
    </section>
  );
};

export default Proposal;
