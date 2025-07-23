import { createContext } from 'react';

export type NotificationType = 'success' | 'error' | 'warning' | 'info';

export interface Notification {
  id: string;
  type: NotificationType;
  title: string;
  message?: string;
  action?: {
    label: string;
    onClick: () => void;
  };
  duration?: number;
  persistent?: boolean;
}

export interface NotificationContextType {
  notifications: Notification[];
  addNotification: (_notification: Omit<Notification, 'id'>) => void;
  removeNotification: (_id: string) => void;
  clearNotifications: () => void;
  success: (
    _title: string,
    _message?: string,
    _options?: Partial<Notification>
  ) => void;
  error: (
    _title: string,
    _message?: string,
    _options?: Partial<Notification>
  ) => void;
  warning: (
    _title: string,
    _message?: string,
    _options?: Partial<Notification>
  ) => void;
  info: (
    _title: string,
    _message?: string,
    _options?: Partial<Notification>
  ) => void;
}

export const NotificationContext =
  createContext<NotificationContextType | null>(null);
