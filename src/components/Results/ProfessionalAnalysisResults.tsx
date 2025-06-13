import React, { useState } from 'react';
import { AnalysisResult, Resume } from '../../types';
import { downloadAsJSON } from '../../utils/helpers';
import ScoreCircle from '../Common/ScoreCircle';
import ResumeParsingDisplay from './ResumeParsingDisplay';
import ProfessionalScoringBreakdown from './ProfessionalScoringBreakdown';

interface ProfessionalAnalysisResultsProps {
  analysis: AnalysisResult;
  resumeName: string;
  jobTitle?: string;
  company?: string;
  resume?: Resume;
}

const ProfessionalAnalysisResults: React.FC<ProfessionalAnalysisResultsProps> = ({
  analysis,
  resumeName,
  jobTitle,
  company,
  resume
}) => {
  const [activeTab, setActiveTab] = useState<'analysis' | 'scoring' | 'resume'>('analysis');
  const handleExportReport = () => {
    const exportData = {
      resume: resumeName,
      job: jobTitle ? `${jobTitle} at ${company}` : company,
      analysis_date: new Date().toISOString(),
      ...analysis
    };
    
    downloadAsJSON(exportData, `ats-analysis-${resumeName.replace(/\s+/g, '-')}.json`);
  };

  if (analysis.error) {
    return (
      <div className="card">
        <div className="text-center py-8">
          <div className="w-16 h-16 mx-auto mb-4 bg-red-100 rounded-full flex items-center justify-center">
            <svg className="w-8 h-8 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </div>
          <h3 className="text-lg font-medium text-gray-900 mb-2">Analysis Error</h3>
          <p className="text-gray-600 mb-4">{analysis.error}</p>
          {analysis.raw_response && (
            <details className="text-left bg-gray-50 rounded-lg p-4">
              <summary className="cursor-pointer font-medium">Raw Response</summary>
              <pre className="mt-2 text-xs overflow-x-auto">{analysis.raw_response}</pre>
            </details>
          )}
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-8">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">ATS Analysis Results</h1>
          <p className="text-gray-600 mt-1">
            {resumeName} • {jobTitle && `${jobTitle} at `}{company}
          </p>
        </div>
        <button
          onClick={handleExportReport}
          className="btn-secondary"
        >
          Export Report
        </button>
      </div>

      {/* Tab Navigation */}
      <div className="border-b border-gray-200">
        <nav className="-mb-px flex space-x-8">
          <button
            onClick={() => setActiveTab('analysis')}
            className={`py-2 px-1 border-b-2 font-medium text-sm ${
              activeTab === 'analysis'
                ? 'border-primary text-primary'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            }`}
          >
            Analysis Results
          </button>
          {analysis.professional_scoring && (
            <button
              onClick={() => setActiveTab('scoring')}
              className={`py-2 px-1 border-b-2 font-medium text-sm ${
                activeTab === 'scoring'
                  ? 'border-primary text-primary'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              Score Breakdown
            </button>
          )}
          {resume && (
            <button
              onClick={() => setActiveTab('resume')}
              className={`py-2 px-1 border-b-2 font-medium text-sm ${
                activeTab === 'resume'
                  ? 'border-primary text-primary'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              Resume Parsing
            </button>
          )}
        </nav>
      </div>

      {/* Tab Content */}
      {activeTab === 'analysis' ? (
        <div className="space-y-8">
          {/* Overall Score & ATS Compatibility */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="card">
          <div className="text-center">
            <ScoreCircle score={analysis.overall_score} size="lg" />
            
            {analysis.ats_compatibility && (
              <div className="mt-4">
                <div className={`inline-flex items-center px-3 py-1 rounded-full text-sm font-medium ${
                  analysis.ats_compatibility.likely_to_pass_screening 
                    ? 'bg-green-100 text-green-800' 
                    : 'bg-red-100 text-red-800'
                }`}>
                  {analysis.ats_compatibility.likely_to_pass_screening ? '✓ Likely to Pass ATS' : '✗ May Not Pass ATS'}
                </div>
              </div>
            )}
          </div>
        </div>

        {/* ATS Compatibility Details */}
        {analysis.ats_compatibility && (
          <div className="card">
            <h3 className="text-lg font-medium text-gray-900 mb-4">ATS Compatibility</h3>
            <div className="space-y-3">
              <div className="flex justify-between items-center">
                <span className="text-gray-600">Compatibility Score:</span>
                <span className="font-medium">{analysis.ats_compatibility.compatibility_score}%</span>
              </div>
              
              {analysis.ats_compatibility.format_issues.length > 0 && (
                <div>
                  <span className="text-sm font-medium text-red-700">Format Issues:</span>
                  <ul className="mt-1 space-y-1">
                    {analysis.ats_compatibility.format_issues.map((issue, index) => (
                      <li key={index} className="text-sm text-red-600 flex items-start">
                        <span className="text-red-500 mr-2">•</span>
                        {issue}
                      </li>
                    ))}
                  </ul>
                </div>
              )}
              
              {analysis.ats_compatibility.parsing_concerns.length > 0 && (
                <div>
                  <span className="text-sm font-medium text-yellow-700">Parsing Concerns:</span>
                  <ul className="mt-1 space-y-1">
                    {analysis.ats_compatibility.parsing_concerns.map((concern, index) => (
                      <li key={index} className="text-sm text-yellow-600 flex items-start">
                        <span className="text-yellow-500 mr-2">•</span>
                        {concern}
                      </li>
                    ))}
                  </ul>
                </div>
              )}
            </div>
          </div>
        )}
      </div>

      {/* Contact Information Analysis */}
      {analysis.contact_analysis && (
        <div className="card">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Contact Information</h3>
          <div className="grid grid-cols-2 md:grid-cols-5 gap-4">
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className={`text-2xl font-bold ${analysis.contact_analysis.has_email ? 'text-success' : 'text-error'}`}>
                {analysis.contact_analysis.has_email ? '✓' : '✗'}
              </div>
              <div className="text-sm text-gray-600">Email</div>
            </div>
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className={`text-2xl font-bold ${analysis.contact_analysis.has_phone ? 'text-success' : 'text-error'}`}>
                {analysis.contact_analysis.has_phone ? '✓' : '✗'}
              </div>
              <div className="text-sm text-gray-600">Phone</div>
            </div>
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className={`text-2xl font-bold ${analysis.contact_analysis.has_location ? 'text-success' : 'text-error'}`}>
                {analysis.contact_analysis.has_location ? '✓' : '✗'}
              </div>
              <div className="text-sm text-gray-600">Location</div>
            </div>
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className={`text-2xl font-bold ${analysis.contact_analysis.has_linkedin ? 'text-success' : 'text-error'}`}>
                {analysis.contact_analysis.has_linkedin ? '✓' : '✗'}
              </div>
              <div className="text-sm text-gray-600">LinkedIn</div>
            </div>
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className="text-2xl font-bold text-primary">{analysis.contact_analysis.contact_score}%</div>
              <div className="text-sm text-gray-600">Contact Score</div>
            </div>
          </div>
        </div>
      )}

      {/* Job Title Analysis */}
      {analysis.job_title_analysis && (
        <div className="card">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Job Title Alignment</h3>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <div className="space-y-2">
                <div className="flex justify-between">
                  <span className="text-gray-600">Current Title:</span>
                  <span className="font-medium">{analysis.job_title_analysis.current_title}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Target Title:</span>
                  <span className="font-medium">{analysis.job_title_analysis.target_title}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Match Level:</span>
                  <span className={`font-medium capitalize ${
                    analysis.job_title_analysis.title_match === 'exact' ? 'text-success' :
                    analysis.job_title_analysis.title_match === 'similar' ? 'text-warning' :
                    'text-error'
                  }`}>
                    {analysis.job_title_analysis.title_match}
                  </span>
                </div>
              </div>
            </div>
            <div className="text-center">
              <div className="text-3xl font-bold text-primary">{analysis.job_title_analysis.title_score}%</div>
              <div className="text-sm text-gray-600">Title Match Score</div>
            </div>
          </div>
        </div>
      )}

      {/* Hard Skills vs Soft Skills */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Hard Skills */}
        {analysis.hard_skills && (
          <div className="card">
            <h3 className="text-lg font-medium text-gray-900 mb-4">
              Hard Skills ({analysis.hard_skills.filter(s => s.found_in_resume).length}/{analysis.hard_skills.length})
            </h3>
            <div className="space-y-3 max-h-96 overflow-y-auto">
              {analysis.hard_skills.map((skill, index) => (
                <div key={index} className={`p-3 rounded-lg border-l-4 ${
                  skill.found_in_resume ? 'border-success bg-green-50' : 'border-error bg-red-50'
                }`}>
                  <div className="flex items-center justify-between mb-1">
                    <span className={`font-medium ${skill.found_in_resume ? 'text-green-900' : 'text-red-900'}`}>
                      {skill.skill}
                    </span>
                    <div className="flex space-x-1">
                      <span className={`px-2 py-1 rounded-full text-xs ${
                        skill.required_for_job ? 'bg-red-100 text-red-800' : 'bg-gray-100 text-gray-600'
                      }`}>
                        {skill.required_for_job ? 'Required' : 'Nice-to-have'}
                      </span>
                      <span className="px-2 py-1 bg-blue-100 text-blue-800 rounded-full text-xs">
                        {skill.skill_category}
                      </span>
                    </div>
                  </div>
                  {skill.evidence && (
                    <p className={`text-sm italic ${skill.found_in_resume ? 'text-green-700' : 'text-red-700'}`}>
                      "{skill.evidence}"
                    </p>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Soft Skills */}
        {analysis.soft_skills && (
          <div className="card">
            <h3 className="text-lg font-medium text-gray-900 mb-4">
              Soft Skills ({analysis.soft_skills.filter(s => s.found_in_resume).length}/{analysis.soft_skills.length})
            </h3>
            <div className="space-y-3 max-h-96 overflow-y-auto">
              {analysis.soft_skills.map((skill, index) => (
                <div key={index} className={`p-3 rounded-lg border-l-4 ${
                  skill.found_in_resume ? 'border-success bg-green-50' : 'border-error bg-red-50'
                }`}>
                  <div className="flex items-center justify-between mb-1">
                    <span className={`font-medium ${skill.found_in_resume ? 'text-green-900' : 'text-red-900'}`}>
                      {skill.skill}
                    </span>
                    <span className={`px-2 py-1 rounded-full text-xs ${
                      skill.required_for_job ? 'bg-red-100 text-red-800' : 'bg-gray-100 text-gray-600'
                    }`}>
                      {skill.required_for_job ? 'Required' : 'Nice-to-have'}
                    </span>
                  </div>
                  {skill.evidence && (
                    <p className={`text-sm italic ${skill.found_in_resume ? 'text-green-700' : 'text-red-700'}`}>
                      "{skill.evidence}"
                    </p>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* Experience Analysis */}
      {analysis.experience_analysis && (
        <div className="card">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Experience Analysis</h3>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className="font-medium text-gray-900">{analysis.experience_analysis.total_years_experience}</div>
              <div className="text-sm text-gray-600">Total Experience</div>
            </div>
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className="font-medium text-gray-900">{analysis.experience_analysis.required_years}</div>
              <div className="text-sm text-gray-600">Required</div>
            </div>
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className="font-medium text-gray-900">{analysis.experience_analysis.relevant_experience}</div>
              <div className="text-sm text-gray-600">Relevant Experience</div>
            </div>
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className={`font-medium capitalize ${
                analysis.experience_analysis.experience_match === 'exceeds' ? 'text-success' :
                analysis.experience_analysis.experience_match === 'meets' ? 'text-warning' :
                'text-error'
              }`}>
                {analysis.experience_analysis.experience_match}
              </div>
              <div className="text-sm text-gray-600">Experience Match</div>
            </div>
          </div>
          
          <div className="mt-4 grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className="font-medium text-gray-900 capitalize">{analysis.experience_analysis.current_level}</div>
              <div className="text-sm text-gray-600">Current Level</div>
            </div>
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className="font-medium text-gray-900 capitalize">{analysis.experience_analysis.required_level}</div>
              <div className="text-sm text-gray-600">Required Level</div>
            </div>
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className="font-medium text-gray-900 capitalize">{analysis.experience_analysis.career_progression}</div>
              <div className="text-sm text-gray-600">Career Progression</div>
            </div>
          </div>
        </div>
      )}

      {/* Keywords & ATS Optimization */}
      {analysis.keyword_optimization && (
        <div className="card">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Keyword Optimization</h3>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className="text-2xl font-bold text-gray-900">{analysis.keyword_optimization.total_job_keywords}</div>
              <div className="text-sm text-gray-600">Total Keywords</div>
            </div>
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className="text-2xl font-bold text-success">{analysis.keyword_optimization.total_matched_keywords}</div>
              <div className="text-sm text-gray-600">Matched</div>
            </div>
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className="text-2xl font-bold text-error">{analysis.keyword_optimization.critical_keywords_missing.length}</div>
              <div className="text-sm text-gray-600">Missing</div>
            </div>
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className="text-2xl font-bold text-primary">{analysis.keyword_optimization.keyword_density}%</div>
              <div className="text-sm text-gray-600">Density</div>
            </div>
          </div>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h4 className="font-medium text-success mb-2">
                Matched Keywords ({analysis.keyword_optimization.critical_keywords_matched.length})
              </h4>
              <div className="flex flex-wrap gap-2">
                {analysis.keyword_optimization.critical_keywords_matched.map((keyword, index) => (
                  <span key={index} className="px-2 py-1 bg-green-100 text-green-800 rounded-full text-sm">
                    {keyword}
                  </span>
                ))}
              </div>
            </div>
            <div>
              <h4 className="font-medium text-error mb-2">
                Missing Critical Keywords ({analysis.keyword_optimization.critical_keywords_missing.length})
              </h4>
              <div className="flex flex-wrap gap-2">
                {analysis.keyword_optimization.critical_keywords_missing.map((keyword, index) => (
                  <span key={index} className="px-2 py-1 bg-red-100 text-red-800 rounded-full text-sm">
                    {keyword}
                  </span>
                ))}
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Resume Structure Analysis */}
      {analysis.resume_structure && (
        <div className="card">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Resume Structure</h3>
          <div className="grid grid-cols-2 md:grid-cols-3 gap-4 mb-4">
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className="text-2xl font-bold text-primary">{analysis.resume_structure.structure_score}%</div>
              <div className="text-sm text-gray-600">Structure Score</div>
            </div>
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className={`text-lg font-bold capitalize ${
                analysis.resume_structure.section_organization === 'excellent' ? 'text-success' :
                analysis.resume_structure.section_organization === 'good' ? 'text-warning' :
                'text-error'
              }`}>
                {analysis.resume_structure.section_organization}
              </div>
              <div className="text-sm text-gray-600">Organization</div>
            </div>
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className={`text-2xl font-bold ${analysis.resume_structure.chronological_format ? 'text-success' : 'text-error'}`}>
                {analysis.resume_structure.chronological_format ? '✓' : '✗'}
              </div>
              <div className="text-sm text-gray-600">Chronological</div>
            </div>
          </div>
          
          <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
            <div className="flex items-center space-x-2">
              <span className={`w-4 h-4 rounded-full ${analysis.resume_structure.has_contact_info ? 'bg-success' : 'bg-error'}`}></span>
              <span className="text-sm">Contact Info</span>
            </div>
            <div className="flex items-center space-x-2">
              <span className={`w-4 h-4 rounded-full ${analysis.resume_structure.has_professional_summary ? 'bg-success' : 'bg-error'}`}></span>
              <span className="text-sm">Summary</span>
            </div>
            <div className="flex items-center space-x-2">
              <span className={`w-4 h-4 rounded-full ${analysis.resume_structure.has_skills_section ? 'bg-success' : 'bg-error'}`}></span>
              <span className="text-sm">Skills Section</span>
            </div>
            <div className="flex items-center space-x-2">
              <span className={`w-4 h-4 rounded-full ${analysis.resume_structure.has_work_experience ? 'bg-success' : 'bg-error'}`}></span>
              <span className="text-sm">Experience</span>
            </div>
            <div className="flex items-center space-x-2">
              <span className={`w-4 h-4 rounded-full ${analysis.resume_structure.has_education_section ? 'bg-success' : 'bg-error'}`}></span>
              <span className="text-sm">Education</span>
            </div>
          </div>
        </div>
      )}

      {/* Measurable Results */}
      {analysis.measurable_results && (
        <div className="card">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Quantified Achievements</h3>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className={`text-2xl font-bold ${analysis.measurable_results.has_quantified_achievements ? 'text-success' : 'text-error'}`}>
                {analysis.measurable_results.has_quantified_achievements ? 'Yes' : 'No'}
              </div>
              <div className="text-sm text-gray-600">Has Metrics</div>
            </div>
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className="text-2xl font-bold text-primary">{analysis.measurable_results.number_of_metrics}</div>
              <div className="text-sm text-gray-600">Total Metrics</div>
            </div>
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className="text-2xl font-bold text-warning">{analysis.measurable_results.results_score}%</div>
              <div className="text-sm text-gray-600">Results Score</div>
            </div>
          </div>
          
          {analysis.measurable_results.examples.length > 0 && (
            <div>
              <h4 className="font-medium text-gray-900 mb-2">Examples Found:</h4>
              <ul className="space-y-1">
                {analysis.measurable_results.examples.map((example, index) => (
                  <li key={index} className="text-sm text-gray-700 flex items-start">
                    <span className="text-green-500 mr-2">•</span>
                    {example}
                  </li>
                ))}
              </ul>
            </div>
          )}
        </div>
      )}

      {/* Industry Alignment */}
      {analysis.industry_alignment && (
        <div className="card">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Industry Alignment</h3>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className={`text-lg font-bold capitalize ${
                analysis.industry_alignment.industry_experience === 'direct' ? 'text-success' :
                analysis.industry_alignment.industry_experience === 'related' ? 'text-warning' :
                'text-error'
              }`}>
                {analysis.industry_alignment.industry_experience}
              </div>
              <div className="text-sm text-gray-600">Industry Match</div>
            </div>
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className="text-lg font-bold text-gray-900 capitalize">{analysis.industry_alignment.company_size_match}</div>
              <div className="text-sm text-gray-600">Company Size</div>
            </div>
            <div className="text-center p-3 bg-gray-50 rounded-lg">
              <div className="text-2xl font-bold text-primary">{analysis.industry_alignment.alignment_score}%</div>
              <div className="text-sm text-gray-600">Alignment Score</div>
            </div>
          </div>
          
          {analysis.industry_alignment.industry_keywords.length > 0 && (
            <div>
              <h4 className="font-medium text-gray-900 mb-2">Industry Keywords Found:</h4>
              <div className="flex flex-wrap gap-2">
                {analysis.industry_alignment.industry_keywords.map((keyword, index) => (
                  <span key={index} className="px-2 py-1 bg-blue-100 text-blue-800 rounded-full text-sm">
                    {keyword}
                  </span>
                ))}
              </div>
            </div>
          )}
        </div>
      )}

      {/* Recommendations */}
      {analysis.recommendations && analysis.recommendations.length > 0 && (
        <div className="card">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Improvement Recommendations</h3>
          <div className="space-y-4">
            {analysis.recommendations.map((rec, index) => (
              <div key={index} className={`border-l-4 p-4 rounded-r-lg ${
                rec.priority === 'high' ? 'border-red-500 bg-red-50' :
                rec.priority === 'medium' ? 'border-yellow-500 bg-yellow-50' :
                'border-blue-500 bg-blue-50'
              }`}>
                <div className="flex items-start justify-between mb-2">
                  <div className="flex items-center space-x-2">
                    <span className={`px-2 py-1 rounded-full text-xs font-medium ${
                      rec.priority === 'high' ? 'bg-red-200 text-red-800' :
                      rec.priority === 'medium' ? 'bg-yellow-200 text-yellow-800' :
                      'bg-blue-200 text-blue-800'
                    }`}>
                      {rec.priority} priority
                    </span>
                    <span className="px-2 py-1 bg-gray-200 text-gray-800 rounded-full text-xs font-medium capitalize">
                      {rec.category}
                    </span>
                  </div>
                  {rec.impact && (
                    <span className={`px-2 py-1 rounded-full text-xs font-medium ${
                      rec.impact === 'high' ? 'bg-green-200 text-green-800' :
                      rec.impact === 'medium' ? 'bg-yellow-200 text-yellow-800' :
                      'bg-gray-200 text-gray-800'
                    }`}>
                      {rec.impact} impact
                    </span>
                  )}
                </div>
                <p className={`font-medium ${
                  rec.priority === 'high' ? 'text-red-900' :
                  rec.priority === 'medium' ? 'text-yellow-900' :
                  'text-blue-900'
                }`}>
                  {rec.suggestion}
                </p>
              </div>
            ))}
          </div>
        </div>
      )}
        </div>
      ) : activeTab === 'scoring' ? (
        <div>
          {analysis.professional_scoring ? (
            <ProfessionalScoringBreakdown 
              professionalScoring={analysis.professional_scoring}
              overallScore={analysis.overall_score}
            />
          ) : (
            <div className="card">
              <div className="text-center py-8">
                <div className="w-16 h-16 mx-auto mb-4 bg-gray-100 rounded-full flex items-center justify-center">
                  <svg className="w-8 h-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                  </svg>
                </div>
                <h3 className="text-lg font-medium text-gray-900 mb-2">Professional Scoring Not Available</h3>
                <p className="text-gray-600">Professional scoring breakdown is not available for this analysis.</p>
              </div>
            </div>
          )}
        </div>
      ) : (
        <div>
          {resume ? (
            <ResumeParsingDisplay resume={resume} />
          ) : (
            <div className="card">
              <div className="text-center py-8">
                <div className="w-16 h-16 mx-auto mb-4 bg-gray-100 rounded-full flex items-center justify-center">
                  <svg className="w-8 h-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                  </svg>
                </div>
                <h3 className="text-lg font-medium text-gray-900 mb-2">Resume Data Not Available</h3>
                <p className="text-gray-600">Resume parsing information is not available for this analysis.</p>
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default ProfessionalAnalysisResults;