import React from "react";
import { Link } from "react-router-dom";
import { motion } from "framer-motion";
import Container from "./Container";
import ProposalCard from "./ProposalCard";
import SectionHeading from "./SectionHeading";
import Loading from "./Loading";
import { useDataProvider } from "./DataProvider";

const variants = {
  hidden: { opacity: 0, y: 10 },
  show: { opacity: 1, y: 0 },
};

const Proposals = () => {
  const { proposals } = useDataProvider();

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
        {proposals && (
          <span className="text-zinc-500 font-normal text-2xl shadow-sm">
            {proposals.size} {proposals.size != 1 ? "proposals" : "proposal"}
          </span>
        )}
      </SectionHeading>

      {proposals ? (
        <motion.ul
          variants={{
            show: {
              transition: {
                staggerChildren: 0.05,
              },
            },
          }}
          initial="hidden"
          animate="show"
        >
          {[...proposals].map(([id, app]) => (
            <motion.li variants={variants} key={id}>
              <ProposalCard application={app} />
            </motion.li>
          ))}
        </motion.ul>
      ) : (
        <Loading />
      )}
    </Container>
  );
};

export default Proposals;
