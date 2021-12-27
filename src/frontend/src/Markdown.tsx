import React, { useMemo } from "react";
import MarkdownIt from "markdown-it";

const md = new MarkdownIt();

type Props = {
  text: string;
};

const Markdown = ({ text }: Props) => {
  const raw = useMemo(() => md.render(text), [text]);

  return <article dangerouslySetInnerHTML={{ __html: raw }} />;
};

export default Markdown;
