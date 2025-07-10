import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { AchievementAnalysis, MLInsights } from '@/types/api';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Input } from '@/components/ui/input';
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog';
import { useAppStore } from '@/store/useAppStore';
import {
  FileText,
  Download,
  Search,
  Filter,
  Calendar,
  BarChart3,
  TrendingUp,
  Trash2,
  RefreshCw,
  SortAsc,
  SortDesc,
  Archive,
} from 'lucide-react';
import { toast } from '@/hooks/use-toast';

interface AnalysisResult {
  id: string;
  resume_id: string;
  job_description_id: string;
  model_used: string;
  overall_score: number;
  skills_score: number;
  experience_score: number;
  education_score: number;
  keywords_score: number;
  format_score: number;
  detailed_feedback: string;
  missing_keywords: string;
  recommendations: string;
  processing_time_ms: number;
  created_at: string;
  resume_filename?: string;
}

interface CommandResult<T> {
  success: boolean;
  data?: T;
  error?: string;
}

interface ResumeInfo {
  id: string;
  filename: string;
  created_at: string;
}

export function ResultsPage() {
  const {
    analysisHistory,
    setAnalysisHistory,
    setCurrentDetailedAnalysis,
    setActiveTab,
  } = useAppStore();
  const [filteredResults, setFilteredResults] = useState<AnalysisResult[]>([]);
  const [searchTerm, setSearchTerm] = useState('');
  const [sortBy, setSortBy] = useState<'date' | 'score' | 'model'>('date');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('desc');
  const [filterScore, setFilterScore] = useState<
    'all' | 'high' | 'medium' | 'low'
  >('all');
  const [isLoading, setIsLoading] = useState(false);
  const [resumeMap, setResumeMap] = useState<Map<string, ResumeInfo>>(
    new Map()
  );
  const [deleteConfirmId, setDeleteConfirmId] = useState<string | null>(null);
  const [isDeleting, setIsDeleting] = useState(false);

  const loadAnalysisHistory = useCallback(async () => {
    setIsLoading(true);
    try {
      const result = await invoke<CommandResult<AnalysisResult[]>>(
        'get_analysis_history',
        {
          limit: 100,
          days: 365,
        }
      );

      if (result.success && result.data) {
        setAnalysisHistory(result.data);
      } else {
        throw new Error(result.error ?? 'Failed to load analysis history');
      }
    } catch (error) {
      toast({
        title: 'Error loading results',
        description: `Failed to load analysis history: ${error}`,
        variant: 'destructive',
      });
    } finally {
      setIsLoading(false);
    }
  }, [setAnalysisHistory]);

  const loadResumeInfo = useCallback(async () => {
    try {
      const result =
        await invoke<CommandResult<ResumeInfo[]>>('get_all_resumes');
      if (result.success && result.data) {
        const map = new Map();
        result.data.forEach(resume => {
          map.set(resume.id, resume);
        });
        setResumeMap(map);
      }
    } catch {
      // Resume info loading failed - continue without it
    }
  }, []);

  const filterAndSortResults = useCallback(() => {
    let filtered = [...analysisHistory];

    // Apply search filter
    if (searchTerm) {
      filtered = filtered.filter(
        result =>
          result.model_used.toLowerCase().includes(searchTerm.toLowerCase()) ||
          result.detailed_feedback
            .toLowerCase()
            .includes(searchTerm.toLowerCase()) ||
          resumeMap
            .get(result.resume_id)
            ?.filename.toLowerCase()
            .includes(searchTerm.toLowerCase())
      );
    }

    // Apply score filter
    if (filterScore !== 'all') {
      filtered = filtered.filter(result => {
        const score = result.overall_score;
        switch (filterScore) {
          case 'high':
            return score >= 80;
          case 'medium':
            return score >= 60 && score < 80;
          case 'low':
            return score < 60;
          default:
            return true;
        }
      });
    }

    // Apply sorting
    filtered.sort((a, b) => {
      let comparison = 0;

      switch (sortBy) {
        case 'date':
          comparison =
            new Date(a.created_at).getTime() - new Date(b.created_at).getTime();
          break;
        case 'score':
          comparison = a.overall_score - b.overall_score;
          break;
        case 'model':
          comparison = a.model_used.localeCompare(b.model_used);
          break;
      }

      return sortOrder === 'asc' ? comparison : -comparison;
    });

    setFilteredResults(filtered);
  }, [analysisHistory, searchTerm, sortBy, sortOrder, filterScore, resumeMap]);

  useEffect(() => {
    void loadAnalysisHistory();
    void loadResumeInfo();
  }, [loadAnalysisHistory, loadResumeInfo]);

  useEffect(() => {
    filterAndSortResults();
  }, [
    analysisHistory,
    searchTerm,
    sortBy,
    sortOrder,
    filterScore,
    filterAndSortResults,
  ]);

  const handleExportResult = async (result: AnalysisResult) => {
    try {
      const exportResult = await invoke<CommandResult<string>>(
        'export_results',
        {
          analysisIds: [result.id],
          format: 'json',
        }
      );

      if (exportResult.success) {
        toast({
          title: 'Export successful',
          description: `Analysis exported to ${exportResult.data}`,
        });
      } else {
        throw new Error(exportResult.error ?? 'Export failed');
      }
    } catch (error) {
      toast({
        title: 'Export failed',
        description: `Error: ${error}`,
        variant: 'destructive',
      });
    }
  };

  const handleDeleteClick = (resultId: string) => {
    setDeleteConfirmId(resultId);
  };

  const handleDeleteConfirm = async () => {
    if (!deleteConfirmId) return;

    setIsDeleting(true);
    try {
      const result = await invoke<CommandResult<boolean>>('delete_analysis', {
        id: deleteConfirmId,
      });

      if (result.success) {
        // Remove the deleted analysis from the local state
        const updatedHistory = analysisHistory.filter(
          analysis => analysis.id !== deleteConfirmId
        );
        setAnalysisHistory(updatedHistory);

        toast({
          title: 'Analysis deleted',
          description: 'The analysis has been successfully deleted.',
        });
      } else {
        throw new Error(result.error ?? 'Failed to delete analysis');
      }
    } catch (error) {
      toast({
        title: 'Delete failed',
        description: `Failed to delete analysis: ${error}`,
        variant: 'destructive',
      });
    } finally {
      setIsDeleting(false);
      setDeleteConfirmId(null);
    }
  };

  const handleDeleteCancel = () => {
    setDeleteConfirmId(null);
  };

  const handleViewFullAnalysis = async (result: AnalysisResult) => {
    try {
      // Convert the analysis result from history to the detailed format
      const resumeInfo = resumeMap.get(result.resume_id);

      // Create a basic analysis result structure
      const basicAnalysisResult = {
        overall_score: result.overall_score,
        category_scores: {
          skills: result.skills_score,
          experience: result.experience_score,
          education: result.education_score,
          keywords: result.keywords_score,
          format: result.format_score,
        },
        detailed_feedback: result.detailed_feedback,
        missing_keywords: parseMissingKeywords(result.missing_keywords),
        recommendations: parseRecommendations(result.recommendations),
        processing_time_ms: result.processing_time_ms,
      };

      // Try to get additional analysis data if available
      let achievementAnalysis: AchievementAnalysis | undefined = undefined;
      let mlInsights: MLInsights | undefined = undefined;

      // Note: We would need backend commands to retrieve these if stored
      // For now, we'll show what we have from the basic analysis

      const detailedAnalysisData = {
        result: basicAnalysisResult,
        achievementAnalysis: achievementAnalysis,
        mlInsights: mlInsights,
        resumeFilename:
          resumeInfo?.filename ?? `Resume ${result.resume_id.slice(0, 8)}`,
        jobDescription: '', // This would need to be retrieved from backend
        modelUsed: result.model_used,
        timestamp: result.created_at,
      };

      setCurrentDetailedAnalysis(detailedAnalysisData);
      setActiveTab('analysis-result');
    } catch {
      toast({
        title: 'Error',
        description: 'Failed to load detailed analysis',
        variant: 'destructive',
      });
    }
  };

  const getScoreColor = (score: number) => {
    if (score >= 80) return 'text-green-600';
    if (score >= 60) return 'text-yellow-600';
    return 'text-red-600';
  };

  const getScoreBadgeColor = (score: number) => {
    if (score >= 80)
      return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200';
    if (score >= 60)
      return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200';
    return 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200';
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  const parseMissingKeywords = (keywordsStr: string): string[] => {
    try {
      return (JSON.parse(keywordsStr) as string[]) ?? [];
    } catch {
      return keywordsStr ? [keywordsStr] : [];
    }
  };

  const parseRecommendations = (recommendationsStr: string): string[] => {
    try {
      return (JSON.parse(recommendationsStr) as string[]) ?? [];
    } catch {
      return recommendationsStr ? [recommendationsStr] : [];
    }
  };

  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <h1 className="text-3xl font-bold tracking-tight">Analysis Results</h1>
        <p className="text-muted-foreground">
          View and manage your past resume analysis results.
        </p>
      </div>

      {/* Filters and Search */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Filter className="h-5 w-5" />
            Filters & Search
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex flex-col gap-4 sm:flex-row">
            {/* Search */}
            <div className="flex-1">
              <div className="relative">
                <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 transform text-muted-foreground" />
                <Input
                  placeholder="Search by filename, model, or content..."
                  value={searchTerm}
                  onChange={e => setSearchTerm(e.target.value)}
                  className="pl-10"
                />
              </div>
            </div>

            {/* Sort */}
            <div className="flex gap-2">
              <select
                value={sortBy}
                onChange={e =>
                  setSortBy(e.target.value as 'date' | 'score' | 'model')
                }
                className="rounded-md border border-input bg-background px-3 py-2 text-sm"
              >
                <option value="date">Sort by Date</option>
                <option value="score">Sort by Score</option>
                <option value="model">Sort by Model</option>
              </select>

              <Button
                variant="outline"
                size="sm"
                onClick={() =>
                  setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc')
                }
              >
                {sortOrder === 'asc' ? (
                  <SortAsc className="h-4 w-4" />
                ) : (
                  <SortDesc className="h-4 w-4" />
                )}
              </Button>
            </div>

            {/* Score Filter */}
            <select
              value={filterScore}
              onChange={e =>
                setFilterScore(
                  e.target.value as 'all' | 'high' | 'medium' | 'low'
                )
              }
              className="rounded-md border border-input bg-background px-3 py-2 text-sm"
            >
              <option value="all">All Scores</option>
              <option value="high">High (80%+)</option>
              <option value="medium">Medium (60-79%)</option>
              <option value="low">Low (&lt;60%)</option>
            </select>

            {/* Refresh */}
            <Button
              variant="outline"
              size="sm"
              onClick={loadAnalysisHistory}
              disabled={isLoading}
            >
              {isLoading ? (
                <RefreshCw className="h-4 w-4 animate-spin" />
              ) : (
                <RefreshCw className="h-4 w-4" />
              )}
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Results Summary */}
      <div className="grid grid-cols-1 gap-4 md:grid-cols-4">
        <Card>
          <CardContent className="p-4">
            <div className="flex items-center gap-2">
              <Archive className="h-4 w-4 text-blue-600" />
              <span className="text-sm font-medium">Total Results</span>
            </div>
            <div className="text-2xl font-bold">{analysisHistory.length}</div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center gap-2">
              <BarChart3 className="h-4 w-4 text-green-600" />
              <span className="text-sm font-medium">Average Score</span>
            </div>
            <div className="text-2xl font-bold">
              {analysisHistory.length > 0
                ? (
                    analysisHistory.reduce(
                      (sum, r) => sum + r.overall_score,
                      0
                    ) / analysisHistory.length
                  ).toFixed(1)
                : 0}
              %
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center gap-2">
              <TrendingUp className="h-4 w-4 text-purple-600" />
              <span className="text-sm font-medium">High Scores</span>
            </div>
            <div className="text-2xl font-bold">
              {analysisHistory.filter(r => r.overall_score >= 80).length}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center gap-2">
              <Calendar className="h-4 w-4 text-orange-600" />
              <span className="text-sm font-medium">This Week</span>
            </div>
            <div className="text-2xl font-bold">
              {
                analysisHistory.filter(r => {
                  const date = new Date(r.created_at);
                  const weekAgo = new Date();
                  weekAgo.setDate(weekAgo.getDate() - 7);
                  return date > weekAgo;
                }).length
              }
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Results List */}
      <Card>
        <CardHeader>
          <CardTitle>Analysis Results ({filteredResults.length})</CardTitle>
          <CardDescription>
            View your resume analysis history and access detailed results
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          {filteredResults.length === 0 ? (
            <div className="py-8 text-center text-muted-foreground">
              {isLoading ? 'Loading results...' : 'No results found'}
            </div>
          ) : (
            filteredResults.map(result => {
              const resumeInfo = resumeMap.get(result.resume_id);
              return (
                <Card
                  key={result.id}
                  className="border-l-4 border-l-primary/20 transition-colors hover:border-l-primary"
                >
                  <CardContent className="p-6">
                    <div className="mb-4 flex items-start justify-between">
                      <div className="flex-1">
                        <div className="mb-3 flex items-center gap-3">
                          <FileText className="h-5 w-5 text-primary" />
                          <span className="text-lg font-semibold">
                            {resumeInfo?.filename ??
                              `Resume ${result.resume_id.slice(0, 8)}`}
                          </span>
                          <Badge variant="outline" className="text-xs">
                            {result.model_used}
                          </Badge>
                          <Badge
                            className={getScoreBadgeColor(result.overall_score)}
                          >
                            {result.overall_score >= 80
                              ? 'Excellent'
                              : result.overall_score >= 60
                                ? 'Good'
                                : 'Needs Work'}
                          </Badge>
                        </div>

                        <div className="mb-4 grid grid-cols-2 gap-4 md:grid-cols-6">
                          <div className="rounded-lg bg-muted p-3 text-center">
                            <div
                              className={`text-2xl font-bold ${getScoreColor(result.overall_score)}`}
                            >
                              {result.overall_score.toFixed(1)}%
                            </div>
                            <div className="text-xs text-muted-foreground">
                              Overall
                            </div>
                          </div>
                          <div className="rounded-lg bg-muted p-3 text-center">
                            <div className="text-lg font-semibold">
                              {result.skills_score.toFixed(1)}%
                            </div>
                            <div className="text-xs text-muted-foreground">
                              Skills
                            </div>
                          </div>
                          <div className="rounded-lg bg-muted p-3 text-center">
                            <div className="text-lg font-semibold">
                              {result.experience_score.toFixed(1)}%
                            </div>
                            <div className="text-xs text-muted-foreground">
                              Experience
                            </div>
                          </div>
                          <div className="rounded-lg bg-muted p-3 text-center">
                            <div className="text-lg font-semibold">
                              {result.education_score.toFixed(1)}%
                            </div>
                            <div className="text-xs text-muted-foreground">
                              Education
                            </div>
                          </div>
                          <div className="rounded-lg bg-muted p-3 text-center">
                            <div className="text-lg font-semibold">
                              {result.keywords_score.toFixed(1)}%
                            </div>
                            <div className="text-xs text-muted-foreground">
                              Keywords
                            </div>
                          </div>
                          <div className="rounded-lg bg-muted p-3 text-center">
                            <div className="text-lg font-semibold">
                              {result.format_score.toFixed(1)}%
                            </div>
                            <div className="text-xs text-muted-foreground">
                              Format
                            </div>
                          </div>
                        </div>

                        {/* Brief Feedback Preview */}
                        <div className="mb-4">
                          <p className="text-sm text-muted-foreground">
                            {result.detailed_feedback.length > 150
                              ? `${result.detailed_feedback.substring(0, 150)}...`
                              : result.detailed_feedback}
                          </p>
                        </div>

                        {/* Missing Keywords Preview */}
                        {parseMissingKeywords(result.missing_keywords).length >
                          0 && (
                          <div className="mb-4">
                            <div className="mb-2 text-xs font-medium text-muted-foreground">
                              Missing Keywords (
                              {
                                parseMissingKeywords(result.missing_keywords)
                                  .length
                              }
                              )
                            </div>
                            <div className="flex flex-wrap gap-1">
                              {parseMissingKeywords(result.missing_keywords)
                                .slice(0, 5)
                                .map((keyword, index) => (
                                  <Badge
                                    key={index}
                                    variant="secondary"
                                    className="text-xs"
                                  >
                                    {keyword}
                                  </Badge>
                                ))}
                              {parseMissingKeywords(result.missing_keywords)
                                .length > 5 && (
                                <Badge variant="outline" className="text-xs">
                                  +
                                  {parseMissingKeywords(result.missing_keywords)
                                    .length - 5}{' '}
                                  more
                                </Badge>
                              )}
                            </div>
                          </div>
                        )}

                        <div className="flex items-center justify-between text-xs text-muted-foreground">
                          <span>{formatDate(result.created_at)}</span>
                          <span>
                            Processed in {result.processing_time_ms}ms
                          </span>
                        </div>
                      </div>
                    </div>

                    {/* Action Buttons */}
                    <div className="flex items-center justify-between border-t pt-4">
                      <div className="flex gap-2">
                        <Button
                          onClick={() => handleViewFullAnalysis(result)}
                          className="px-6"
                        >
                          <FileText className="mr-2 h-4 w-4" />
                          View Full Analysis
                        </Button>
                        <Button
                          variant="outline"
                          onClick={() => handleExportResult(result)}
                          size="sm"
                        >
                          <Download className="mr-2 h-4 w-4" />
                          Export
                        </Button>
                      </div>
                      <Button
                        variant="ghost"
                        onClick={() => handleDeleteClick(result.id)}
                        size="sm"
                        className="text-muted-foreground hover:text-destructive"
                      >
                        <Trash2 className="h-4 w-4" />
                      </Button>
                    </div>
                  </CardContent>
                </Card>
              );
            })
          )}
        </CardContent>
      </Card>

      {/* Delete Confirmation Dialog */}
      <AlertDialog
        open={!!deleteConfirmId}
        onOpenChange={() => !isDeleting && setDeleteConfirmId(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete Analysis</AlertDialogTitle>
            <AlertDialogDescription>
              Are you sure you want to delete this analysis? This action cannot
              be undone.
              {deleteConfirmId && (
                <div className="mt-2 text-sm text-muted-foreground">
                  Analysis ID: {deleteConfirmId.slice(0, 8)}...
                </div>
              )}
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel
              onClick={handleDeleteCancel}
              disabled={isDeleting}
            >
              Cancel
            </AlertDialogCancel>
            <AlertDialogAction
              onClick={handleDeleteConfirm}
              disabled={isDeleting}
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
            >
              {isDeleting ? 'Deleting...' : 'Delete'}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}
