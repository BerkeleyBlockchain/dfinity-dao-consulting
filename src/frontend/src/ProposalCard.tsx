import React from "react";
import { Link } from "react-router-dom";
import { Application } from "../../declarations/tokens/tokens.did";

type Props = {
  application: Application;
};

const ProposalCard = ({ application }: Props) => (
  <Link
    className="flex group items-center rounded-xl py-4 px-6 mx-[-1.5rem] mb-4 shadow-none
      hover:shadow-lg 
      transition hover:bg-zinc-800"
    to={`/proposal/${application.principal}`}
  >
    <div className="flex-1 text-lg">
      <h3 className="text-zinc-300 group-hover:text-zinc-200">
        {application.proposal.name}
      </h3>
      <p>{application.proposal.description}</p>
    </div>
    <div className="ml-4 text-4xl font-light text-zinc-300 group-hover:text-zinc-200 shrink-0">
      →
    </div>
  </Link>
);

export default ProposalCard;
