<script lang="ts">
    import MainLayout from "$lib/layouts/MainLayout.svelte";
    import CreationButtons from "$lib/components/CreationButtons.svelte";
    import NavigationButtons from "$lib/components/NavigationButtons.svelte";
    import AcceptCancelButtons from "$lib/components/AcceptCancelButtons.svelte";
    import {goto} from "$app/navigation";
    import {invoke} from "@tauri-apps/api/tauri";

    const formData: {
      name: string,
      transaction_type: string
    } = {
      name: '',
      transaction_type: ''
    }

    async function accept() {
      const val = await invoke('create_transaction_source', { formData: JSON.stringify(formData) });
      console.log(val);
    }
</script>

<MainLayout>
  <div slot="header" class="flex">
    <div class="flex">
      <NavigationButtons/>
    </div>
    <div class="px-2 flex justify-center flex-1">
      <h1 class="text-2xl">Create Transaction Source</h1>
    </div>
    <div class="flex">
      <AcceptCancelButtons on:accept={async () => await accept()} on:cancel={() => goto('/')} />
    </div>
  </div>

  <div slot="body" class="px-1">
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

      <div class="pt-2 flex justify-center">
<!--        <button class="btn btn-sm btn-primary" on:click={add}>Add</button>-->
      </div>
  </div>

  <CreationButtons slot="footer"/>
</MainLayout>