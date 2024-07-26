import { FC } from "react";

import { TextField } from "@mui/material";

const CoordInput: FC<{
  label: string;
  value: number;
  setValue: (value: number) => void;
}> = ({ label, value, setValue }) => {
  return (
    <TextField
      label={label}
      type="number"
      value={value}
      onChange={(e: any) => setValue(Number(e.target.value))}
      inputProps={{ step: 0.1 }}
    />
  );
};

export default CoordInput;
