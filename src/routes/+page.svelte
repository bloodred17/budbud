<script>
    import Greet from "$lib/Greet.svelte";
    import MainLayout from "$lib/layouts/MainLayout.svelte";
</script>

<MainLayout>
  <h1 slot="header">Screens</h1>

  <div slot="body">
    
  </div>

  <div slot="footer" class="join">
    <button class="btn join-item">Button</button>
    <button class="btn join-item">Button</button>
    <button class="btn join-item">Button</button>
  </div>
</MainLayout>

<!--<Greet />-->