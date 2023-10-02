<script lang="ts">
  import {invoke} from "@tauri-apps/api/tauri";

  const formData: any = {}

  async function add() {
    await invoke('create_transaction', { form_data: formData })
  }

</script>

<div class="px-1">
  <div class="form-control w-full max-w-xs">
    <label class="label">
      <span class="label-text">What do you want to call this source?</span>
    </label>
    <input type="text" placeholder="Type here" class="input input-bordered input-sm w-full max-w-xs" bind:value={formData['name']} />
  </div>

  <div class="form-control w-full max-w-xs mt-2">
    <label class="label">
      <span class="label-text">Is it an Income or and Expense?</span>
    </label>
    <select class="select select-bordered select-sm" bind:value={formData['transaction_type']}>
      <option disabled selected>Pick one</option>
      <option>Income</option>
      <option>Expense</option>
    </select>
  </div>

  <div class="pt-2 flex justify-center">
    <button class="btn btn-sm btn-primary" on:click={add}>Add</button>
  </div>
</div>