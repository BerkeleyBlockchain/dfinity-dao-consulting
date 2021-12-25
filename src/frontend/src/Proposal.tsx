import React, { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";
import useSWR from "swr";
import { tokens } from "../../declarations/tokens";
import { Application } from "../../declarations/tokens/tokens.did";
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
      <SectionHeading>{proposal.proposal.name}</SectionHeading>
      <div>
        <Markdown text={proposal.proposal.description} />
      </div>
    </Container>
  );
};

export default Proposal;
