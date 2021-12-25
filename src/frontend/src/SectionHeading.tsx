import React, { ReactNode } from "react";

type Props = {
  attachment?: ReactNode;
  children: ReactNode;
};

const SectionHeading = ({ attachment, children }: Props) => (
  <div className="flex flex-wrap justify-between items-center mb-8">
    <h1 className="text-zinc-100 text-4xl font-medium flex leading-loose items-center mr-8">
      {children}
    </h1>
    {attachment}
  </div>
);

export default SectionHeading;
