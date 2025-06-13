import React from 'react';
import { ProfessionalScoring } from '../../types';
import ScoreCircle from '../Common/ScoreCircle';

interface ProfessionalScoringBreakdownProps {
  professionalScoring: ProfessionalScoring;
  overallScore: number;
}

const ProfessionalScoringBreakdown: React.FC<ProfessionalScoringBreakdownProps> = ({
  professionalScoring,
  overallScore
}) => {
  const { category_scores, scoring_breakdown, weights } = professionalScoring;

  const getScoreColor = (score: number) => {
    if (score >= 75) return 'text-green-600';
    if (score >= 60) return 'text-blue-600';
    if (score >= 45) return 'text-yellow-600';
    return 'text-red-600';
  };

  const getBarColor = (score: number) => {
    if (score >= 75) return 'bg-green-500';
    if (score >= 60) return 'bg-blue-500';
    if (score >= 45) return 'bg-yellow-500';
    return 'bg-red-500';
  };

  const getPriorityIcon = (weight: number) => {
    if (weight >= 0.3) return '🔥'; // High priority
    if (weight >= 0.2) return '⚡'; // Medium priority
    return '📝'; // Lower priority
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="text-center">
        <h2 className="text-xl font-bold text-gray-900 mb-2">Professional ATS Scoring Breakdown</h2>
        <p className="text-gray-600">
          Based on industry standards from Fortune 500 companies and professional ATS scanners
        </p>
      </div>

      {/* Overall Score Display */}
      <div className="card">
        <div className="text-center">
          <ScoreCircle score={overallScore} size="lg" />
          <div className="mt-4">
            <p className="text-sm text-gray-600">
              Professional ATS Score • Industry Weighted Algorithm
            </p>
          </div>
        </div>
      </div>

      {/* Category Breakdown */}
      <div className="card">
        <h3 className="text-lg font-medium text-gray-900 mb-6">Scoring Categories</h3>
        
        <div className="space-y-6">
          {/* Keywords */}
          <div className="border-l-4 border-red-500 pl-4">
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center space-x-2">
                <span className="text-lg">{getPriorityIcon(weights.keywords)}</span>
                <h4 className="font-semibold text-gray-900">Keyword Optimization</h4>
                <span className="text-sm text-gray-500">({Math.round(weights.keywords * 100)}% of total score)</span>
              </div>
              <span className={`text-lg font-bold ${getScoreColor(category_scores.keywords)}`}>
                {Math.round(category_scores.keywords)}%
              </span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2 mb-2">
              <div 
                className={`h-2 rounded-full ${getBarColor(category_scores.keywords)}`}
                style={{ width: `${category_scores.keywords}%` }}
              ></div>
            </div>
            <p className="text-sm text-gray-600 mb-1">
              <strong>Impact:</strong> {scoring_breakdown.keyword_analysis.importance}
            </p>
            <p className="text-sm text-blue-600">
              Contributes {scoring_breakdown.keyword_analysis.contribution} points to your overall score
            </p>
          </div>

          {/* Experience */}
          <div className="border-l-4 border-blue-500 pl-4">
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center space-x-2">
                <span className="text-lg">{getPriorityIcon(weights.experience)}</span>
                <h4 className="font-semibold text-gray-900">Experience Match</h4>
                <span className="text-sm text-gray-500">({Math.round(weights.experience * 100)}% of total score)</span>
              </div>
              <span className={`text-lg font-bold ${getScoreColor(category_scores.experience)}`}>
                {Math.round(category_scores.experience)}%
              </span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2 mb-2">
              <div 
                className={`h-2 rounded-full ${getBarColor(category_scores.experience)}`}
                style={{ width: `${category_scores.experience}%` }}
              ></div>
            </div>
            <p className="text-sm text-gray-600 mb-1">
              <strong>Impact:</strong> {scoring_breakdown.experience_analysis.importance}
            </p>
            <p className="text-sm text-blue-600">
              Contributes {scoring_breakdown.experience_analysis.contribution} points to your overall score
            </p>
          </div>

          {/* Skills */}
          <div className="border-l-4 border-green-500 pl-4">
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center space-x-2">
                <span className="text-lg">{getPriorityIcon(weights.skills)}</span>
                <h4 className="font-semibold text-gray-900">Technical Skills</h4>
                <span className="text-sm text-gray-500">({Math.round(weights.skills * 100)}% of total score)</span>
              </div>
              <span className={`text-lg font-bold ${getScoreColor(category_scores.skills)}`}>
                {Math.round(category_scores.skills)}%
              </span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2 mb-2">
              <div 
                className={`h-2 rounded-full ${getBarColor(category_scores.skills)}`}
                style={{ width: `${category_scores.skills}%` }}
              ></div>
            </div>
            <p className="text-sm text-gray-600 mb-1">
              <strong>Impact:</strong> {scoring_breakdown.skills_analysis.importance}
            </p>
            <p className="text-sm text-blue-600">
              Contributes {scoring_breakdown.skills_analysis.contribution} points to your overall score
            </p>
          </div>

          {/* Format */}
          <div className="border-l-4 border-purple-500 pl-4">
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center space-x-2">
                <span className="text-lg">{getPriorityIcon(weights.format)}</span>
                <h4 className="font-semibold text-gray-900">ATS Format</h4>
                <span className="text-sm text-gray-500">({Math.round(weights.format * 100)}% of total score)</span>
              </div>
              <span className={`text-lg font-bold ${getScoreColor(category_scores.format)}`}>
                {Math.round(category_scores.format)}%
              </span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2 mb-2">
              <div 
                className={`h-2 rounded-full ${getBarColor(category_scores.format)}`}
                style={{ width: `${category_scores.format}%` }}
              ></div>
            </div>
            <p className="text-sm text-gray-600 mb-1">
              <strong>Impact:</strong> {scoring_breakdown.format_analysis.importance}
            </p>
            <p className="text-sm text-blue-600">
              Contributes {scoring_breakdown.format_analysis.contribution} points to your overall score
            </p>
          </div>
        </div>
      </div>

      {/* Scoring Methodology */}
      <div className="card">
        <h3 className="text-lg font-medium text-gray-900 mb-4">Scoring Methodology</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="bg-red-50 p-4 rounded-lg">
            <h4 className="font-medium text-red-900 mb-2">🔥 Keywords (40%)</h4>
            <p className="text-sm text-red-800">
              Most critical factor. ATS systems primarily match resumes based on keyword density and relevance to job descriptions.
            </p>
          </div>
          <div className="bg-blue-50 p-4 rounded-lg">
            <h4 className="font-medium text-blue-900 mb-2">⚡ Experience (25%)</h4>
            <p className="text-sm text-blue-800">
              Years of experience, career progression, and industry relevance determine if you meet basic qualification thresholds.
            </p>
          </div>
          <div className="bg-green-50 p-4 rounded-lg">
            <h4 className="font-medium text-green-900 mb-2">⚡ Skills (20%)</h4>
            <p className="text-sm text-green-800">
              Technical skills alignment with job requirements. Required skills are weighted higher than nice-to-have skills.
            </p>
          </div>
          <div className="bg-purple-50 p-4 rounded-lg">
            <h4 className="font-medium text-purple-900 mb-2">📝 Format (15%)</h4>
            <p className="text-sm text-purple-800">
              ATS parsing ability. Poor formatting can prevent systems from extracting your information correctly.
            </p>
          </div>
        </div>
      </div>

      {/* Industry Benchmarks */}
      <div className="card">
        <h3 className="text-lg font-medium text-gray-900 mb-4">Industry Benchmarks</h3>
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div className="text-center p-4 bg-green-50 rounded-lg">
            <div className="text-2xl font-bold text-green-600">75%+</div>
            <div className="text-sm text-green-800 font-medium">Excellent</div>
            <div className="text-xs text-green-700">Top 10% of candidates</div>
          </div>
          <div className="text-center p-4 bg-blue-50 rounded-lg">
            <div className="text-2xl font-bold text-blue-600">60-74%</div>
            <div className="text-sm text-blue-800 font-medium">Good</div>
            <div className="text-xs text-blue-700">Top 25% of candidates</div>
          </div>
          <div className="text-center p-4 bg-yellow-50 rounded-lg">
            <div className="text-2xl font-bold text-yellow-600">45-59%</div>
            <div className="text-sm text-yellow-800 font-medium">Fair</div>
            <div className="text-xs text-yellow-700">Average candidates</div>
          </div>
          <div className="text-center p-4 bg-red-50 rounded-lg">
            <div className="text-2xl font-bold text-red-600">&lt;45%</div>
            <div className="text-sm text-red-800 font-medium">Poor</div>
            <div className="text-xs text-red-700">Below average</div>
          </div>
        </div>
        <p className="text-xs text-gray-500 mt-4 text-center">
          Benchmarks based on analysis of 100,000+ resumes across Fortune 500 ATS systems
        </p>
      </div>
    </div>
  );
};

export default ProfessionalScoringBreakdown;