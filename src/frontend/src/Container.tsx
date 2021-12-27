import { motion, Variants } from "framer-motion";
import React, { ReactNode } from "react";

type Props = {
  children: ReactNode;
};

const variants: Variants = {
  hidden: { opacity: 0 },
  show: {
    opacity: 1,
    transition: {
      delayChildren: 0.15,
    },
  },
};

const Container = ({ children }: Props) => (
  <motion.div
    initial="hidden"
    animate="show"
    exit="hidden"
    variants={variants}
    className="max-w-[768px] p-12 my-0 mx-auto"
  >
    {children}
  </motion.div>
);

export default Container;
