import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import {
  ArrowLeft,
  Target,
  Award,
  TrendingUp,
  Brain,
  MapPin,
  DollarSign,
  Download,
  CheckCircle,
  AlertCircle,
  Info,
  Search,
  Shield,
  Zap,
  Users,
  Sparkles,
} from 'lucide-react';
import type {
  AnalysisResult,
  AchievementAnalysis,
  MLInsights,
  SemanticAnalysisResult,
  FormatCompatibilityReport,
  IndustryAnalysisResult,
  ValidationReport,
  ATSSimulationResult,
  EnhancedAnalysisResult,
  CompetitiveAnalysis,
  MarketPositionAnalysis,
  SalaryInsightsResponse,
  HiringProbabilityResponse,
  ApplicationSuccessResponse,
  CareerPathSuggestionsResponse,
  SalaryPredictionMLResponse,
  MLRecommendationsResponse,
} from '@/types';

interface AnalysisResultPageProps {
  analysisData: {
    result: AnalysisResult;
    achievementAnalysis?: AchievementAnalysis;
    mlInsights?: MLInsights;
    semanticAnalysis?: SemanticAnalysisResult;
    formatCompatibility?: FormatCompatibilityReport;
    industryAnalysis?: IndustryAnalysisResult;
    atsValidation?: ValidationReport;
    atsSimulation?: ATSSimulationResult;
    comprehensiveAnalysis?: EnhancedAnalysisResult;
    competitiveAnalysis?: CompetitiveAnalysis;
    marketPositionAnalysis?: MarketPositionAnalysis;
    salaryInsights?: SalaryInsightsResponse;
    hiringProbabilityAnalysis?: HiringProbabilityResponse;
    applicationSuccessPrediction?: ApplicationSuccessResponse;
    careerPathSuggestions?: CareerPathSuggestionsResponse;
    salaryPredictionML?: SalaryPredictionMLResponse;
    mlRecommendations?: MLRecommendationsResponse;
    resumeFilename: string;
    jobDescription: string;
    modelUsed: string;
    timestamp: string;
  } | null;
  onBack: () => void;
}

