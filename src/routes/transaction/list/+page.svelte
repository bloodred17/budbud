<script lang="ts">
  import NavigationButtons from "$lib/components/NavigationButtons.svelte";
  import CreationButtons from "$lib/components/CreationButtons.svelte";
  import MainLayout from "$lib/layouts/MainLayout.svelte";
  import {onMount} from "svelte";
  import {invoke} from "@tauri-apps/api/tauri";
  import {notificationStore, Notify} from "$lib/components/notification/notification.store";
  import TransactionSelector from "$lib/components/TransactionSelector.svelte";
  import dayjs, {type Dayjs} from "dayjs";
  import Datepicker from "$lib/ui/datepicker/Datepicker.svelte";

  let tableData = [];

  let total_income: number = 0;
  let total_expense: number = 0;

  let startDate: Dayjs = dayjs();
  let endDate: Dayjs = dayjs().add(1, 'day');

  onMount(() => {
    get_data();
  });

  $: console.log(total_income);
  $: console.log(total_expense);

  $: console.log(startDate);
  $: console.log(endDate);

  const get_data = () => {
    invoke<string>('list_transactions')
      .then((response: string) => {
        tableData = JSON.parse(response).data;
        total_income = get_total_income();
        total_expense = get_total_expense();
      })
  }

  const delete_transaction = (id: string) => {
    invoke('delete_transaction', {id})
      .then(() => {
        get_data();
        notificationStore.update(() => {
          return new Notify({
            message: `${id} deleted`,
          })
        })
      })
  };

  const get_total_income = () => {
    return tableData?.reduce((acc: number, next) => {
      if (next?.transaction_source?.transaction_type == 'Income') {
        return acc + next?.amount;
      }
      return acc;
    }, 0);
  }

  const get_total_expense = () => {
    return tableData?.reduce((acc: number, next) => {
      if (next?.transaction_source?.transaction_type == 'Expense') {
        return acc + next?.amount;
      }
      return acc;
    }, 0);
  }

  const toggleExpander = (id: string) => {
    const row = document?.getElementById(id);
    console.log(row?.classList?.entries());
    if (row?.classList?.contains('hidden')) {
      row?.classList?.remove('hidden');
    } else {
      row?.classList?.add('hidden');
    }
  }

  const checkEndDate = (date: string) => {
    const _date = dayjs(date);
    return _date.diff(startDate, 'hour') > 20;
  }

</script>

<MainLayout>
  <div slot="header" class="flex">
    <div class="flex">
      <NavigationButtons disableBack={true} disableForward={true}/>
    </div>
    <div class="px-2 flex justify-center flex-1">
      <h1 class="text-xl">Transactions</h1>
    </div>
    <div class="flex">
<!--      <div class="invisible">-->
<!--        <AcceptCancelButtons on:accept={async () => await accept()} on:cancel={() => goto('/')} />-->
<!--      </div>-->
      <TransactionSelector/>
    </div>
  </div>


  <div slot="body">
    <div class="flex justify-center p-2 gap-2 bg-base-200 rounded-lg">
      <Datepicker
          label="Start Date"
          selectedDate={startDate.format('YYYY-MM-DD')}
          on:selectedDate={(event) => startDate = dayjs(event?.detail)}
      />
      <Datepicker
          label="End Date"
          selectedDate={endDate.format('YYYY-MM-DD')}
          constraints={[
              {
                  fn: checkEndDate,
                  errorMsg: "End date should be greater than Start date"
              }
          ]}
          on:selectedDate={(event) => endDate = dayjs(event?.detail)}
      />
    </div>
    <div class="overflow-x-auto">
      <table class="table">
        <!-- head -->
        {#if tableData.length > 0}
          <thead>
          <tr>
            <th></th>
            <th>Name</th>
            <th>Amount</th>
            <th>Transaction Source</th>
            <th>Date</th>
            <th></th>
          </tr>
          </thead>
        {/if}

        <tbody>
        {#each tableData as row, index (row?.id?.id?.String)}
          <tr id={row?.id?.id?.String} class="hover:bg-primary hover:text-primary-content" on:click={() => toggleExpander("expand-" + row?.id?.id?.String)}>
            <th>{index + 1}</th>
            <td on:click={() => console.log(row)}>{row?.name}</td>
            <td>{row?.amount}</td>
            <td>
              {#if row?.transaction_source?.transaction_type === 'Expense'}
              <div class="badge badge-error gap-2">
                <span>{row?.transaction_source?.name}</span>
              </div>
              {:else if row?.transaction_source?.transaction_type === 'Income'}
              <div class="badge badge-success gap-2">
                <span>{row?.transaction_source?.name}</span>
              </div>
              {/if}
            </td>
            <td> {dayjs(row?.start_date)?.format('DD MMM, HH:mm')} </td>
            <td>
              <div class="join">
                <!-- Edit-->
                <button class="btn btn-xs join-item">
                  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L10.582 16.07a4.5 4.5 0 01-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 011.13-1.897l8.932-8.931zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0115.75 21H5.25A2.25 2.25 0 013 18.75V8.25A2.25 2.25 0 015.25 6H10" />
                  </svg>
                </button>
                <!-- Delete-->
                <button class="btn btn-xs join-item hover:btn-error" on:click={() => delete_transaction(row?.id?.id?.String)}>
                  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>
            </td>
          </tr>
          <tr id={"expand-" + row?.id?.id?.String} class="hidden">

            <th colspan="6">
              <div class="mockup-code">
                <pre data-prefix="$"><code>run {row?.transaction_source?.name}</code></pre>
                <pre data-prefix=">" class="text-warning"><code>{JSON.stringify(row, undefined, 2)}</code></pre>
                <pre data-prefix=">" class="text-success"><code>Done!</code></pre>
              </div>
            </th>

          </tr>
        {:else}
          <div class="hero">
            <div class="hero-content text-center">
              <div class="max-w-md">
                <h1 class="text-3xl font-bold">Oops!</h1>
                <p class="py-6">You currently have no transactions.</p>
                <a class="btn btn-sm btn-primary" href="/transaction/create">
                  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                  </svg>

                  <span class="ml-2">Add Now</span>
                </a>
              </div>
            </div>
          </div>
        {/each}
        </tbody>
      </table>

      {#if tableData.length > 0}
        <div class="flex justify-center mt-4">
          <div class="stats shadow bg-base-300">

            <div class="stat">
              <div class="stat-figure text-secondary">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 18L9 11.25l4.306 4.307a11.95 11.95 0 015.814-5.519l2.74-1.22m0 0l-5.94-2.28m5.94 2.28l-2.28 5.941" />
                </svg>
              </div>
              <div class="stat-title">Income</div>
              <div class="stat-value">{total_income}</div>
              <div class="stat-desc">Jan 1st - Feb 1st</div>
            </div>

            <div class="stat">
              <div class="stat-figure text-secondary">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 6L9 12.75l4.286-4.286a11.948 11.948 0 014.306 6.43l.776 2.898m0 0l3.182-5.511m-3.182 5.51l-5.511-3.181" />
                </svg>
              </div>
              <div class="stat-title">Expense</div>
              <div class="stat-value">{total_expense}</div>
              <div class="stat-desc">Jan 1st - Feb 1st</div>
            </div>

          </div>
        </div>
      {/if}
    </div>

  </div>

  <CreationButtons slot="footer"/>
</MainLayout>