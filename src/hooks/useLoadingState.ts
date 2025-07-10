import React from 'react';

// Hook for managing loading states
export function useLoadingState(initialState = false) {
  const [loading, setLoading] = React.useState(initialState);
  const [error, setError] = React.useState<string | null>(null);

  const startLoading = React.useCallback(() => {
    setLoading(true);
    setError(null);
  }, []);

  const stopLoading = React.useCallback(() => {
    setLoading(false);
  }, []);

  const setLoadingError = React.useCallback((errorMessage: string) => {
    setError(errorMessage);
    setLoading(false);
  }, []);

  const clearError = React.useCallback(() => {
    setError(null);
  }, []);

  return {
    loading,
    error,
    startLoading,
    stopLoading,
    setLoadingError,
    clearError,
  };
}
