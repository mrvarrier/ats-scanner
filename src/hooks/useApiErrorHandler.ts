import { useCallback } from 'react';
import { useNotifications } from './useNotifications';

// Utility hook for handling API errors with notifications
export function useApiErrorHandler() {
  const { error } = useNotifications();

  const handleError = useCallback(
    (err: Error | unknown, context?: string) => {
      let title = 'An error occurred';
      let message = 'Please try again later';

      if (err instanceof Error && err.message) {
        message = err.message;
      } else if (typeof err === 'string') {
        message = err;
      }

      if (context) {
        title = `${context} error`;
      }

      error({ title, message });
    },
    [error]
  );

  return { handleError };
}
