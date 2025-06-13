import React, { useState, useEffect } from 'react';
import { Scan, Resume } from '../types';
import { scanAPI, resumeAPI } from '../utils/api';
import AnalysisResults from '../components/Results/AnalysisResults';
import LoadingSpinner from '../components/Common/LoadingSpinner';
import ErrorMessage from '../components/Common/ErrorMessage';

interface ResultsViewProps {
  scanId: number;
  onBack: () => void;
}

const ResultsView: React.FC<ResultsViewProps> = ({ scanId, onBack }) => {
  const [scan, setScan] = useState<Scan | null>(null);
  const [resume, setResume] = useState<Resume | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadScan();
  }, [scanId]);

  const loadScan = async () => {
    try {
      setLoading(true);
      setError(null);
      const scanData = await scanAPI.getById(scanId);
      setScan(scanData);
      
      // Fetch resume data if resume_id is available
      if (scanData.resume_id) {
        try {
          const resumeData = await resumeAPI.getById(scanData.resume_id);
          setResume(resumeData);
        } catch (resumeErr) {
          console.warn('Failed to load resume data:', resumeErr);
          // Continue without resume data rather than failing completely
        }
      }
    } catch (err: any) {
      setError(err.response?.data?.error || err.message || 'Failed to load scan results');
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <LoadingSpinner size="lg" text="Loading analysis results..." />
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="max-w-md mx-auto">
          <ErrorMessage message={error} onRetry={loadScan} />
          <div className="text-center mt-4">
            <button onClick={onBack} className="btn-secondary">
              ← Back to Dashboard
            </button>
          </div>
        </div>
      </div>
    );
  }

  if (!scan || !scan.analysis) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <p className="text-gray-500">No analysis data found.</p>
          <button onClick={onBack} className="btn-secondary mt-4">
            ← Back to Dashboard
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="mb-6">
          <button onClick={onBack} className="btn-secondary">
            ← Back to Dashboard
          </button>
        </div>
        
        <AnalysisResults
          analysis={scan.analysis}
          resumeName={scan.resume_name || 'Unknown Resume'}
          jobTitle={scan.job_title}
          company={scan.company}
          resume={resume || undefined}
        />
      </div>
    </div>
  );
};

export default ResultsView;