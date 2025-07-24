import { useState, useEffect, useCallback, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { VirtualizedTable, Column } from '@/components/VirtualizedTable';
import { useAppStore } from '@/store/useAppStore';
import {
  History,
  Download,
  Search,
  Filter,
  FileText,
  Trash2,
  Eye,
  RotateCcw,
  Loader2,
  AlertCircle,
  CheckCircle,
  X,
} from 'lucide-react';
import { toast } from '@/hooks/use-toast';
import type { Analysis, CommandResult } from '@/types';

// Extended analysis interface for history display
interface HistoryAnalysis extends Analysis, Record<string, unknown> {
  resume_filename?: string;
  job_description_preview?: string;
}

interface HistoryPageState {
  analyses: HistoryAnalysis[];
  loading: boolean;
  error: string | null;
  selectedAnalyses: HistoryAnalysis[];
  searchTerm: string;
  dateRange: {
    start: string;
    end: string;
  };
  scoreRange: {
    min: number;
    max: number;
  };
  modelFilter: string;
  showFilters: boolean;
}

type ExportFormat = 'pdf' | 'csv' | 'json';

export function HistoryPage() {
  const { setCurrentDetailedAnalysis, setActiveTab } = useAppStore();

  const [state, setState] = useState<HistoryPageState>({
    analyses: [],
    loading: true,
    error: null,
    selectedAnalyses: [],
    searchTerm: '',
    dateRange: {
      start: '',
      end: '',
    },
    scoreRange: {
      min: 0,
      max: 100,
    },
    modelFilter: '',
    showFilters: false,
  });

  const [exporting, setExporting] = useState(false);

  // Load analysis history
  const loadHistory = useCallback(async () => {
    try {
      setState(prev => ({ ...prev, loading: true, error: null }));

      const result = await invoke<CommandResult<Analysis[]>>(
        'get_analysis_history',
        {
          limit: null, // Get all analyses
        }
      );

      if (result.success && result.data) {
        const enrichedAnalyses: HistoryAnalysis[] = result.data.map(
          analysis => ({
            ...analysis,
            resume_filename: `Analysis_${analysis.id.slice(0, 8)}`,
            job_description_preview:
              analysis.detailed_feedback.slice(0, 100) + '...',
          })
        );

        setState(prev => ({
          ...prev,
          analyses: enrichedAnalyses,
          loading: false,
        }));
      } else {
        throw new Error(result.error ?? 'Failed to load analysis history');
      }
    } catch (error) {
      setState(prev => ({
        ...prev,
        loading: false,
        error: `Failed to load history: ${error}`,
      }));
      toast({
        title: 'Error loading history',
        description: `${error}`,
        variant: 'destructive',
      });
    }
  }, []);

  // Load history on component mount
  useEffect(() => {
    void loadHistory();
  }, [loadHistory]);

  // Filter analyses based on search and filter criteria
  const filteredAnalyses = useMemo(() => {
    return state.analyses.filter(analysis => {
      // Search filter
      if (state.searchTerm) {
        const searchLower = state.searchTerm.toLowerCase();
        const matchesSearch =
          (analysis.resume_filename?.toLowerCase().includes(searchLower) ??
            false) ||
          analysis.model_used.toLowerCase().includes(searchLower) ||
          analysis.detailed_feedback.toLowerCase().includes(searchLower);
        if (!matchesSearch) return false;
      }

      // Date range filter
      if (state.dateRange.start && state.dateRange.end) {
        const analysisDate = new Date(analysis.created_at);
        const startDate = new Date(state.dateRange.start);
        const endDate = new Date(state.dateRange.end);
        if (analysisDate < startDate || analysisDate > endDate) return false;
      }

      // Score range filter
      if (
        analysis.overall_score < state.scoreRange.min ||
        analysis.overall_score > state.scoreRange.max
      ) {
        return false;
      }

      // Model filter
      if (state.modelFilter && analysis.model_used !== state.modelFilter) {
        return false;
      }

      return true;
    });
  }, [
    state.analyses,
    state.searchTerm,
    state.dateRange,
    state.scoreRange,
    state.modelFilter,
  ]);

  // Get unique models for filter dropdown
  const availableModels = useMemo(() => {
    const models = new Set(state.analyses.map(a => a.model_used));
    return Array.from(models);
  }, [state.analyses]);

  // Handle analysis selection
  const handleSelectionChange = useCallback(
    (selectedItems: HistoryAnalysis[]) => {
      setState(prev => ({ ...prev, selectedAnalyses: selectedItems }));
    },
    []
  );

  // View analysis details
  const handleViewDetails = useCallback(
    (analysis: HistoryAnalysis) => {
      // Convert to detailed analysis format
      const detailedAnalysis = {
        result: {
          overall_score: analysis.overall_score,
          category_scores: {
            skills: analysis.skills_score,
            experience: analysis.experience_score,
            education: analysis.education_score,
            keywords: analysis.keywords_score,
            format: analysis.format_score,
          },
          detailed_feedback: analysis.detailed_feedback,
          missing_keywords: analysis.missing_keywords
            .split(',')
            .filter(k => k.trim()),
          recommendations: analysis.recommendations
            .split('\n')
            .filter(r => r.trim()),
          processing_time_ms: analysis.processing_time_ms,
        },
        resumeFilename:
          analysis.resume_filename ?? `Analysis_${analysis.id.slice(0, 8)}`,
        jobDescription: 'Job description not stored in legacy analyses',
        modelUsed: analysis.model_used,
        timestamp: analysis.created_at,
      };

      setCurrentDetailedAnalysis(detailedAnalysis);
      setActiveTab('analysis-result');
    },
    [setCurrentDetailedAnalysis, setActiveTab]
  );

  // Delete analysis
  const handleDelete = useCallback(async (analysisId: string) => {
    try {
      const result = await invoke<CommandResult<boolean>>('delete_analysis', {
        analysisId,
      });

      if (result.success) {
        setState(prev => ({
          ...prev,
          analyses: prev.analyses.filter(a => a.id !== analysisId),
          selectedAnalyses: prev.selectedAnalyses.filter(
            a => a.id !== analysisId
          ),
        }));

        toast({
          title: 'Analysis deleted',
          description: 'The analysis has been removed from your history.',
        });
      } else {
        throw new Error(result.error ?? 'Failed to delete analysis');
      }
    } catch (error) {
      toast({
        title: 'Error deleting analysis',
        description: `${error}`,
        variant: 'destructive',
      });
    }
  }, []);

  // Export analyses
  const handleExport = useCallback(
    async (format: ExportFormat, analyses: HistoryAnalysis[]) => {
      if (analyses.length === 0) {
        toast({
          title: 'No analyses selected',
          description: 'Please select at least one analysis to export.',
          variant: 'destructive',
        });
        return;
      }

      try {
        setExporting(true);

        const analysisIds = analyses.map(a => a.id);
        const result = await invoke<CommandResult<string>>('export_results', {
          analysisIds,
          format: format.toUpperCase(),
        });

        if (result.success && result.data) {
          toast({
            title: 'Export successful',
            description: `${analyses.length} analyses exported to ${format.toUpperCase()} format.`,
          });
        } else {
          throw new Error(result.error ?? 'Export failed');
        }
      } catch (error) {
        toast({
          title: 'Export failed',
          description: `${error}`,
          variant: 'destructive',
        });
      } finally {
        setExporting(false);
      }
    },
    []
  );

  // Clear filters
  const clearFilters = useCallback(() => {
    setState(prev => ({
      ...prev,
      searchTerm: '',
      dateRange: { start: '', end: '' },
      scoreRange: { min: 0, max: 100 },
      modelFilter: '',
    }));
  }, []);

  // Define table columns
  const columns: Column<HistoryAnalysis>[] = useMemo(
    () => [
      {
        key: 'resume_filename',
        title: 'Resume',
        width: 200,
        render: (value, item) => (
          <div className="flex items-center gap-2">
            <FileText className="h-4 w-4 text-blue-500" />
            <span className="font-medium">
              {String(value) || `Analysis_${item.id.slice(0, 8)}`}
            </span>
          </div>
        ),
      },
      {
        key: 'overall_score',
        title: 'Score',
        width: 100,
        render: value => {
          const score = Number(value);
          return (
            <div className="flex items-center gap-2">
              <div
                className={`rounded px-2 py-1 text-sm font-medium ${
                  score >= 80
                    ? 'bg-green-100 text-green-800'
                    : score >= 60
                      ? 'bg-yellow-100 text-yellow-800'
                      : 'bg-red-100 text-red-800'
                }`}
              >
                {score.toFixed(1)}%
              </div>
            </div>
          );
        },
        sortFn: (a, b) => a.overall_score - b.overall_score,
      },
      {
        key: 'model_used',
        title: 'Model',
        width: 150,
        render: value => (
          <span className="rounded bg-gray-100 px-2 py-1 text-xs font-medium text-gray-700">
            {String(value)}
          </span>
        ),
      },
      {
        key: 'processing_time_ms',
        title: 'Processing Time',
        width: 120,
        render: value => (
          <span className="text-sm text-gray-600">
            {(Number(value) / 1000).toFixed(1)}s
          </span>
        ),
      },
      {
        key: 'created_at',
        title: 'Date',
        width: 150,
        render: value => (
          <span className="text-sm text-gray-600">
            {new Date(value as string).toLocaleDateString()}
          </span>
        ),
        sortFn: (a, b) =>
          new Date(a.created_at).getTime() - new Date(b.created_at).getTime(),
      },
      {
        key: 'id',
        title: 'Actions',
        width: -150,
        sortable: false,
        filterable: false,
        render: (_, item) => (
          <div className="flex items-center gap-1">
            <Button
              variant="ghost"
              size="sm"
              onClick={() => handleViewDetails(item)}
              title="View details"
            >
              <Eye className="h-4 w-4" />
            </Button>
            <Button
              variant="ghost"
              size="sm"
              onClick={() => void handleDelete(item.id)}
              title="Delete analysis"
              className="text-red-600 hover:text-red-700"
            >
              <Trash2 className="h-4 w-4" />
            </Button>
          </div>
        ),
      },
    ],
    [handleViewDetails, handleDelete]
  );

  if (state.loading) {
    return (
      <div className="flex h-96 items-center justify-center">
        <div className="text-center">
          <Loader2 className="mx-auto mb-4 h-8 w-8 animate-spin text-blue-600" />
          <p className="text-gray-600">Loading analysis history...</p>
        </div>
      </div>
    );
  }

  if (state.error) {
    return (
      <div className="flex h-96 items-center justify-center">
        <div className="text-center">
          <AlertCircle className="mx-auto mb-4 h-8 w-8 text-red-500" />
          <p className="text-gray-600">{state.error}</p>
          <Button onClick={loadHistory} className="mt-4">
            <RotateCcw className="mr-2 h-4 w-4" />
            Retry
          </Button>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="space-y-3">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="flex items-center gap-2 text-3xl font-bold tracking-tight">
              <History className="h-7 w-7" />
              Analysis History
            </h1>
            <p className="text-muted-foreground">
              View and manage your previous resume analyses
            </p>
          </div>
          <div className="flex items-center gap-2">
            <Button
              variant="outline"
              onClick={() =>
                setState(prev => ({ ...prev, showFilters: !prev.showFilters }))
              }
            >
              <Filter className="mr-2 h-4 w-4" />
              Filters
            </Button>
            <Button onClick={loadHistory} variant="outline">
              <RotateCcw className="mr-2 h-4 w-4" />
              Refresh
            </Button>
          </div>
        </div>
      </div>

      {/* Stats Cards */}
      <div className="grid gap-4 md:grid-cols-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              Total Analyses
            </CardTitle>
            <FileText className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{state.analyses.length}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Average Score</CardTitle>
            <CheckCircle className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {state.analyses.length > 0
                ? (
                    state.analyses.reduce(
                      (sum, a) => sum + a.overall_score,
                      0
                    ) / state.analyses.length
                  ).toFixed(1)
                : '0.0'}
              %
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">High Scores</CardTitle>
            <CheckCircle className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {state.analyses.filter(a => a.overall_score >= 80).length}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Selected</CardTitle>
            <CheckCircle className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {state.selectedAnalyses.length}
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Search and Filters */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <CardTitle>Search & Filter</CardTitle>
              <CardDescription>
                Find specific analyses in your history
              </CardDescription>
            </div>
            {(state.searchTerm ||
              state.dateRange.start ||
              state.dateRange.end ||
              state.modelFilter ||
              state.scoreRange.min > 0 ||
              state.scoreRange.max < 100) && (
              <Button variant="outline" onClick={clearFilters}>
                <X className="mr-2 h-4 w-4" />
                Clear Filters
              </Button>
            )}
          </div>
        </CardHeader>
        <CardContent className="space-y-4">
          {/* Search Input */}
          <div className="flex items-center gap-2">
            <Search className="h-4 w-4 text-gray-400" />
            <Input
              placeholder="Search analyses..."
              value={state.searchTerm}
              onChange={e =>
                setState(prev => ({ ...prev, searchTerm: e.target.value }))
              }
              className="flex-1"
            />
          </div>

          {/* Advanced Filters */}
          {state.showFilters && (
            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
              <div>
                <label className="text-sm font-medium">Date From</label>
                <Input
                  type="date"
                  value={state.dateRange.start}
                  onChange={e =>
                    setState(prev => ({
                      ...prev,
                      dateRange: { ...prev.dateRange, start: e.target.value },
                    }))
                  }
                />
              </div>
              <div>
                <label className="text-sm font-medium">Date To</label>
                <Input
                  type="date"
                  value={state.dateRange.end}
                  onChange={e =>
                    setState(prev => ({
                      ...prev,
                      dateRange: { ...prev.dateRange, end: e.target.value },
                    }))
                  }
                />
              </div>
              <div>
                <label className="text-sm font-medium">Model</label>
                <select
                  value={state.modelFilter}
                  onChange={e =>
                    setState(prev => ({ ...prev, modelFilter: e.target.value }))
                  }
                  className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                >
                  <option value="">All Models</option>
                  {availableModels.map(model => (
                    <option key={model} value={model}>
                      {model}
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="text-sm font-medium">
                  Score Range ({state.scoreRange.min}% - {state.scoreRange.max}
                  %)
                </label>
                <div className="flex gap-2">
                  <Input
                    type="number"
                    min="0"
                    max="100"
                    value={state.scoreRange.min}
                    onChange={e =>
                      setState(prev => ({
                        ...prev,
                        scoreRange: {
                          ...prev.scoreRange,
                          min: Number(e.target.value),
                        },
                      }))
                    }
                    placeholder="Min"
                  />
                  <Input
                    type="number"
                    min="0"
                    max="100"
                    value={state.scoreRange.max}
                    onChange={e =>
                      setState(prev => ({
                        ...prev,
                        scoreRange: {
                          ...prev.scoreRange,
                          max: Number(e.target.value),
                        },
                      }))
                    }
                    placeholder="Max"
                  />
                </div>
              </div>
            </div>
          )}
        </CardContent>
      </Card>

      {/* Export Controls */}
      {state.selectedAnalyses.length > 0 && (
        <Card>
          <CardContent className="pt-6">
            <div className="flex items-center justify-between">
              <p className="text-sm text-gray-600">
                {state.selectedAnalyses.length} analyses selected
              </p>
              <div className="flex items-center gap-2">
                <Button
                  variant="outline"
                  onClick={() =>
                    void handleExport('csv', state.selectedAnalyses)
                  }
                  disabled={exporting}
                >
                  <Download className="mr-2 h-4 w-4" />
                  Export CSV
                </Button>
                <Button
                  variant="outline"
                  onClick={() =>
                    void handleExport('json', state.selectedAnalyses)
                  }
                  disabled={exporting}
                >
                  <Download className="mr-2 h-4 w-4" />
                  Export JSON
                </Button>
                <Button
                  variant="outline"
                  onClick={() =>
                    void handleExport('pdf', state.selectedAnalyses)
                  }
                  disabled={exporting}
                >
                  {exporting ? (
                    <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                  ) : (
                    <Download className="mr-2 h-4 w-4" />
                  )}
                  Export PDF
                </Button>
              </div>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Analysis Table */}
      <Card>
        <CardHeader>
          <CardTitle>
            Analysis History ({filteredAnalyses.length}{' '}
            {filteredAnalyses.length !== state.analyses.length &&
              `of ${state.analyses.length}`}
            )
          </CardTitle>
        </CardHeader>
        <CardContent>
          <VirtualizedTable
            data={filteredAnalyses}
            columns={columns}
            height={600}
            selectable={true}
            selectedItems={state.selectedAnalyses}
            onSelectionChange={handleSelectionChange}
            getItemKey={item => String(item.id)}
            emptyMessage="No analyses found matching your criteria"
            filterable={false} // We handle filtering above
            sortable={true}
          />
        </CardContent>
      </Card>
    </div>
  );
}
