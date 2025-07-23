import { useState, useCallback, useRef } from 'react';

export interface RetryOptions {
  maxAttempts?: number;
  baseDelay?: number;
  maxDelay?: number;
  backoffFactor?: number;
  shouldRetry?: (_error: Error, _attemptNumber: number) => boolean;
  onRetry?: (_attemptNumber: number, _error: Error) => void;
  onSuccess?: (_result: unknown, _attemptNumber: number) => void;
  onFailure?: (_error: Error, _totalAttempts: number) => void;
}

export interface RetryState {
  isRetrying: boolean;
  attemptNumber: number;
  lastError: Error | null;
  canRetry: boolean;
}

const defaultOptions: Required<RetryOptions> = {
  maxAttempts: 3,
  baseDelay: 1000,
  maxDelay: 10000,
  backoffFactor: 2,
  shouldRetry: (error: Error) => {
    // Default: retry on network errors and 5xx status codes
    if (error?.name === 'NetworkError' || error?.message?.includes('network')) {
      return true;
    }
    if (
      'status' in error &&
      typeof error.status === 'number' &&
      error.status >= 500 &&
      error.status < 600
    ) {
      return true;
    }
    if (error?.message?.includes('timeout')) {
      return true;
    }
    return false;
  },
  onRetry: () => {},
  onSuccess: () => {},
  onFailure: () => {},
};

export function useRetry<T>(
  asyncFunction: () => Promise<T>,
  options: RetryOptions = {}
) {
  const [state, setState] = useState<RetryState>({
    isRetrying: false,
    attemptNumber: 0,
    lastError: null,
    canRetry: false,
  });

  const abortControllerRef = useRef<AbortController | null>(null);

  const execute = useCallback(async (): Promise<T> => {
    // Move opts inside the callback to avoid dependencies changing on every render
    const {
      maxAttempts,
      baseDelay,
      backoffFactor,
      maxDelay,
      shouldRetry,
      onSuccess,
      onRetry,
      onFailure,
    } = { ...defaultOptions, ...options };

    // Cancel any ongoing retry
    if (abortControllerRef.current) {
      abortControllerRef.current.abort();
    }

    abortControllerRef.current = new AbortController();
    const { signal } = abortControllerRef.current;

    setState({
      isRetrying: false,
      attemptNumber: 0,
      lastError: null,
      canRetry: false,
    });

    let attempt = 1;
    let lastError: Error = new Error('Unknown error');

    while (attempt <= maxAttempts) {
      try {
        if (signal.aborted) {
          throw new Error('Operation was cancelled');
        }

        setState(prev => ({
          ...prev,
          attemptNumber: attempt,
          isRetrying: attempt > 1,
        }));

        const result = await asyncFunction();

        setState(prev => ({
          ...prev,
          isRetrying: false,
          canRetry: false,
        }));

        onSuccess(result, attempt);
        return result;
      } catch (err) {
        const error = err instanceof Error ? err : new Error(String(err));
        lastError = error;

        setState(prev => ({
          ...prev,
          lastError: error,
          canRetry: attempt < maxAttempts && shouldRetry(error, attempt),
        }));

        // Don't retry if this is the last attempt or if we shouldn't retry
        if (attempt >= maxAttempts || !shouldRetry(error, attempt)) {
          break;
        }

        // Calculate delay with exponential backoff
        const delay = Math.min(
          baseDelay * Math.pow(backoffFactor, attempt - 1),
          maxDelay
        );

        onRetry(attempt, error);

        // Wait before retrying
        await new Promise((resolve, reject) => {
          const timeoutId = setTimeout(resolve, delay);

          signal.addEventListener('abort', () => {
            clearTimeout(timeoutId);
            reject(new Error('Operation was cancelled'));
          });
        });

        attempt++;
      }
    }

    setState(prev => ({
      ...prev,
      isRetrying: false,
      canRetry: false,
    }));

    onFailure(lastError, attempt - 1);
    throw lastError;
  }, [asyncFunction, options]);

  const cancel = useCallback(() => {
    if (abortControllerRef.current) {
      abortControllerRef.current.abort();
    }
    setState(prev => ({
      ...prev,
      isRetrying: false,
      canRetry: false,
    }));
  }, []);

  const retry = useCallback(() => {
    if (state.canRetry) {
      void execute();
    }
  }, [state.canRetry, execute]);

  return {
    execute,
    retry,
    cancel,
    ...state,
  };
}

