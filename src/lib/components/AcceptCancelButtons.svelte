<script lang="ts">
  import {createEventDispatcher} from "svelte";

  const dispatch = createEventDispatcher();

  let disableCancel = false;
  let disableAccept = false;
</script>

<div class="join">
  <button disabled={disableCancel} class="btn btn-sm join-item" on:click|preventDefault|stopPropagation={() => dispatch('cancel', true)}>
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
      <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
    </svg>
  </button>
  <button disabled={disableAccept} class="btn btn-sm join-item" on:click|preventDefault|stopPropagation={() => dispatch('accept', true)}>
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
      <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
    </svg>
  </button>
</div>