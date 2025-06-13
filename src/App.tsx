import React, { useState, useEffect } from 'react';
import { Resume } from './types';
import { healthAPI } from './utils/api';
import Dashboard from './pages/Dashboard';
import ResultsView from './pages/ResultsView';
import LoadingSpinner from './components/Common/LoadingSpinner';
import ErrorMessage from './components/Common/ErrorMessage';

type AppState = 'dashboard' | 'results';

function App() {
  const [currentView, setCurrentView] = useState<AppState>('dashboard');
  const [selectedScanId, setSelectedScanId] = useState<number | null>(null);
  const [isHealthy, setIsHealthy] = useState<boolean | null>(null);
  const [healthError, setHealthError] = useState<string | null>(null);

  useEffect(() => {
    checkHealth();
  }, []);

  const checkHealth = async () => {
    try {
      await healthAPI.check();
      setIsHealthy(true);
      setHealthError(null);
    } catch (error: any) {
      setIsHealthy(false);
      setHealthError(error.message || 'Server is not responding');
    }
  };

  const handleViewResults = (resume: Resume) => {
    if (resume.last_scan_id) {
      setSelectedScanId(resume.last_scan_id);
      setCurrentView('results');
    } else {
      console.warn('No scan ID found for resume:', resume.original_name);
    }
  };

  const handleBackToDashboard = () => {
    setCurrentView('dashboard');
    setSelectedScanId(null);
  };

  // Health check loading state
  if (isHealthy === null) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <LoadingSpinner size="lg" text="Connecting to server..." />
      </div>
    );
  }

  // Health check failed
  if (!isHealthy) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="max-w-md mx-auto text-center">
          <div className="card">
            <div className="w-16 h-16 mx-auto mb-4 bg-red-100 rounded-full flex items-center justify-center">
              <svg className="w-8 h-8 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            
            <h1 className="text-xl font-bold text-gray-900 mb-2">Server Connection Failed</h1>
            <p className="text-gray-600 mb-4">
              Unable to connect to the ATS Scanner server. Please make sure the server is running.
            </p>
            
            <ErrorMessage message={healthError || 'Unknown error'} onRetry={checkHealth} />
            
            <div className="mt-6 bg-blue-50 rounded-lg p-4">
              <h3 className="font-medium text-blue-900 mb-2">💡 Quick Setup:</h3>
              <ol className="text-sm text-blue-800 text-left space-y-1">
                <li>1. Open a terminal in the project directory</li>
                <li>2. Run: <code className="bg-blue-100 px-1 rounded">npm run dev</code></li>
                <li>3. Wait for "Server running on http://localhost:3001"</li>
                <li>4. Refresh this page</li>
              </ol>
            </div>
          </div>
        </div>
      </div>
    );
  }

  // Main app content
  return (
    <div className="App">
      {currentView === 'dashboard' && (
        <Dashboard onViewResults={handleViewResults} />
      )}
      
      {currentView === 'results' && selectedScanId && (
        <ResultsView
          scanId={selectedScanId}
          onBack={handleBackToDashboard}
        />
      )}
    </div>
  );
}

export default App;