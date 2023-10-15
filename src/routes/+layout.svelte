<script lang="ts" context="module">

  import {invoke} from "@tauri-apps/api/tauri";
  import {notificationStore, Notify as Notify} from "$lib/components/notification/notification.store";
  import {fetchTransactionSources} from "$lib/stores/transaction-source.store";

  invoke('establish_connection')
    .then((response) => {
      if (response === 'Connection Established') {
        notificationStore.update(() => new Notify({message: 'DB Connected', timeout: 3000}))

        fetchTransactionSources();
      }
    })

</script>


<script>
	import "../app.css";
  import Notification from "$lib/components/notification/Notification.svelte";
</script>

<div class="p-4">
  <slot/>
  <Notification />
</div>
