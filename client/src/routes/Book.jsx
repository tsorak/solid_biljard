import BookingForm from "../components/BookingForm.jsx";
import styles from "./Book/Book.module.sass";
// import DateSelector from "../components/BookingForm/DateSelector.js";

export default function Book() {
  return (
    <main class={styles.main}>
      <BookingForm />
    </main>
  );
}
