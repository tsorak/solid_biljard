import { createSignal } from "solid-js";

const daysPerMonth = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

const months = [
  "Januari",
  "Februari",
  "Mars",
  "April",
  "Maj",
  "Juni",
  "Juli",
  "Augusti",
  "September",
  "Oktober",
  "November",
  "December",
];

/**
 * @class DateSelector
 */
export default class DateSelector {
  /**
   * @type {() => number}
   */
  year;

  /**
   * @type {(function(function(number): number): void) | function(number): void}
   */
  setYear;
  /**
   * @type {() => number}
   */
  month;
  /**
   * @type {(function(function(number): number): void) | function(number): void}
   */
  setMonth;
  /**
   * @type {() => number}
   */
  day;
  /**
   * @type {(function(function(number): number): void) | function(number): void}
   */
  _setDay;

  /**
   * @type {() => string}
   */
  selectedDate;
  /**
   * @type {(function(function(string): string): void) | function(string): void}
   */
  setSelectedDate;

  /**
   * @type {Date}
   */
  currentDate;
  /**
   * @type {number}
   */
  currentYear;

  /**
   * @type {() => number}
   */
  daysInMonth;

  /**
   * @type {(function(function(number): number): void) | function(number): void}
   */
  setDaysInMonth;

  constructor() {
    this.currentDate = new Date();
    this.currentYear = this.currentDate.getUTCFullYear();

    const [year, setYear] = createSignal(this.currentYear);
    const [month, setMonth] = createSignal(0);
    const [daysInMonth, setDaysInMonth] = createSignal(31);
    const [day, setDay] = createSignal(1);
    const [selectedDate, setSelectedDate] = createSignal(
      null,
    );

    this.year = year;
    this.setYear = setYear;

    this.month = month;
    this.setMonth = setMonth;

    this.daysInMonth = daysInMonth;
    this.setDaysInMonth = setDaysInMonth;

    this.day = day;
    this._setDay = setDay;

    this.selectedDate = selectedDate;
    this.setSelectedDate = setSelectedDate;

    //

    this.updateSelectedDate();
  }

  updateSelectedDate() {
    const day = this.day();
    const month = this.month();
    const year = this.year();

    this.setSelectedDate(`${day}-${month}-${year}`);
  }

  nextYear() {
    this.setYear((p) => {
      switch (p) {
        case 4000:
          return 4000;
        default:
          return p + 1;
      }
    });
    this.updateSelectedDate();
  }

  prevYear() {
    this.setYear((p) => {
      // if (p == this.currentYear) {
      //   return p;
      // } else {
      //   return p - 1;
      // }
      return p - 1;
    });
    this.updateSelectedDate();
  }

  getMonthName() {
    return months[this.month()];
  }

  nextMonth() {
    this.setMonth((p) => {
      let newMonth;

      switch (p) {
        case 11:
          this.nextYear();
          newMonth = 0;
          break;
        default:
          newMonth = p + 1;
          break;
      }

      const newMonthsDayCount = this.getMonthsDayCount(newMonth);
      this.setDaysInMonth(newMonthsDayCount);

      return newMonth;
    });
    this.updateSelectedDate();
  }

  prevMonth() {
    this.setMonth((p) => {
      let newMonth;

      switch (p) {
        case 0:
          this.prevYear();
          newMonth = 11;
          break;
        default:
          newMonth = p - 1;
          break;
      }

      const newMonthsDayCount = this.getMonthsDayCount(newMonth);
      this.setDaysInMonth(newMonthsDayCount);

      return newMonth;
    });
    this.updateSelectedDate();
  }

  setDay(day) {
    this._setDay(day);
    this.updateSelectedDate();
  }

  getMonthsDayCount(month) {
    return daysPerMonth[month];
  }
}
