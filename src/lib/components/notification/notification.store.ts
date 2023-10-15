import {type Writable, writable} from 'svelte/store';

export enum NotificationType {
  Info = 'info',
  Success = 'success',
  Warning = 'warning',
  Danger = 'danger',
}

export interface Notification {
  id: string,
  notificationType: NotificationType,
  message: string,
  timeout: number,
}

export const notificationStore: Writable<Notification> = writable<Notification>();

export const count = writable(0);

