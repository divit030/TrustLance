
import React from "react";
import ReactDOM from "react-dom/client";
import { Buffer } from "buffer";
import App from "./App.jsx";
import SolanaProvider from "./Components/SolanaProvider.jsx";
import "./index.css";

globalThis.Buffer = Buffer;

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <SolanaProvider>
      <App />
    </SolanaProvider>
  </React.StrictMode>
);
