<section>
  <div class="header border">
    <slot name="header" />
  </div>

  <div class="body overflow-y-auto border border-red-500">
    <slot name="body" />
  </div>


  <div class="footer border border-blue-500">
    <slot name="footer"/>
  </div>
</section>

<style lang="postcss">
  :global(html) {
      background-color: theme(colors.gray.100);
  }
  .body {
      height: 80vh;
  }
</style>

