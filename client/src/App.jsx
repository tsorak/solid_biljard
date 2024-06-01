import { HashRouter, Route } from "@solidjs/router";

import Layout from "./components/Layout.jsx";

import Home from "./routes/Home.jsx";
import Motd from "./routes/Motd.jsx";

export default function App() {
  return (
    <>
      <HashRouter root={Layout}>
        <Route path="/" component={Home} />
        <Route path="/motd" component={Motd} />
      </HashRouter>
    </>
  );
}
