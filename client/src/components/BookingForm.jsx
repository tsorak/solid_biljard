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

  const days = () => {
    const daysInMonth = dateSelector.daysInMonth();

    const list = [];
    for (let index = 0; index < daysInMonth; index++) {
      list[index] = index + 1;
    }
    return list;
  };

  return (
    <div class={styles.days_wrapper}>
      <div class={styles.day_grid}>
        <For each={days()}>
          {(v, _i) => (
            <button
              onClick={() => {
                dateSelector.setDay(v);
              }}
            >
              {v}
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
