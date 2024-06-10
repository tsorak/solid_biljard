import styles from "./Generic.module.sass";

export default function Button(props) {
  const onclick = props.onClick ?? null;

  const { type } = props;

  return (
    <button
      class={styles.button}
      type={type}
      onClick={onclick}
    >
      {props.children}
    </button>
  );
}
