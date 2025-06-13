import React, { useState } from 'react';
import { Resume } from '../types';
import ResumeHistory from '../components/Dashboard/ResumeHistory';
import NewScan from '../pages/NewScan';

interface DashboardProps {
  onViewResults: (resume: Resume) => void;
}

const Dashboard: React.FC<DashboardProps> = ({ onViewResults }) => {
  const [activeTab, setActiveTab] = useState<'history' | 'new'>('history');
  const [refreshTrigger, setRefreshTrigger] = useState(0);

  const handleRescan = (resume: Resume) => {
    setActiveTab('new');
  };

  const handleScanComplete = () => {
    setActiveTab('history');
    setRefreshTrigger(prev => prev + 1);
  };

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Header */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900">Personal ATS Scanner</h1>
          <p className="text-gray-600 mt-2">
            Analyze how well your resume matches job descriptions using AI
          </p>
        </div>

        {/* Tab Navigation */}
        <div className="border-b border-gray-200 mb-8">
          <nav className="-mb-px flex space-x-8">
            <button
              onClick={() => setActiveTab('history')}
              className={`py-2 px-1 border-b-2 font-medium text-sm ${
                activeTab === 'history'
                  ? 'border-primary text-primary'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              Resume History
            </button>
            <button
              onClick={() => setActiveTab('new')}
              className={`py-2 px-1 border-b-2 font-medium text-sm ${
                activeTab === 'new'
                  ? 'border-primary text-primary'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              New Scan
            </button>
          </nav>
        </div>

        {/* Tab Content */}
        <div className="tab-content">
          {activeTab === 'history' ? (
            <ResumeHistory
              onRescan={handleRescan}
              onViewResults={onViewResults}
              refreshTrigger={refreshTrigger}
            />
          ) : (
            <NewScan onScanComplete={handleScanComplete} />
          )}
        </div>
      </div>
    </div>
  );
};

export default Dashboard;