// Hook for retrying with exponential backoff
export function useRetryWithBackoff<T>(
  asyncFunction: () => Promise<T>,
  maxAttempts: number = 3,
  baseDelay: number = 1000
) {
  return useRetry(asyncFunction, {
    maxAttempts,
    baseDelay,
    backoffFactor: 2,
    maxDelay: 30000,
    shouldRetry: (error: Error) => {
      // Retry on network errors, timeouts, and 5xx errors
      const retryableErrors = [
        'NetworkError',
        'TimeoutError',
        'fetch',
        'network',
        'timeout',
        'connection',
      ];

      const errorMessage = error?.message?.toLowerCase() || '';
      const errorName = error?.name?.toLowerCase() || '';

      return (
        retryableErrors.some(
          keyword =>
            errorMessage.includes(keyword) || errorName.includes(keyword)
        ) ||
        ('status' in error &&
          typeof error.status === 'number' &&
          error.status >= 500 &&
          error.status < 600)
      );
    },
  });
}

// Utility function for retry with Promise
export async function retryAsync<T>(
  asyncFunction: () => Promise<T>,
  options: RetryOptions = {}
): Promise<T> {
  const opts = { ...defaultOptions, ...options };
  let attempt = 1;
  let lastError: Error = new Error('Unknown error');

  while (attempt <= opts.maxAttempts) {
    try {
      const result = await asyncFunction();
      opts.onSuccess(result, attempt);
      return result;
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err));
      lastError = error;

      if (attempt >= opts.maxAttempts || !opts.shouldRetry(error, attempt)) {
        break;
      }

      const delay = Math.min(
        opts.baseDelay * Math.pow(opts.backoffFactor, attempt - 1),
        opts.maxDelay
      );

      opts.onRetry(attempt, error);
      await new Promise(resolve => setTimeout(resolve, delay));
      attempt++;
    }
  }

  opts.onFailure(lastError, attempt - 1);
  throw lastError;
}

// Specific retry functions for common scenarios

// Retry for API calls
export function useApiRetry<T>(
  apiCall: () => Promise<T>,
  customOptions?: Partial<RetryOptions>
) {
  return useRetry(apiCall, {
    maxAttempts: 3,
    baseDelay: 1000,
    backoffFactor: 2,
    maxDelay: 10000,
    shouldRetry: (error: Error) => {
      // Retry on network errors and 5xx status codes, but not 4xx
      if (
        'status' in error &&
        typeof error.status === 'number' &&
        error.status >= 400 &&
        error.status < 500
      ) {
        return false; // Don't retry client errors
      }
      return true;
    },
    ...customOptions,
  });
}

// Retry for file operations
export function useFileOperationRetry<T>(
  fileOperation: () => Promise<T>,
  customOptions?: Partial<RetryOptions>
) {
  return useRetry(fileOperation, {
    maxAttempts: 2,
    baseDelay: 500,
    backoffFactor: 2,
    shouldRetry: (error: Error) => {
      // Retry on file system errors but not permission errors
      const errorMessage = error?.message?.toLowerCase() || '';
      return (
        !errorMessage.includes('permission') &&
        !errorMessage.includes('unauthorized')
      );
    },
    ...customOptions,
  });
}

// Circuit breaker pattern
export class CircuitBreaker {
  private failures: number = 0;
  private lastFailureTime: number = 0;
  private state: 'closed' | 'open' | 'half-open' = 'closed';

  constructor(
    private _failureThreshold: number = 5,
    private _resetTimeout: number = 60000, // 1 minute
    private _monitoringPeriod: number = 300000 // 5 minutes
  ) {}

  get monitoringPeriod(): number {
    return this._monitoringPeriod;
  }

  async execute<T>(operation: () => Promise<T>): Promise<T> {
    if (this.state === 'open') {
      if (Date.now() - this.lastFailureTime >= this._resetTimeout) {
        this.state = 'half-open';
      } else {
        throw new Error('Circuit breaker is open');
      }
    }

    try {
      const result = await operation();

      if (this.state === 'half-open') {
        this.reset();
      }

      return result;
    } catch (err) {
      this.recordFailure();
      throw err instanceof Error ? err : new Error(String(err));
    }
  }

  private recordFailure() {
    this.failures++;
    this.lastFailureTime = Date.now();

    if (this.failures >= this._failureThreshold) {
      this.state = 'open';
    }
  }

  private reset() {
    this.failures = 0;
    this.state = 'closed';
    this.lastFailureTime = 0;
  }

  getState() {
    return {
      state: this.state,
      failures: this.failures,
      lastFailureTime: this.lastFailureTime,
    };
  }
}
