<script lang="ts">
  import {
    type Notification,
    notificationStore,
    NotificationType
  } from "$lib/components/notification/notification.store";
  import {onMount} from "svelte";

  const notificationClass: Record<NotificationType, string> = {
    [NotificationType.Info]: 'alert-info',
    [NotificationType.Success]: 'alert-success',
    [NotificationType.Danger]: 'alert-error',
    [NotificationType.Warning]: 'alert-warning'
  }

  const delay = (timeout: number, callback?: () => void): Promise<void> => new Promise((resolve) => {
    setTimeout(() => {
      if (callback) {
        callback();
        resolve();
      } else {
        resolve();
      }
    }, timeout)
  });
  let notificationQueue: Notification[] = [];

  onMount(() => {
    notificationStore.subscribe((notification) => {
      console.log(notification);
      if (notification) {
        notificationQueue = [...notificationQueue, notification];
      }
    });

  })

  function removeFromQueue(notification: Notification) {
    delay(notification?.timeout, () => {
      notificationQueue = [...notificationQueue.filter((item) => item?.id !== notification?.id)];
    })
  }

  $: console.log(notificationQueue)
</script>


<div class="toast toast-end">
  {#each notificationQueue as notification (notification.id)}
  <div>
    <div class="alert {notificationClass[notification?.notificationType]}">
      <span>{notification?.message}</span>
    </div>
    {#await removeFromQueue(notification) then value}
      <div class="hidden">{value}</div>
    {/await}
  </div>
  {/each}

</div>
