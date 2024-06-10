import { createSignal, For } from "solid-js";

import styles from "./Generic.module.sass";

import { Button, DigitInput, TextInput } from "../generic.js";

// const form = {
//   fields: {
//     email: {
//       type: "text",
//       placeholder: "Ange mejladress",
//       errors: [
//         [(field) => field === "", "Mejladress måste anges"],
//       ],
//     },
//   },
//   submitElement: {
//     text: "Skicka kod",
//   },
//   submitFn: (values) => {
//     const { email } = values;
//   },
// };

//
// INTERNAL TYPES
//

/**
 * @typedef {Object} InternalField
 * @property {string} type
 * @property {string | undefined} placeholder
 * @property {ErrorCase[] | undefined} errors
 * @property {[Function, Function]} signal
 * @property {[Function, Function]} errSignal
 */

/**
 * @typedef {[string, InternalField][]} InternalFieldsArray
 */

//
// PUBLIC TYPES
//

/**
 * @typedef {Object.<string, string>} FormValues
 */

/**
 * @callback SubmitFn
 * @param {FormValues} param0
 */

/**
 * @callback ErrorEvaluator
 * @param {string} currentValue
 * @returns {boolean} Returns whether currentValue is considered invalid
 */

/**
 * @typedef {[ErrorEvaluator, string]} ErrorCase
 */

/**
 * @typedef {Object} Field
 * @property {string} type
 * @property {string | undefined} placeholder
 * @property {ErrorCase[] | undefined} errors
 * @property {number} digitCount
 */

/**
 * @typedef {Object.<string, Field>} Fields
 * @property {Field} [key]
 */

/**
 * @typedef {Object} FormSpec
 * @property {Fields} fields
 * @property {{text:string}} submitElement
 * @property {SubmitFn} submitFn
 */

/**
 * @param {Object} param0
 * @param {FormSpec} param0.form
 */
export default function Form({ form }) {
  const { submitElement, submitFn } = form;
  let { fields } = form;

  fields = Object.entries(fields).map((field) => {
    const [fieldName, fieldProps] = field;

    const signals = Object.fromEntries([
      ["signal", createSignal("")],
      ["errSignal", createSignal("")],
    ]);

    return [fieldName, { ...fieldProps, ...signals }];
  });

  const submit = async (e) => {
    e.preventDefault();

    const hadErrors = runErrorChecks(fields);
    if (hadErrors) return;

    const values = fields.map(([name, props]) => [name, props.signal[0]()]);
    await submitFn(Object.fromEntries(values));
  };

  return (
    <form class={styles.form} onSubmit={(e) => submit(e)}>
      <For each={fields}>
        {([name, props]) => {
          switch (props.type) {
            case "digits":
              return (
                <>
                  <DigitInput
                    digitCount={props.digitCount}
                    signal={props.signal}
                  />
                  <Show when={!!props.errSignal[0]()}>
                    <p class={styles.form_error_text}>{props.errSignal[0]()}</p>
                  </Show>
                </>
              );

            default:
              return (
                <>
                  <TextInput
                    name={name}
                    type={props.type}
                    placeholder={props.placeholder ?? "Skriv här..."}
                    signal={props.signal}
                  />
                  <Show when={!!props.errSignal[0]()}>
                    <p class={styles.form_error_text}>{props.errSignal[0]()}</p>
                  </Show>
                </>
              );
          }
        }}
      </For>
      <Button type="submit">{submitElement.text}</Button>
    </form>
  );
}

/**
 * @param {FieldsArray} fields
 */
function runErrorChecks(fields) {
  let hadErrors = false;

  fields.forEach(([_name, { signal, errSignal, errors }]) => {
    if (!Array.isArray(errors)) return;

    const results = runIndividualFieldErrorChecks(
      signal[0](),
      errSignal[1],
      errors,
    );
    const didError = results.some((hadError) => hadError === true);
    hadErrors = didError;
  });

  return hadErrors;
}

function runIndividualFieldErrorChecks(current, setError, errorCases) {
  return errorCases.map(([evaluate, msg]) => {
    const isError = evaluate(current);

    if (isError) {
      setError(msg);
      return true;
    } else {
      return false;
    }
  });
}
