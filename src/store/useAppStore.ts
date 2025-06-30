import { create } from 'zustand';
import { devtools } from 'zustand/middleware';

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


interface Analysis {
  id: string;
  resume_id: string;
  job_description_id: string;
  model_used: string;
  overall_score: number;
  skills_score: number;
  experience_score: number;
  education_score: number;
  keywords_score: number;
  format_score: number;
  detailed_feedback: string;
  missing_keywords: string;
  recommendations: string;
  processing_time_ms: number;
  created_at: string;
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
  selectedModel: string | null;
  isOllamaConnected: boolean;
  
  // Analysis state
  currentAnalysis: AnalysisResult | null;
  analysisHistory: Analysis[];
  isAnalyzing: boolean;
  
  // Current detailed analysis result for the dedicated results page
  currentDetailedAnalysis: {
    result: AnalysisResult;
    achievementAnalysis?: any;
    mlInsights?: any;
    resumeFilename: string;
    jobDescription: string;
    modelUsed: string;
    timestamp: string;
  } | null;
  
  // User preferences
  userPreferences: UserPreferences | null;
  isLoadingPreferences: boolean;
  
  // UI state
  activeTab: string;
  isDarkMode: boolean;
  
  // Actions
  setModels: (models: OllamaModel[]) => void;
  setSelectedModel: (model: string) => void;
  setOllamaConnection: (connected: boolean) => void;
  setCurrentAnalysis: (analysis: AnalysisResult | null) => void;
  setAnalysisHistory: (history: Analysis[]) => void;
  setIsAnalyzing: (analyzing: boolean) => void;
  setCurrentDetailedAnalysis: (analysis: AppState['currentDetailedAnalysis']) => void;
  setUserPreferences: (preferences: UserPreferences | null) => void;
  setIsLoadingPreferences: (loading: boolean) => void;
  setActiveTab: (tab: string) => void;
  toggleDarkMode: () => void;
}

export const useAppStore = create<AppState>()(
  devtools(
    (set, get) => ({
      // Initial state
      models: [],
      selectedModel: null,
      isOllamaConnected: false,
      currentAnalysis: null,
      analysisHistory: [],
      isAnalyzing: false,
      currentDetailedAnalysis: null,
      userPreferences: null,
      isLoadingPreferences: false,
      activeTab: 'dashboard',
      isDarkMode: false,
      
      // Actions
      setModels: (models) => set({ models }),
      setSelectedModel: (model) => {
        set({ selectedModel: model });
        // Also update user preferences if available
        const state = get();
        if (state.userPreferences && model !== state.userPreferences.default_model) {
          // Would trigger a preference update here in a real implementation
        }
      },
      setOllamaConnection: (connected) => set({ isOllamaConnected: connected }),
      setCurrentAnalysis: (analysis) => set({ currentAnalysis: analysis }),
      setAnalysisHistory: (history) => set({ analysisHistory: history }),
      setIsAnalyzing: (analyzing) => set({ isAnalyzing: analyzing }),
      setCurrentDetailedAnalysis: (analysis) => set({ currentDetailedAnalysis: analysis }),
      setUserPreferences: (preferences) => {
        set({ userPreferences: preferences });
        // Apply theme preference
        if (preferences) {
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
                isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
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
            if ((window as any).__themeChangeCleanup) {
              (window as any).__themeChangeCleanup();
            }
            
            const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
            const handleSystemThemeChange = () => {
              const currentPrefs = useAppStore.getState().userPreferences;
              if (currentPrefs?.theme === 'System') {
                applyTheme('System');
              }
            };
            
            mediaQuery.addEventListener('change', handleSystemThemeChange);
            
            // Store cleanup function
            (window as any).__themeChangeCleanup = () => {
              mediaQuery.removeEventListener('change', handleSystemThemeChange);
            };
          } else {
            // Cleanup listener if not using System theme
            if ((window as any).__themeChangeCleanup) {
              (window as any).__themeChangeCleanup();
              delete (window as any).__themeChangeCleanup;
            }
          }
        }
      },
      setIsLoadingPreferences: (loading) => set({ isLoadingPreferences: loading }),
      setActiveTab: (tab) => set({ activeTab: tab }),
      toggleDarkMode: () => set((state) => ({ isDarkMode: !state.isDarkMode })),
    }),
    {
      name: 'ats-scanner-store',
    }
  )
);