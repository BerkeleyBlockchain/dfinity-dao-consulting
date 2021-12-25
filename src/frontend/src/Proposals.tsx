import React, { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import useSWR from "swr";
import { motion } from "framer-motion";
import { tokens } from "../../declarations/tokens";
import { Application } from "../../declarations/tokens/tokens.did";
import Container from "./Container";
import ProposalCard from "./ProposalCard";
import SectionHeading from "./SectionHeading";

const variants = {
  hidden: { opacity: 0, y: 10 },
  show: { opacity: 1, y: 0 },
};

const Proposals = () => {
  const { data, error } = useSWR([0], tokens.getApps);

  return (
    <Container>
      <SectionHeading
        attachment={
          <Link
            className="transition bg-amber-500 hover:bg-amber-600 px-5 py-2 text-sm rounded-md font-semibold text-white"
            to="/proposal/new"
          >
            New Proposal
          </Link>
        }
      >
        <span className="mr-4">All Proposals </span>
        {data && (
          <span className="text-zinc-500 font-normal text-2xl shadow-sm">
            {data.length} {data.length != 1 ? "proposals" : "proposal"}
          </span>
        )}
      </SectionHeading>

      {data && (
        <motion.ul
          variants={{
            hidden: {},
            show: {
              transition: {
                staggerChildren: 0.05,
              },
            },
          }}
          initial="hidden"
          animate="show"
        >
          {data.map((app) => (
            <motion.li variants={variants} key={app.principal.toString()}>
              <ProposalCard application={app} />
            </motion.li>
          ))}
        </motion.ul>
      )}
    </Container>
  );
};

export default Proposals;
