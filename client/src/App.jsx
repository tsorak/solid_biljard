import { HashRouter, Route } from "@solidjs/router";

import connectSocket from "./websocket/main.js";

import Layout from "./components/Layout.jsx";

import Home from "./routes/Home.jsx";
import Motd from "./routes/Motd.jsx";

export default function App() {
  const socket = connectSocket();

  return (
    <>
      <HashRouter root={Layout}>
        <Route path="/" component={Home} />
        <Route path="/motd" component={Motd} />
      </HashRouter>
    </>
  );
}
