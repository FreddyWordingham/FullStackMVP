import { useContext } from "react";

import APIContext from "../contexts/APIContext";

// This hook is a custom hook that allows us to access the APIContext across the app.
const useAPI = () => {
  const context = useContext(APIContext);
  if (context === undefined) {
    throw new Error("useAPI must be used within an APIProvider");
  }
  return context;
};

export default useAPI;
