import React, { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";
import useSWR from "swr";
import { tokens } from "../../declarations/tokens";
import { Application } from "../../declarations/tokens/tokens.did";
import Container from "./Container";
import SectionHeading from "./SectionHeading";

const Proposal = () => {
  const { principal } = useParams<"principal">();
  return (
    <Container>
      <SectionHeading>{principal}</SectionHeading>
    </Container>
  );
};

export default Proposal;
