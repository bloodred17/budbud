<script>
    import Greet from "$lib/Greet.svelte";
    import MainLayout from "$lib/layouts/MainLayout.svelte";
    import CreationButtons from "$lib/components/CreationButtons.svelte";
</script>

<MainLayout>
  <h1 slot="header">Screens</h1>

  <div slot="body">

  </div>

  <CreationButtons slot="footer"/>
</MainLayout>

<!--<Greet />-->