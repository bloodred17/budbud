import {type Writable, writable} from 'svelte/store';
import { v4 as uuidv4 } from 'uuid';

export enum NotificationType {
  Neutral = '',
  Info = 'info',
  Success = 'success',
  Warning = 'warning',
  Error = 'danger',
}

export interface NotificationInterface {
  id: string,
  notificationType: NotificationType,
  message: string,
  timeout: number,
}

export type NotificationParams =
  Partial<Omit<NotificationInterface, 'id'>> &
  Required<Pick<NotificationInterface, 'message'>>;

export class Notification implements NotificationInterface {
  id = uuidv4()
  notificationType: NotificationType = NotificationType.Neutral;
  message = '';
  timeout = 5000;

  constructor(init?: NotificationParams) {
    if (init) {
      Object.assign(this, init);
    }
  }
}

export const notificationStore: Writable<NotificationInterface> = writable<NotificationInterface>();

export const count = writable(0);

