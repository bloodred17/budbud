<script lang="ts">

  import {createEventDispatcher, onMount} from "svelte";
  import {type DisplayDate, displayMonths, getDatepickerDisplay} from "$lib/ui/datepicker/datepicker";
  import dayjs, {Dayjs} from "dayjs";
  import Dropdown from "$lib/ui/dropdown/Dropdown.svelte";

  const dispatch = createEventDispatcher();
  export let selectedDate: string;

  let selectedDateStr: string;
  $: dispatch('selectedDate', selectedDateStr);

  let selectedMonth: number;
  let selectedYear: number;
  $: {
    if (selectedMonth && selectedYear) {
      displayDates = getDatepickerDisplay(selectedMonth, selectedYear);
      console.log(displayDates)
      setYearOptions();
    }
  }

  let yearOptions: number[] = [];
  let currentDate: Dayjs;

  let displayDates: DisplayDate[] = [];
  onMount(() => {
    currentDate = (selectedDate) ? dayjs(selectedDate) : dayjs();
    selectedDateStr = currentDate?.format('YYYY-MM-DD');
    selectedMonth = currentDate?.month();
    selectedYear = currentDate?.year();
  });

  const dateSelect = (displayDate: DisplayDate) => {
    if (displayDate?.dateForCurrentMonth) {
      selectedDateStr = displayDate?.dateStr;
    }
  }

  const nextMonth = () => {
    selectedMonth += 1;
    if (selectedMonth > 11) {
      selectedMonth = 12 - selectedMonth;
      selectedYear += 1;
    }
  }
  const previousMonth = () => {
    selectedMonth -= 1;
    if (selectedMonth < 0) {
      selectedMonth = 12 + selectedMonth;
      selectedYear -= 1;
    }
  }
  const setYearOptions = () => {
    yearOptions = [];
    for (let i = 0; i < 3; i++) {
      yearOptions.push(selectedYear + i);
    }
    for (let i = 1; i < 3; i++) {
      yearOptions.unshift(selectedYear - i);
    }
  }
</script>

<Dropdown modify="w-full max-w-xs bg-base-100/90 shadow">
  <input slot="trigger" tabindex="0" type="text" placeholder="Type here" class="input input-bordered input-xs w-full" />
  <div slot="content" class="pt-2 rounded-lg text-center lg:col-start-8 lg:col-end-13 lg:row-start-1 xl:col-start-9">
    <div class="flex justify-between items-center mx-2">
      <button type="button"
              class="-m-1.5 flex flex-none items-center justify-center p-1.5 text-gray-400 hover:bg-primary rounded-full"
              on:click={() => previousMonth()}
      >
        <span class="sr-only">Previous month</span>
        <!-- Heroicon name: solid/chevron-left -->
        <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
          <path fill-rule="evenodd" d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z" clip-rule="evenodd" />
        </svg>
      </button>

      <Dropdown modify="bg-base-100 shadow max-w-xs w-fit h-60 overflow-y-auto">
        <div slot="trigger" class="font-semibold btn btn-sm">
          <span>{displayMonths.get(selectedMonth)}</span>
        </div>

        <ul slot="content">
          {#each displayMonths as [value, month]}
            <li>
              <div on:click={() => selectedMonth = value}
                   class:bg-primary={selectedMonth == value}
                   class:text-primary-content={selectedMonth == value}>
                {month}
              </div>
            </li>
          {/each}
        </ul>
      </Dropdown>

      <Dropdown modify="bg-base-100 shadow max-w-xs w-fit overflow-y-auto">
        <div slot="trigger" class="font-semibold btn btn-sm">
          <span tabindex="1">{selectedYear}</span>
        </div>

        <ul slot="content">
          {#each yearOptions as yearOption}
            <li><div on:click={() => selectedYear = yearOption}
                     class:bg-primary={selectedYear == yearOption}
                     class:text-primary-content={selectedYear == yearOption}
            >{yearOption}</div></li>
          {/each}
        </ul>
      </Dropdown>

      <button type="button"
              class="-m-1.5 flex flex-none items-center justify-center p-1.5 text-gray-400 hover:bg-primary rounded-full"
              on:click={() => nextMonth()}
      >
        <span class="sr-only">Next month</span>
        <!-- Heroicon name: solid/chevron-right -->
        <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
          <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
        </svg>
      </button>
    </div>

    <div class="mt-6 grid grid-cols-7 text-xs leading-6 text-gray-500">
      <div>S</div>
      <div>M</div>
      <div>T</div>
      <div>W</div>
      <div>T</div>
      <div>F</div>
      <div>S</div>
    </div>

    <div class="isolate mt-2 grid grid-cols-7 gap-px rounded-lg bg-gray-200 text-sm shadow ring-1 ring-gray-200">
      <!--
        Always include: "py-1.5 hover:bg-gray-100 focus:z-10"
        Is current month, include: "bg-base"
        Is not current month, include: "bg-gray-50"
        Is selected or is today, include: "font-semibold"
        Is selected, include: "text-white"
        Is not selected, is not today, and is current month, include: "text-gray-900"
        Is not selected, is not today, and is not current month, include: "text-gray-400"
        Is today and is not selected, include: "text-indigo-600"

        Top left day, include: "rounded-tl-lg"
        Top right day, include: "rounded-tr-lg"
        Bottom left day, include: "rounded-bl-lg"
        Bottom right day, include: "rounded-br-lg"
      -->
      <!--
        Always include: "mx-auto flex h-7 w-7 items-center justify-center rounded-full"
        Is selected and is today, include: "bg-indigo-600"
        Is selected and is not today, include: "bg-gray-900"
      -->

      {#each displayDates as displayDate, index}
        <button type="button"
                class="py-1.5 text-gray-400 hover:bg-gray-100 z-10"
                class:rounded-tl-lg={index === 0}
                class:rounded-tr-lg={index === 6}
                class:rounded-bl-lg={index === 35}
                class:rounded-br-lg={index === 41}
                class:bg-base-200={!displayDate?.dateForCurrentMonth}
                class:text-gray-400={!displayDate?.dateForCurrentMonth}
                class:bg-base-100={displayDate?.dateForCurrentMonth}
                class:text-gray-900={displayDate?.dateForCurrentMonth}
                on:click={() => dateSelect(displayDate)}
        >
          <time datetime={displayDate?.dateStr}
                class="mx-auto flex h-7 w-7 items-center justify-center rounded-full"
                class:bg-primary={selectedDateStr === displayDate?.dateStr}
                class:text-primary-content={selectedDateStr === displayDate?.dateStr}
          >
            {+displayDate.dayDate}
          </time>
        </button>
      {/each}
    </div>
  </div>
</Dropdown>
