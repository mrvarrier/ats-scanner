import React from 'react';
import { Loader2, FileText, Brain, Upload, BarChart3 } from 'lucide-react';
import { Progress } from './ui/progress';

export interface LoadingProps {
  size?: 'sm' | 'md' | 'lg';
  text?: string;
  className?: string;
}

// Basic spinner component
export function Spinner({
  size = 'md',
  className = '',
}: Omit<LoadingProps, 'text'>) {
  const sizeClasses = {
    sm: 'h-4 w-4',
    md: 'h-6 w-6',
    lg: 'h-8 w-8',
  };

  return (
    <Loader2 className={`animate-spin ${sizeClasses[size]} ${className}`} />
  );
}

// Button loading state
export function ButtonLoading({
  text = 'Loading...',
  size = 'sm',
}: LoadingProps) {
  return (
    <div className="flex items-center gap-2">
      <Spinner size={size} />
      <span>{text}</span>
    </div>
  );
}

// Page loading overlay
export function PageLoading({ text = 'Loading...' }: LoadingProps) {
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-white bg-opacity-80">
      <div className="text-center">
        <Spinner size="lg" className="mx-auto mb-4 text-blue-600" />
        <p className="text-lg font-medium text-gray-900">{text}</p>
      </div>
    </div>
  );
}

// Section loading (for parts of a page)
export function SectionLoading({
  text = 'Loading...',
  className = '',
}: LoadingProps) {
  return (
    <div className={`flex items-center justify-center py-8 ${className}`}>
      <div className="text-center">
        <Spinner size="md" className="mx-auto mb-2 text-blue-600" />
        <p className="text-sm text-gray-600">{text}</p>
      </div>
    </div>
  );
}

// Skeleton loaders for different content types
export function SkeletonCard() {
  return (
    <div className="animate-pulse rounded-lg border bg-white p-6">
      <div className="mb-3 h-4 w-3/4 rounded bg-gray-200"></div>
      <div className="mb-2 h-4 w-1/2 rounded bg-gray-200"></div>
      <div className="h-4 w-5/6 rounded bg-gray-200"></div>
    </div>
  );
}

export function SkeletonTable({
  rows = 5,
  columns = 4,
}: {
  rows?: number;
  columns?: number;
}) {
  return (
    <div className="animate-pulse">
      <div className="mb-4 h-10 rounded bg-gray-200"></div>
      {Array.from({ length: rows }).map((_, i) => (
        <div key={i} className="mb-3 flex space-x-4">
          {Array.from({ length: columns }).map((_, j) => (
            <div key={j} className="h-8 flex-1 rounded bg-gray-200"></div>
          ))}
        </div>
      ))}
    </div>
  );
}

export function SkeletonText({ lines = 3 }: { lines?: number }) {
  return (
    <div className="animate-pulse space-y-2">
      {Array.from({ length: lines }).map((_, i) => (
        <div
          key={i}
          className="h-4 rounded bg-gray-200"
          style={{ width: `${Math.random() * 40 + 60}%` }}
        ></div>
      ))}
    </div>
  );
}

// Analysis-specific loading states
export function AnalysisLoading({
  stage = 'Analyzing resume...',
  progress,
}: {
  stage?: string;
  progress?: number;
}) {
  return (
    <div className="rounded-lg border bg-white p-8 text-center">
      <div className="mb-6">
        <Brain className="mx-auto h-16 w-16 animate-pulse text-blue-600" />
      </div>

      <h3 className="mb-2 text-lg font-semibold text-gray-900">
        AI Analysis in Progress
      </h3>

      <p className="mb-6 text-gray-600">{stage}</p>

      {progress !== undefined && (
        <div className="mb-4">
          <Progress value={progress} className="w-full" />
          <p className="mt-2 text-sm text-gray-500">{progress}% complete</p>
        </div>
      )}

      <div className="flex items-center justify-center gap-4 text-sm text-gray-500">
        <div className="flex items-center gap-2">
          <FileText className="h-4 w-4" />
          Parsing document
        </div>
        <div className="flex items-center gap-2">
          <Brain className="h-4 w-4" />
          AI analysis
        </div>
        <div className="flex items-center gap-2">
          <BarChart3 className="h-4 w-4" />
          Generating scores
        </div>
      </div>
    </div>
  );
}

