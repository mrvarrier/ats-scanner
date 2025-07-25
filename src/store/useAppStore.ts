import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import { AchievementAnalysis, MLInsights, Analysis } from '../types/api';
import { invoke } from '@tauri-apps/api/tauri';

export interface OllamaModel {
  name: string;
  size: number;
  digest: string;
  modified_at: string;
}

interface AnalysisResult {
  overall_score: number;
  category_scores: {
    skills: number;
    experience: number;
    education: number;
    keywords: number;
    format: number;
  };
  detailed_feedback: string;
  missing_keywords: string[];
  recommendations: string[];
  processing_time_ms: number;
}

export interface UserPreferences {
  id: string;
  user_id: string;

  // Ollama Settings
  ollama_host: string;
  ollama_port: number;
  default_model: string | null;
  connection_timeout_seconds: number;
  auto_connect_on_startup: boolean;

  // Analysis Settings
  default_optimization_level: 'Conservative' | 'Balanced' | 'Aggressive';
  auto_save_analyses: boolean;
  analysis_history_retention_days: number;

  // UI Preferences
  theme: 'Light' | 'Dark' | 'System' | 'HighContrast';
  language: string;
  sidebar_collapsed: boolean;
  show_advanced_features: boolean;
  animation_speed: 'None' | 'Reduced' | 'Normal' | 'Fast';

  // Data & Privacy
  data_storage_location: string | null;
  auto_backup_enabled: boolean;
  backup_frequency_hours: number;
  analytics_enabled: boolean;
  telemetry_enabled: boolean;

  // Notifications
  desktop_notifications: boolean;
  sound_notifications: boolean;
  email_notifications: boolean;
  notification_email: string | null;
  enable_batch_notifications: boolean;

  // Performance
  max_concurrent_analyses: number;
  cache_size_mb: number;
  enable_gpu_acceleration: boolean;

  // Export Settings
  default_export_format: 'JSON' | 'CSV' | 'PDF' | 'HTML';
  include_metadata_in_exports: boolean;
  compress_exports: boolean;

  created_at: string;
  updated_at: string;
}

interface AppState {
  // Ollama state
  models: OllamaModel[];
  selectedModel: string;
  isOllamaConnected: boolean;
  connectionLastChecked: number | null;
  connectionRetryCount: number;
  isMonitoringConnection: boolean;

  // Analysis state
  currentAnalysis: AnalysisResult | null;
  analysisHistory: Analysis[];
  isAnalyzing: boolean;

  // Current detailed analysis result for the dedicated results page
  currentDetailedAnalysis: {
    result: AnalysisResult;
    achievementAnalysis?: AchievementAnalysis;
    mlInsights?: MLInsights;
    resumeFilename: string;
    jobDescription: string;
    modelUsed: string;
    timestamp: string;
  } | null;

  // User preferences
  userPreferences: UserPreferences | null;
  isLoadingPreferences: boolean;

  // UI state
  activeTab:
    | 'dashboard'
    | 'analysis'
    | 'jobs'
    | 'personal-analytics'
    | 'career-development'
    | 'optimization'
    | 'writing-assistant'
    | 'results'
    | 'history'
    | 'settings'
    | 'analysis-result';
  isDarkMode: boolean;

  // Actions
  setModels: (_models: OllamaModel[]) => void;
  setSelectedModel: (_model: string) => void;
  setOllamaConnection: (_connected: boolean) => void;
  setCurrentAnalysis: (_analysis: AnalysisResult | null) => void;
  setAnalysisHistory: (_history: Analysis[]) => void;
  setIsAnalyzing: (_analyzing: boolean) => void;
  setCurrentDetailedAnalysis: (
    _analysis: AppState['currentDetailedAnalysis']
  ) => void;
  setUserPreferences: (_preferences: UserPreferences | null) => void;
  setIsLoadingPreferences: (_loading: boolean) => void;
  setActiveTab: (
    _tab:
      | 'dashboard'
      | 'analysis'
      | 'jobs'
      | 'personal-analytics'
      | 'career-development'
      | 'optimization'
      | 'writing-assistant'
      | 'results'
      | 'history'
      | 'settings'
      | 'analysis-result'
  ) => void;
  toggleDarkMode: () => void;

  // Connection monitoring actions
  startConnectionMonitoring: () => void;
  stopConnectionMonitoring: () => void;
  checkConnectionHealth: () => Promise<void>;
  setConnectionRetryCount: (_count: number) => void;
}

// Extend window type for cleanup functions
interface WindowWithCleanup extends Window {
  __themeChangeCleanup?: () => void;
  __connectionMonitoringCleanup?: () => void;
}

