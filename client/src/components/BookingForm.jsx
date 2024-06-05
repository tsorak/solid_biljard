import { createContext, For, useContext } from "solid-js";

import styles from "./BookingForm/BookingForm.module.sass";

import DateSelector from "./BookingForm/DateSelector.js";

/**
 * @type {DateSelector}
 */
let Context = null;

export default function BookingForm() {
  const dateSelector = new DateSelector();
  Context = createContext(dateSelector);

  return (
    <Context.Provider value={dateSelector}>
      <div class={styles.booking_form}>
        <YearSelector />
        <MonthSelector />
        <DayGrid />
        <p>Valt datum: {dateSelector.selectedDate()}</p>
      </div>
    </Context.Provider>
  );
}

function MonthSelector() {
  /**
   * @type {DateSelector}
   */
  const dateSelector = useContext(Context);

  return (
    <div class={styles.month_selector}>
      <button onClick={() => dateSelector.prevMonth()}>{"<"}</button>
      <h3>{dateSelector.getMonthName()}</h3>
      <button onClick={() => dateSelector.nextMonth()}>{">"}</button>
    </div>
  );
}

function DayGrid() {
  /**
   * @type {DateSelector}
   */
  const dateSelector = useContext(Context);

  const booked_style = (bookedBy) => (bookedBy ? styles.booked_day : "");
  const selected_style = (
    day,
  ) => (day == dateSelector.day() ? styles.selected_day : "");

  return (
    <div class={styles.days_wrapper}>
      <div class={styles.day_grid}>
        <For each={dateSelector.days()}>
          {({ day, bookedBy }, _i) => (
            <button
              onClick={() => dateSelector.setDay(day)}
              disabled={bookedBy != null}
              class={`${booked_style(bookedBy)} ${selected_style(day)}`}
            >
              {day}
            </button>
          )}
        </For>
      </div>
    </div>
  );
}

function YearSelector() {
  /**
   * @type {DateSelector}
   */
  const dateSelector = useContext(Context);

  return (
    <div class={styles.year_selector}>
      <button onClick={() => dateSelector.prevYear()}>{"<"}</button>
      <h3>{dateSelector.year()}</h3>
      <button onClick={() => dateSelector.nextYear()}>{">"}</button>
    </div>
  );
}
