import React from 'react';
import { createContext, useState, useCallback } from 'react';

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

interface NotificationContextType {
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

export function NotificationProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const [notifications, setNotifications] = useState<Notification[]>([]);

  const addNotification = useCallback(
    (notification: Omit<Notification, 'id'>) => {
      const id = Math.random().toString(36).substring(2, 9);
      const newNotification = { ...notification, id };

      setNotifications(prev => [...prev, newNotification]);

      // Auto-remove notification after duration (default 5 seconds)
      if (!notification.persistent) {
        const duration = notification.duration ?? 5000;
        setTimeout(() => {
          setNotifications(prev => prev.filter(n => n.id !== id));
        }, duration);
      }
    },
    []
  );

  const removeNotification = useCallback((id: string) => {
    setNotifications(prev =>
      prev.filter(notification => notification.id !== id)
    );
  }, []);

  const clearNotifications = useCallback(() => {
    setNotifications([]);
  }, []);

  const success = useCallback(
    (title: string, message?: string, options?: Partial<Notification>) => {
      addNotification({ type: 'success', title, message, ...options });
    },
    [addNotification]
  );

  const error = useCallback(
    (title: string, message?: string, options?: Partial<Notification>) => {
      addNotification({ type: 'error', title, message, ...options });
    },
    [addNotification]
  );

  const warning = useCallback(
    (title: string, message?: string, options?: Partial<Notification>) => {
      addNotification({ type: 'warning', title, message, ...options });
    },
    [addNotification]
  );

  const info = useCallback(
    (title: string, message?: string, options?: Partial<Notification>) => {
      addNotification({ type: 'info', title, message, ...options });
    },
    [addNotification]
  );

  const value = {
    notifications,
    addNotification,
    removeNotification,
    clearNotifications,
    success,
    error,
    warning,
    info,
  };

  return (
    <NotificationContext.Provider value={value}>
      {children}
    </NotificationContext.Provider>
  );
}
