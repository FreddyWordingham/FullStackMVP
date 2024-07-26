import { FC, useEffect, useState } from "react";

import { Typography } from "@mui/material";

import useAPI from "../hooks/useAPI";

const Ping: FC = () => {
  const { testsAPI } = useAPI();

  const [isOnline, setIsOnline] = useState<boolean>(false);

  const getPingStatus = async () => {
    const response = await testsAPI.ping();
    setIsOnline(response.status === 200);
  };

  useEffect(() => {
    const interval = setInterval(() => {
      getPingStatus();
    }, 1000);

    return () => clearInterval(interval);
  }, [getPingStatus, testsAPI]);

  return (
    <Typography variant="h4">{isOnline ? "Online" : "Offline"}</Typography>
  );
};

export default Ping;
