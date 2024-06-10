import { createSignal } from "solid-js";

const stages = ["new_code", "validate_code", "end"];

export default function useStaging() {
  const [_stage, setStage] = createSignal(0);
  const [current, setCurrent] = createSignal(stages[0]);

  const next = () => {
    setStage((p) => {
      if (p == 2) return p;

      const next = p + 1;
      setCurrent(stages[next]);
      return next;
    });
  };

  const prev = () => {
    setStage((p) => {
      if (p == 0) return p;

      const prev = p - 1;
      setCurrent(stages[prev]);
      return prev;
    });
  };

  const set = (i) => {
    setStage(() => {
      setCurrent(stages[i]);
      return i;
    });
  };

  const is = (pattern) => {
    return current() === pattern;
  };

  return {
    is,
    current,
    next,
    prev,
    set,
  };
}
