import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useAppStore } from "./store/useAppStore";
import { useUserPreferences } from "./hooks/useUserPreferences";
import { MainLayout } from "./components/layout/MainLayout";
import { Dashboard } from "./components/pages/Dashboard";
import { AnalysisPage } from "./components/pages/AnalysisPage";
import { OptimizationPage } from "./components/pages/OptimizationPage";
import { ResultsPage } from "./components/pages/ResultsPage";
import { AnalysisResultPage } from "./components/pages/AnalysisResultPage";
import { SettingsPage } from "./components/pages/SettingsPage";
import { Toaster } from "./components/ui/toaster";
import { ErrorBoundary } from "./components/ui/error-boundary";

function App() {
  const { 
    activeTab, 
    setModels, 
    setOllamaConnection, 
    setAnalysisHistory,
    isDarkMode,
    userPreferences,
    setSelectedModel,
    currentDetailedAnalysis,
    setActiveTab,
    isOllamaConnected
  } = useAppStore();
  
  // Initialize user preferences
  const { userPreferences: loadedPreferences } = useUserPreferences();

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

  // Initialize light theme on component mount
  useEffect(() => {
    const html = document.documentElement;
    html.classList.add('light');
    html.classList.remove('dark', 'high-contrast');
  }, []);

  useEffect(() => {
    // Initialize app data when preferences are available
    const initializeApp = async () => {
      try {
        // Only auto-connect if user preferences allow it
        const shouldAutoConnect = !userPreferences || userPreferences.auto_connect_on_startup;
        
        if (shouldAutoConnect) {
          // Test Ollama connection
          const connectionResult = await invoke<any>('test_ollama_connection');
          if (connectionResult.success && connectionResult.data) {
            setOllamaConnection(true);
            
            // Get available models
            const modelsResult = await invoke<any>('get_ollama_models');
            if (modelsResult.success) {
              const models = modelsResult.data || [];
              setModels(models);
              
              // Auto-select default model if set in preferences
              if (userPreferences?.default_model && models.some((m: any) => m.name === userPreferences.default_model)) {
                setSelectedModel(userPreferences.default_model);
              }
            }
          } else {
            setOllamaConnection(false);
          }
        }

        // Get analysis history (respecting retention settings)
        const retentionDays = userPreferences?.analysis_history_retention_days || 90;
        const historyResult = await invoke<any>('get_analysis_history', { 
          limit: 50,
          days: retentionDays 
        });
        if (historyResult.success) {
          setAnalysisHistory(historyResult.data || []);
        }

      } catch (error) {
        console.error('Failed to initialize app:', error);
      }
    };

    // Only initialize when preferences have been loaded (or failed to load)
    if (loadedPreferences !== undefined) {
      initializeApp();
    }
  }, [setModels, setOllamaConnection, setAnalysisHistory, userPreferences, loadedPreferences, setSelectedModel]);

  // Periodic Ollama connection monitoring
  useEffect(() => {
    const checkOllamaConnection = async () => {
      try {
        const result = await invoke<any>('test_ollama_connection');
        const isConnected = result.success && result.data;
        
        // Only update if connection status changed
        if (isConnected !== isOllamaConnected) {
          setOllamaConnection(isConnected);
          
          // If connected, refresh models list
          if (isConnected) {
            const modelsResult = await invoke<any>('get_ollama_models');
            if (modelsResult.success) {
              setModels(modelsResult.data || []);
            }
          } else {
            // If disconnected, clear models
            setModels([]);
            setSelectedModel('');
          }
        }
      } catch (error) {
        console.error('Failed to check Ollama connection:', error);
        setOllamaConnection(false);
        setModels([]);
        setSelectedModel(null);
      }
    };

    // Start periodic monitoring after app is initialized
    if (loadedPreferences !== undefined) {
      const interval = setInterval(checkOllamaConnection, 5000); // Check every 5 seconds
      
      return () => clearInterval(interval);
    }
  }, [loadedPreferences, isOllamaConnected, setOllamaConnection, setModels, setSelectedModel]);

  const renderActivePage = () => {
    switch (activeTab) {
      case 'dashboard':
        return (
          <ErrorBoundary onError={(error) => console.error('Dashboard error:', error)}>
            <Dashboard />
          </ErrorBoundary>
        );
      case 'analysis':
        return (
          <ErrorBoundary onError={(error) => console.error('Analysis page error:', error)}>
            <AnalysisPage />
          </ErrorBoundary>
        );
      case 'optimization':
        return (
          <ErrorBoundary onError={(error) => console.error('Optimization page error:', error)}>
            <OptimizationPage />
          </ErrorBoundary>
        );
      case 'results':
        return (
          <ErrorBoundary onError={(error) => console.error('Results page error:', error)}>
            <ResultsPage />
          </ErrorBoundary>
        );
      case 'settings':
        return (
          <ErrorBoundary onError={(error) => console.error('Settings page error:', error)}>
            <SettingsPage />
          </ErrorBoundary>
        );
      case 'analysis-result':
        return (
          <ErrorBoundary onError={(error) => console.error('Analysis result page error:', error)}>
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