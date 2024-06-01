import logo from "../logo.svg";
import styles from "./Home/Home.module.sass";

export default function Home() {
  return (
    <main class={styles.main}>
      <img src={logo} class={styles._logo} alt="logo" />
      <p>
        Edit <code>src/App.jsx</code> and save to reload.
      </p>
      <a
        class={styles._link}
        href="https://github.com/solidjs/solid"
        target="_blank"
        rel="noopener noreferrer"
      >
        Learn Solid
      </a>
    </main>
  );
}
