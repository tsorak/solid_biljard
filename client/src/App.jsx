import { HashRouter, Route } from "@solidjs/router";

import Layout from "./components/Layout.jsx";

import Home from "./routes/Home.jsx";
import Book from "./routes/Book.jsx";

import ClientSocket from "./websocket/socket.js";
import * as appState from "./appState.js";

export default function App() {
  const socket = new ClientSocket("/api/ws");
  socket.doReloadPageOnReconnect(true);
  socket.devtools();

  const AppState = appState.create({ socket });
  const state = appState.use();

  return (
    <AppState.Provider value={state}>
      <HashRouter root={Layout}>
        <Route path="/" component={Home} />
        <Route path="/book" component={Book} />
      </HashRouter>
    </AppState.Provider>
  );
}
