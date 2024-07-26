import React from "react";
import ReactDOM from "react-dom/client";

import "./index.css";
import APIProvider from "./providers/APIProvider";
import App from "./App";

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);
root.render(
  <React.StrictMode>
    <APIProvider>
      <App />
    </APIProvider>
  </React.StrictMode>
);
