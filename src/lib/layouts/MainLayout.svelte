<script>
</script>

<section class="overflow-x-hidden">
  <div class="header">
    <slot name="header" />
  </div>

  <div class="body">
    <slot name="body" />
  </div>


  <div class="footer">
    <slot name="footer" />
  </div>
</section>

<style lang="postcss">
  :global(html) {
      background-color: theme(colors.gray.100);
  }
  .header {
      height: 50px;
  }

  .footer {
      position: absolute;
      @apply bottom-0 left-0 p-4;
  }
</style>

