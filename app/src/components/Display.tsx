import { FC } from "react";

import { Typography } from "@mui/material";

const Display: FC<{
  answer: number;
}> = ({ answer }) => {
  return <Typography variant="h4">Result: {answer}</Typography>;
};

export default Display;
