import dayjs from "dayjs";

export interface DisplayDate {
  dateStr: string,
  week: number,
  dateForCurrentMonth: boolean,
  dayDate: string,
}

export const displayMonths = new Map([
  [0, 'January'],
  [1, 'February'],
  [2, 'March'],
  [3, 'April'],
  [4, 'May'],
  [5, 'June'],
  [6, 'July'],
  [7, 'August'],
  [8, 'September'],
  [9, 'October'],
  [10, 'November'],
  [11, 'December'],
]);

const setDisplayData = (
  dateStr: string,
  week: number,
  dateForCurrentMonth: boolean = false,
): DisplayDate => {
  return {
    dateStr,
    week,
    dateForCurrentMonth,
    dayDate: dateStr.substring(8),
  }
}

const normalize = (i: number) => (i < 10) ? '0' + i : i;


export  const getDatepickerDisplay = (month: number, year: number) => {
  const daysInMonth = dayjs().month(month).daysInMonth();

  const displayDates = [];
  for (let i = 1; i <= daysInMonth; i++) {
    const currentDateStr = `${year}-${normalize(month + 1)}-${normalize(i)}`;
    displayDates.push(
      setDisplayData(
        currentDateStr,
        dayjs(currentDateStr).day(),
        true,
      )
    );
  }

  // list should always start with 0
  const {week: startWeek} = displayDates?.at(0) as DisplayDate;
  if (startWeek !== 0) {
    const daysToAddInBeginning = startWeek;
    let yearToUse = year;
    let previousMonth = month - 1;
    if (previousMonth < 0) {
      previousMonth += 12;
      yearToUse -= 1;
    }
    const daysInPreviousMonth = dayjs().month(previousMonth).daysInMonth();
    for (let i = daysInPreviousMonth, j = 0; j < daysToAddInBeginning; i--, j++) {
      const currentDateStr = `${yearToUse}-${normalize(previousMonth+1)}-${normalize(i)}`;
      displayDates.unshift(
        setDisplayData(
          currentDateStr,
          dayjs(currentDateStr).day(),
        )
      );
    }
  }

  if (displayDates?.length < 42) {
    const daysToAddInEnd = 42 - displayDates.length;
    let yearToUse = year;
    let nextMonth = month + 1;
    if (nextMonth > 11) {
      nextMonth -= 12;
      yearToUse += 1;
    }
    for (let j = 0; j < daysToAddInEnd; j++) {
      const i = j + 1;
      const currentDateStr = `${yearToUse}-${normalize(nextMonth+1)}-${normalize(i)}`;
      displayDates.push(
        setDisplayData(
          currentDateStr,
          dayjs(currentDateStr).day(),
        )
      );
    }
  }

  return displayDates;
}
