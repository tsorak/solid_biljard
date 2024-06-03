import { createResource, Show } from "solid-js";

import logo from "../logo.svg";
import styles from "./Motd/Motd.module.sass";
import getMotd from "./Motd/getMotd.js";

export default function Motd() {
  const [motd] = createResource(getMotd);

  return (
    <main class={styles.main}>
      <img src={logo} class={styles._logo} alt="logo" />
      <Show when={!motd.loading} fallback={() => <p>Loading motd...</p>}>
        <p>{motd().motd}</p>
      </Show>
    </main>
  );
}
