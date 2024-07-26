import { FC, useState } from "react";

import { Box, Button, Container, Paper } from "@mui/material";

import CoordInput from "./components/CoordInput";
import Display from "./components/Display";
import Ping from "./components/Ping";
import useAPI from "./hooks/useAPI";

const App: FC = () => {
  const [answer, setAnswer] = useState<number>(0.0);

  return (
    <Box
      display="flex"
      justifyContent="center"
      alignItems="center"
      height="100vh"
    >
      <Container maxWidth="sm">
        <Paper>
          <Box p={2}>
            <Ping />
          </Box>
          <Box p={2}>
            <Display answer={answer} />
          </Box>
          <Input setAnswer={setAnswer} />
        </Paper>
      </Container>
    </Box>
  );
};

const Input: FC<{
  setAnswer: (setAnswer: number) => void;
}> = ({ setAnswer }) => {
  const { calculatorAPI } = useAPI();

  const [real, setReal] = useState<number>(0.0);
  const [imag, setImag] = useState<number>(0.0);

  const run = async () => {
    const response = await calculatorAPI.samplePoint({
      real: real,
      imag: imag,
      max_iterations: 1000,
    });
    setAnswer(response.data.iterations);
  };

  return (
    <>
      <Box display="flex" justifyContent="space-around" p={2}>
        <CoordInput label="real" value={real} setValue={setReal} />
        <CoordInput label="imag" value={imag} setValue={setImag} />
      </Box>
      <Box p={2}>
        <Button variant="contained" fullWidth onClick={run}>
          Calculate
        </Button>
      </Box>
    </>
  );
};

export default App;
