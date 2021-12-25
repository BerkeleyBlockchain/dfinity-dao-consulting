import React from "react";
import ReactDOM from "react-dom";
import { BrowserRouter } from "react-router-dom";
import App from "./App";
import { DataProvider } from "./DataProvider";
import { IdentityProvider } from "./Identity";
import "./style.css";

ReactDOM.render(
  <React.StrictMode>
    <DataProvider>
      <IdentityProvider>
        <BrowserRouter>
          <App />
        </BrowserRouter>
      </IdentityProvider>
    </DataProvider>
  </React.StrictMode>,
  document.getElementById("root")
);

// import { tokens } from "../../declarations/tokens";

// tokens
//   .getApps(0)
//   .then((apps) =>
//     apps.forEach(
//       (application) =>
//         (document.getElementById("proposalList").innerHTML += `<li><strong>${
//           application.proposal.name
//         }</strong>: "${application.proposal.description}" for ${
//           application.proposal.grant_size
//         } created on ${new Date(
//           Number(application.create_timestamp / 1000n)
//         )}</li>`)
//     )
//   );