// Batch analysis loading
export function BatchAnalysisLoading({
  current = 0,
  total = 0,
  currentFile = '',
  stage = 'Processing...',
}: {
  current?: number;
  total?: number;
  currentFile?: string;
  stage?: string;
}) {
  const progress = total > 0 ? (current / total) * 100 : 0;

  return (
    <div className="rounded-lg border bg-white p-8">
      <div className="mb-6 text-center">
        <div className="mb-4 flex items-center justify-center gap-3">
          <Upload className="h-8 w-8 text-blue-600" />
          <Spinner size="lg" className="text-blue-600" />
          <BarChart3 className="h-8 w-8 text-blue-600" />
        </div>

        <h3 className="mb-2 text-xl font-semibold text-gray-900">
          Batch Analysis in Progress
        </h3>

        <p className="text-gray-600">
          Processing resume {current} of {total}
        </p>
      </div>

      <div className="mb-6">
        <Progress value={progress} className="h-3 w-full" />
        <div className="mt-2 flex justify-between text-sm text-gray-500">
          <span>{current} completed</span>
          <span>{total - current} remaining</span>
        </div>
      </div>

      {currentFile && (
        <div className="rounded-lg bg-gray-50 p-4">
          <p className="mb-1 text-sm font-medium text-gray-700">
            Currently processing:
          </p>
          <p className="text-sm text-gray-600">{currentFile}</p>
          <p className="mt-1 text-xs text-gray-500">{stage}</p>
        </div>
      )}
    </div>
  );
}

// File upload loading
export function FileUploadLoading({ fileName }: { fileName: string }) {
  return (
    <div className="rounded-lg border border-blue-200 bg-blue-50 p-4">
      <div className="flex items-center gap-3">
        <Upload className="h-5 w-5 animate-bounce text-blue-600" />
        <div className="flex-1">
          <p className="text-sm font-medium text-blue-900">Uploading file...</p>
          <p className="text-xs text-blue-700">{fileName}</p>
        </div>
        <Spinner size="sm" className="text-blue-600" />
      </div>
    </div>
  );
}

// Optimization loading
export function OptimizationLoading({
  stage = 'Optimizing resume...',
}: {
  stage?: string;
}) {
  return (
    <div className="rounded-lg border-2 border-dashed border-blue-300 bg-white p-6">
      <div className="text-center">
        <div className="mb-4">
          <div className="relative">
            <FileText className="mx-auto h-12 w-12 text-gray-400" />
            <Brain className="absolute -right-1 -top-1 h-6 w-6 animate-pulse text-blue-600" />
          </div>
        </div>

        <h4 className="mb-2 text-lg font-medium text-gray-900">
          AI Optimization in Progress
        </h4>

        <p className="mb-4 text-sm text-gray-600">{stage}</p>

        <div className="flex justify-center">
          <Spinner size="md" className="text-blue-600" />
        </div>
      </div>
    </div>
  );
}

// Export loading
export function ExportLoading({ format }: { format: string }) {
  return (
    <div className="rounded-lg border bg-white p-6 text-center">
      <div className="mb-4">
        <BarChart3 className="mx-auto h-12 w-12 animate-pulse text-green-600" />
      </div>

      <h4 className="mb-2 text-lg font-medium text-gray-900">
        Preparing Export
      </h4>

      <p className="mb-4 text-sm text-gray-600">
        Generating {format.toUpperCase()} file...
      </p>

      <Spinner size="md" className="text-green-600" />
    </div>
  );
}

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

  const setLoadingError = React.useCallback((error: string) => {
    setLoading(false);
    setError(error);
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

// Hook for managing async operations with loading states
export function useAsyncOperation<T>() {
  const [loading, setLoading] = React.useState(false);
  const [error, setError] = React.useState<string | null>(null);
  const [data, setData] = React.useState<T | null>(null);

  const execute = React.useCallback(async (asyncFn: () => Promise<T>) => {
    setLoading(true);
    setError(null);

    try {
      const result = await asyncFn();
      setData(result);
      return result;
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'An error occurred';
      setError(errorMessage);
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  const reset = React.useCallback(() => {
    setLoading(false);
    setError(null);
    setData(null);
  }, []);

  return {
    loading,
    error,
    data,
    execute,
    reset,
  };
}