export const useAppStore = create<AppState>()(
  devtools(
    (set, get) => ({
      // Initial state
      models: [],
      selectedModel: '',
      isOllamaConnected: false,
      connectionLastChecked: null,
      connectionRetryCount: 0,
      isMonitoringConnection: false,
      currentAnalysis: null,
      analysisHistory: [],
      isAnalyzing: false,
      currentDetailedAnalysis: null,
      userPreferences: null,
      isLoadingPreferences: false,
      activeTab: 'dashboard',
      isDarkMode: false,

      // Actions
      setModels: models => set({ models }),
      setSelectedModel: model => {
        set({ selectedModel: model });
        // Also update user preferences if available
        const state = get();
        if (
          state.userPreferences &&
          model !== state.userPreferences.default_model
        ) {
          // Would trigger a preference update here in a real implementation
        }
      },
      setOllamaConnection: connected => set({ isOllamaConnected: connected }),
      setCurrentAnalysis: analysis => set({ currentAnalysis: analysis }),
      setAnalysisHistory: history => set({ analysisHistory: history }),
      setIsAnalyzing: analyzing => set({ isAnalyzing: analyzing }),
      setCurrentDetailedAnalysis: analysis =>
        set({ currentDetailedAnalysis: analysis }),
      setUserPreferences: preferences => {
        const currentState = get();
        const previousPreferences = currentState.userPreferences;

        set({ userPreferences: preferences });

        // Only apply theme if it actually changed or if this is the first time loading preferences
        if (
          preferences &&
          (!previousPreferences ||
            previousPreferences.theme !== preferences.theme)
        ) {
          const applyTheme = (theme: string) => {
            let isDark = false;

            switch (theme) {
              case 'Dark':
                isDark = true;
                break;
              case 'Light':
                isDark = false;
                break;
              case 'System':
                isDark = window.matchMedia(
                  '(prefers-color-scheme: dark)'
                ).matches;
                break;
              case 'HighContrast':
                isDark = true; // High contrast typically uses dark background
                break;
              default:
                isDark = false;
            }

            set({ isDarkMode: isDark });

            // Apply additional theme classes
            const html = document.documentElement;
            html.classList.remove('dark', 'light', 'high-contrast');

            if (theme === 'HighContrast') {
              html.classList.add('dark', 'high-contrast');
            } else if (isDark) {
              html.classList.add('dark');
            } else {
              html.classList.add('light');
            }
          };

          applyTheme(preferences.theme);

          // Listen for system theme changes if using System theme
          if (preferences.theme === 'System') {
            // Cleanup any existing listener
            const cleanup = (window as WindowWithCleanup).__themeChangeCleanup;
            if (cleanup && typeof cleanup === 'function') {
              cleanup();
            }

            const mediaQuery = window.matchMedia(
              '(prefers-color-scheme: dark)'
            );
            const handleSystemThemeChange = () => {
              const currentPrefs = useAppStore.getState().userPreferences;
              if (currentPrefs?.theme === 'System') {
                applyTheme('System');
              }
            };

            mediaQuery.addEventListener('change', handleSystemThemeChange);

            // Store cleanup function
            (window as WindowWithCleanup).__themeChangeCleanup = () => {
              mediaQuery.removeEventListener('change', handleSystemThemeChange);
            };
          } else {
            // Cleanup listener if not using System theme
            const cleanup = (window as WindowWithCleanup).__themeChangeCleanup;
            if (cleanup && typeof cleanup === 'function') {
              cleanup();
              delete (window as WindowWithCleanup).__themeChangeCleanup;
            }
          }
        }
      },
      setIsLoadingPreferences: loading =>
        set({ isLoadingPreferences: loading }),
      setActiveTab: tab => set({ activeTab: tab }),
      toggleDarkMode: () => set(state => ({ isDarkMode: !state.isDarkMode })),

      // Connection monitoring actions
      setConnectionRetryCount: count => set({ connectionRetryCount: count }),

      checkConnectionHealth: async () => {
        try {
          const result = await invoke<{ success: boolean; data: boolean }>(
            'ollama_health_check'
          );
          const isConnected = result.success && result.data;
          const currentState = get();

          set({
            isOllamaConnected: isConnected,
            connectionLastChecked: Date.now(),
            connectionRetryCount: isConnected
              ? 0
              : currentState.connectionRetryCount + 1,
          });
        } catch (_error) {
          // Silent failure for health checks - no logging needed
          const currentState = get();
          set({
            isOllamaConnected: false,
            connectionLastChecked: Date.now(),
            connectionRetryCount: currentState.connectionRetryCount + 1,
          });
        }
      },

      startConnectionMonitoring: () => {
        const currentState = get();
        if (currentState.isMonitoringConnection) return;

        set({ isMonitoringConnection: true });

        // Cleanup any existing monitoring
        const cleanup = (window as WindowWithCleanup)
          .__connectionMonitoringCleanup;
        if (cleanup && typeof cleanup === 'function') {
          cleanup();
        }

        // Initial health check
        void get().checkConnectionHealth();

        // Set up periodic health checks with exponential backoff on failures
        const scheduleHealthCheck = () => {
          const state = get();
          if (!state.isMonitoringConnection) return;

          const baseInterval = 10000; // 10 seconds base interval for better responsiveness
          const maxInterval = 300000; // 5 minutes max interval
          const backoffFactor = 1.5;

          // Calculate delay based on retry count (exponential backoff for failures)
          const interval = state.isOllamaConnected
            ? baseInterval
            : Math.min(
                baseInterval *
                  Math.pow(backoffFactor, state.connectionRetryCount),
                maxInterval
              );

          const timeoutId = setTimeout(async () => {
            await get().checkConnectionHealth();
            scheduleHealthCheck();
          }, interval);

          // Store cleanup function
          (window as WindowWithCleanup).__connectionMonitoringCleanup = () => {
            clearTimeout(timeoutId);
          };
        };

        scheduleHealthCheck();
      },

      stopConnectionMonitoring: () => {
        set({ isMonitoringConnection: false });

        // Cleanup monitoring
        const cleanup = (window as WindowWithCleanup)
          .__connectionMonitoringCleanup;
        if (cleanup && typeof cleanup === 'function') {
          cleanup();
          delete (window as WindowWithCleanup).__connectionMonitoringCleanup;
        }
      },
    }),
    {
      name: 'ats-scanner-store',
    }
  )
);
