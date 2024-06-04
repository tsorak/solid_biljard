import { createContext, useContext } from "solid-js";

let appState = null;

function create(state) {
  state ||= {
    socket: null,
  };

  appState = createContext(state);

  return appState;
}

function use() {
  return useContext(appState);
}

export { create, use };
