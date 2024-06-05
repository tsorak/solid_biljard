import { createSignal } from "solid-js";

function createObjSignal(getterKey, setterKey, value) {
  const [g, s] = createSignal(value);
  return Object.fromEntries([[getterKey, g], [setterKey, s]]);
}

export { createObjSignal };
