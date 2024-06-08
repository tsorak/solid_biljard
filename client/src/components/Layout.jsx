import { A } from "@solidjs/router";
import logo from "../logo.svg";

import styles from "./Layout/Layout.module.sass";
import { For } from "solid-js";

const navLinks = [
  ["Home", "/"],
  ["Book", "/book"],
];

export default function Layout(props) {
  return (
    <div class={styles.main_layout}>
      <Header />
      {props.children}
      <Footer />
    </div>
  );
}

function Header() {
  return (
    <header class={styles.header}>
      <nav>
        <NavItems />
      </nav>
    </header>
  );
}

function Footer() {
  return (
    <footer class={styles.footer}>
      <nav>
        <NavItems />
      </nav>
      <div>
        <img src={logo} width="32" alt="logo" />
      </div>
      <div>
        <p>Don't call me :)</p>
      </div>
    </footer>
  );
}

function NavItems() {
  return (
    <For each={navLinks}>
      {([title, href]) => {
        return (
          <A href={href} class={styles.nav_link}>
            {title}
          </A>
        );
      }}
    </For>
  );
}
