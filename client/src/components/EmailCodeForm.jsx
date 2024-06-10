import { createSignal, Match, Switch } from "solid-js";

import { Form } from "./generic.js";

import styles from "./EmailCodeForm/EmailCodeForm.module.sass";
import useStaging from "./EmailCodeForm/staging.js";

export default function EmailCodeForm() {
  const stage = useStaging();
  stage.set(0);

  const [email, setEmail] = createSignal("");

  return (
    <div class={styles.main}>
      <Switch>
        <Match when={stage.is("new_code")}>
          <NewCode stage={stage} signals={{ setEmail }} />
        </Match>
        <Match when={stage.is("validate_code")}>
          <ValidateCode stage={stage} signals={{ email }} />
        </Match>
        <Match when={stage.is("end")}>
          Du är nu inloggad.
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

function ValidateCode({ stage, signals }) {
  const { email } = signals;

  /** @type import('./generic/Form.jsx').SubmitFn */
  const submitFn = async (values) => {
    const { code } = values;

    const res = await fetch("/api/auth/email_code/validate", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "Accepts": "application/json",
      },
      body: JSON.stringify({ email: email(), code }),
    });

    if (!res.ok) {
      // TODO: handle returned error fields and have Form handle them
      // return await res.json();
      return false;
    }

    const json = await res.json();

    if (json?.uid === "correct_code") {
      stage.next();
      return true;
    }

    return false;
  };

  /** @type import('./generic/Form.jsx').FormSpec */
  const form = {
    fields: {
      code: {
        type: "digits",
        digitCount: 4,
        placeholder: "Ange kod...",
        errors: [
          [(field) => field.length < 4, "Koden innehåller 4 siffror"],
          [(field) => field.length > 4, "Koden är endast 4-siffrig"],
          [(field) => isNaN(field), "Koden innehåller endast siffror"],
          [(field) => field === "", "En kod måste anges"],
        ],
      },
    },
    submitElement: {
      text: "Verifiera",
    },
    submitFn,
  };

  return <Form form={form} />;
}
