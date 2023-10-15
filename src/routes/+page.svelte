<script lang="ts">
  import MainLayout from "$lib/layouts/MainLayout.svelte";
  import CreationButtons from "$lib/components/CreationButtons.svelte";
  import {notificationStore, NotificationType} from "$lib/components/notification/notification.store";
  import { v4 as uuidv4 } from 'uuid';

  const push = () => {
    notificationStore.set({
      id: uuidv4(),
      notificationType: NotificationType.Success,
      message: 'Test',
      timeout: 3000,
    })
  }
</script>

<MainLayout>
  <h1 slot="header">Screens</h1>

  <div slot="body">
    <button class="btn btn-primary" on:click={() => push()}>Push Notification</button>
  </div>

  <div slot="footer">
    <CreationButtons/>
  </div>
</MainLayout>
