<script lang="ts">
  import MainLayout from "$lib/layouts/MainLayout.svelte";
  import CreationButtons from "$lib/components/CreationButtons.svelte";
  import NavigationButtons from "$lib/components/NavigationButtons.svelte";
  import AcceptCancelButtons from "$lib/components/AcceptCancelButtons.svelte";
  import {goto} from "$app/navigation";
  import {invoke} from "@tauri-apps/api/tauri";
  import TestDb from "$lib/components/TestDb.svelte";
  import {
    Notify as Notify,
    notificationStore,
    NotificationType
  } from "$lib/components/notification/notification.store";

  // let message = '';
    // onMount(() => {
    //   listen('rs2js', (event) => {
    //     console.log("rs2js: ");
    //     console.log(event);
    //   })
    // })
    //
    // async function greet(){
    //   message = formData?.name;
    //   await invoke("js2rs", { message })
    // }

    const formData: {
      name: string,
      transaction_type: string
    } = {
      name: '',
      transaction_type: ''
    }

    async function accept() {
      invoke<string>('create_transaction_source', { formData: JSON.stringify(formData) })
        .then((value: string) =>
          notificationStore.update(() => {
            return new Notify({
              notificationType: NotificationType.Success,
              message: value,
              timeout: 5000,
            });
          })
        ).catch((e) =>
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

<TestDb/>
<MainLayout>
  <div slot="header" class="flex">
    <div class="flex">
      <NavigationButtons disableBack={true} disableForward={true}/>
    </div>
    <div class="px-2 flex justify-center flex-1">
      <h1 class="text-xl">Create Transaction Source</h1>
    </div>
    <div class="flex">
      <AcceptCancelButtons on:accept={async () => await accept()} on:cancel={() => goto('/')} />
    </div>
  </div>

  <div slot="body" class="px-1 overflow-y-hidden">
    <div class="form-control w-full max-w-xs">
      <label class="label" for="transaction_name">
        <span class="label-text">What do you want to call this source?</span>
      </label>
      <input id="transaction_name" type="text" placeholder="Type here" class="input input-bordered input-sm w-full max-w-xs" bind:value={formData['name']} />
    </div>

    <div class="form-control w-full max-w-xs mt-2">
      <label class="label" for="transaction_type">
        <span class="label-text">Is it an Income or and Expense?</span>
      </label>
      <select id="transaction_type" class="select select-bordered select-sm" bind:value={formData['transaction_type']}>
        <option disabled selected>Pick one</option>
        <option>Income</option>
        <option>Expense</option>
      </select>
    </div>

  </div>

  <CreationButtons slot="footer"/>
</MainLayout>