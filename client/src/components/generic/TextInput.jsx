import styles from "./Generic.module.sass";

export default function TextInput(props) {
  const [get, set] = props.signal ?? [null, null];

  const { name, type, placeholder } = props;

  return (
    <input
      name={name}
      type={type}
      class={styles.text_input}
      placeholder={placeholder ?? ""}
      value={get?.()}
      onInput={(e) => set?.(e.target.value)}
    />
  );
}
