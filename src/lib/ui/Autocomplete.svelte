<script lang="ts">
    import type {SelectionOption} from "$lib/types/ui.interface";
    import {createEventDispatcher} from "svelte";

    export let items: SelectionOption[] | string[] = [];
    export let selectedItem: string;
    export let inputStyleModifiers = '';
    export let placeholder = '';

    const dispatch = createEventDispatcher();

    let inputVal = '';

    $: {
      if (selectedItem) {
        const selectedOption = items?.find((item) => {
          if (typeof item === 'string') {
            selectedItem === item;
          } else {
            selectedItem === item?.display;
          }
        });

        if (selectedOption) {
          if (typeof selectedOption == 'string') {
            inputVal = selectedOption;
          } else {
            inputVal = selectedOption?.display;
          }
        }
      }
    }

    function onItemClicked(item: SelectionOption | string) {
      document?.activeElement?.blur();

      if (typeof item === 'string') {
        selectedItem = item;
      } else {
        selectedItem = item?.display;
      }
      inputVal = selectedItem;
      dispatch('value', item);
    }

    $: console.log(items);
    $: console.log(inputVal);

    $: filteredItems = items.filter((item) => {
      if (typeof item === 'string') {
        return item.toLowerCase().includes(inputVal.toLowerCase());
      } else {
        return item.value.includes(inputVal.toLowerCase());
      }
    });
</script>


<div class="dropdown">
  <input
      tabindex="0"
      id="transaction_amount"
      type="text"
      placeholder={placeholder}
      class="input {inputStyleModifiers || 'input-bordered input-sm w-full max-w-xs'}"
      bind:value={inputVal}
  />

  <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-full max-w-xs max-h-80 flex-nowrap overflow-auto">
    {#each filteredItems as item}
      <li>
        {#if typeof item === 'string'}
        <a on:click|preventDefault={() => onItemClicked(item)}>{item}</a>
        {:else}
        <a on:click|preventDefault={() => onItemClicked(item)}>{item?.display}</a>
        {/if}
      </li>
    {/each}
  </ul>
</div>