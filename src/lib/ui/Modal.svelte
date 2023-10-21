<script lang="ts">
  export let acceptCallback: () => void;
  export let closeCallback: () => void;

  let modal;

  export const openModal = (acceptFn?: () => void) => {
    // __modal__?.showModal();
    if (acceptFn) {
      acceptCallback = acceptFn;
    }
    modal?.showModal();
  }

  const accept = (event: CustomEvent) => {
    if (acceptCallback) {
      acceptCallback();
    }
  }

  const close = (event: CustomEvent) => {
    if (closeCallback) {
      closeCallback();
    }
    modal?.close();
    // __modal__.close();
  }
</script>

<dialog id="__modal__" class="modal" bind:this={modal}>
  <div class="modal-box">
    <slot name="body"/>

    <div class="modal-action">
      <form method="dialog" class="flex gap-4">
        <!-- if there is a button in form, it will close the modal -->
        <button class="btn btn-sm" on:click={accept}>Accept</button>
        <button class="btn btn-sm btn-error" on:click={close}>Cancel</button>
      </form>
    </div>
  </div>
</dialog>