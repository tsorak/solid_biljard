import logo from "../logo.svg";
import styles from "./Motd/Motd.module.sass";

export default function Motd() {
  return (
    <main class={styles.main}>
      <img src={logo} class={styles._logo} alt="logo" />
      <p>Foo</p>
    </main>
  );
}
