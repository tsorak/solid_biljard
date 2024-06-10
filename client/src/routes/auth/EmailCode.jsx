import styles from "./EmailCode/EmailCode.module.sass";
import EmailCodeForm from "../../components/EmailCodeForm.jsx";

export default function EmailCode() {
  return (
    <main class={styles.main}>
      <EmailCodeForm />
    </main>
  );
}
