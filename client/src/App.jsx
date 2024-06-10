import { Route, Router } from "@solidjs/router";

import Layout from "./components/Layout.jsx";

import Home from "./routes/Home.jsx";
import Book from "./routes/Book.jsx";
import * as auth from "./routes/auth.js";

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
      <Router root={Layout}>
        <Route path="/" component={Home} />
        <Route path="/book" component={Book} />
        <Route path="/auth/email_code" component={auth.EmailCode} />
      </Router>
    </AppState.Provider>
  );
}
