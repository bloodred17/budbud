<script lang="ts">
  import {goto} from "$app/navigation";
  import NavigationButtons from "$lib/components/NavigationButtons.svelte";
  import CreationButtons from "$lib/components/CreationButtons.svelte";
  import MainLayout from "$lib/layouts/MainLayout.svelte";
  import AcceptCancelButtons from "$lib/components/AcceptCancelButtons.svelte";
  import {invoke} from "@tauri-apps/api/tauri";
  import {notificationStore, NotificationType, Notify} from "$lib/components/notification/notification.store";

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

  <div slot="body" class="px-1 overflow-y-hidden">
    <div class="form-control w-full max-w-xs">
      <label class="label" for="transaction_name">
        <span class="label-text">What is this transaction for?</span>
      </label>
      <input id="transaction_name" type="text" placeholder="Type here" class="input input-bordered input-sm w-full max-w-xs" bind:value={formData['name']} />
    </div>

    <div class="form-control w-full max-w-xs">
      <label class="label" for="transaction_source">
        <span class="label-text">Set Payment source: </span>
      </label>
      <input id="transaction_source" type="text" placeholder="Type here" class="input input-bordered input-sm w-full max-w-xs" bind:value={formData['transaction_source']} />
    </div>

<!--    <div class="form-control w-full max-w-xs mt-2">-->
<!--      <label class="label" for="transaction_type">-->
<!--        <span class="label-text">Is it an Income or and Expense?</span>-->
<!--      </label>-->
<!--      <select id="transaction_type" class="select select-bordered select-sm" bind:value={formData['transaction_type']}>-->
<!--        <option disabled selected>Pick one</option>-->
<!--        <option>Income</option>-->
<!--        <option>Expense</option>-->
<!--      </select>-->
<!--    </div>-->

  </div>

  <CreationButtons slot="footer"/>
</MainLayout>