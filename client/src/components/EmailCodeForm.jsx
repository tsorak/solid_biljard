import { createSignal, Match, Switch } from "solid-js";

import { Form } from "./generic.js";

import styles from "./EmailCodeForm/EmailCodeForm.module.sass";
import useStaging from "./EmailCodeForm/staging.js";

export default function EmailCodeForm() {
  const stage = useStaging();

  const [email, setEmail] = createSignal("");

  return (
    <div class={styles.main}>
      <Switch>
        <Match when={stage.is("new_code")}>
          <NewCode stage={stage} signals={{ setEmail }} />
        </Match>
        <Match when={stage.is("validate_code")}>
          2
        </Match>
        <Match when={stage.is("end")}>
          3
        </Match>
      </Switch>
    </div>
  );
}

function NewCode({ stage, signals }) {
  const { setEmail } = signals;

  /** @type import('./generic/Form.jsx').SubmitFn */
  const submitFn = async (values) => {
    const { email } = values;

    const res = await fetch("/api/auth/email_code/new", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "Accepts": "application/json",
      },
      body: JSON.stringify({ email }),
    });

    if (!res.ok) {
      // TODO: handle returned error fields and have Form handle them
      // return await res.json();
      return false;
    }

    const json = await res.json();

    if (json?.uid === "session_created") {
      setEmail(email);
      stage.next();
      return true;
    }

    return false;
  };

  /** @type import('./generic/Form.jsx').FormSpec */
  const form = {
    fields: {
      email: {
        type: "text",
        placeholder: "Ange mejladress...",
        errors: [
          [(field) => field === "", "Mejladress måste anges"],
        ],
      },
    },
    submitElement: {
      text: "Skicka kod",
    },
    submitFn,
  };

  return <Form form={form} />;
}
