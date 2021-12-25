import React, { ReactNode } from "react";

type Props = {
  children: ReactNode;
};

const Container = ({ children }: Props) => (
  <div className="max-w-[768px] p-12 my-0 mx-auto">{children}</div>
);

export default Container;