export function AnalysisResultPage({
  analysisData,
  onBack,
}: AnalysisResultPageProps) {
  if (!analysisData) {
    return (
      <div className="flex h-full items-center justify-center">
        <div className="text-center">
          <AlertCircle className="mx-auto mb-4 h-12 w-12 text-muted-foreground" />
          <h3 className="mb-2 text-lg font-semibold">No Analysis Data</h3>
          <p className="mb-4 text-muted-foreground">
            No analysis results to display.
          </p>
          <Button onClick={onBack} variant="outline">
            <ArrowLeft className="mr-2 h-4 w-4" />
            Go Back
          </Button>
        </div>
      </div>
    );
  }

  const {
    result,
    achievementAnalysis,
    mlInsights,
    semanticAnalysis,
    formatCompatibility,
    industryAnalysis,
    atsValidation,
    atsSimulation,
    comprehensiveAnalysis,
    competitiveAnalysis: _competitiveAnalysis,
    marketPositionAnalysis: _marketPositionAnalysis,
    salaryInsights: _salaryInsights,
    hiringProbabilityAnalysis: _hiringProbabilityAnalysis,
    applicationSuccessPrediction,
    careerPathSuggestions,
    salaryPredictionML,
    mlRecommendations,
    resumeFilename,
    modelUsed,
    timestamp,
  } = analysisData;

  const getScoreColor = (score: number) => {
    if (score >= 80) return 'text-green-600';
    if (score >= 60) return 'text-yellow-600';
    return 'text-red-600';
  };

  const getScoreBadgeVariant = (
    score: number
  ): 'default' | 'secondary' | 'destructive' | 'outline' => {
    if (score >= 80) return 'default';
    if (score >= 60) return 'secondary';
    return 'destructive';
  };

  return (
    <div className="mx-auto max-w-7xl space-y-6 p-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center space-x-4">
          <Button onClick={onBack} variant="outline" size="sm">
            <ArrowLeft className="mr-2 h-4 w-4" />
            Back to Analysis
          </Button>
          <div>
            <h1 className="text-2xl font-bold">Analysis Results</h1>
            <p className="text-muted-foreground">
              {resumeFilename} • {new Date(timestamp).toLocaleString()}
            </p>
          </div>
        </div>
        <div className="flex items-center space-x-2">
          <Badge variant="outline">
            <Brain className="mr-1 h-3 w-3" />
            {modelUsed}
          </Badge>
          <Button variant="outline" size="sm">
            <Download className="mr-2 h-4 w-4" />
            Export Results
          </Button>
        </div>
      </div>

      {/* Overall Score */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center">
            <Target className="mr-2 h-5 w-5" />
            Overall ATS Score
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex items-center space-x-6">
            <div className="text-center">
              <div
                className={`text-4xl font-bold ${getScoreColor(result.overall_score)}`}
              >
                {result.overall_score}%
              </div>
              <p className="text-sm text-muted-foreground">Overall Score</p>
            </div>
            <div className="flex-1">
              <Progress value={result.overall_score} className="h-3" />
            </div>
            <Badge variant={getScoreBadgeVariant(result.overall_score)}>
              {result.overall_score >= 80
                ? 'Excellent'
                : result.overall_score >= 60
                  ? 'Good'
                  : 'Needs Improvement'}
            </Badge>
          </div>
        </CardContent>
      </Card>

      {/* Category Scores */}
      <Card>
        <CardHeader>
          <CardTitle>Category Breakdown</CardTitle>
          <CardDescription>
            Detailed scores across different evaluation categories
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
            {Object.entries(result.category_scores).map(([category, score]) => {
              const numericScore = score as number;
              return (
                <div key={category} className="space-y-2">
                  <div className="flex items-center justify-between">
                    <span className="text-sm font-medium capitalize">
                      {category}
                    </span>
                    <span
                      className={`text-sm font-semibold ${getScoreColor(numericScore)}`}
                    >
                      {numericScore}%
                    </span>
                  </div>
                  <Progress value={numericScore} className="h-2" />
                </div>
              );
            })}
          </div>
        </CardContent>
      </Card>

      {/* Detailed Feedback */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center">
            <Info className="mr-2 h-5 w-5" />
            Detailed Feedback
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="whitespace-pre-wrap text-sm leading-relaxed">
            {result.detailed_feedback}
          </div>
        </CardContent>
      </Card>

      {/* Missing Keywords & Recommendations */}
      <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center">
              <AlertCircle className="mr-2 h-5 w-5 text-yellow-600" />
              Missing Keywords
            </CardTitle>
          </CardHeader>
          <CardContent>
            {result.missing_keywords.length > 0 ? (
              <div className="flex flex-wrap gap-2">
                {result.missing_keywords.map((keyword, index) => (
                  <Badge key={index} variant="secondary">
                    {keyword}
                  </Badge>
                ))}
              </div>
            ) : (
              <p className="text-muted-foreground">
                No missing keywords identified
              </p>
            )}
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="flex items-center">
              <CheckCircle className="mr-2 h-5 w-5 text-green-600" />
              Recommendations
            </CardTitle>
          </CardHeader>
          <CardContent>
            <ul className="space-y-2">
              {result.recommendations.map((recommendation, index) => (
                <li key={index} className="flex items-start">
                  <div className="mr-3 mt-2 h-2 w-2 flex-shrink-0 rounded-full bg-primary" />
                  <span className="text-sm">{recommendation}</span>
                </li>
              ))}
            </ul>
          </CardContent>
        </Card>
      </div>

      {/* Achievement Analysis */}
      {achievementAnalysis && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center">
              <Award className="mr-2 h-5 w-5" />
              Achievement Analysis
            </CardTitle>
            <CardDescription>
              Analysis of accomplishments and impact statements
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* Achievement Metrics */}
            <div className="grid grid-cols-2 gap-4 md:grid-cols-4">
              <div className="text-center">
                <div className="text-2xl font-bold text-blue-600">
                  {achievementAnalysis.xyz_formula_usage.toFixed(1)}%
                </div>
                <p className="text-xs text-muted-foreground">
                  XYZ Formula Usage
                </p>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-green-600">
                  {achievementAnalysis.achievement_density.toFixed(1)}%
                </div>
                <p className="text-xs text-muted-foreground">
                  Achievement Density
                </p>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-purple-600">
                  {achievementAnalysis.quantification_score.toFixed(1)}%
                </div>
                <p className="text-xs text-muted-foreground">
                  Quantification Score
                </p>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-orange-600">
                  {achievementAnalysis.overall_achievement_score.toFixed(1)}%
                </div>
                <p className="text-xs text-muted-foreground">Overall Score</p>
              </div>
            </div>

            <Separator />

            {/* Bullet Point Analysis */}
            {achievementAnalysis.bullet_points &&
              achievementAnalysis.bullet_points.length > 0 && (
                <div>
                  <h4 className="mb-3 font-semibold">Bullet Point Analysis</h4>
                  <div className="space-y-3">
                    {achievementAnalysis.bullet_points
                      .slice(0, 5)
                      .map((bullet, index) => (
                        <Card
                          key={index}
                          className="border-l-4 border-l-blue-500"
                        >
                          <CardContent className="pt-4">
                            <p className="mb-2 text-sm">{bullet.text}</p>
                            <div className="mb-2 flex flex-wrap gap-2">
                              <Badge
                                variant={
                                  bullet.has_xyz_structure
                                    ? 'default'
                                    : 'secondary'
                                }
                              >
                                {bullet.has_xyz_structure
                                  ? 'XYZ Structure'
                                  : 'No XYZ Structure'}
                              </Badge>
                              <Badge variant="outline">{bullet.section}</Badge>
                              <Badge variant="outline">
                                Score: {bullet.strength_score.toFixed(1)}
                              </Badge>
                            </div>
                            {bullet.suggestions &&
                              bullet.suggestions.length > 0 && (
                                <div className="text-xs text-muted-foreground">
                                  <strong>Suggestions:</strong>{' '}
                                  {bullet.suggestions.join(', ')}
                                </div>
                              )}
                          </CardContent>
                        </Card>
                      ))}
                  </div>
                </div>
              )}
          </CardContent>
        </Card>
      )}

      {/* ML Insights */}
      {mlInsights && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center">
              <Brain className="mr-2 h-5 w-5" />
              ML-Powered Insights
            </CardTitle>
            <CardDescription>
              AI-generated predictions and career guidance
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* Success Prediction */}
            {mlInsights.success_prediction && (
              <div>
                <h4 className="mb-3 flex items-center font-semibold">
                  <TrendingUp className="mr-2 h-4 w-4" />
                  Success Prediction
                </h4>
                <div className="mb-4 grid grid-cols-1 gap-4 md:grid-cols-3">
                  <div className="rounded-lg bg-muted p-3 text-center">
                    <div className="text-2xl font-bold text-green-600">
                      {(
                        mlInsights.success_prediction
                          .application_success_probability * 100
                      ).toFixed(1)}
                      %
                    </div>
                    <p className="text-xs text-muted-foreground">
                      Application Success
                    </p>
                  </div>
                  <div className="rounded-lg bg-muted p-3 text-center">
                    <div className="text-2xl font-bold text-blue-600">
                      {(
                        mlInsights.success_prediction.interview_likelihood * 100
                      ).toFixed(1)}
                      %
                    </div>
                    <p className="text-xs text-muted-foreground">
                      Interview Likelihood
                    </p>
                  </div>
                  <div className="rounded-lg bg-muted p-3 text-center">
                    <div className="text-2xl font-bold text-purple-600">
                      {(
                        mlInsights.success_prediction.hiring_probability * 100
                      ).toFixed(1)}
                      %
                    </div>
                    <p className="text-xs text-muted-foreground">
                      Hiring Probability
                    </p>
                  </div>
                </div>

                {/* Success Factors */}
                {mlInsights.success_prediction.success_factors &&
                  mlInsights.success_prediction.success_factors.length > 0 && (
                    <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                      <div>
                        <h5 className="mb-2 font-medium text-green-700">
                          Success Factors
                        </h5>
                        <ul className="space-y-1">
                          {mlInsights.success_prediction.success_factors.map(
                            (factor, index) => (
                              <li
                                key={index}
                                className="flex items-start text-sm"
                              >
                                <CheckCircle className="mr-2 mt-0.5 h-3 w-3 flex-shrink-0 text-green-600" />
                                {factor}
                              </li>
                            )
                          )}
                        </ul>
                      </div>
                      {mlInsights.success_prediction.risk_factors &&
                        mlInsights.success_prediction.risk_factors.length >
                          0 && (
                          <div>
                            <h5 className="mb-2 font-medium text-red-700">
                              Risk Factors
                            </h5>
                            <ul className="space-y-1">
                              {mlInsights.success_prediction.risk_factors.map(
                                (factor, index) => (
                                  <li
                                    key={index}
                                    className="flex items-start text-sm"
                                  >
                                    <AlertCircle className="mr-2 mt-0.5 h-3 w-3 flex-shrink-0 text-red-600" />
                                    {factor}
                                  </li>
                                )
                              )}
                            </ul>
                          </div>
                        )}
                    </div>
                  )}
              </div>
            )}

            <Separator />

            {/* Salary Prediction */}
            {mlInsights.salary_prediction && (
              <div>
                <h4 className="mb-3 flex items-center font-semibold">
                  <DollarSign className="mr-2 h-4 w-4" />
                  Salary Prediction
                </h4>
                <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
                  <div className="rounded-lg bg-muted p-3 text-center">
                    <div className="text-lg font-bold">
                      $
                      {mlInsights.salary_prediction.predicted_salary_range?.min?.toLocaleString() ||
                        'N/A'}{' '}
                      - $
                      {mlInsights.salary_prediction.predicted_salary_range?.max?.toLocaleString() ||
                        'N/A'}
                    </div>
                    <p className="text-xs text-muted-foreground">
                      Predicted Range
                    </p>
                  </div>
                  <div className="rounded-lg bg-muted p-3 text-center">
                    <div className="text-lg font-bold">
                      {mlInsights.salary_prediction.market_percentile}th
                    </div>
                    <p className="text-xs text-muted-foreground">
                      Market Percentile
                    </p>
                  </div>
                  <div className="rounded-lg bg-muted p-3 text-center">
                    <div className="text-lg font-bold text-green-600">
                      +{mlInsights.salary_prediction.improvement_potential}%
                    </div>
                    <p className="text-xs text-muted-foreground">
                      Improvement Potential
                    </p>
                  </div>
                </div>
              </div>
            )}

            <Separator />

            {/* Career Path Suggestions */}
            {mlInsights.career_path_suggestions && (
              <div>
                <h4 className="mb-3 flex items-center font-semibold">
                  <MapPin className="mr-2 h-4 w-4" />
                  Career Path Suggestions
                </h4>
                <div className="space-y-3">
                  <div className="text-sm">
                    <strong>Current Level:</strong>{' '}
                    {mlInsights.career_path_suggestions.current_level}
                  </div>

                  {mlInsights.career_path_suggestions.suggested_roles &&
                    mlInsights.career_path_suggestions.suggested_roles.length >
                      0 && (
                      <div>
                        <h5 className="mb-2 font-medium">Suggested Roles</h5>
                        <div className="grid gap-2">
                          {mlInsights.career_path_suggestions.suggested_roles
                            .slice(0, 3)
                            .map((role, index) => (
                              <Card key={index} className="p-3">
                                <div className="mb-2 flex items-start justify-between">
                                  <h6 className="font-medium">{role.title}</h6>
                                  <Badge variant="outline">
                                    {role.match_score}% match
                                  </Badge>
                                </div>
                                <div className="text-sm text-muted-foreground">
                                  <div>
                                    Requirements met: {role.requirements_met}%
                                  </div>
                                  {role.salary_range && (
                                    <div>
                                      Salary: $
                                      {role.salary_range.min?.toLocaleString()}{' '}
                                      - $
                                      {role.salary_range.max?.toLocaleString()}
                                    </div>
                                  )}
                                </div>
                              </Card>
                            ))}
                        </div>
                      </div>
                    )}
                </div>
              </div>
            )}

            <Separator />

            {/* ML Recommendations */}
            {mlInsights.ml_recommendations &&
              mlInsights.ml_recommendations.length > 0 && (
                <div>
                  <h4 className="mb-3 flex items-center font-semibold">
                    <Sparkles className="mr-2 h-4 w-4" />
                    ML Recommendations
                  </h4>
                  <div className="grid gap-3">
                    {mlInsights.ml_recommendations
                      .slice(0, 5)
                      .map((rec, index) => (
                        <Card key={index} className="p-3">
                          <div className="mb-2 flex items-start justify-between">
                            <div className="flex items-center space-x-2">
                              <Badge
                                variant={
                                  rec.priority === 'High'
                                    ? 'destructive'
                                    : rec.priority === 'Medium'
                                      ? 'secondary'
                                      : 'outline'
                                }
                              >
                                {rec.priority}
                              </Badge>
                              <Badge variant="outline">{rec.category}</Badge>
                            </div>
                            <div className="text-right">
                              <div className="text-sm font-medium">
                                Impact: {rec.impact_score}/10
                              </div>
                              <div className="text-xs text-muted-foreground">
                                {rec.implementation_difficulty}
                              </div>
                            </div>
                          </div>
                          <p className="mb-2 text-sm font-medium">
                            {rec.recommendation}
                          </p>
                          <p className="text-xs text-muted-foreground">
                            {rec.expected_outcome}
                          </p>
                        </Card>
                      ))}
                  </div>
                </div>
              )}

            <Separator />

            {/* Feature Analysis */}
            {mlInsights.feature_analysis && (
              <div>
                <h4 className="mb-3 flex items-center font-semibold">
                  <Brain className="mr-2 h-4 w-4" />
                  ML Feature Analysis
                </h4>
                <div className="mb-4 grid grid-cols-1 gap-4 md:grid-cols-3">
                  <div className="rounded-lg bg-muted p-3 text-center">
                    <div className="text-2xl font-bold text-blue-600">
                      {(
                        mlInsights.feature_analysis.model_accuracy * 100
                      ).toFixed(1)}
                      %
                    </div>
                    <p className="text-xs text-muted-foreground">
                      Model Accuracy
                    </p>
                  </div>
                  <div className="rounded-lg bg-muted p-3 text-center">
                    <div className="text-2xl font-bold text-green-600">
                      {(
                        mlInsights.feature_analysis.prediction_confidence * 100
                      ).toFixed(1)}
                      %
                    </div>
                    <p className="text-xs text-muted-foreground">
                      Prediction Confidence
                    </p>
                  </div>
                  <div className="rounded-lg bg-muted p-3 text-center">
                    <div className="text-2xl font-bold text-purple-600">
                      {(mlInsights.confidence_score * 100).toFixed(1)}%
                    </div>
                    <p className="text-xs text-muted-foreground">
                      Overall Confidence
                    </p>
                  </div>
                </div>

                {/* Feature Importance */}
                {mlInsights.feature_analysis.feature_importance &&
                  mlInsights.feature_analysis.feature_importance.length > 0 && (
                    <div>
                      <h5 className="mb-2 font-medium">Feature Importance</h5>
                      <div className="space-y-2">
                        {mlInsights.feature_analysis.feature_importance
                          .slice(0, 5)
                          .map((feature, index) => (
                            <div
                              key={index}
                              className="flex items-center justify-between rounded-lg bg-muted p-3"
                            >
                              <div>
                                <div className="font-medium">
                                  {feature.feature}
                                </div>
                                <div className="text-xs text-muted-foreground">
                                  {feature.description}
                                </div>
                              </div>
                              <div className="text-right">
                                <div className="font-bold">
                                  {(feature.importance * 100).toFixed(1)}%
                                </div>
                                <Progress
                                  value={feature.importance * 100}
                                  className="mt-1 w-20"
                                />
                              </div>
                            </div>
                          ))}
                      </div>
                    </div>
                  )}
              </div>
            )}

            {/* Enhanced Career Path Details */}
            {mlInsights.career_path_suggestions && (
              <>
                <Separator />
                <div>
                  <h4 className="mb-3 flex items-center font-semibold">
                    <TrendingUp className="mr-2 h-4 w-4" />
                    Career Development Analysis
                  </h4>

                  {/* Skill Gaps */}
                  {mlInsights.career_path_suggestions.skill_gaps &&
                    mlInsights.career_path_suggestions.skill_gaps.length >
                      0 && (
                      <div className="mb-4">
                        <h5 className="mb-2 font-medium">Skill Gap Analysis</h5>
                        <div className="grid gap-2">
                          {mlInsights.career_path_suggestions.skill_gaps
                            .slice(0, 3)
                            .map((gap, index) => (
                              <Card key={index} className="p-3">
                                <div className="mb-2 flex items-center justify-between">
                                  <div className="font-medium">{gap.skill}</div>
                                  <Badge variant="outline">
                                    {gap.current_level}/{gap.required_level}
                                  </Badge>
                                </div>
                                <Progress
                                  value={
                                    (gap.current_level / gap.required_level) *
                                    100
                                  }
                                  className="mb-2"
                                />
                                {gap.learning_resources &&
                                  gap.learning_resources.length > 0 && (
                                    <div className="text-xs text-muted-foreground">
                                      <strong>Resources:</strong>{' '}
                                      {gap.learning_resources
                                        .slice(0, 2)
                                        .join(', ')}
                                      {gap.learning_resources.length > 2 &&
                                        ` +${gap.learning_resources.length - 2} more`}
                                    </div>
                                  )}
                              </Card>
                            ))}
                        </div>
                      </div>
                    )}

                  {/* Growth Trajectory */}
                  {mlInsights.career_path_suggestions.growth_trajectory &&
                    mlInsights.career_path_suggestions.growth_trajectory
                      .length > 0 && (
                      <div>
                        <h5 className="mb-2 font-medium">Growth Trajectory</h5>
                        <div className="space-y-2">
                          {mlInsights.career_path_suggestions.growth_trajectory
                            .slice(0, 3)
                            .map((path, index) => (
                              <Card key={index} className="p-3">
                                <div className="mb-2 flex items-center justify-between">
                                  <div className="font-medium">{path.role}</div>
                                  <div className="text-right">
                                    <div className="text-sm font-bold text-green-600">
                                      +{path.salary_increase}%
                                    </div>
                                    <div className="text-xs text-muted-foreground">
                                      {path.timeline}
                                    </div>
                                  </div>
                                </div>
                                {path.requirements &&
                                  path.requirements.length > 0 && (
                                    <div className="text-xs text-muted-foreground">
                                      <strong>Requirements:</strong>{' '}
                                      {path.requirements.slice(0, 2).join(', ')}
                                      {path.requirements.length > 2 &&
                                        ` +${path.requirements.length - 2} more`}
                                    </div>
                                  )}
                              </Card>
                            ))}
                        </div>
                      </div>
                    )}
                </div>
              </>
            )}

            {/* Enhanced Salary Analysis */}
            {mlInsights.salary_prediction && (
              <>
                <Separator />
                <div>
                  <h4 className="mb-3 flex items-center font-semibold">
                    <DollarSign className="mr-2 h-4 w-4" />
                    Advanced Salary Analysis
                  </h4>

                  {/* Salary Factors */}
                  {mlInsights.salary_prediction.factors_affecting_salary &&
                    mlInsights.salary_prediction.factors_affecting_salary
                      .length > 0 && (
                      <div className="mb-4">
                        <h5 className="mb-2 font-medium">
                          Salary Impact Factors
                        </h5>
                        <div className="space-y-2">
                          {mlInsights.salary_prediction.factors_affecting_salary
                            .slice(0, 5)
                            .map((factor, index) => (
                              <div
                                key={index}
                                className="flex items-center justify-between rounded-lg bg-muted p-3"
                              >
                                <div>
                                  <div className="font-medium">
                                    {factor.factor}
                                  </div>
                                  <div className="text-xs text-muted-foreground">
                                    {factor.description}
                                  </div>
                                </div>
                                <div className="text-right">
                                  <div
                                    className={`font-bold ${factor.impact_percentage > 0 ? 'text-green-600' : 'text-red-600'}`}
                                  >
                                    {factor.impact_percentage > 0 ? '+' : ''}
                                    {factor.impact_percentage}%
                                  </div>
                                </div>
                              </div>
                            ))}
                        </div>
                      </div>
                    )}

                  {/* Location Adjustments */}
                  {mlInsights.salary_prediction.location_adjustments &&
                    mlInsights.salary_prediction.location_adjustments.length >
                      0 && (
                      <div>
                        <h5 className="mb-2 font-medium">
                          Location-Based Salary Adjustments
                        </h5>
                        <div className="grid grid-cols-1 gap-2 md:grid-cols-2">
                          {mlInsights.salary_prediction.location_adjustments
                            .slice(0, 4)
                            .map((location, index) => (
                              <Card key={index} className="p-3">
                                <div className="mb-1 flex items-center justify-between">
                                  <div className="font-medium">
                                    {location.location}
                                  </div>
                                  <div
                                    className={`font-bold ${location.adjustment_factor > 1 ? 'text-green-600' : 'text-red-600'}`}
                                  >
                                    {(
                                      (location.adjustment_factor - 1) *
                                      100
                                    ).toFixed(0)}
                                    %
                                  </div>
                                </div>
                                <div className="text-xs text-muted-foreground">
                                  Cost of Living:{' '}
                                  {location.cost_of_living.toFixed(1)}x
                                </div>
                              </Card>
                            ))}
                        </div>
                      </div>
                    )}
                </div>
              </>
            )}
          </CardContent>
        </Card>
      )}

      {/* Semantic Analysis */}
      {semanticAnalysis && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center">
              <Search className="mr-2 h-5 w-5" />
              Semantic Analysis
            </CardTitle>
            <CardDescription>
              Deep understanding of keyword context and industry alignment
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* Semantic Scores */}
            <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
              <div className="text-center">
                <div className="text-2xl font-bold text-blue-600">
                  {semanticAnalysis.overall_score.toFixed(1)}%
                </div>
                <p className="text-xs text-muted-foreground">
                  Overall Semantic Score
                </p>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-green-600">
                  {semanticAnalysis.keyword_relevance_score.toFixed(1)}%
                </div>
                <p className="text-xs text-muted-foreground">
                  Keyword Relevance
                </p>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-purple-600">
                  {semanticAnalysis.industry_alignment_score.toFixed(1)}%
                </div>
                <p className="text-xs text-muted-foreground">
                  Industry Alignment
                </p>
              </div>
            </div>

            <Separator />

            {/* Semantic Matches */}
            {semanticAnalysis.semantic_matches &&
              semanticAnalysis.semantic_matches.length > 0 && (
                <div>
                  <h4 className="mb-3 font-semibold">Top Semantic Matches</h4>
                  <div className="space-y-2">
                    {semanticAnalysis.semantic_matches
                      .slice(0, 5)
                      .map((match, index) => (
                        <div
                          key={index}
                          className="flex items-center justify-between rounded-lg border p-3"
                        >
                          <div className="flex-1">
                            <span className="font-medium">{match.keyword}</span>
                            <p className="text-sm text-muted-foreground">
                              {match.context}
                            </p>
                          </div>
                          <div className="flex items-center gap-2">
                            <Badge
                              variant={
                                match.match_type === 'exact'
                                  ? 'default'
                                  : 'secondary'
                              }
                            >
                              {match.match_type}
                            </Badge>
                            <Badge variant="outline">
                              {(match.relevance_score * 100).toFixed(0)}%
                            </Badge>
                          </div>
                        </div>
                      ))}
                  </div>
                </div>
              )}

            {/* Conceptual Gaps */}
            {semanticAnalysis.conceptual_gaps &&
              semanticAnalysis.conceptual_gaps.length > 0 && (
                <div>
                  <h4 className="mb-3 font-semibold">
                    Conceptual Gaps to Address
                  </h4>
                  <div className="space-y-2">
                    {semanticAnalysis.conceptual_gaps
                      .slice(0, 3)
                      .map((gap, index) => (
                        <Card
                          key={index}
                          className="border-l-4 border-l-orange-500"
                        >
                          <CardContent className="pt-4">
                            <div className="mb-2 flex items-center justify-between">
                              <span className="font-medium">{gap.concept}</span>
                              <Badge variant="outline">
                                Impact: {gap.importance_score.toFixed(1)}
                              </Badge>
                            </div>
                            <p className="mb-2 text-sm text-muted-foreground">
                              {gap.explanation}
                            </p>
                            <div className="flex flex-wrap gap-1">
                              {gap.suggested_keywords.map((keyword, kidx) => (
                                <Badge
                                  key={kidx}
                                  variant="secondary"
                                  className="text-xs"
                                >
                                  {keyword}
                                </Badge>
                              ))}
                            </div>
                          </CardContent>
                        </Card>
                      ))}
                  </div>
                </div>
              )}
          </CardContent>
        </Card>
      )}

      {/* Format Compatibility */}
      {formatCompatibility && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center">
              <Shield className="mr-2 h-5 w-5" />
              ATS Format Compatibility
            </CardTitle>
            <CardDescription>
              Analysis of resume format and ATS parsing compatibility
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* Compatibility Score */}
            <div className="text-center">
              <div
                className={`text-4xl font-bold ${
                  formatCompatibility.ats_compatibility_rating === 'excellent'
                    ? 'text-green-600'
                    : formatCompatibility.ats_compatibility_rating === 'good'
                      ? 'text-blue-600'
                      : formatCompatibility.ats_compatibility_rating === 'fair'
                        ? 'text-yellow-600'
                        : 'text-red-600'
                }`}
              >
                {formatCompatibility.overall_compatibility_score.toFixed(1)}%
              </div>
              <Badge
                variant={
                  formatCompatibility.ats_compatibility_rating === 'excellent'
                    ? 'default'
                    : formatCompatibility.ats_compatibility_rating === 'good'
                      ? 'secondary'
                      : 'destructive'
                }
                className="mt-2"
              >
                {formatCompatibility.ats_compatibility_rating.toUpperCase()}
              </Badge>
            </div>

            <Separator />

            {/* Format Issues */}
            {formatCompatibility.format_issues &&
              formatCompatibility.format_issues.length > 0 && (
                <div>
                  <h4 className="mb-3 font-semibold">Format Issues Detected</h4>
                  <div className="space-y-2">
                    {formatCompatibility.format_issues.map((issue, index) => (
                      <div
                        key={index}
                        className="flex items-start gap-3 rounded-lg border p-3"
                      >
                        <AlertCircle
                          className={`mt-1 h-4 w-4 flex-shrink-0 ${
                            issue.severity === 'critical'
                              ? 'text-red-600'
                              : issue.severity === 'major'
                                ? 'text-orange-600'
                                : 'text-yellow-600'
                          }`}
                        />
                        <div className="flex-1">
                          <div className="flex items-center gap-2">
                            <span className="font-medium">
                              {issue.issue_type}
                            </span>
                            <Badge
                              variant={
                                issue.severity === 'critical'
                                  ? 'destructive'
                                  : issue.severity === 'major'
                                    ? 'secondary'
                                    : 'outline'
                              }
                              className="text-xs"
                            >
                              {issue.severity}
                            </Badge>
                          </div>
                          <p className="text-sm text-muted-foreground">
                            {issue.description}
                          </p>
                          <p className="text-sm font-medium text-green-700">
                            {issue.suggestion}
                          </p>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              )}

            {/* Format Recommendations */}
            {formatCompatibility.recommendations &&
              formatCompatibility.recommendations.length > 0 && (
                <div>
                  <h4 className="mb-3 font-semibold">Format Recommendations</h4>
                  <div className="space-y-2">
                    {formatCompatibility.recommendations.map((rec, index) => (
                      <div key={index} className="rounded-lg border p-3">
                        <div className="mb-2 flex items-center justify-between">
                          <span className="font-medium">{rec.category}</span>
                          <Badge
                            variant={
                              rec.priority === 'high'
                                ? 'destructive'
                                : rec.priority === 'medium'
                                  ? 'secondary'
                                  : 'outline'
                            }
                            className="text-xs"
                          >
                            {rec.priority} priority
                          </Badge>
                        </div>
                        <p className="text-sm text-muted-foreground">
                          {rec.description}
                        </p>
                        <p className="text-sm font-medium text-blue-700">
                          {rec.expected_impact}
                        </p>
                      </div>
                    ))}
                  </div>
                </div>
              )}
          </CardContent>
        </Card>
      )}

      {/* Industry Analysis */}
      {industryAnalysis && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center">
              <TrendingUp className="mr-2 h-5 w-5" />
              Industry Analysis
            </CardTitle>
            <CardDescription>
              Industry-specific analysis and role-level assessment
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* Industry Detection */}
            <div className="text-center">
              <div className="text-3xl font-bold text-blue-600">
                {industryAnalysis.detected_industry}
              </div>
              <div className="mt-1 text-sm text-muted-foreground">
                Detected Industry
              </div>
              <div className="mt-2 text-lg font-semibold">
                {(industryAnalysis.confidence_score * 100).toFixed(1)}%
                Confidence
              </div>
            </div>

            <Separator />

            {/* Role Level Assessment */}
            <div>
              <h4 className="mb-3 font-semibold">Role Level Assessment</h4>
              <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                <div className="rounded-lg bg-blue-50 p-3">
                  <div className="font-medium">Detected Level</div>
                  <div className="text-lg font-bold text-blue-600">
                    {industryAnalysis.role_level_assessment.detected_level}
                  </div>
                  <div className="text-sm text-muted-foreground">
                    {(
                      industryAnalysis.role_level_assessment.confidence * 100
                    ).toFixed(1)}
                    % confidence
                  </div>
                </div>
                <div className="rounded-lg bg-green-50 p-3">
                  <div className="font-medium">Domain Expertise</div>
                  <div className="text-lg font-bold text-green-600">
                    {(industryAnalysis.domain_expertise_score * 100).toFixed(1)}
                    %
                  </div>
                  <div className="text-sm text-muted-foreground">
                    Industry alignment score
                  </div>
                </div>
              </div>

              {industryAnalysis.role_level_assessment
                .years_of_experience_estimate && (
                <div className="mt-3 rounded-lg bg-gray-50 p-3">
                  <div className="font-medium">Estimated Experience</div>
                  <div className="text-lg font-bold">
                    {
                      industryAnalysis.role_level_assessment
                        .years_of_experience_estimate
                    }{' '}
                    years
                  </div>
                </div>
              )}
            </div>

            <Separator />

            {/* Industry Keywords */}
            {industryAnalysis.industry_keywords.length > 0 && (
              <div>
                <h4 className="mb-3 font-semibold">
                  Industry Keywords Analysis
                </h4>
                <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                  {industryAnalysis.industry_keywords
                    .slice(0, 6)
                    .map((keyword, index) => (
                      <div key={index} className="rounded-lg border p-3">
                        <div className="flex items-center justify-between">
                          <span className="font-medium">{keyword.keyword}</span>
                          {keyword.found ? (
                            <CheckCircle className="h-4 w-4 text-green-500" />
                          ) : (
                            <AlertCircle className="h-4 w-4 text-red-500" />
                          )}
                        </div>
                        <div className="mt-1 text-sm text-muted-foreground">
                          {keyword.category} • Weight:{' '}
                          {keyword.weight.toFixed(1)}
                        </div>
                        {keyword.found && (
                          <div className="mt-1 text-sm text-green-600">
                            Found {keyword.frequency} times
                          </div>
                        )}
                      </div>
                    ))}
                </div>
              </div>
            )}

            <Separator />

            {/* Experience Indicators */}
            {industryAnalysis.role_level_assessment.experience_indicators
              .length > 0 && (
              <div>
                <h4 className="mb-3 font-semibold">Experience Indicators</h4>
                <div className="space-y-2">
                  {industryAnalysis.role_level_assessment.experience_indicators
                    .slice(0, 4)
                    .map((indicator, index) => (
                      <div
                        key={index}
                        className="flex items-start space-x-3 rounded-lg bg-gray-50 p-3"
                      >
                        <Info className="mt-0.5 h-4 w-4 text-blue-500" />
                        <div className="flex-1">
                          <div className="font-medium">
                            {indicator.indicator_type}
                          </div>
                          <div className="text-sm text-muted-foreground">
                            {indicator.description}
                          </div>
                          <div className="mt-1 text-sm text-blue-600">
                            Weight: {indicator.weight.toFixed(1)}
                          </div>
                        </div>
                      </div>
                    ))}
                </div>
              </div>
            )}

            <Separator />

            {/* Industry Recommendations */}
            {industryAnalysis.industry_specific_recommendations.length > 0 && (
              <div>
                <h4 className="mb-3 font-semibold">
                  Industry-Specific Recommendations
                </h4>
                <div className="space-y-2">
                  {industryAnalysis.industry_specific_recommendations
                    .slice(0, 5)
                    .map((recommendation, index) => (
                      <div
                        key={index}
                        className="flex items-start space-x-3 rounded-lg bg-blue-50 p-3"
                      >
                        <Target className="mt-0.5 h-4 w-4 text-blue-500" />
                        <div className="text-sm">{recommendation}</div>
                      </div>
                    ))}
                </div>
              </div>
            )}

            {/* Industry Trends */}
            {industryAnalysis.industry_trends.length > 0 && (
              <div>
                <h4 className="mb-3 font-semibold">Industry Trends</h4>
                <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                  {industryAnalysis.industry_trends
                    .slice(0, 4)
                    .map((trend, index) => (
                      <div key={index} className="rounded-lg border p-3">
                        <div className="flex items-center justify-between">
                          <span className="font-medium">
                            {trend.trend_name}
                          </span>
                          <Badge
                            variant={
                              trend.trend_type === 'growing'
                                ? 'default'
                                : 'secondary'
                            }
                          >
                            {trend.trend_type}
                          </Badge>
                        </div>
                        <div className="mt-1 text-sm text-muted-foreground">
                          Relevance: {(trend.relevance_score * 100).toFixed(1)}%
                        </div>
                        {trend.found_in_resume && (
                          <div className="mt-1 text-sm text-green-600">
                            ✓ Found in resume
                          </div>
                        )}
                      </div>
                    ))}
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* ATS Validation Suite */}
      {atsValidation && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center">
              <Zap className="mr-2 h-5 w-5" />
              ATS Validation Suite
            </CardTitle>
            <CardDescription>
              Comprehensive ATS validation testing and benchmarking
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* Overall Accuracy */}
            <div className="text-center">
              <div className="text-3xl font-bold text-blue-600">
                {(atsValidation.overall_accuracy * 100).toFixed(1)}%
              </div>
              <div className="mt-1 text-sm text-muted-foreground">
                Overall ATS Accuracy
              </div>
              <div className="mt-2 text-lg font-semibold">
                {(atsValidation.confidence_score * 100).toFixed(1)}% Confidence
                Score
              </div>
            </div>

            {/* Accuracy Metrics */}
            <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
              <div className="text-center">
                <div className="text-2xl font-bold text-green-600">
                  {(atsValidation.format_detection_accuracy * 100).toFixed(1)}%
                </div>
                <div className="text-sm text-muted-foreground">
                  Format Detection
                </div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-blue-600">
                  {(atsValidation.parsing_simulation_accuracy * 100).toFixed(1)}
                  %
                </div>
                <div className="text-sm text-muted-foreground">
                  Parsing Simulation
                </div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-purple-600">
                  {(atsValidation.keyword_extraction_accuracy * 100).toFixed(1)}
                  %
                </div>
                <div className="text-sm text-muted-foreground">
                  Keyword Extraction
                </div>
              </div>
            </div>

            {/* Per-ATS Accuracy */}
            <div>
              <h4 className="mb-3 font-semibold">ATS System Accuracy</h4>
              <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                {Object.entries(atsValidation.per_ats_accuracy).map(
                  ([system, accuracy]) => (
                    <div key={system} className="rounded-lg border p-3">
                      <div className="flex items-center justify-between">
                        <span className="font-medium capitalize">{system}</span>
                        <Badge
                          variant={
                            accuracy > 0.8
                              ? 'default'
                              : accuracy > 0.6
                                ? 'secondary'
                                : 'destructive'
                          }
                        >
                          {(accuracy * 100).toFixed(1)}%
                        </Badge>
                      </div>
                      <Progress value={accuracy * 100} className="mt-2" />
                    </div>
                  )
                )}
              </div>
            </div>

            {/* Benchmark Comparison */}
            <div>
              <h4 className="mb-3 font-semibold">Benchmark Comparison</h4>
              <div className="rounded-lg border p-4">
                <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
                  <div className="text-center">
                    <div className="text-2xl font-bold text-gray-600">
                      {(
                        atsValidation.benchmark_comparison.baseline_accuracy *
                        100
                      ).toFixed(1)}
                      %
                    </div>
                    <div className="text-sm text-muted-foreground">
                      Baseline Accuracy
                    </div>
                  </div>
                  <div className="text-center">
                    <div className="text-2xl font-bold text-blue-600">
                      {(
                        atsValidation.benchmark_comparison.current_accuracy *
                        100
                      ).toFixed(1)}
                      %
                    </div>
                    <div className="text-sm text-muted-foreground">
                      Current Accuracy
                    </div>
                  </div>
                  <div className="text-center">
                    <div
                      className={`text-2xl font-bold ${
                        atsValidation.benchmark_comparison
                          .improvement_percentage > 0
                          ? 'text-green-600'
                          : 'text-red-600'
                      }`}
                    >
                      {atsValidation.benchmark_comparison
                        .improvement_percentage > 0
                        ? '+'
                        : ''}
                      {atsValidation.benchmark_comparison.improvement_percentage.toFixed(
                        1
                      )}
                      %
                    </div>
                    <div className="text-sm text-muted-foreground">
                      Improvement
                    </div>
                    <Badge
                      variant={
                        atsValidation.benchmark_comparison.performance_trend ===
                        'improving'
                          ? 'default'
                          : 'secondary'
                      }
                      className="mt-1"
                    >
                      {atsValidation.benchmark_comparison.performance_trend}
                    </Badge>
                  </div>
                </div>
              </div>
            </div>

            {/* Improvement Suggestions */}
            {atsValidation.improvement_suggestions.length > 0 && (
              <div>
                <h4 className="mb-3 font-semibold">Improvement Suggestions</h4>
                <div className="space-y-3">
                  {atsValidation.improvement_suggestions.map(
                    (suggestion, index) => (
                      <div key={index} className="rounded-lg border p-3">
                        <div className="flex items-start justify-between">
                          <div className="flex-1">
                            <div className="flex items-center">
                              <span className="font-medium">
                                {suggestion.category}
                              </span>
                              <Badge
                                variant={
                                  suggestion.priority === 'high'
                                    ? 'destructive'
                                    : suggestion.priority === 'medium'
                                      ? 'secondary'
                                      : 'outline'
                                }
                                className="ml-2 text-xs"
                              >
                                {suggestion.priority}
                              </Badge>
                            </div>
                            <p className="mt-1 text-sm text-muted-foreground">
                              {suggestion.description}
                            </p>
                            <div className="mt-2 flex items-center gap-2 text-sm">
                              <span className="text-muted-foreground">
                                Effort: {suggestion.implementation_effort}
                              </span>
                              <span className="text-green-600">
                                Expected: +
                                {(
                                  suggestion.expected_improvement * 100
                                ).toFixed(1)}
                                %
                              </span>
                            </div>
                          </div>
                        </div>
                      </div>
                    )
                  )}
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* ATS Simulation Results */}
      {atsSimulation && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center">
              <Users className="mr-2 h-5 w-5" />
              ATS System Simulation
            </CardTitle>
            <CardDescription>
              Multi-ATS system compatibility analysis and optimization
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* Overall ATS Score */}
            <div className="text-center">
              <div className="text-3xl font-bold text-blue-600">
                {(atsSimulation.overall_ats_score * 100).toFixed(1)}%
              </div>
              <div className="mt-1 text-sm text-muted-foreground">
                Overall ATS Compatibility Score
              </div>
            </div>

            {/* System Simulations */}
            <div>
              <h4 className="mb-3 font-semibold">ATS System Results</h4>
              <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                {Object.entries(atsSimulation.system_simulations).map(
                  ([systemName, result]) => (
                    <div key={systemName} className="rounded-lg border p-4">
                      <div className="mb-3 flex items-center justify-between">
                        <span className="font-medium">
                          {result.system_name}
                        </span>
                        <Badge
                          variant={
                            result.compatibility_score > 0.8
                              ? 'default'
                              : result.compatibility_score > 0.6
                                ? 'secondary'
                                : 'destructive'
                          }
                        >
                          {(result.compatibility_score * 100).toFixed(1)}%
                        </Badge>
                      </div>
                      <div className="space-y-2">
                        <div className="flex justify-between text-sm">
                          <span>Parsing Success:</span>
                          <span>
                            {(result.parsing_success_rate * 100).toFixed(1)}%
                          </span>
                        </div>
                        <div className="flex justify-between text-sm">
                          <span>Keyword Detection:</span>
                          <span>
                            {(result.keyword_detection_rate * 100).toFixed(1)}%
                          </span>
                        </div>
                        <div className="flex justify-between text-sm">
                          <span>Format Compliance:</span>
                          <span
                            className={
                              result.format_compliance.meets_standards
                                ? 'text-green-600'
                                : 'text-red-600'
                            }
                          >
                            {result.format_compliance.meets_standards
                              ? 'Meets Standards'
                              : 'Issues Found'}
                          </span>
                        </div>
                      </div>
                    </div>
                  )
                )}
              </div>
            </div>

            {/* Parsing Analysis */}
            <div>
              <h4 className="mb-3 font-semibold">Parsing Analysis</h4>
              <div className="rounded-lg border p-4">
                <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                  <div>
                    <h5 className="mb-2 font-medium">Contact Information</h5>
                    <div className="space-y-1 text-sm">
                      <div className="flex items-center">
                        <CheckCircle
                          className={`mr-2 h-4 w-4 ${
                            atsSimulation.parsing_analysis
                              .contact_info_extraction.email_detected
                              ? 'text-green-600'
                              : 'text-red-600'
                          }`}
                        />
                        <span>Email</span>
                      </div>
                      <div className="flex items-center">
                        <CheckCircle
                          className={`mr-2 h-4 w-4 ${
                            atsSimulation.parsing_analysis
                              .contact_info_extraction.phone_detected
                              ? 'text-green-600'
                              : 'text-red-600'
                          }`}
                        />
                        <span>Phone</span>
                      </div>
                      <div className="flex items-center">
                        <CheckCircle
                          className={`mr-2 h-4 w-4 ${
                            atsSimulation.parsing_analysis
                              .contact_info_extraction.linkedin_detected
                              ? 'text-green-600'
                              : 'text-red-600'
                          }`}
                        />
                        <span>LinkedIn</span>
                      </div>
                    </div>
                  </div>
                  <div>
                    <h5 className="mb-2 font-medium">Experience Parsing</h5>
                    <div className="space-y-1 text-sm">
                      <div className="flex justify-between">
                        <span>Jobs Detected:</span>
                        <span>
                          {
                            atsSimulation.parsing_analysis
                              .work_experience_parsing.jobs_detected
                          }
                        </span>
                      </div>
                      <div className="flex justify-between">
                        <span>Date Accuracy:</span>
                        <span>
                          {(
                            atsSimulation.parsing_analysis
                              .work_experience_parsing.date_parsing_accuracy *
                            100
                          ).toFixed(1)}
                          %
                        </span>
                      </div>
                      <div className="flex justify-between">
                        <span>Title Accuracy:</span>
                        <span>
                          {(
                            atsSimulation.parsing_analysis
                              .work_experience_parsing
                              .title_extraction_accuracy * 100
                          ).toFixed(1)}
                          %
                        </span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            {/* Keyword Extraction */}
            <div>
              <h4 className="mb-3 font-semibold">Keyword Extraction</h4>
              <div className="rounded-lg border p-4">
                <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                  <div>
                    <div className="text-2xl font-bold text-blue-600">
                      {(
                        atsSimulation.keyword_extraction.extraction_accuracy *
                        100
                      ).toFixed(1)}
                      %
                    </div>
                    <div className="text-sm text-muted-foreground">
                      Extraction Accuracy
                    </div>
                  </div>
                  <div>
                    <div className="text-2xl font-bold text-green-600">
                      {atsSimulation.keyword_extraction.keywords_found.length}
                    </div>
                    <div className="text-sm text-muted-foreground">
                      Keywords Found
                    </div>
                  </div>
                </div>
                {atsSimulation.keyword_extraction.missed_keywords.length >
                  0 && (
                  <div className="mt-4">
                    <h5 className="mb-2 font-medium">Missed Keywords</h5>
                    <div className="flex flex-wrap gap-2">
                      {atsSimulation.keyword_extraction.missed_keywords
                        .slice(0, 8)
                        .map((keyword, index) => (
                          <Badge
                            key={index}
                            variant="outline"
                            className="text-xs"
                          >
                            {keyword}
                          </Badge>
                        ))}
                      {atsSimulation.keyword_extraction.missed_keywords.length >
                        8 && (
                        <Badge variant="outline" className="text-xs">
                          +
                          {atsSimulation.keyword_extraction.missed_keywords
                            .length - 8}{' '}
                          more
                        </Badge>
                      )}
                    </div>
                  </div>
                )}
              </div>
            </div>

            {/* Optimization Recommendations */}
            {atsSimulation.optimization_recommendations.length > 0 && (
              <div>
                <h4 className="mb-3 font-semibold">
                  Optimization Recommendations
                </h4>
                <div className="space-y-3">
                  {atsSimulation.optimization_recommendations
                    .slice(0, 6)
                    .map((rec, index) => (
                      <div key={index} className="rounded-lg border p-3">
                        <div className="flex items-start justify-between">
                          <div className="flex-1">
                            <div className="flex items-center">
                              <span className="font-medium">{rec.title}</span>
                              <Badge
                                variant={
                                  rec.priority === 'critical'
                                    ? 'destructive'
                                    : rec.priority === 'high'
                                      ? 'default'
                                      : 'secondary'
                                }
                                className="ml-2 text-xs"
                              >
                                {rec.priority}
                              </Badge>
                            </div>
                            <p className="mt-1 text-sm text-muted-foreground">
                              {rec.description}
                            </p>
                            <div className="mt-2 flex items-center gap-2 text-sm">
                              <span className="text-muted-foreground">
                                Category: {rec.category}
                              </span>
                              <span className="text-green-600">
                                Expected: +
                                {(rec.expected_improvement * 100).toFixed(1)}%
                              </span>
                            </div>
                            {rec.affected_systems.length > 0 && (
                              <div className="mt-2 flex gap-1">
                                {rec.affected_systems.map(
                                  (system, sysIndex) => (
                                    <Badge
                                      key={sysIndex}
                                      variant="outline"
                                      className="text-xs"
                                    >
                                      {system}
                                    </Badge>
                                  )
                                )}
                              </div>
                            )}
                          </div>
                        </div>
                      </div>
                    ))}
                </div>
              </div>
            )}

            {/* Compatibility Issues */}
            {atsSimulation.compatibility_issues.length > 0 && (
              <div>
                <h4 className="mb-3 font-semibold">Compatibility Issues</h4>
                <div className="space-y-3">
                  {atsSimulation.compatibility_issues
                    .filter(
                      issue =>
                        issue.severity === 'critical' ||
                        issue.severity === 'major'
                    )
                    .map((issue, index) => (
                      <div key={index} className="rounded-lg border p-3">
                        <div className="flex items-start justify-between">
                          <div className="flex-1">
                            <div className="flex items-center">
                              <AlertCircle
                                className={`mr-2 h-4 w-4 ${
                                  issue.severity === 'critical'
                                    ? 'text-red-600'
                                    : issue.severity === 'major'
                                      ? 'text-yellow-600'
                                      : 'text-blue-600'
                                }`}
                              />
                              <span className="font-medium">
                                {issue.issue_type}
                              </span>
                              <Badge
                                variant={
                                  issue.severity === 'critical'
                                    ? 'destructive'
                                    : issue.severity === 'major'
                                      ? 'default'
                                      : 'secondary'
                                }
                                className="ml-2 text-xs"
                              >
                                {issue.severity}
                              </Badge>
                            </div>
                            <p className="mt-1 text-sm text-muted-foreground">
                              {issue.description}
                            </p>
                            <div className="mt-2 flex items-center gap-2 text-sm">
                              <span className="text-muted-foreground">
                                Impact: {(issue.impact_score * 100).toFixed(1)}%
                              </span>
                              <span className="text-muted-foreground">
                                Difficulty: {issue.resolution_difficulty}
                              </span>
                            </div>
                            {issue.affected_systems.length > 0 && (
                              <div className="mt-2 flex gap-1">
                                {issue.affected_systems.map(
                                  (system, sysIndex) => (
                                    <Badge
                                      key={sysIndex}
                                      variant="outline"
                                      className="text-xs"
                                    >
                                      {system}
                                    </Badge>
                                  )
                                )}
                              </div>
                            )}
                          </div>
                        </div>
                      </div>
                    ))}
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Comprehensive Analysis */}
      {comprehensiveAnalysis && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center">
              <Zap className="mr-2 h-5 w-5" />
              Comprehensive Analysis Dashboard
            </CardTitle>
            <CardDescription>
              AI-powered multi-dimensional analysis with advanced scoring,
              benchmarking, and optimization insights
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-8">
            {/* Scoring Breakdown Section */}
            {comprehensiveAnalysis.scoring_breakdown && (
              <div className="space-y-6">
                <div className="flex items-center">
                  <Target className="mr-2 h-5 w-5 text-blue-600" />
                  <h4 className="text-lg font-semibold">
                    Advanced Scoring Breakdown
                  </h4>
                </div>

                {/* Final Score Calculations */}
                {comprehensiveAnalysis.scoring_breakdown.final_calculations && (
                  <div className="rounded-lg border p-4">
                    <h5 className="mb-3 font-medium">
                      Score Components Analysis
                    </h5>
                    <div className="grid grid-cols-2 gap-4 md:grid-cols-3">
                      <div className="text-center">
                        <div className="text-2xl font-bold text-blue-600">
                          {comprehensiveAnalysis.scoring_breakdown.final_calculations.base_score.toFixed(
                            1
                          )}
                        </div>
                        <div className="text-sm text-muted-foreground">
                          Base Score
                        </div>
                      </div>
                      <div className="text-center">
                        <div className="text-2xl font-bold text-green-600">
                          +
                          {comprehensiveAnalysis.scoring_breakdown.final_calculations.industry_bonus.toFixed(
                            1
                          )}
                        </div>
                        <div className="text-sm text-muted-foreground">
                          Industry Bonus
                        </div>
                      </div>
                      <div className="text-center">
                        <div className="text-2xl font-bold text-purple-600">
                          +
                          {comprehensiveAnalysis.scoring_breakdown.final_calculations.role_level_bonus.toFixed(
                            1
                          )}
                        </div>
                        <div className="text-sm text-muted-foreground">
                          Role Level Bonus
                        </div>
                      </div>
                      <div className="text-center">
                        <div className="text-2xl font-bold text-cyan-600">
                          +
                          {comprehensiveAnalysis.scoring_breakdown.final_calculations.semantic_bonus.toFixed(
                            1
                          )}
                        </div>
                        <div className="text-sm text-muted-foreground">
                          Semantic Bonus
                        </div>
                      </div>
                      <div className="text-center">
                        <div className="text-2xl font-bold text-red-600">
                          {comprehensiveAnalysis.scoring_breakdown.final_calculations.ats_penalty.toFixed(
                            1
                          )}
                        </div>
                        <div className="text-sm text-muted-foreground">
                          ATS Penalty
                        </div>
                      </div>
                      <div className="border-l-2 border-gray-200 pl-4 text-center">
                        <div className="text-3xl font-bold text-indigo-600">
                          {comprehensiveAnalysis.scoring_breakdown.final_calculations.final_score.toFixed(
                            1
                          )}
                        </div>
                        <div className="text-sm font-medium text-muted-foreground">
                          Final Score
                        </div>
                      </div>
                    </div>
                  </div>
                )}

                {/* Weighted Scores */}
                {Object.keys(
                  comprehensiveAnalysis.scoring_breakdown.weighted_scores
                ).length > 0 && (
                  <div className="space-y-3">
                    <h5 className="font-medium">Category Weight Analysis</h5>
                    <div className="space-y-2">
                      {Object.entries(
                        comprehensiveAnalysis.scoring_breakdown.weighted_scores
                      ).map(([category, score]) => (
                        <div
                          key={category}
                          className="flex items-center justify-between rounded bg-gray-50 p-3"
                        >
                          <div className="space-y-1">
                            <div className="font-medium capitalize">
                              {category.replace('_', ' ')}
                            </div>
                            <div className="text-sm text-muted-foreground">
                              {score.explanation}
                            </div>
                          </div>
                          <div className="text-right">
                            <div className="text-lg font-semibold">
                              {score.adjusted_score.toFixed(1)}
                            </div>
                            <div className="text-xs text-muted-foreground">
                              {score.raw_score.toFixed(1)} ×{' '}
                              {score.weight.toFixed(2)}
                            </div>
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            )}

            <Separator />

            {/* Benchmark Comparison Section */}
            {comprehensiveAnalysis.benchmarks_comparison && (
              <div className="space-y-6">
                <div className="flex items-center">
                  <TrendingUp className="mr-2 h-5 w-5 text-green-600" />
                  <h4 className="text-lg font-semibold">
                    Industry Benchmark Analysis
                  </h4>
                </div>

                <div className="grid grid-cols-1 gap-6 md:grid-cols-3">
                  <div className="rounded-lg border p-4 text-center">
                    <div className="mb-2 text-3xl font-bold text-blue-600">
                      {comprehensiveAnalysis.benchmarks_comparison.percentile_ranking.toFixed(
                        0
                      )}
                      %
                    </div>
                    <div className="text-sm font-medium text-muted-foreground">
                      Percentile Ranking
                    </div>
                    <div className="mt-1 text-xs text-muted-foreground">
                      Among similar resumes
                    </div>
                  </div>

                  <div className="rounded-lg border p-4 text-center">
                    <div className="mb-2 text-3xl font-bold text-green-600">
                      {comprehensiveAnalysis.benchmarks_comparison.industry_benchmark.toFixed(
                        1
                      )}
                    </div>
                    <div className="text-sm font-medium text-muted-foreground">
                      Industry Benchmark
                    </div>
                    <div className="mt-1 text-xs text-muted-foreground">
                      Average industry score
                    </div>
                  </div>

                  <div className="rounded-lg border p-4 text-center">
                    <div className="mb-2 text-3xl font-bold text-purple-600">
                      {comprehensiveAnalysis.benchmarks_comparison.improvement_potential.toFixed(
                        1
                      )}
                    </div>
                    <div className="text-sm font-medium text-muted-foreground">
                      Improvement Potential
                    </div>
                    <div className="mt-1 text-xs text-muted-foreground">
                      Points available
                    </div>
                  </div>
                </div>

                {/* Peer Comparison */}
                {comprehensiveAnalysis.benchmarks_comparison
                  .peer_comparison && (
                  <div className="grid grid-cols-1 gap-6 md:grid-cols-2">
                    <div className="space-y-3">
                      <h5 className="font-medium text-green-600">
                        Strengths vs Peers
                      </h5>
                      <div className="space-y-2">
                        {comprehensiveAnalysis.benchmarks_comparison.peer_comparison.above_average_areas.map(
                          (area, index) => (
                            <div key={index} className="flex items-center">
                              <CheckCircle className="mr-2 h-4 w-4 text-green-600" />
                              <span className="text-sm">{area}</span>
                            </div>
                          )
                        )}
                        {comprehensiveAnalysis.benchmarks_comparison.peer_comparison.standout_strengths.map(
                          (strength, index) => (
                            <div key={index} className="flex items-center">
                              <Award className="mr-2 h-4 w-4 text-yellow-600" />
                              <span className="text-sm font-medium">
                                {strength}
                              </span>
                            </div>
                          )
                        )}
                      </div>
                    </div>

                    <div className="space-y-3">
                      <h5 className="font-medium text-orange-600">
                        Improvement Areas
                      </h5>
                      <div className="space-y-2">
                        {comprehensiveAnalysis.benchmarks_comparison.peer_comparison.below_average_areas.map(
                          (area, index) => (
                            <div key={index} className="flex items-center">
                              <AlertCircle className="mr-2 h-4 w-4 text-orange-600" />
                              <span className="text-sm">{area}</span>
                            </div>
                          )
                        )}
                        {comprehensiveAnalysis.benchmarks_comparison.peer_comparison.critical_gaps.map(
                          (gap, index) => (
                            <div key={index} className="flex items-center">
                              <Info className="mr-2 h-4 w-4 text-red-600" />
                              <span className="text-sm font-medium">{gap}</span>
                            </div>
                          )
                        )}
                      </div>
                    </div>
                  </div>
                )}
              </div>
            )}

            <Separator />

            {/* Advanced Optimization Roadmap */}
            {comprehensiveAnalysis.optimization_suggestions &&
              comprehensiveAnalysis.optimization_suggestions.length > 0 && (
                <div className="space-y-6">
                  <div className="flex items-center">
                    <Target className="mr-2 h-5 w-5 text-purple-600" />
                    <h4 className="text-lg font-semibold">
                      Strategic Optimization Roadmap
                    </h4>
                  </div>

                  <div className="space-y-4">
                    {comprehensiveAnalysis.optimization_suggestions
                      .sort((a, b) => {
                        const priorityOrder = { high: 3, medium: 2, low: 1 };
                        return (
                          priorityOrder[
                            b.priority as keyof typeof priorityOrder
                          ] -
                          priorityOrder[
                            a.priority as keyof typeof priorityOrder
                          ]
                        );
                      })
                      .map((suggestion, index) => (
                        <div
                          key={index}
                          className="space-y-3 rounded-lg border p-4"
                        >
                          <div className="flex items-start justify-between">
                            <div className="space-y-1">
                              <div className="flex items-center gap-2">
                                <h5 className="font-medium">
                                  {suggestion.title}
                                </h5>
                                <Badge
                                  variant={
                                    suggestion.priority === 'high'
                                      ? 'destructive'
                                      : suggestion.priority === 'medium'
                                        ? 'default'
                                        : 'secondary'
                                  }
                                >
                                  {suggestion.priority} priority
                                </Badge>
                                <Badge variant="outline">
                                  {suggestion.implementation_difficulty} to
                                  implement
                                </Badge>
                              </div>
                              <p className="text-sm text-muted-foreground">
                                {suggestion.description}
                              </p>
                            </div>
                            <div className="text-right">
                              <div className="text-lg font-bold text-green-600">
                                +{suggestion.expected_impact.toFixed(1)}
                              </div>
                              <div className="text-xs text-muted-foreground">
                                Impact Score
                              </div>
                            </div>
                          </div>

                          {suggestion.specific_actions.length > 0 && (
                            <div className="space-y-2">
                              <h6 className="text-sm font-medium">
                                Action Steps:
                              </h6>
                              <ul className="space-y-1 text-sm">
                                {suggestion.specific_actions.map(
                                  (action, actionIndex) => (
                                    <li
                                      key={actionIndex}
                                      className="flex items-start"
                                    >
                                      <span className="mr-2 text-muted-foreground">
                                        •
                                      </span>
                                      <span>{action}</span>
                                    </li>
                                  )
                                )}
                              </ul>
                            </div>
                          )}

                          {suggestion.ats_systems_helped.length > 0 && (
                            <div className="flex items-center gap-2 text-sm">
                              <Shield className="h-4 w-4 text-blue-600" />
                              <span className="text-muted-foreground">
                                Improves compatibility with:
                              </span>
                              <div className="flex gap-1">
                                {suggestion.ats_systems_helped.map(
                                  (system, sysIndex) => (
                                    <Badge
                                      key={sysIndex}
                                      variant="outline"
                                      className="text-xs"
                                    >
                                      {system}
                                    </Badge>
                                  )
                                )}
                              </div>
                            </div>
                          )}
                        </div>
                      ))}
                  </div>
                </div>
              )}

            <Separator />

            {/* ATS Compatibility Summary */}
            {comprehensiveAnalysis.ats_compatibility && (
              <div className="space-y-4">
                <div className="flex items-center">
                  <Shield className="mr-2 h-5 w-5 text-blue-600" />
                  <h4 className="text-lg font-semibold">
                    ATS Compatibility Overview
                  </h4>
                </div>

                <div className="grid grid-cols-1 gap-6 md:grid-cols-2">
                  <div className="space-y-4">
                    <div className="rounded-lg border p-4 text-center">
                      <div className="mb-2 text-3xl font-bold text-blue-600">
                        {comprehensiveAnalysis.ats_compatibility.overall_compatibility_score.toFixed(
                          1
                        )}
                        %
                      </div>
                      <div className="text-sm font-medium text-muted-foreground">
                        Overall ATS Compatibility
                      </div>
                    </div>

                    {Object.keys(
                      comprehensiveAnalysis.ats_compatibility
                        .system_specific_scores
                    ).length > 0 && (
                      <div className="space-y-2">
                        <h5 className="font-medium">System-Specific Scores</h5>
                        {Object.entries(
                          comprehensiveAnalysis.ats_compatibility
                            .system_specific_scores
                        ).map(([system, score]) => (
                          <div
                            key={system}
                            className="flex items-center justify-between"
                          >
                            <span className="text-sm font-medium">
                              {system}
                            </span>
                            <div className="flex items-center gap-2">
                              <div className="h-2 w-20 rounded-full bg-gray-200">
                                <div
                                  className="h-2 rounded-full bg-blue-600"
                                  style={{ width: `${score}%` }}
                                ></div>
                              </div>
                              <span className="w-12 text-right text-sm font-medium">
                                {score.toFixed(1)}%
                              </span>
                            </div>
                          </div>
                        ))}
                      </div>
                    )}
                  </div>

                  <div className="space-y-4">
                    {comprehensiveAnalysis.ats_compatibility
                      .ats_optimization_suggestions.length > 0 && (
                      <div className="space-y-3">
                        <h5 className="font-medium">Quick ATS Improvements</h5>
                        <div className="space-y-2">
                          {comprehensiveAnalysis.ats_compatibility.ats_optimization_suggestions
                            .slice(0, 5)
                            .map((suggestion, index) => (
                              <div key={index} className="flex items-start">
                                <CheckCircle className="mr-2 mt-0.5 h-4 w-4 flex-shrink-0 text-green-600" />
                                <span className="text-sm">{suggestion}</span>
                              </div>
                            ))}
                        </div>
                      </div>
                    )}
                  </div>
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Competitive Analysis Dashboard */}
      {analysisData.competitiveAnalysis && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Users className="h-5 w-5 text-purple-600" />
              Competitive Analysis Dashboard
            </CardTitle>
            <CardDescription>
              Market positioning, peer comparison, and competitive intelligence
            </CardDescription>
            <div className="mt-4 flex items-center gap-2 rounded-lg border border-yellow-200 bg-yellow-50 p-3">
              <AlertCircle className="h-4 w-4 flex-shrink-0 text-yellow-600" />
              <div className="text-sm text-yellow-800">
                <strong>Beta Feature:</strong> This analysis is currently using
                simplified data models. Results may not reflect actual market
                conditions.
              </div>
            </div>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* Market Position Overview */}
            <div className="grid gap-4 md:grid-cols-3">
              <Card>
                <CardContent className="pt-4">
                  <div className="text-center">
                    <div className="text-2xl font-bold text-purple-600">
                      {analysisData.competitiveAnalysis.market_position.percentile_ranking.toFixed(
                        0
                      )}
                      th
                    </div>
                    <div className="text-sm text-muted-foreground">
                      Market Percentile
                    </div>
                  </div>
                </CardContent>
              </Card>
              <Card>
                <CardContent className="pt-4">
                  <div className="text-center">
                    <div className="text-2xl font-bold text-green-600">
                      {
                        analysisData.competitiveAnalysis.market_position
                          .competitive_advantages.length
                      }
                    </div>
                    <div className="text-sm text-muted-foreground">
                      Competitive Advantages
                    </div>
                  </div>
                </CardContent>
              </Card>
              <Card>
                <CardContent className="pt-4">
                  <div className="text-center">
                    <div className="text-2xl font-bold text-blue-600">
                      {analysisData.competitiveAnalysis.market_position.market_demand_score.toFixed(
                        1
                      )}
                    </div>
                    <div className="text-sm text-muted-foreground">
                      Market Demand Score
                    </div>
                  </div>
                </CardContent>
              </Card>
            </div>

            {/* Positioning Statement */}
            <div className="rounded-lg bg-purple-50 p-4 dark:bg-purple-900/20">
              <h4 className="mb-2 font-semibold text-purple-900 dark:text-purple-300">
                Market Positioning Statement
              </h4>
              <p className="text-purple-800 dark:text-purple-200">
                {
                  analysisData.competitiveAnalysis.market_position
                    .positioning_statement
                }
              </p>
            </div>

            {/* Strength Areas */}
            {analysisData.competitiveAnalysis.market_position.strength_areas
              .length > 0 && (
              <div>
                <h4 className="mb-3 flex items-center gap-2 font-semibold">
                  <CheckCircle className="h-4 w-4 text-green-600" />
                  Strength Areas
                </h4>
                <div className="grid gap-3 md:grid-cols-2">
                  {analysisData.competitiveAnalysis.market_position.strength_areas.map(
                    (strength, index) => (
                      <Card
                        key={index}
                        className="border-green-200 bg-green-50 dark:border-green-800 dark:bg-green-900/20"
                      >
                        <CardContent className="pt-4">
                          <div className="mb-2 flex items-center justify-between">
                            <span className="font-medium text-green-900 dark:text-green-300">
                              {strength.area}
                            </span>
                            <Badge
                              variant="secondary"
                              className="bg-green-100 text-green-800 dark:bg-green-800 dark:text-green-100"
                            >
                              {strength.market_percentile.toFixed(0)}th
                              percentile
                            </Badge>
                          </div>
                          <div className="mb-2 text-sm text-green-700 dark:text-green-200">
                            Position: {strength.relative_to_competition}
                          </div>
                          <Progress value={strength.score} className="h-2" />
                        </CardContent>
                      </Card>
                    )
                  )}
                </div>
              </div>
            )}

            {/* Competitive Advantages */}
            {analysisData.competitiveAnalysis.market_position
              .competitive_advantages.length > 0 && (
              <div>
                <h4 className="mb-3 flex items-center gap-2 font-semibold">
                  <Award className="h-4 w-4 text-yellow-600" />
                  Competitive Advantages
                </h4>
                <div className="space-y-3">
                  {analysisData.competitiveAnalysis.market_position.competitive_advantages.map(
                    (advantage, index) => (
                      <Card
                        key={index}
                        className="border-yellow-200 bg-yellow-50 dark:border-yellow-800 dark:bg-yellow-900/20"
                      >
                        <CardContent className="pt-4">
                          <div className="mb-2 flex items-center justify-between">
                            <span className="font-medium text-yellow-900 dark:text-yellow-300">
                              {advantage.advantage}
                            </span>
                            <Badge
                              variant="secondary"
                              className={`${
                                advantage.strength_level === 'strong'
                                  ? 'bg-green-100 text-green-800 dark:bg-green-800 dark:text-green-100'
                                  : advantage.strength_level === 'moderate'
                                    ? 'bg-yellow-100 text-yellow-800 dark:bg-yellow-800 dark:text-yellow-100'
                                    : 'bg-orange-100 text-orange-800 dark:bg-orange-800 dark:text-orange-100'
                              }`}
                            >
                              {advantage.strength_level}
                            </Badge>
                          </div>
                          <div className="mb-2 text-sm text-yellow-700 dark:text-yellow-200">
                            Market Rarity:{' '}
                            {(advantage.market_rarity * 100).toFixed(1)}% of
                            candidates have this
                          </div>
                          <div className="text-sm text-yellow-700 dark:text-yellow-200">
                            Sustainability: {advantage.sustainability}
                          </div>
                        </CardContent>
                      </Card>
                    )
                  )}
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Salary Intelligence Dashboard */}
      {analysisData.salaryInsights && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <DollarSign className="h-5 w-5 text-green-600" />
              Salary Intelligence Dashboard
            </CardTitle>
            <CardDescription>
              Compensation benchmarking, negotiation insights, and market
              positioning
            </CardDescription>
            <div className="mt-4 flex items-center gap-2 rounded-lg border border-yellow-200 bg-yellow-50 p-3">
              <AlertCircle className="h-4 w-4 flex-shrink-0 text-yellow-600" />
              <div className="text-sm text-yellow-800">
                <strong>Beta Feature:</strong> Salary predictions are currently
                using simplified models. Consult market research for final
                negotiations.
              </div>
            </div>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* Salary Overview */}
            <div className="grid gap-4 md:grid-cols-3">
              <Card>
                <CardContent className="pt-4">
                  <div className="text-center">
                    <div className="text-2xl font-bold text-green-600">
                      {analysisData.salaryInsights.competitive_salary_analysis?.market_percentile?.toFixed(
                        0
                      ) || 'N/A'}
                      th
                    </div>
                    <div className="text-sm text-muted-foreground">
                      Salary Percentile
                    </div>
                  </div>
                </CardContent>
              </Card>
              <Card>
                <CardContent className="pt-4">
                  <div className="text-center">
                    <div className="text-2xl font-bold text-blue-600">
                      {analysisData.salaryInsights.negotiation_positioning?.negotiation_strength?.toFixed(
                        1
                      ) || 'N/A'}
                    </div>
                    <div className="text-sm text-muted-foreground">
                      Negotiation Strength
                    </div>
                  </div>
                </CardContent>
              </Card>
              <Card>
                <CardContent className="pt-4">
                  <div className="text-center">
                    <div className="text-2xl font-bold text-purple-600">
                      {analysisData.salaryInsights.negotiation_positioning
                        ?.market_timing || 'N/A'}
                    </div>
                    <div className="text-sm text-muted-foreground">
                      Market Timing
                    </div>
                  </div>
                </CardContent>
              </Card>
            </div>

            {/* Salary Growth Trajectory */}
            {analysisData.salaryInsights.competitive_salary_analysis
              ?.salary_potential && (
              <div>
                <h4 className="mb-3 flex items-center gap-2 font-semibold">
                  <TrendingUp className="h-4 w-4 text-green-600" />
                  Salary Growth Trajectory
                </h4>
                <Card className="border-green-200 bg-green-50 dark:border-green-800 dark:bg-green-900/20">
                  <CardContent className="pt-4">
                    <div className="grid gap-4 md:grid-cols-4">
                      <div className="text-center">
                        <div className="text-lg font-bold text-green-700 dark:text-green-300">
                          $
                          {analysisData.salaryInsights.competitive_salary_analysis.salary_potential.current_estimated.toLocaleString()}
                        </div>
                        <div className="text-sm text-green-600 dark:text-green-400">
                          Current Est.
                        </div>
                      </div>
                      <div className="text-center">
                        <div className="text-lg font-bold text-green-700 dark:text-green-300">
                          $
                          {analysisData.salaryInsights.competitive_salary_analysis.salary_potential.short_term_potential.toLocaleString()}
                        </div>
                        <div className="text-sm text-green-600 dark:text-green-400">
                          6-12 Months
                        </div>
                      </div>
                      <div className="text-center">
                        <div className="text-lg font-bold text-green-700 dark:text-green-300">
                          $
                          {analysisData.salaryInsights.competitive_salary_analysis.salary_potential.medium_term_potential.toLocaleString()}
                        </div>
                        <div className="text-sm text-green-600 dark:text-green-400">
                          1-3 Years
                        </div>
                      </div>
                      <div className="text-center">
                        <div className="text-lg font-bold text-green-700 dark:text-green-300">
                          $
                          {analysisData.salaryInsights.competitive_salary_analysis.salary_potential.ceiling_estimate.toLocaleString()}
                        </div>
                        <div className="text-sm text-green-600 dark:text-green-400">
                          Ceiling
                        </div>
                      </div>
                    </div>
                    <div className="mt-4 text-center">
                      <Badge
                        variant="secondary"
                        className="bg-green-100 text-green-800 dark:bg-green-800 dark:text-green-100"
                      >
                        {
                          analysisData.salaryInsights
                            .competitive_salary_analysis.salary_potential
                            .growth_trajectory
                        }
                      </Badge>
                    </div>
                  </CardContent>
                </Card>
              </div>
            )}

            {/* Negotiation Strategies */}
            {analysisData.salaryInsights.negotiation_positioning
              ?.negotiation_strategies?.length > 0 && (
              <div>
                <h4 className="mb-3 flex items-center gap-2 font-semibold">
                  <Target className="h-4 w-4 text-blue-600" />
                  Negotiation Strategies
                </h4>
                <div className="space-y-3">
                  {analysisData.salaryInsights.negotiation_positioning.negotiation_strategies
                    .slice(0, 3)
                    .map((strategy, index) => (
                      <Card
                        key={index}
                        className="border-blue-200 bg-blue-50 dark:border-blue-800 dark:bg-blue-900/20"
                      >
                        <CardContent className="pt-4">
                          <div className="mb-2 flex items-center justify-between">
                            <span className="font-medium text-blue-900 dark:text-blue-300">
                              {strategy.strategy}
                            </span>
                            <Badge
                              variant="secondary"
                              className={`${
                                strategy.success_probability > 0.7
                                  ? 'bg-green-100 text-green-800 dark:bg-green-800 dark:text-green-100'
                                  : strategy.success_probability > 0.5
                                    ? 'bg-yellow-100 text-yellow-800 dark:bg-yellow-800 dark:text-yellow-100'
                                    : 'bg-red-100 text-red-800 dark:bg-red-800 dark:text-red-100'
                              }`}
                            >
                              {(strategy.success_probability * 100).toFixed(0)}%
                              success
                            </Badge>
                          </div>
                          <div className="mb-2 text-sm text-blue-700 dark:text-blue-200">
                            Risk Level: {strategy.risk_level} | Potential
                            Upside: +
                            {(strategy.potential_upside * 100).toFixed(1)}%
                          </div>
                          <Progress
                            value={strategy.success_probability * 100}
                            className="h-2"
                          />
                        </CardContent>
                      </Card>
                    ))}
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Hiring Probability Dashboard */}
      {analysisData.hiringProbabilityAnalysis && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Target className="h-5 w-5 text-orange-600" />
              Hiring Probability Dashboard
            </CardTitle>
            <CardDescription>
              Success predictions, probability analysis, and improvement
              opportunities
            </CardDescription>
            <div className="mt-4 flex items-center gap-2 rounded-lg border border-yellow-200 bg-yellow-50 p-3">
              <AlertCircle className="h-4 w-4 flex-shrink-0 text-yellow-600" />
              <div className="text-sm text-yellow-800">
                <strong>Beta Feature:</strong> Probability predictions are based
                on simplified algorithms. Use as guidance only.
              </div>
            </div>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* Probability Overview */}
            <div className="text-center">
              <div className="mb-2 text-4xl font-bold text-orange-600">
                {(
                  analysisData.hiringProbabilityAnalysis.overall_probability *
                  100
                ).toFixed(1)}
                %
              </div>
              <div className="mb-4 text-lg text-muted-foreground">
                Overall Hiring Probability
              </div>
              <Progress
                value={
                  analysisData.hiringProbabilityAnalysis.overall_probability *
                  100
                }
                className="mx-auto h-4 max-w-md"
              />
            </div>

            {/* Company Type Probabilities */}
            {analysisData.hiringProbabilityAnalysis.probability_by_company_type
              ?.length > 0 && (
              <div>
                <h4 className="mb-3 flex items-center gap-2 font-semibold">
                  <Users className="h-4 w-4 text-orange-600" />
                  Probability by Company Type
                </h4>
                <div className="grid gap-3 md:grid-cols-2">
                  {analysisData.hiringProbabilityAnalysis.probability_by_company_type.map(
                    (companyType, index) => (
                      <Card
                        key={index}
                        className="border-orange-200 bg-orange-50 dark:border-orange-800 dark:bg-orange-900/20"
                      >
                        <CardContent className="pt-4">
                          <div className="mb-2 flex items-center justify-between">
                            <span className="font-medium capitalize text-orange-900 dark:text-orange-300">
                              {companyType.company_type}
                            </span>
                            <Badge
                              variant="secondary"
                              className="bg-orange-100 text-orange-800 dark:bg-orange-800 dark:text-orange-100"
                            >
                              {(companyType.probability * 100).toFixed(1)}%
                            </Badge>
                          </div>
                          <div className="mb-2 text-sm text-orange-700 dark:text-orange-200">
                            Match Strength:{' '}
                            {companyType.match_strength.toFixed(1)}/10
                          </div>
                          <Progress
                            value={companyType.probability * 100}
                            className="h-2"
                          />
                        </CardContent>
                      </Card>
                    )
                  )}
                </div>
              </div>
            )}

            {/* Success Scenarios */}
            {analysisData.hiringProbabilityAnalysis.success_scenarios?.length >
              0 && (
              <div>
                <h4 className="mb-3 flex items-center gap-2 font-semibold">
                  <CheckCircle className="h-4 w-4 text-green-600" />
                  Success Scenarios
                </h4>
                <div className="space-y-3">
                  {analysisData.hiringProbabilityAnalysis.success_scenarios
                    .slice(0, 3)
                    .map((scenario, index) => (
                      <Card
                        key={index}
                        className="border-green-200 bg-green-50 dark:border-green-800 dark:bg-green-900/20"
                      >
                        <CardContent className="pt-4">
                          <div className="mb-2 flex items-center justify-between">
                            <span className="font-medium text-green-900 dark:text-green-300">
                              {scenario.scenario_name}
                            </span>
                            <Badge
                              variant="secondary"
                              className="bg-green-100 text-green-800 dark:bg-green-800 dark:text-green-100"
                            >
                              {(scenario.probability * 100).toFixed(1)}% chance
                            </Badge>
                          </div>
                          <div className="mb-2 text-sm text-green-700 dark:text-green-200">
                            Timeline: {scenario.timeline}
                          </div>
                          <Progress
                            value={scenario.probability * 100}
                            className="h-2"
                          />
                        </CardContent>
                      </Card>
                    ))}
                </div>
              </div>
            )}

            {/* Improvement Impact */}
            {analysisData.hiringProbabilityAnalysis.improvement_impact?.length >
              0 && (
              <div>
                <h4 className="mb-3 flex items-center gap-2 font-semibold">
                  <TrendingUp className="h-4 w-4 text-blue-600" />
                  High-Impact Improvements
                </h4>
                <div className="space-y-3">
                  {analysisData.hiringProbabilityAnalysis.improvement_impact
                    .slice(0, 3)
                    .map((improvement, index) => (
                      <Card
                        key={index}
                        className="border-blue-200 bg-blue-50 dark:border-blue-800 dark:bg-blue-900/20"
                      >
                        <CardContent className="pt-4">
                          <div className="mb-2 flex items-center justify-between">
                            <span className="font-medium text-blue-900 dark:text-blue-300">
                              {improvement.improvement}
                            </span>
                            <Badge
                              variant="secondary"
                              className="bg-blue-100 text-blue-800 dark:bg-blue-800 dark:text-blue-100"
                            >
                              +
                              {(improvement.probability_increase * 100).toFixed(
                                1
                              )}
                              %
                            </Badge>
                          </div>
                          <div className="mb-2 text-sm text-blue-700 dark:text-blue-200">
                            {improvement.current_probability.toFixed(1)}% →{' '}
                            {improvement.improved_probability.toFixed(1)}%
                          </div>
                          <div className="text-sm text-blue-700 dark:text-blue-200">
                            Effort: {improvement.implementation_effort} | ROI:{' '}
                            {improvement.roi_score.toFixed(1)}
                          </div>
                        </CardContent>
                      </Card>
                    ))}
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Market Intelligence Dashboard */}
      {analysisData.marketPositionAnalysis && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Brain className="h-5 w-5 text-indigo-600" />
              Market Intelligence Dashboard
            </CardTitle>
            <CardDescription>
              Competitive landscape analysis and strategic positioning insights
            </CardDescription>
            <div className="mt-4 flex items-center gap-2 rounded-lg border border-yellow-200 bg-yellow-50 p-3">
              <AlertCircle className="h-4 w-4 flex-shrink-0 text-yellow-600" />
              <div className="text-sm text-yellow-800">
                <strong>Beta Feature:</strong> Market intelligence is currently
                based on simplified models and may not reflect current market
                conditions.
              </div>
            </div>
          </CardHeader>
          <CardContent className="space-y-6">
            <div className="rounded-lg bg-indigo-50 p-4 dark:bg-indigo-900/20">
              <h4 className="mb-2 font-semibold text-indigo-900 dark:text-indigo-300">
                Market Intelligence Summary
              </h4>
              <p className="text-indigo-800 dark:text-indigo-200">
                Advanced market positioning and competitive intelligence
                analysis provides deep insights into your competitive standing
                and strategic opportunities in the current market landscape.
              </p>
            </div>

            <div className="grid gap-4 md:grid-cols-2">
              <Card className="border-indigo-200 bg-indigo-50 dark:border-indigo-800 dark:bg-indigo-900/20">
                <CardContent className="pt-4">
                  <div className="mb-2 flex items-center gap-2">
                    <Shield className="h-4 w-4 text-indigo-600" />
                    <span className="font-medium text-indigo-900 dark:text-indigo-300">
                      Competitive Positioning
                    </span>
                  </div>
                  <p className="text-sm text-indigo-700 dark:text-indigo-200">
                    Strategic analysis of your market position relative to peer
                    competitors and industry benchmarks.
                  </p>
                </CardContent>
              </Card>
              <Card className="border-indigo-200 bg-indigo-50 dark:border-indigo-800 dark:bg-indigo-900/20">
                <CardContent className="pt-4">
                  <div className="mb-2 flex items-center gap-2">
                    <Zap className="h-4 w-4 text-indigo-600" />
                    <span className="font-medium text-indigo-900 dark:text-indigo-300">
                      Strategic Opportunities
                    </span>
                  </div>
                  <p className="text-sm text-indigo-700 dark:text-indigo-200">
                    Identification of market gaps and emerging opportunities for
                    competitive advantage.
                  </p>
                </CardContent>
              </Card>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Individual ML Command Results */}

      {/* Application Success Prediction Dashboard */}
      {applicationSuccessPrediction && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center">
              <Target className="mr-2 h-5 w-5" />
              Application Success Prediction
            </CardTitle>
            <CardDescription>
              Detailed success probability analysis based on ML predictions
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            {applicationSuccessPrediction.success_prediction && (
              <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
                <div className="rounded-lg bg-muted p-4 text-center">
                  <div className="text-3xl font-bold text-green-600">
                    {(
                      applicationSuccessPrediction.success_prediction * 100
                    ).toFixed(1)}
                    %
                  </div>
                  <p className="text-sm text-muted-foreground">
                    Success Probability
                  </p>
                </div>
                <div className="rounded-lg bg-muted p-4 text-center">
                  <div className="text-3xl font-bold text-blue-600">
                    {(
                      applicationSuccessPrediction.confidence_metrics
                        ?.overall_confidence * 100 || 0
                    ).toFixed(1)}
                    %
                  </div>
                  <p className="text-sm text-muted-foreground">
                    Prediction Confidence
                  </p>
                </div>
                <div className="rounded-lg bg-muted p-4 text-center">
                  <div className="text-lg font-medium text-purple-600">
                    {new Date(
                      applicationSuccessPrediction.generated_at
                    ).toLocaleDateString()}
                  </div>
                  <p className="text-sm text-muted-foreground">Analysis Date</p>
                </div>
              </div>
            )}

            {applicationSuccessPrediction.confidence_metrics && (
              <div>
                <h4 className="mb-3 font-semibold">Confidence Metrics</h4>
                <div className="space-y-2">
                  {Object.entries(
                    applicationSuccessPrediction.confidence_metrics
                  )
                    .filter(([key]) => key !== 'overall_confidence')
                    .slice(0, 4)
                    .map(([key, value]) => (
                      <div
                        key={key}
                        className="flex items-center justify-between rounded-lg bg-muted p-3"
                      >
                        <span className="font-medium capitalize">
                          {key.replace(/_/g, ' ')}
                        </span>
                        <div className="flex items-center space-x-2">
                          <span className="font-bold">
                            {typeof value === 'number'
                              ? (value * 100).toFixed(1) + '%'
                              : value}
                          </span>
                          <Progress
                            value={typeof value === 'number' ? value * 100 : 0}
                            className="w-20"
                          />
                        </div>
                      </div>
                    ))}
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Career Path Suggestions Dashboard */}
      {careerPathSuggestions && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center">
              <TrendingUp className="mr-2 h-5 w-5" />
              AI Career Path Suggestions
            </CardTitle>
            <CardDescription>
              Advanced career guidance and growth trajectory analysis
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            {careerPathSuggestions.career_path_suggestions && (
              <div>
                <h4 className="mb-3 font-semibold">Recommended Career Paths</h4>
                <div className="grid gap-3">
                  {careerPathSuggestions.career_path_suggestions
                    .slice(0, 3)
                    .map((path, index: number) => (
                      <Card key={index} className="p-3">
                        <div className="mb-2 flex items-center justify-between">
                          <div className="font-medium">
                            {path.role || path.title}
                          </div>
                          <Badge variant="outline">
                            {path.match_score || path.confidence}% match
                          </Badge>
                        </div>
                        <div className="text-sm text-muted-foreground">
                          {path.description ||
                            path.requirements?.slice(0, 2).join(', ')}
                        </div>
                        {path.salary_range && (
                          <div className="mt-2 text-sm font-medium text-green-600">
                            ${path.salary_range.min?.toLocaleString()} - $
                            {path.salary_range.max?.toLocaleString()}
                          </div>
                        )}
                      </Card>
                    ))}
                </div>
              </div>
            )}

            {careerPathSuggestions.skill_demand_forecast && (
              <div>
                <h4 className="mb-3 font-semibold">Skill Demand Forecast</h4>
                <div className="grid gap-2">
                  {careerPathSuggestions.skill_demand_forecast
                    .slice(0, 4)
                    .map((skill, index: number) => (
                      <div
                        key={index}
                        className="flex items-center justify-between rounded-lg bg-muted p-3"
                      >
                        <div>
                          <div className="font-medium">{skill.skill}</div>
                          <div className="text-xs text-muted-foreground">
                            {skill.trend || 'Growing demand'}
                          </div>
                        </div>
                        <div className="text-right">
                          <Badge
                            variant={
                              skill.demand > 70 ? 'default' : 'secondary'
                            }
                          >
                            {skill.demand || 85}% demand
                          </Badge>
                        </div>
                      </div>
                    ))}
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* ML Salary Prediction Dashboard */}
      {salaryPredictionML && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center">
              <DollarSign className="mr-2 h-5 w-5" />
              ML Salary Prediction
            </CardTitle>
            <CardDescription>
              Machine learning-based salary predictions with confidence
              intervals
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            {salaryPredictionML.salary_prediction && (
              <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
                <div className="rounded-lg bg-muted p-4 text-center">
                  <div className="text-2xl font-bold text-green-600">
                    $
                    {salaryPredictionML.salary_prediction.predicted_range?.median?.toLocaleString() ||
                      salaryPredictionML.salary_prediction.base_prediction?.toLocaleString() ||
                      'N/A'}
                  </div>
                  <p className="text-sm text-muted-foreground">
                    Predicted Salary
                  </p>
                </div>
                <div className="rounded-lg bg-muted p-4 text-center">
                  <div className="text-2xl font-bold text-blue-600">
                    {(
                      salaryPredictionML.confidence_metrics
                        ?.prediction_reliability * 100 || 0
                    ).toFixed(1)}
                    %
                  </div>
                  <p className="text-sm text-muted-foreground">ML Confidence</p>
                </div>
                <div className="rounded-lg bg-muted p-4 text-center">
                  <div className="text-2xl font-bold text-purple-600">
                    {salaryPredictionML.salary_prediction.market_percentile ||
                      'N/A'}
                  </div>
                  <p className="text-sm text-muted-foreground">
                    Market Percentile
                  </p>
                </div>
              </div>
            )}

            {salaryPredictionML.salary_prediction?.predicted_range && (
              <div>
                <h4 className="mb-3 font-semibold">Salary Range Prediction</h4>
                <div className="space-y-2">
                  <div className="flex items-center justify-between rounded-lg bg-muted p-3">
                    <span>Minimum</span>
                    <span className="font-bold">
                      $
                      {salaryPredictionML.salary_prediction.predicted_range.min?.toLocaleString()}
                    </span>
                  </div>
                  <div className="flex items-center justify-between rounded-lg bg-muted p-3">
                    <span>Maximum</span>
                    <span className="font-bold">
                      $
                      {salaryPredictionML.salary_prediction.predicted_range.max?.toLocaleString()}
                    </span>
                  </div>
                  <div className="flex items-center justify-between rounded-lg bg-green-50 p-3">
                    <span>Expected (Median)</span>
                    <span className="font-bold text-green-600">
                      $
                      {salaryPredictionML.salary_prediction.predicted_range.median?.toLocaleString()}
                    </span>
                  </div>
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* ML Recommendations Dashboard */}
      {mlRecommendations && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center">
              <Sparkles className="mr-2 h-5 w-5" />
              ML-Powered Recommendations
            </CardTitle>
            <CardDescription>
              Personalized optimization recommendations based on machine
              learning analysis
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            {mlRecommendations.recommendations && (
              <div>
                <h4 className="mb-3 font-semibold">Priority Recommendations</h4>
                <div className="grid gap-3">
                  {mlRecommendations.recommendations
                    .slice(0, 5)
                    .map((rec, index: number) => (
                      <Card key={index} className="p-3">
                        <div className="mb-2 flex items-start justify-between">
                          <div className="flex items-center space-x-2">
                            <Badge
                              variant={
                                rec.priority === 'High'
                                  ? 'destructive'
                                  : rec.priority === 'Medium'
                                    ? 'secondary'
                                    : 'outline'
                              }
                            >
                              {rec.priority}
                            </Badge>
                            <Badge variant="outline">{rec.category}</Badge>
                          </div>
                          <div className="text-right">
                            <div className="text-sm font-bold text-green-600">
                              ROI: {rec.impact_score}/10
                            </div>
                            <div className="text-xs text-muted-foreground">
                              {rec.difficulty || 'Medium'}
                            </div>
                          </div>
                        </div>
                        <p className="mb-2 text-sm font-medium">
                          {rec.recommendation}
                        </p>
                        <p className="text-xs text-muted-foreground">
                          {rec.expected_outcome}
                        </p>
                      </Card>
                    ))}
                </div>
              </div>
            )}

            {mlRecommendations.optimization_prioritization && (
              <div>
                <h4 className="mb-3 font-semibold">Optimization Priorities</h4>
                <div className="space-y-2">
                  {mlRecommendations.optimization_prioritization
                    .slice(0, 4)
                    .map((priority, index: number) => (
                      <div
                        key={index}
                        className="flex items-center justify-between rounded-lg bg-muted p-3"
                      >
                        <div>
                          <div className="font-medium">
                            {priority.area || priority.category}
                          </div>
                          <div className="text-xs text-muted-foreground">
                            {priority.description}
                          </div>
                        </div>
                        <div className="text-right">
                          <div className="font-bold text-blue-600">
                            {priority.priority_score}/10
                          </div>
                          <Progress
                            value={priority.priority_score * 10}
                            className="mt-1 w-16"
                          />
                        </div>
                      </div>
                    ))}
                </div>
              </div>
            )}

            {mlRecommendations.confidence_metrics && (
              <div className="rounded-lg bg-blue-50 p-3 text-center">
                <div className="text-lg font-bold text-blue-600">
                  {(
                    mlRecommendations.confidence_metrics.overall_confidence *
                      100 || 0
                  ).toFixed(1)}
                  %
                </div>
                <p className="text-sm text-muted-foreground">
                  Recommendation Confidence
                </p>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Processing Information */}
      <Card>
        <CardContent className="pt-6">
          <div className="flex justify-between text-sm text-muted-foreground">
            <span>Processing time: {result.processing_time_ms}ms</span>
            <span>Analyzed on: {new Date(timestamp).toLocaleString()}</span>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
