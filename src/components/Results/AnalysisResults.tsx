import React from 'react';
import { AnalysisResult, Resume } from '../../types';
import ProfessionalAnalysisResults from './ProfessionalAnalysisResults';

interface AnalysisResultsProps {
  analysis: AnalysisResult;
  resumeName: string;
  jobTitle?: string;
  company?: string;
  resume?: Resume;
}

const AnalysisResults: React.FC<AnalysisResultsProps> = ({
  analysis,
  resumeName,
  jobTitle,
  company,
  resume
}) => {
  // Use the new professional analysis results component
  return (
    <ProfessionalAnalysisResults
      analysis={analysis}
      resumeName={resumeName}
      jobTitle={jobTitle}
      company={company}
      resume={resume}
    />
  );
};

export default AnalysisResults;