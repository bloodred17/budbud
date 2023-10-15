<script>
	import "../app.css";
  import Notification from "$lib/components/notification/Notification.svelte";
</script>

<div class="p-4">
  <slot/>
  <Notification />
</div>
