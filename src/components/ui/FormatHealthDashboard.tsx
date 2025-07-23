import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import {
  AlertTriangle,
  CheckCircle,
  XCircle,
  Zap,
  TrendingDown,
  FileText,
  RefreshCw,
  Info,
  ArrowRight,
} from 'lucide-react';
import { CommandResult } from '@/types/api';

interface FormatIssue {
  issue_type: string;
  severity: string;
  description: string;
  location: string;
  recommendation: string;
  ats_impact: string;
}

interface ImprovementRecommendation {
  category: string;
  title: string;
  description: string;
  priority: string;
  implementation_difficulty: string;
  time_estimate: string;
  step_by_step_guide: string[];
  tools_needed: string[];
  expected_improvement: number;
  related_issues: string[];
}

interface BeforeAfterExample {
  issue_type: string;
  before_example: string;
  after_example: string;
  explanation: string;
  improvement_score: number;
}

interface ATSImpact {
  ats_name: string;
  compatibility_score: number;
  parsing_issues: string[];
  keyword_detection_impact: number;
  overall_impact: string;
  specific_recommendations: string[];
}

interface FormatIssueReport {
  critical_issues: FormatIssue[];
  high_priority_issues: FormatIssue[];
  medium_priority_issues: FormatIssue[];
  low_priority_issues: FormatIssue[];
  overall_format_score: number;
  improvement_recommendations: ImprovementRecommendation[];
  before_after_examples: BeforeAfterExample[];
  ats_specific_impacts: Record<string, ATSImpact>;
}

interface FormatHealthDashboardProps {
  resumeContent: string;
  onIssueSelect?: (_issue: FormatIssue) => void;
}

