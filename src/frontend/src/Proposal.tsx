import React, { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";
import Container from "./Container";
import { useDataProvider } from "./DataProvider";
import Markdown from "./Markdown";
import NotFound from "./NotFound";
import SectionHeading from "./SectionHeading";

const Proposal = () => {
  const { principal } = useParams<"principal">();
  const { proposals } = useDataProvider();

  const proposal = proposals.get(principal);

  if (proposal === undefined) {
    return <NotFound />;
  }

  return (
    <Container>
      <Link to="/proposals" className="text-zinc-500 font-medium font-mono">
        ← All proposals
      </Link>
      <SectionHeading>{proposal.proposal.name}</SectionHeading>
      <div>
        <Markdown text={proposal.proposal.description} />
      </div>
    </Container>
  );
};

export default Proposal;
