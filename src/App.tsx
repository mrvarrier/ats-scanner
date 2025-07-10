import React, { useEffect } from 'react';
import { useAppStore } from './store/useAppStore';
import { useUserPreferences } from './hooks/useUserPreferences';
import { MainLayout } from './components/layout/MainLayout';
import { Dashboard } from './components/pages/Dashboard';
import { AnalysisPage } from './components/pages/AnalysisPage';
import { OptimizationPage } from './components/pages/OptimizationPage';
import { ResultsPage } from './components/pages/ResultsPage';
import { AnalysisResultPage } from './components/pages/AnalysisResultPage';
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

  // Initialize light theme on component mount
  useEffect(() => {
    const html = document.documentElement;
    html.classList.add('light');
    html.classList.remove('dark', 'high-contrast');
  }, []);

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
