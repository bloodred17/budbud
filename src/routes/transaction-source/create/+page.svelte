<script>
    import MainLayout from "$lib/layouts/MainLayout.svelte";
    import CreationButtons from "$lib/components/CreationButtons.svelte";
    import NavigationButtons from "$lib/components/NavigationButtons.svelte";
    import TransactionSourceCreator from "$lib/TransactionSourceCreator.svelte";
</script>

<MainLayout>
  <div slot="header" class="flex">
    <div class="flex">
      <NavigationButtons/>
    </div>
    <div class="px-2 flex justify-center flex-1">
      <h1 class="text-2xl">Create Transaction Source</h1>
    </div>
  </div>

  <TransactionSourceCreator slot="body"/>

  <CreationButtons slot="footer"/>
</MainLayout>