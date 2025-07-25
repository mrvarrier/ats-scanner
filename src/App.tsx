import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useAppStore, OllamaModel } from './store/useAppStore';
import { useUserPreferences } from './hooks/useUserPreferences';
import { MainLayout } from './components/layout/MainLayout';
import { Dashboard } from './components/pages/Dashboard';
import { AnalysisPage } from './components/pages/AnalysisPage';
import { JobsPage } from './components/pages/JobsPage';
import { OptimizationPage } from './components/pages/OptimizationPage';
import { ResultsPage } from './components/pages/ResultsPage';
import { AnalysisResultPage } from './components/pages/AnalysisResultPage';
import { HistoryPage } from './components/pages/HistoryPage';
import { SettingsPage } from './components/pages/SettingsPage';
import { Toaster } from './components/ui/toaster';
import { ErrorBoundary } from './components/ui/error-boundary';

function App() {
  const {
    activeTab,
    isDarkMode,
    userPreferences,
    currentDetailedAnalysis,
    setActiveTab,
    setOllamaConnection,
    setModels,
    startConnectionMonitoring,
  } = useAppStore();

  // Initialize user preferences
  useUserPreferences();

  useEffect(() => {
    // Apply theme classes to html element
    const html = document.documentElement;

    if (userPreferences?.theme === 'HighContrast') {
      html.classList.add('dark', 'high-contrast');
      html.classList.remove('light');
    } else if (isDarkMode) {
      html.classList.add('dark');
      html.classList.remove('light', 'high-contrast');
    } else {
      html.classList.add('light');
      html.classList.remove('dark', 'high-contrast');
    }
  }, [isDarkMode, userPreferences?.theme]);

  // Auto-connect to Ollama on startup if enabled
  useEffect(() => {
    const autoConnect = async () => {
      if (userPreferences?.auto_connect_on_startup) {
        try {
          const result = await invoke<{ success: boolean }>(
            'test_ollama_connection'
          );
          if (result.success) {
            setOllamaConnection(true);
            // Fetch available models
            const modelsResult = await invoke<{
              success: boolean;
              data?: OllamaModel[];
            }>('get_ollama_models');
            if (modelsResult.success && modelsResult.data) {
              setModels(modelsResult.data);
            }
          } else {
            setOllamaConnection(false);
          }
        } catch {
          setOllamaConnection(false);
        }
      }
    };

    void autoConnect();
  }, [
    userPreferences?.auto_connect_on_startup,
    setOllamaConnection,
    setModels,
  ]);

  // Start connection monitoring when app initializes
  useEffect(() => {
    startConnectionMonitoring();
  }, [startConnectionMonitoring]);

  const renderActivePage = () => {
    switch (activeTab) {
      case 'dashboard':
        return (
          <ErrorBoundary>
            <Dashboard />
          </ErrorBoundary>
        );
      case 'analysis':
        return (
          <ErrorBoundary>
            <AnalysisPage />
          </ErrorBoundary>
        );
      case 'jobs':
        return (
          <ErrorBoundary>
            <JobsPage />
          </ErrorBoundary>
        );
      case 'optimization':
        return (
          <ErrorBoundary>
            <OptimizationPage />
          </ErrorBoundary>
        );
      case 'results':
        return (
          <ErrorBoundary>
            <ResultsPage />
          </ErrorBoundary>
        );
      case 'history':
        return (
          <ErrorBoundary>
            <HistoryPage />
          </ErrorBoundary>
        );
      case 'settings':
        return (
          <ErrorBoundary>
            <SettingsPage />
          </ErrorBoundary>
        );
      case 'analysis-result':
        return (
          <ErrorBoundary>
            <AnalysisResultPage
              analysisData={currentDetailedAnalysis}
              onBack={() => setActiveTab('analysis')}
            />
          </ErrorBoundary>
        );
      default:
        return (
          <ErrorBoundary>
            <Dashboard />
          </ErrorBoundary>
        );
    }
  };

  return (
    <MainLayout>
      {renderActivePage()}
      <Toaster />
    </MainLayout>
  );
}

export default App;
