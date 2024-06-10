import { createSignal, For } from "solid-js";
import styles from "./Generic.module.sass";

export default function DigitInput(props) {
  const { placeholder, signal } = props;
  const digitCount = props.digitCount ?? 4;

  const digits = new Array(digitCount).fill(0);
  digits.forEach((_, i, arr) => {
    arr[i] = createSignal("");
  });

  const updateSignal = () => {
    let code = "";

    digits.forEach(([get, _]) => {
      code = `${code}${get()}`;
    });

    signal[1](code);
  };

  const swapFocus = (digit_id) => {
    document.querySelector(`#digit_${digit_id}`)?.focus();
  };

  const onInput = (value, set, i) => {
    set((p) => {
      // swap focus to next field if current value is empty
      if (p === "") {
        swapFocus(i + 1);
      }

      return value;
    });

    updateSignal();
  };

  const onKeyDown = (e, i) => {
    switch (e.key) {
      case "ArrowRight":
        swapFocus(i + 1);
        break;

      case "ArrowLeft":
        swapFocus(i - 1);
        break;

      default:
        break;
    }
  };

  return (
    <div class={styles.digit_container}>
      <For each={digits}>
        {([get, set], i) => (
          <input
            id={`digit_${i()}`}
            type="number"
            class={styles.digit_input}
            placeholder={placeholder ?? "_"}
            autocomplete="off"
            value={get?.()}
            onInput={(e) => onInput(e.target.value, set, i())}
            onKeyDown={(e) => onKeyDown(e, i())}
            min={0}
            max={9}
            step={1}
          />
        )}
      </For>
    </div>
  );
}
