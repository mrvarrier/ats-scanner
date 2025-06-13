import React, { useState, useEffect } from 'react';
import { Resume } from '../../types';
import { resumeAPI } from '../../utils/api';
import ResumeCard from './ResumeCard';
import LoadingSpinner from '../Common/LoadingSpinner';
import ErrorMessage from '../Common/ErrorMessage';

interface ResumeHistoryProps {
  onRescan: (resume: Resume) => void;
  onViewResults: (resume: Resume) => void;
  refreshTrigger?: number;
}

const ResumeHistory: React.FC<ResumeHistoryProps> = ({ 
  onRescan, 
  onViewResults,
  refreshTrigger 
}) => {
  const [resumes, setResumes] = useState<Resume[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [sortBy, setSortBy] = useState<'date' | 'name' | 'score'>('date');

  useEffect(() => {
    loadResumes();
  }, [refreshTrigger]);

  const loadResumes = async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await resumeAPI.getAll();
      setResumes(data);
    } catch (err: any) {
      setError(err.response?.data?.error || err.message || 'Failed to load resumes');
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (resume: Resume) => {
    if (!window.confirm(`Are you sure you want to delete "${resume.original_name}"? This action cannot be undone.`)) {
      return;
    }

    try {
      await resumeAPI.delete(resume.id);
      setResumes(prev => prev.filter(r => r.id !== resume.id));
    } catch (err: any) {
      alert(err.response?.data?.error || err.message || 'Failed to delete resume');
    }
  };

  const filteredAndSortedResumes = React.useMemo(() => {
    let filtered = resumes.filter(resume =>
      resume.original_name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      (resume.last_job_title?.toLowerCase().includes(searchTerm.toLowerCase())) ||
      (resume.last_company?.toLowerCase().includes(searchTerm.toLowerCase()))
    );

    filtered.sort((a, b) => {
      switch (sortBy) {
        case 'name':
          return a.original_name.localeCompare(b.original_name);
        case 'score':
          return (b.last_score || 0) - (a.last_score || 0);
        case 'date':
        default:
          return new Date(b.upload_date).getTime() - new Date(a.upload_date).getTime();
      }
    });

    return filtered;
  }, [resumes, searchTerm, sortBy]);

  if (loading) {
    return <LoadingSpinner size="lg" text="Loading your resumes..." />;
  }

  if (error) {
    return <ErrorMessage message={error} onRetry={loadResumes} />;
  }

  return (
    <div className="space-y-6">
      {resumes.length > 0 ? (
        <>
          <div className="flex flex-col sm:flex-row gap-4 items-start sm:items-center justify-between">
            <div className="flex-1 max-w-md">
              <input
                type="text"
                placeholder="Search resumes..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="input-field"
              />
            </div>
            
            <div className="flex items-center space-x-4">
              <label className="text-sm text-gray-600">Sort by:</label>
              <select
                value={sortBy}
                onChange={(e) => setSortBy(e.target.value as 'date' | 'name' | 'score')}
                className="input-field w-auto"
              >
                <option value="date">Upload Date</option>
                <option value="name">Name</option>
                <option value="score">Last Score</option>
              </select>
            </div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {filteredAndSortedResumes.map((resume) => (
              <ResumeCard
                key={resume.id}
                resume={resume}
                onRescan={onRescan}
                onViewResults={onViewResults}
                onDelete={handleDelete}
              />
            ))}
          </div>

          {filteredAndSortedResumes.length === 0 && searchTerm && (
            <div className="text-center py-8">
              <p className="text-gray-500">No resumes match your search criteria.</p>
              <button
                onClick={() => setSearchTerm('')}
                className="text-primary hover:text-blue-600 text-sm mt-2"
              >
                Clear search
              </button>
            </div>
          )}
        </>
      ) : (
        <div className="text-center py-12">
          <div className="w-24 h-24 mx-auto mb-6 bg-gray-100 rounded-full flex items-center justify-center">
            <svg className="w-12 h-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
          </div>
          
          <h3 className="text-lg font-medium text-gray-900 mb-2">No resumes yet</h3>
          <p className="text-gray-500 mb-6 max-w-sm mx-auto">
            Upload your first resume to start analyzing how well it matches job descriptions.
          </p>
          
          <div className="bg-blue-50 rounded-lg p-6 max-w-md mx-auto">
            <h4 className="font-medium text-blue-900 mb-2">🚀 Get started in 3 steps:</h4>
            <ol className="text-sm text-blue-800 text-left space-y-1">
              <li>1. Switch to the "New Scan" tab</li>
              <li>2. Upload your resume (PDF or Word)</li>
              <li>3. Paste a job description and analyze</li>
            </ol>
          </div>
        </div>
      )}
    </div>
  );
};

export default ResumeHistory;