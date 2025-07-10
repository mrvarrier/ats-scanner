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
} from 'lucide-react';
import type { AnalysisResult, AchievementAnalysis, MLInsights } from '@/types';

interface AnalysisResultPageProps {
  analysisData: {
    result: AnalysisResult;
    achievementAnalysis?: AchievementAnalysis;
    mlInsights?: MLInsights;
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
