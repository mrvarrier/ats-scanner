// Re-export all types for easy importing
export * from './api';

// Additional UI-specific types
export interface TabType {
  id: string;
  label: string;
  icon: string;
  description?: string;
}

export interface ThemeConfig {
  name: string;
  colors: {
    primary: string;
    secondary: string;
    background: string;
    foreground: string;
    accent: string;
  };
}

export interface ProgressState {
  current: number;
  total: number;
  message?: string;
  isIndeterminate?: boolean;
}

export interface ToastMessage {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  title: string;
  description?: string;
  duration?: number;
}
