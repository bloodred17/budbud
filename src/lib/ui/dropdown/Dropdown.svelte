<script lang="ts">
  import {clickOutside} from "$lib/ui/dropdown/click-outside";
  export let mode: 'option' | 'window' = 'window';
  export let modify: string = '';

  let open = false;
  export function close() {
    open = false;
  }
</script>

<div class="inline-block">
  <div on:click={() => open = true}>
    <slot name="trigger" />
  </div>
  <div use:clickOutside
       on:click_outside={() => open = false}
       class:hidden={open === false}
       class="z-[1] menu p-2 rounded-box absolute {modify}">
    <slot name="content" />
  </div>
</div>
