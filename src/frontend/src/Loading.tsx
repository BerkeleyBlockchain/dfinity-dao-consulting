import React, { useEffect, useState } from "react";

/**
 * This is a 'deferred' loading sign. It only shows up after ~250ms.
 * The point of that is to avoid a loading sign flash for a second
 * for very fast loading content.
 */
const Loading = () => {
  const [isShowing, setIsShowing] = useState(false);
  useEffect(() => {
    const id = setTimeout(() => {
      setIsShowing(true);
    }, 250);

    return () => clearTimeout(id);
  }, []);

  return (
    <div className="w-full min-h-[300px] flex items-center justify-center">
      {isShowing && <>Loading...</>}
    </div>
  );
};

export default Loading;