export function FormatHealthDashboard({
  resumeContent,
  onIssueSelect,
}: FormatHealthDashboardProps) {
  const [report, setReport] = useState<FormatIssueReport | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [selectedTab, setSelectedTab] = useState<
    'overview' | 'issues' | 'recommendations' | 'examples'
  >('overview');

  const analyzeFormatIssues = useCallback(async () => {
    if (!resumeContent.trim()) return;

    try {
      setIsAnalyzing(true);
      const result = await invoke<CommandResult<FormatIssueReport>>(
        'analyze_format_issues',
        { resumeContent }
      );

      if (result.success && result.data) {
        setReport(result.data);
      }
    } catch {
      // Format analysis failed - continue without report
    } finally {
      setIsAnalyzing(false);
    }
  }, [resumeContent]);

  useEffect(() => {
    if (resumeContent.trim()) {
      void analyzeFormatIssues();
    }
  }, [resumeContent, analyzeFormatIssues]);

  const getTotalIssues = () => {
    if (!report) return 0;
    return (
      report.critical_issues.length +
      report.high_priority_issues.length +
      report.medium_priority_issues.length +
      report.low_priority_issues.length
    );
  };

  const getHealthStatus = () => {
    if (!report) return { color: 'gray', icon: Info, text: 'No data' };

    const score = report.overall_format_score;
    if (score >= 90) {
      return { color: 'green', icon: CheckCircle, text: 'Excellent' };
    } else if (score >= 70) {
      return { color: 'blue', icon: CheckCircle, text: 'Good' };
    } else if (score >= 50) {
      return { color: 'yellow', icon: AlertTriangle, text: 'Needs Work' };
    } else {
      return { color: 'red', icon: XCircle, text: 'Critical' };
    }
  };

  const getSeverityColor = (severity: string) => {
    switch (severity.toLowerCase()) {
      case 'critical':
        return 'bg-red-100 text-red-800 border-red-200';
      case 'high':
        return 'bg-orange-100 text-orange-800 border-orange-200';
      case 'medium':
        return 'bg-yellow-100 text-yellow-800 border-yellow-200';
      case 'low':
        return 'bg-blue-100 text-blue-800 border-blue-200';
      default:
        return 'bg-gray-100 text-gray-800 border-gray-200';
    }
  };

  const getDifficultyColor = (difficulty: string) => {
    switch (difficulty.toLowerCase()) {
      case 'easy':
        return 'text-green-600';
      case 'medium':
        return 'text-yellow-600';
      case 'hard':
        return 'text-red-600';
      default:
        return 'text-gray-600';
    }
  };

  const healthStatus = getHealthStatus();
  const HealthIcon = healthStatus.icon;

  if (!resumeContent.trim()) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <FileText className="h-5 w-5" />
            Format Health Dashboard
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="py-8 text-center text-muted-foreground">
            <FileText className="mx-auto mb-4 h-12 w-12 opacity-50" />
            <p>Upload a resume to analyze format health</p>
          </div>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center justify-between">
          <span className="flex items-center gap-2">
            <FileText className="h-5 w-5" />
            Format Health Dashboard
          </span>
          <Button
            onClick={analyzeFormatIssues}
            disabled={isAnalyzing}
            size="sm"
            variant="outline"
          >
            {isAnalyzing ? (
              <>
                <RefreshCw className="mr-2 h-4 w-4 animate-spin" />
                Analyzing...
              </>
            ) : (
              <>
                <Zap className="mr-2 h-4 w-4" />
                Refresh
              </>
            )}
          </Button>
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* Health Overview */}
        <div className="grid gap-4 md:grid-cols-3">
          <div className="rounded-lg border p-4 text-center">
            <div className="mb-2 flex items-center justify-center">
              <HealthIcon
                className={`h-8 w-8 text-${healthStatus.color}-600`}
              />
            </div>
            <div className="text-2xl font-bold">
              {report?.overall_format_score.toFixed(0) ?? '—'}%
            </div>
            <div className="text-sm text-muted-foreground">Format Health</div>
            <Badge
              variant="outline"
              className={`mt-2 text-${healthStatus.color}-600 border-${healthStatus.color}-200`}
            >
              {healthStatus.text}
            </Badge>
          </div>

          <div className="rounded-lg border p-4 text-center">
            <div className="mb-2 flex items-center justify-center">
              <AlertTriangle className="h-8 w-8 text-orange-600" />
            </div>
            <div className="text-2xl font-bold">{getTotalIssues()}</div>
            <div className="text-sm text-muted-foreground">Total Issues</div>
            {report && (
              <div className="mt-2 text-xs text-muted-foreground">
                {report.critical_issues.length} critical,{' '}
                {report.high_priority_issues.length} high
              </div>
            )}
          </div>

          <div className="rounded-lg border p-4 text-center">
            <div className="mb-2 flex items-center justify-center">
              <TrendingDown className="h-8 w-8 text-blue-600" />
            </div>
            <div className="text-2xl font-bold">
              {report?.improvement_recommendations.length ?? 0}
            </div>
            <div className="text-sm text-muted-foreground">Recommendations</div>
            {report && (
              <div className="mt-2 text-xs text-muted-foreground">
                {
                  report.improvement_recommendations.filter(
                    r => r.priority === 'high'
                  ).length
                }{' '}
                high priority
              </div>
            )}
          </div>
        </div>

        {/* Progress Bars for ATS Compatibility */}
        {report && Object.keys(report.ats_specific_impacts).length > 0 && (
          <div className="space-y-3">
            <h3 className="font-semibold">ATS Compatibility</h3>
            <div className="space-y-2">
              {Object.entries(report.ats_specific_impacts)
                .slice(0, 3)
                .map(([atsName, impact]) => (
                  <div key={atsName} className="space-y-1">
                    <div className="flex justify-between text-sm">
                      <span className="font-medium">{impact.ats_name}</span>
                      <span>{Math.round(impact.compatibility_score)}%</span>
                    </div>
                    <Progress value={impact.compatibility_score} />
                  </div>
                ))}
            </div>
          </div>
        )}

        {/* Tab Navigation */}
        <div className="border-b">
          <div className="flex space-x-8">
            {[
              { id: 'overview', label: 'Overview', count: getTotalIssues() },
              { id: 'issues', label: 'Issues', count: getTotalIssues() },
              {
                id: 'recommendations',
                label: 'Fixes',
                count: report?.improvement_recommendations.length ?? 0,
              },
              {
                id: 'examples',
                label: 'Examples',
                count: report?.before_after_examples.length ?? 0,
              },
            ].map(tab => (
              <button
                key={tab.id}
                onClick={() =>
                  setSelectedTab(
                    tab.id as
                      | 'overview'
                      | 'issues'
                      | 'recommendations'
                      | 'examples'
                  )
                }
                className={`border-b-2 px-1 py-2 text-sm font-medium ${
                  selectedTab === tab.id
                    ? 'border-primary text-primary'
                    : 'border-transparent text-muted-foreground hover:text-foreground'
                }`}
              >
                {tab.label}
                {tab.count > 0 && (
                  <Badge variant="secondary" className="ml-2 text-xs">
                    {tab.count}
                  </Badge>
                )}
              </button>
            ))}
          </div>
        </div>

        {/* Tab Content */}
        {selectedTab === 'overview' && report && (
          <div className="space-y-4">
            <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
              <div className="rounded-lg border border-red-200 bg-red-50 p-3 text-center">
                <div className="text-lg font-bold text-red-700">
                  {report.critical_issues.length}
                </div>
                <div className="text-xs text-red-600">Critical</div>
              </div>
              <div className="rounded-lg border border-orange-200 bg-orange-50 p-3 text-center">
                <div className="text-lg font-bold text-orange-700">
                  {report.high_priority_issues.length}
                </div>
                <div className="text-xs text-orange-600">High Priority</div>
              </div>
              <div className="rounded-lg border border-yellow-200 bg-yellow-50 p-3 text-center">
                <div className="text-lg font-bold text-yellow-700">
                  {report.medium_priority_issues.length}
                </div>
                <div className="text-xs text-yellow-600">Medium</div>
              </div>
              <div className="rounded-lg border border-blue-200 bg-blue-50 p-3 text-center">
                <div className="text-lg font-bold text-blue-700">
                  {report.low_priority_issues.length}
                </div>
                <div className="text-xs text-blue-600">Low Priority</div>
              </div>
            </div>
          </div>
        )}

        {selectedTab === 'issues' && report && (
          <div className="space-y-4">
            {[
              ...report.critical_issues,
              ...report.high_priority_issues,
              ...report.medium_priority_issues,
            ]
              .slice(0, 10)
              .map((issue, index) => (
                <div
                  key={index}
                  className="cursor-pointer rounded-lg border p-4 transition-colors hover:bg-gray-50"
                  onClick={() => onIssueSelect?.(issue)}
                >
                  <div className="mb-2 flex items-start justify-between">
                    <div className="flex items-center gap-2">
                      <Badge className={getSeverityColor(issue.severity)}>
                        {issue.severity}
                      </Badge>
                      <span className="font-medium">{issue.issue_type}</span>
                    </div>
                    <ArrowRight className="h-4 w-4 text-muted-foreground" />
                  </div>
                  <p className="mb-2 text-sm text-muted-foreground">
                    {issue.description}
                  </p>
                  <div className="text-xs text-muted-foreground">
                    Location: {issue.location} • ATS Impact: {issue.ats_impact}
                  </div>
                </div>
              ))}
          </div>
        )}

        {selectedTab === 'recommendations' && report && (
          <div className="space-y-4">
            {report.improvement_recommendations
              .slice(0, 8)
              .map((rec, index) => (
                <div key={index} className="rounded-lg border p-4">
                  <div className="mb-2 flex items-start justify-between">
                    <div className="flex items-center gap-2">
                      <Badge variant="outline">{rec.category}</Badge>
                      <span className="font-medium">{rec.title}</span>
                    </div>
                    <div className="flex items-center gap-2 text-sm">
                      <span
                        className={getDifficultyColor(
                          rec.implementation_difficulty
                        )}
                      >
                        {rec.implementation_difficulty}
                      </span>
                      <span className="text-muted-foreground">•</span>
                      <span className="text-muted-foreground">
                        {rec.time_estimate}
                      </span>
                    </div>
                  </div>
                  <p className="mb-3 text-sm text-muted-foreground">
                    {rec.description}
                  </p>
                  <div className="space-y-2">
                    <div className="text-xs font-medium">Steps to fix:</div>
                    <ol className="space-y-1 text-xs text-muted-foreground">
                      {rec.step_by_step_guide
                        .slice(0, 3)
                        .map((step, stepIndex) => (
                          <li
                            key={stepIndex}
                            className="flex items-start gap-2"
                          >
                            <span className="font-medium text-primary">
                              {stepIndex + 1}.
                            </span>
                            {step}
                          </li>
                        ))}
                    </ol>
                    {rec.expected_improvement > 0 && (
                      <div className="text-xs font-medium text-green-600">
                        Expected improvement: +
                        {rec.expected_improvement.toFixed(1)}%
                      </div>
                    )}
                  </div>
                </div>
              ))}
          </div>
        )}

        {selectedTab === 'examples' && report && (
          <div className="space-y-4">
            {report.before_after_examples.slice(0, 5).map((example, index) => (
              <div key={index} className="rounded-lg border p-4">
                <div className="mb-3 flex items-center justify-between">
                  <Badge variant="outline">{example.issue_type}</Badge>
                  <div className="text-sm font-medium text-green-600">
                    +{example.improvement_score.toFixed(1)}% improvement
                  </div>
                </div>
                <div className="grid gap-4 md:grid-cols-2">
                  <div>
                    <div className="mb-1 text-xs font-medium text-red-600">
                      Before:
                    </div>
                    <div className="rounded border-l-2 border-red-200 bg-red-50 p-2 text-sm">
                      {example.before_example}
                    </div>
                  </div>
                  <div>
                    <div className="mb-1 text-xs font-medium text-green-600">
                      After:
                    </div>
                    <div className="rounded border-l-2 border-green-200 bg-green-50 p-2 text-sm">
                      {example.after_example}
                    </div>
                  </div>
                </div>
                <p className="mt-3 text-xs text-muted-foreground">
                  {example.explanation}
                </p>
              </div>
            ))}
          </div>
        )}
      </CardContent>
    </Card>
  );
}
