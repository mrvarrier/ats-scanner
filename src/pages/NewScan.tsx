import React, { useState } from 'react';
import { Resume, AnalysisResult } from '../types';
import { scanAPI } from '../utils/api';
import FileUpload from '../components/Upload/FileUpload';
import JobDescriptionInput from '../components/Upload/JobDescriptionInput';
import ModelSelector from '../components/Upload/ModelSelector';
import LoadingSpinner from '../components/Common/LoadingSpinner';
import ErrorMessage from '../components/Common/ErrorMessage';
import AnalysisResults from '../components/Results/AnalysisResults';

interface NewScanProps {
  onScanComplete?: () => void;
}

const NewScan: React.FC<NewScanProps> = ({ onScanComplete }) => {
  const [currentStep, setCurrentStep] = useState(1);
  const [uploadedResume, setUploadedResume] = useState<Resume | null>(null);
  const [jobDescription, setJobDescription] = useState('');
  const [jobTitle, setJobTitle] = useState('');
  const [company, setCompany] = useState('');
  const [selectedModel, setSelectedModel] = useState('mistral:latest');
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [analysisResult, setAnalysisResult] = useState<AnalysisResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleUploadSuccess = (resume: Resume) => {
    setUploadedResume(resume);
    setCurrentStep(2);
    setError(null);
  };

  const handleAnalyze = async () => {
    if (!uploadedResume || !jobDescription.trim()) {
      setError('Please upload a resume and provide a job description');
      return;
    }

    setIsAnalyzing(true);
    setError(null);

    try {
      const result = await scanAPI.create({
        resumeId: uploadedResume.id,
        jobDescription: jobDescription.trim(),
        model: selectedModel,
        jobTitle: jobTitle.trim() || undefined,
        company: company.trim() || undefined
      });

      setAnalysisResult(result.analysis);
      setCurrentStep(4);
      
      if (onScanComplete) {
        onScanComplete();
      }
    } catch (err: any) {
      setError(err.response?.data?.error || err.message || 'Analysis failed');
    } finally {
      setIsAnalyzing(false);
    }
  };

  const canProceedToAnalysis = uploadedResume && jobDescription.trim().length >= 50;

  const resetScan = () => {
    setCurrentStep(1);
    setUploadedResume(null);
    setJobDescription('');
    setJobTitle('');
    setCompany('');
    setSelectedModel('mistral:latest');
    setAnalysisResult(null);
    setError(null);
  };

  if (isAnalyzing) {
    return (
      <div className="max-w-2xl mx-auto">
        <div className="card text-center">
          <LoadingSpinner size="lg" text="Analyzing your resume..." />
          <div className="mt-6">
            <h3 className="text-lg font-medium text-gray-900 mb-2">AI Analysis in Progress</h3>
            <p className="text-gray-600 mb-4">
              We're comparing your resume against the job description using {selectedModel}.
              This typically takes 30-60 seconds.
            </p>
            <div className="bg-blue-50 rounded-lg p-4">
              <h4 className="font-medium text-blue-900 mb-2">What we're analyzing:</h4>
              <ul className="text-sm text-blue-800 space-y-1">
                <li>• Skills alignment and gaps</li>
                <li>• Experience level matching</li>
                <li>• Keyword optimization</li>
                <li>• ATS compatibility</li>
                <li>• Improvement recommendations</li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    );
  }

  if (analysisResult) {
    return (
      <div>
        <div className="mb-6">
          <button
            onClick={resetScan}
            className="btn-secondary"
          >
            ← Start New Analysis
          </button>
        </div>
        <AnalysisResults
          analysis={analysisResult}
          resumeName={uploadedResume?.original_name || 'Unknown Resume'}
          jobTitle={jobTitle}
          company={company}
        />
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto space-y-8">
      {/* Progress Indicator */}
      <div className="flex items-center justify-center space-x-4 mb-8">
        {[1, 2, 3].map((step) => (
          <div key={step} className="flex items-center">
            <div className={`w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium ${
              currentStep >= step
                ? 'bg-primary text-white'
                : 'bg-gray-200 text-gray-600'
            }`}>
              {step}
            </div>
            {step < 3 && (
              <div className={`w-12 h-1 mx-2 ${
                currentStep > step ? 'bg-primary' : 'bg-gray-200'
              }`} />
            )}
          </div>
        ))}
      </div>

      <div className="text-center mb-8">
        <h2 className="text-xl font-medium text-gray-900">
          {currentStep === 1 && 'Upload Your Resume'}
          {currentStep === 2 && 'Job Description & Settings'}
          {currentStep === 3 && 'Review & Analyze'}
        </h2>
        <p className="text-gray-600 mt-1">
          {currentStep === 1 && 'Start by uploading your resume (PDF or Word document)'}
          {currentStep === 2 && 'Provide the job description and select your AI model'}
          {currentStep === 3 && 'Review your settings and start the analysis'}
        </p>
      </div>

      {error && (
        <ErrorMessage message={error} onRetry={() => setError(null)} />
      )}

      {/* Step 1: Resume Upload */}
      {currentStep === 1 && (
        <FileUpload onUploadSuccess={handleUploadSuccess} />
      )}

      {/* Step 2: Job Description & Settings */}
      {currentStep >= 2 && (
        <div className="space-y-6">
          {uploadedResume && (
            <div className="card bg-green-50 border-green-200">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <svg className="w-5 h-5 text-green-400" fill="currentColor" viewBox="0 0 20 20">
                    <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
                  </svg>
                </div>
                <div className="ml-3">
                  <p className="text-sm font-medium text-green-800">
                    Resume uploaded: {uploadedResume.original_name}
                  </p>
                </div>
                <button
                  onClick={() => setCurrentStep(1)}
                  className="ml-auto text-sm text-green-600 hover:text-green-800"
                >
                  Change
                </button>
              </div>
            </div>
          )}

          <JobDescriptionInput
            value={jobDescription}
            onChange={setJobDescription}
            jobTitle={jobTitle}
            onJobTitleChange={setJobTitle}
            company={company}
            onCompanyChange={setCompany}
          />

          <ModelSelector
            selectedModel={selectedModel}
            onModelChange={setSelectedModel}
          />

          {currentStep === 2 && (
            <div className="flex justify-center">
              <button
                onClick={() => setCurrentStep(3)}
                disabled={!canProceedToAnalysis}
                className="btn-primary px-8"
              >
                Continue to Review
              </button>
            </div>
          )}
        </div>
      )}

      {/* Step 3: Review & Analyze */}
      {currentStep === 3 && (
        <div className="space-y-6">
          <div className="card">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Review Your Analysis Settings</h3>
            
            <div className="space-y-4">
              <div className="flex justify-between items-center py-2 border-b border-gray-200">
                <span className="font-medium text-gray-700">Resume:</span>
                <span className="text-gray-900">{uploadedResume?.original_name}</span>
              </div>
              
              <div className="flex justify-between items-center py-2 border-b border-gray-200">
                <span className="font-medium text-gray-700">Job Title:</span>
                <span className="text-gray-900">{jobTitle || 'Not specified'}</span>
              </div>
              
              <div className="flex justify-between items-center py-2 border-b border-gray-200">
                <span className="font-medium text-gray-700">Company:</span>
                <span className="text-gray-900">{company || 'Not specified'}</span>
              </div>
              
              <div className="flex justify-between items-center py-2 border-b border-gray-200">
                <span className="font-medium text-gray-700">AI Model:</span>
                <span className="text-gray-900">{selectedModel}</span>
              </div>
              
              <div className="flex justify-between items-start py-2">
                <span className="font-medium text-gray-700">Job Description:</span>
                <span className="text-gray-900 text-right max-w-md">
                  {jobDescription.length.toLocaleString()} characters
                </span>
              </div>
            </div>
          </div>

          <div className="flex justify-center space-x-4">
            <button
              onClick={() => setCurrentStep(2)}
              className="btn-secondary px-8"
            >
              Back to Edit
            </button>
            <button
              onClick={handleAnalyze}
              disabled={!canProceedToAnalysis}
              className="btn-primary px-8"
            >
              Start Analysis
            </button>
          </div>
        </div>
      )}
    </div>
  );
};

export default NewScan;