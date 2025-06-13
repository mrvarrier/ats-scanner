import React from 'react';
import { Resume } from '../../types';
import { formatDate, formatFileSize, truncateText } from '../../utils/helpers';
import ScoreCircle from '../Common/ScoreCircle';

interface ResumeCardProps {
  resume: Resume;
  onRescan: (resume: Resume) => void;
  onViewResults: (resume: Resume) => void;
  onDelete: (resume: Resume) => void;
}

const ResumeCard: React.FC<ResumeCardProps> = ({
  resume,
  onRescan,
  onViewResults,
  onDelete
}) => {
  const hasScans = resume.last_score !== undefined && resume.last_scan_id !== undefined;

  return (
    <div className="card hover:shadow-md transition-shadow duration-200">
      <div className="flex justify-between items-start mb-4">
        <div className="flex-1 min-w-0">
          <h3 className="text-lg font-medium text-gray-900 truncate" title={resume.original_name}>
            {resume.original_name}
          </h3>
          <p className="text-sm text-gray-500 mt-1">
            Uploaded {formatDate(resume.upload_date)} • {formatFileSize(resume.file_size)}
          </p>
        </div>
        
        {hasScans && resume.last_score !== undefined && (
          <div className="ml-4 flex-shrink-0">
            <ScoreCircle score={resume.last_score} size="sm" showLabel={false} />
          </div>
        )}
      </div>

      {hasScans ? (
        <div className="space-y-3">
          <div className="bg-gray-50 rounded-lg p-3">
            <p className="text-sm font-medium text-gray-900">Last Analysis</p>
            <p className="text-sm text-gray-600 mt-1">
              {resume.last_job_title && `${resume.last_job_title} at `}
              {resume.last_company || 'Unknown Company'}
            </p>
            <p className="text-xs text-gray-500 mt-1">
              {resume.last_scan_date && formatDate(resume.last_scan_date)}
            </p>
          </div>
          
          <div className="flex space-x-2">
            <button
              onClick={() => {
                if (resume.last_scan_id) {
                  onViewResults(resume);
                } else {
                  alert('No analysis results found. Please run a scan first.');
                }
              }}
              className={`flex-1 text-sm py-2 ${
                resume.last_scan_id 
                  ? 'btn-primary' 
                  : 'bg-gray-300 text-gray-500 cursor-not-allowed'
              }`}
              disabled={!resume.last_scan_id}
            >
              View Results
            </button>
            <button
              onClick={() => onRescan(resume)}
              className="flex-1 btn-secondary text-sm py-2"
            >
              Re-scan
            </button>
          </div>
        </div>
      ) : (
        <div className="space-y-3">
          <div className="bg-blue-50 rounded-lg p-3">
            <p className="text-sm text-blue-900 font-medium">Ready to analyze</p>
            <p className="text-sm text-blue-700 mt-1">
              This resume hasn't been scanned yet. Start your first analysis!
            </p>
          </div>
          
          <button
            onClick={() => onRescan(resume)}
            className="w-full btn-primary text-sm py-2"
          >
            Start Analysis
          </button>
        </div>
      )}

      <div className="mt-4 pt-4 border-t border-gray-200">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4 text-xs text-gray-500">
            <span>PDF/Word document</span>
            <span>•</span>
            <span>{resume.extracted_text.length.toLocaleString()} characters</span>
          </div>
          
          <button
            onClick={() => onDelete(resume)}
            className="text-xs text-red-600 hover:text-red-800 transition-colors"
          >
            Delete
          </button>
        </div>
        
        {resume.extracted_text && (
          <div className="mt-2">
            <p className="text-xs text-gray-500 mb-1">Preview:</p>
            <p className="text-xs text-gray-600 bg-gray-50 rounded p-2 font-mono">
              {truncateText(resume.extracted_text, 150)}
            </p>
          </div>
        )}
      </div>
    </div>
  );
};

export default ResumeCard;