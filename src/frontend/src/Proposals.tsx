import React, { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import useSWR from "swr";
import { tokens } from "../../declarations/tokens";
import { Application } from "../../declarations/tokens/tokens.did";

const Proposals = () => {
  const { data, error } = useSWR([0], tokens.getApps);
  return (
    <section>
      <h1>Proposals</h1>
      <ul>
        {data &&
          data.map((app) => (
            <li key={app.principal.toString()}>
              <Link to={`/proposal/${app.principal}`}>{app.proposal.name}</Link>
            </li>
          ))}
      </ul>
    </section>
  );
};

export default Proposals;
