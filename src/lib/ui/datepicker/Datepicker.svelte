<script lang="ts">

  import {createEventDispatcher, onMount} from "svelte";
  import {type DisplayDate, displayMonths, getDatepickerDisplay} from "$lib/ui/datepicker/datepicker";
  import dayjs, {Dayjs} from "dayjs";
  import Dropdown from "$lib/ui/dropdown/Dropdown.svelte";

  const dispatch = createEventDispatcher();

  export let selectedDate: string;
  export let label: string;
  export let placeholder: string;

  let selectedMonth: number;
  let selectedYear: number;
  $: {
    if (selectedMonth > -1 && selectedYear) {
      displayDates = getDatepickerDisplay(selectedMonth, selectedYear);
      console.log(displayDates)
      setYearOptions();
    }
  }

  let yearOptions: number[] = [];
  let currentDate: Dayjs;

  let displayDates: DisplayDate[] = [];
  onMount(() => {
    setCurrent(selectedDate);
  });

  const dateSelect = (displayDate: DisplayDate) => {
    if (displayDate?.dateForCurrentMonth) {
      selectedDate = displayDate?.dateStr;
      dispatch('selectedDate', selectedDate);
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

  const setCurrent = (selected_date?: string) => {
    currentDate = (selected_date) ? dayjs(selected_date) : dayjs();
    selectedDate = currentDate?.format('YYYY-MM-DD');
    selectedMonth = currentDate?.month();
    selectedYear = currentDate?.year();
  }

  let monthSelector;
  let yearSelector;
</script>

<Dropdown modify="w-full max-w-xs bg-base-100/90 shadow">
  <div slot="trigger" class="form-control w-full max-w-xs">
    <label class="label" for="transaction_name">
      <span class="label-text">{label}</span>
    </label>
    <input type="text" placeholder={placeholder} class="input input-bordered input-xs w-full" bind:value={selectedDate} />
  </div>

  <div slot="content" class="pt-2 rounded-lg text-center lg:col-start-8 lg:col-end-13 lg:row-start-1 xl:col-start-9">
    <div class="flex justify-between items-center mx-2">
      <button type="button"
              class="-m-1.5 flex flex-none items-center justify-center p-1.5 text-gray-400 hover:bg-primary hover:text-primary-content rounded-full"
              on:click={() => previousMonth()}
      >
        <span class="sr-only">Previous month</span>
        <!-- Heroicon name: solid/chevron-left -->
        <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
          <path fill-rule="evenodd" d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z" clip-rule="evenodd" />
        </svg>
      </button>

      <Dropdown bind:this={monthSelector} modify="bg-base-100 shadow max-w-xs w-fit h-60 overflow-y-auto">
        <div slot="trigger" class="font-semibold btn btn-sm">
          <span>{displayMonths.get(selectedMonth)}</span>
        </div>

        <ul slot="content">
          {#each displayMonths as [value, month]}
            <li>
              <div on:click={() => {
                  selectedMonth = value;
                  monthSelector?.close();
              }}
                   class:bg-primary={selectedMonth == value}
                   class:text-primary-content={selectedMonth == value}>
                {month}
              </div>
            </li>
          {/each}
        </ul>
      </Dropdown>

      <button class="btn btn-sm" on:click={() => setCurrent()}>
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
          <path stroke-linecap="round" stroke-linejoin="round" d="M9 15L3 9m0 0l6-6M3 9h12a6 6 0 010 12h-3" />
        </svg>
      </button>

      <Dropdown bind:this={yearSelector} modify="bg-base-100 shadow max-w-xs w-fit overflow-y-auto">
        <div slot="trigger" class="font-semibold btn btn-sm">
          <span>{selectedYear}</span>
        </div>

        <ul slot="content">
          {#each yearOptions as yearOption}
            <li>
              <div on:click={() => {
                  selectedYear = yearOption;
                  yearSelector?.close();
              }}
                 class:bg-primary={selectedYear == yearOption}
                 class:text-primary-content={selectedYear == yearOption}
              >{yearOption}</div>
            </li>
          {/each}
        </ul>
      </Dropdown>

      <button type="button"
              class="-m-1.5 flex flex-none items-center justify-center p-1.5 text-gray-400 hover:bg-primary hover:text-primary-content rounded-full"
              on:click={() => nextMonth()}
      >
        <span class="sr-only">Next month</span>
        <!-- Heroicon name: solid/chevron-right -->
        <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
          <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
        </svg>
      </button>
    </div>

    <div class="mt-6 grid grid-cols-7 text-xs leading-6 text-gray-500 bg-base-100 rounded-xl">
      <div>S</div>
      <div>M</div>
      <div>T</div>
      <div>W</div>
      <div>T</div>
      <div>F</div>
      <div>S</div>
    </div>

    <div class="isolate mt-2 grid grid-cols-7 gap-px rounded-lg bg-gray-200 text-sm shadow ring-1 ring-gray-200">
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
                class:bg-primary={selectedDate === displayDate?.dateStr}
                class:text-primary-content={selectedDate === displayDate?.dateStr}
          >
            {+displayDate.dayDate}
          </time>
        </button>
      {/each}
    </div>
  </div>
</Dropdown>
