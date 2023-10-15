<script lang="ts">
  import {goto} from "$app/navigation";
  import NavigationButtons from "$lib/components/NavigationButtons.svelte";
  import CreationButtons from "$lib/components/CreationButtons.svelte";
  import MainLayout from "$lib/layouts/MainLayout.svelte";
  import AcceptCancelButtons from "$lib/components/AcceptCancelButtons.svelte";
  import {invoke} from "@tauri-apps/api/tauri";
  import {notificationStore, NotificationType, Notify} from "$lib/components/notification/notification.store";
  import Autocomplete from "$lib/ui/Autocomplete.svelte";
  import {onMount} from "svelte";
  import {type TransactionSource, transactionSourceStore} from "$lib/stores/transaction-source.store";
  import type {SelectionOption} from "$lib/types/ui.interface";


  let transactionSources: TransactionSource[] = [];
  let options: SelectionOption[] = [];

  onMount(() => {
    transactionSourceStore.subscribe((_transactionSources) => {
      transactionSources = _transactionSources;
      options = transactionSources.map((source) => ({
        display: source?.name,
        value: source?.id?.id?.String || null,
      }))
    });
  })

  interface TransactionForm {
    name?: string,
    transaction_source: string,
    amount: number,

    // if is_recurring true then cron string is required
    // is_recurring: bool,
    // start_date: Option<DateTime<Utc>>,
    // end_date: Option<DateTime<Utc>>,
    // chron_expression: Option<String>,
  }

  const formData: TransactionForm = {
    name: undefined,
    transaction_source: '',
    amount: 0,
  }

  async function accept() {
    // console.log(formData);

    invoke<string>('create_transaction', { formData: JSON.stringify(formData) })
      .then((value: string) =>
        // console.log(value)
        notificationStore.update(() => {
          return new Notify({
            notificationType: NotificationType.Success,
            message: value,
            timeout: 5000,
          });
        })
      ).catch((e) =>
      // console.log(e)
      notificationStore.update(() => {
        return new Notify({
          notificationType: NotificationType.Error,
          message: e?.message,
          timeout: 5000,
        });
      })
    )
  }
</script>

<MainLayout>
  <div slot="header" class="flex">
    <div class="flex">
      <NavigationButtons disableBack={true} disableForward={true}/>
    </div>
    <div class="px-2 flex justify-center flex-1">
      <h1 class="text-xl">Create Transaction</h1>
    </div>
    <div class="flex">
      <AcceptCancelButtons on:accept={async () => await accept()} on:cancel={() => goto('/')} />
    </div>
  </div>

  <div slot="body" class="px-1 min-h-full">
    <div class="form-control w-full max-w-xs">
      <label class="label" for="transaction_name">
        <span class="label-text">What is this transaction for?</span>
      </label>
      <input id="transaction_name" type="text" placeholder="Type here" class="input input-bordered input-sm w-full max-w-xs" bind:value={formData['name']} />
    </div>

    <div class="form-control w-full max-w-xs">
      <label class="label" for="transaction_amount">
        <span class="label-text">Source: </span>
      </label>
      <Autocomplete
          items={options}
          selectedItem="",
          on:value={(event) => formData['transaction_source'] = event?.detail?.value}
      />
    </div>


    <div class="form-control w-full max-w-xs">
      <label class="label" for="transaction_amount">
        <span class="label-text">Amount: </span>
      </label>
      <input id="transaction_amount" type="number" placeholder="$" class="input input-bordered input-sm w-full max-w-xs" bind:value={formData['amount']} />
    </div>



  </div>

  <CreationButtons slot="footer"/>
</MainLayout>