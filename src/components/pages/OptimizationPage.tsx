import React, { useState, useCallback, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { save } from '@tauri-apps/api/dialog';
import { writeTextFile } from '@tauri-apps/api/fs';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';
import { Textarea } from '@/components/ui/textarea';
import { useAppStore } from '@/store/useAppStore';
import { 
  Edit3, 
  Zap, 
  Download, 
  RefreshCw, 
  BarChart3, 
  TrendingUp, 
  AlertCircle,
  FileText,
  Split,
  Target,
  Award,
  Lightbulb,
  Brain
} from 'lucide-react';
import { toast } from '@/hooks/use-toast';

interface OptimizationChange {
  section: string;
  change_type: string;
  original: string;
  optimized: string;
  impact_score: number;
}

interface OptimizationResult {
  optimized_content: string;
  changes_made: OptimizationChange[];
  before_score: number;
  after_score: number;
  improvement_percentage: number;
}

interface AnalysisResult {
  overall_score: number;
  category_scores: {
    skills: number;
    experience: number;
    education: number;
    keywords: number;
    format: number;
  };
  detailed_feedback: string;
  missing_keywords: string[];
  recommendations: string[];
  processing_time_ms: number;
}

interface CommandResult<T> {
  success: boolean;
  data?: T;
  error?: string;
}

interface ComprehensiveOptimization {
  optimized_content: string;
  content_improvements: ContentImprovement[];
  structure_improvements: StructureImprovement[];
  keyword_improvements: KeywordImprovement[];
  achievement_improvements: AchievementImprovement[];
  ats_improvements: ATSImprovement[];
  overall_improvement_score: number;
  before_score: number;
  after_score: number;
}

interface ContentImprovement {
  section: string;
  improvement_type: string;
  original: string;
  improved: string;
  reasoning: string;
  impact_score: number;
}

interface StructureImprovement {
  improvement_type: string;
  description: string;
  priority: string;
  impact_score: number;
}

interface KeywordImprovement {
  keyword: string;
  context: string;
  suggested_placement: string;
  importance: number;
}

interface AchievementImprovement {
  original_bullet: string;
  improved_bullet: string;
  improvement_type: string;
  xyz_structure_applied: boolean;
  impact_score: number;
}

interface ATSImprovement {
  issue: string;
  recommendation: string;
  priority: string;
  ats_systems_affected: string[];
}

interface LiveSuggestion {
  suggestion_type: string;
  title: string;
  description: string;
  suggested_text: string;
  position: number;
  priority: string;
  confidence: number;
}

export function OptimizationPage() {
  const {
    models,
    selectedModel,
    setSelectedModel,
    isOllamaConnected
  } = useAppStore();

  const [originalContent, setOriginalContent] = useState('');
  const [optimizedContent, setOptimizedContent] = useState('');
  const [jobDescription, setJobDescription] = useState('');
  const [optimizationLevel, setOptimizationLevel] = useState<'Conservative' | 'Balanced' | 'Aggressive'>('Balanced');
  const [optimizationResult, setOptimizationResult] = useState<OptimizationResult | null>(null);
  const [currentScore, setCurrentScore] = useState<AnalysisResult | null>(null);
  const [isOptimizing, setIsOptimizing] = useState(false);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [isRealTimeMode, setIsRealTimeMode] = useState(false);
  const [comprehensiveOptimization, setComprehensiveOptimization] = useState<ComprehensiveOptimization | null>(null);
  const [liveSuggestions, setLiveSuggestions] = useState<LiveSuggestion[]>([]);
  const [showComprehensiveAnalysis, setShowComprehensiveAnalysis] = useState(false);

  // Real-time analysis with debouncing
  const [debounceTimer, setDebounceTimer] = useState<NodeJS.Timeout | null>(null);
  const [suggestionTimer, setSuggestionTimer] = useState<NodeJS.Timeout | null>(null);

  const performAnalysis = useCallback(async (content: string) => {
    if (!content.trim() || !jobDescription.trim() || !selectedModel) return;

    try {
      setIsAnalyzing(true);
      const result = await invoke<CommandResult<AnalysisResult>>('analyze_resume', {
        request: {
          resume_content: content,
          job_description: jobDescription,
          model_name: selectedModel
        }
      });

      if (result.success && result.data) {
        setCurrentScore(result.data);
      }
    } catch (error) {
      console.error('Analysis error:', error);
    } finally {
      setIsAnalyzing(false);
    }
  }, [jobDescription, selectedModel]);

  // Debounced real-time analysis
  useEffect(() => {
    if (!isRealTimeMode || !optimizedContent.trim()) return;

    if (debounceTimer) {
      clearTimeout(debounceTimer);
    }

    const timer = setTimeout(() => {
      performAnalysis(optimizedContent);
    }, 1000);

    setDebounceTimer(timer);

    return () => {
      if (timer) clearTimeout(timer);
    };
  }, [optimizedContent, isRealTimeMode, performAnalysis]);

  // Debounced real-time suggestions
  useEffect(() => {
    if (!isRealTimeMode || !optimizedContent.trim() || !jobDescription.trim()) return;

    if (suggestionTimer) {
      clearTimeout(suggestionTimer);
    }

    const timer = setTimeout(() => {
      fetchLiveSuggestions(optimizedContent, jobDescription);
    }, 2000);

    setSuggestionTimer(timer);

    return () => {
      if (timer) clearTimeout(timer);
    };
  }, [optimizedContent, jobDescription, isRealTimeMode]);

  // Fetch live suggestions
  const fetchLiveSuggestions = async (content: string, jobDesc: string) => {
    try {
      const result = await invoke<CommandResult<LiveSuggestion[]>>('get_realtime_suggestions', {
        resumeContent: content,
        jobDescription: jobDesc
      });

      if (result.success && result.data) {
        setLiveSuggestions(result.data);
      }
    } catch (error) {
      console.error('Live suggestions error:', error);
    }
  };

  // Comprehensive optimization
  const handleComprehensiveOptimize = async () => {
    if (!originalContent.trim() || !jobDescription.trim() || !selectedModel) {
      toast({
        title: "Missing information",
        description: "Please provide resume content, job description, and select a model",
        variant: "destructive"
      });
      return;
    }

    try {
      setIsOptimizing(true);

      const result = await invoke<CommandResult<ComprehensiveOptimization>>('generate_comprehensive_optimization', {
        resumeContent: originalContent,
        jobDescription: jobDescription,
        optimizationLevel: optimizationLevel
      });

      if (result.success && result.data) {
        setComprehensiveOptimization(result.data);
        setOptimizedContent(result.data.optimized_content);
        setShowComprehensiveAnalysis(true);
        toast({
          title: "Comprehensive optimization completed",
          description: `${result.data.overall_improvement_score.toFixed(1)}% improvement achieved`
        });
      } else {
        throw new Error(result.error || 'Comprehensive optimization failed');
      }
    } catch (error) {
      console.error('Comprehensive optimization error:', error);
      toast({
        title: "Optimization failed",
        description: `Error: ${error}`,
        variant: "destructive"
      });
    } finally {
      setIsOptimizing(false);
    }
  };

  const handleOptimize = async () => {
    if (!originalContent.trim() || !jobDescription.trim() || !selectedModel) {
      toast({
        title: "Missing information",
        description: "Please provide resume content, job description, and select a model",
        variant: "destructive"
      });
      return;
    }

    try {
      setIsOptimizing(true);
      setOptimizationResult(null);

      const result = await invoke<CommandResult<OptimizationResult>>('optimize_resume', {
        request: {
          resume_content: originalContent,
          job_description: jobDescription,
          model_name: selectedModel,
          optimization_level: optimizationLevel
        }
      });

      if (result.success && result.data) {
        setOptimizationResult(result.data);
        setOptimizedContent(result.data.optimized_content);
        toast({
          title: "Optimization completed",
          description: `${result.data.improvement_percentage.toFixed(1)}% improvement achieved`
        });
      } else {
        throw new Error(result.error || 'Optimization failed');
      }
    } catch (error) {
      console.error('Optimization error:', error);
      toast({
        title: "Optimization failed",
        description: `Error: ${error}`,
        variant: "destructive"
      });
    } finally {
      setIsOptimizing(false);
    }
  };

  const handleExport = async () => {
    if (!optimizedContent.trim()) {
      toast({
        title: "Nothing to export",
        description: "Please optimize your resume first",
        variant: "destructive"
      });
      return;
    }

    try {
      const filePath = await save({
        filters: [{
          name: 'Text',
          extensions: ['txt']
        }]
      });

      if (filePath) {
        await writeTextFile(filePath, optimizedContent);
        toast({
          title: "Export successful",
          description: `Resume saved to ${filePath}`
        });
      }
    } catch (error) {
      console.error('Export error:', error);
      toast({
        title: "Export failed",
        description: `Error: ${error}`,
        variant: "destructive"
      });
    }
  };

  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <h1 className="text-3xl font-bold tracking-tight">Resume Optimization</h1>
        <p className="text-muted-foreground">
          Optimize your resume with AI-powered suggestions for better ATS compatibility.
        </p>
      </div>

      {!isOllamaConnected && (
        <Card className="border-yellow-200 bg-yellow-50 dark:border-yellow-800 dark:bg-yellow-900/20">
          <CardContent className="flex items-center gap-2 pt-6">
            <AlertCircle className="h-4 w-4 text-yellow-600" />
            <p className="text-sm text-yellow-700 dark:text-yellow-300">
              Ollama is not connected. Please check your Ollama installation and try again.
            </p>
          </CardContent>
        </Card>
      )}

      {/* Configuration Section */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Zap className="h-5 w-5" />
            Optimization Settings
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid gap-4 md:grid-cols-3">
            <div className="space-y-2">
              <label className="text-sm font-medium">AI Model</label>
              <select
                value={selectedModel || ''}
                onChange={(e) => setSelectedModel(e.target.value)}
                className="w-full px-3 py-2 border border-input bg-background rounded-md text-sm"
                disabled={models.length === 0}
              >
                <option value="">Select a model...</option>
                {models.map((model) => (
                  <option key={model.name} value={model.name}>
                    {model.name}
                  </option>
                ))}
              </select>
            </div>

            <div className="space-y-2">
              <label className="text-sm font-medium">Optimization Level</label>
              <select
                value={optimizationLevel}
                onChange={(e) => setOptimizationLevel(e.target.value as any)}
                className="w-full px-3 py-2 border border-input bg-background rounded-md text-sm"
              >
                <option value="Conservative">Conservative</option>
                <option value="Balanced">Balanced</option>
                <option value="Aggressive">Aggressive</option>
              </select>
            </div>

            <div className="space-y-2">
              <label className="text-sm font-medium">Real-time Analysis</label>
              <div className="flex items-center space-x-2">
                <input
                  type="checkbox"
                  id="realtime"
                  checked={isRealTimeMode}
                  onChange={(e) => setIsRealTimeMode(e.target.checked)}
                  className="rounded border-input"
                />
                <label htmlFor="realtime" className="text-sm">Enable live scoring</label>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Job Description */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <FileText className="h-5 w-5" />
            Job Description
          </CardTitle>
          <CardDescription>
            Paste the job description you want to optimize against
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Textarea
            placeholder="Paste the job description here..."
            value={jobDescription}
            onChange={(e) => setJobDescription(e.target.value)}
            className="min-h-[120px] resize-none"
          />
        </CardContent>
      </Card>

      {/* Editor Section */}
      <div className="grid gap-6 lg:grid-cols-2">
        {/* Original Resume */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Edit3 className="h-5 w-5" />
              Original Resume
            </CardTitle>
            <CardDescription>
              Paste your current resume content
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <Textarea
              placeholder="Paste your resume content here..."
              value={originalContent}
              onChange={(e) => setOriginalContent(e.target.value)}
              className="min-h-[400px] font-mono text-sm"
              spellCheck={false}
            />
            <div className="flex justify-between items-center">
              <p className="text-xs text-muted-foreground">
                {originalContent.length} characters
              </p>
              <div className="flex gap-2">
                <Button
                  onClick={handleOptimize}
                  disabled={!originalContent.trim() || !jobDescription.trim() || !selectedModel || isOptimizing}
                  size="sm"
                >
                  {isOptimizing ? (
                    <>
                      <RefreshCw className="h-4 w-4 mr-2 animate-spin" />
                      Optimizing...
                    </>
                  ) : (
                    <>
                      <Zap className="h-4 w-4 mr-2" />
                      Quick Optimize
                    </>
                  )}
                </Button>
                <Button
                  onClick={handleComprehensiveOptimize}
                  disabled={!originalContent.trim() || !jobDescription.trim() || !selectedModel || isOptimizing}
                  variant="outline"
                  size="sm"
                >
                  {isOptimizing ? (
                    <>
                      <RefreshCw className="h-4 w-4 mr-2 animate-spin" />
                      Processing...
                    </>
                  ) : (
                    <>
                      <Brain className="h-4 w-4 mr-2" />
                      Comprehensive
                    </>
                  )}
                </Button>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Optimized Resume */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center justify-between">
              <span className="flex items-center gap-2">
                <Split className="h-5 w-5" />
                Optimized Resume
              </span>
              {currentScore && (
                <div className="flex items-center gap-2">
                  {isRealTimeMode && isAnalyzing && (
                    <RefreshCw className="h-4 w-4 animate-spin text-muted-foreground" />
                  )}
                  <div className="text-sm font-medium">
                    Score: {currentScore.overall_score.toFixed(1)}%
                  </div>
                </div>
              )}
            </CardTitle>
            <CardDescription>
              AI-optimized version of your resume
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <Textarea
              placeholder="Optimized content will appear here..."
              value={optimizedContent}
              onChange={(e) => setOptimizedContent(e.target.value)}
              className="min-h-[400px] font-mono text-sm"
              spellCheck={false}
            />
            <div className="flex justify-between items-center">
              <p className="text-xs text-muted-foreground">
                {optimizedContent.length} characters
              </p>
              <div className="flex gap-2">
                <Button
                  onClick={() => performAnalysis(optimizedContent)}
                  disabled={!optimizedContent.trim() || !jobDescription.trim() || !selectedModel || isAnalyzing}
                  variant="outline"
                  size="sm"
                >
                  {isAnalyzing ? (
                    <>
                      <RefreshCw className="h-4 w-4 mr-2 animate-spin" />
                      Analyzing...
                    </>
                  ) : (
                    <>
                      <BarChart3 className="h-4 w-4 mr-2" />
                      Analyze
                    </>
                  )}
                </Button>
                <Button
                  onClick={handleExport}
                  disabled={!optimizedContent.trim()}
                  size="sm"
                >
                  <Download className="h-4 w-4 mr-2" />
                  Export
                </Button>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Live Suggestions */}
      {isRealTimeMode && liveSuggestions.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Lightbulb className="h-5 w-5" />
              Live Suggestions ({liveSuggestions.length})
            </CardTitle>
            <CardDescription>
              Real-time optimization suggestions as you type
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-3 max-h-64 overflow-y-auto">
              {liveSuggestions.slice(0, 5).map((suggestion, index) => (
                <div key={index} className="border rounded-lg p-3 space-y-2">
                  <div className="flex items-center justify-between">
                    <span className="px-2 py-1 bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 text-xs rounded-md">
                      {suggestion.suggestion_type}
                    </span>
                    <div className="flex items-center gap-2">
                      <span className={`px-2 py-1 text-xs rounded-md ${
                        suggestion.priority === 'High' ? 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-300' :
                        suggestion.priority === 'Medium' ? 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-300' :
                        'bg-gray-100 text-gray-700 dark:bg-gray-900/30 dark:text-gray-300'
                      }`}>
                        {suggestion.priority}
                      </span>
                      <span className="text-xs text-muted-foreground">
                        {(suggestion.confidence * 100).toFixed(0)}%
                      </span>
                    </div>
                  </div>
                  <h4 className="font-medium text-sm">{suggestion.title}</h4>
                  <p className="text-xs text-muted-foreground">{suggestion.description}</p>
                  {suggestion.suggested_text && (
                    <div className="bg-green-50 dark:bg-green-900/20 p-2 rounded border-l-2 border-green-200 dark:border-green-800">
                      <div className="text-xs font-medium text-green-600 dark:text-green-400 mb-1">Suggested:</div>
                      <div className="text-sm">{suggestion.suggested_text}</div>
                    </div>
                  )}
                </div>
              ))}
              {liveSuggestions.length > 5 && (
                <p className="text-xs text-muted-foreground text-center">
                  And {liveSuggestions.length - 5} more suggestions...
                </p>
              )}
            </div>
          </CardContent>
        </Card>
      )}

      {/* Comprehensive Optimization Results */}
      {comprehensiveOptimization && showComprehensiveAnalysis && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center justify-between">
              <span className="flex items-center gap-2">
                <Brain className="h-5 w-5" />
                Comprehensive Optimization
              </span>
              <div className="text-2xl font-bold text-primary">
                +{comprehensiveOptimization.overall_improvement_score.toFixed(1)}%
              </div>
            </CardTitle>
            <CardDescription>
              Detailed analysis with advanced optimization techniques
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* Improvement Summary */}
            <div className="grid gap-4 md:grid-cols-3">
              <div className="text-center p-4 border rounded-lg">
                <div className="text-2xl font-bold text-red-600">{comprehensiveOptimization.before_score.toFixed(1)}%</div>
                <div className="text-sm text-muted-foreground">Before</div>
              </div>
              <div className="text-center p-4 border rounded-lg">
                <div className="text-2xl font-bold text-green-600">{comprehensiveOptimization.after_score.toFixed(1)}%</div>
                <div className="text-sm text-muted-foreground">After</div>
              </div>
              <div className="text-center p-4 border rounded-lg bg-green-50 dark:bg-green-900/20">
                <div className="text-2xl font-bold text-green-600">+{comprehensiveOptimization.overall_improvement_score.toFixed(1)}%</div>
                <div className="text-sm text-muted-foreground">Improvement</div>
              </div>
            </div>

            {/* Content Improvements */}
            {comprehensiveOptimization.content_improvements.length > 0 && (
              <div className="space-y-3">
                <h3 className="font-semibold flex items-center gap-2">
                  <Target className="h-4 w-4" />
                  Content Improvements ({comprehensiveOptimization.content_improvements.length})
                </h3>
                <div className="space-y-3 max-h-64 overflow-y-auto">
                  {comprehensiveOptimization.content_improvements.map((improvement, index) => (
                    <div key={index} className="border rounded-lg p-4 space-y-2">
                      <div className="flex items-center justify-between">
                        <span className="px-2 py-1 bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 text-xs rounded-md">
                          {improvement.section}
                        </span>
                        <div className="flex items-center gap-2">
                          <span className="px-2 py-1 bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300 text-xs rounded-md">
                            {improvement.improvement_type}
                          </span>
                          <span className="text-sm font-medium">
                            +{improvement.impact_score.toFixed(1)}%
                          </span>
                        </div>
                      </div>
                      <div className="grid gap-2 md:grid-cols-2">
                        <div>
                          <div className="text-xs font-medium text-red-600 dark:text-red-400 mb-1">Original:</div>
                          <div className="text-sm bg-red-50 dark:bg-red-900/20 p-2 rounded border-l-2 border-red-200 dark:border-red-800">
                            {improvement.original}
                          </div>
                        </div>
                        <div>
                          <div className="text-xs font-medium text-green-600 dark:text-green-400 mb-1">Improved:</div>
                          <div className="text-sm bg-green-50 dark:bg-green-900/20 p-2 rounded border-l-2 border-green-200 dark:border-green-800">
                            {improvement.improved}
                          </div>
                        </div>
                      </div>
                      <p className="text-xs text-muted-foreground">
                        {improvement.reasoning}
                      </p>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {/* Achievement Improvements */}
            {comprehensiveOptimization.achievement_improvements.length > 0 && (
              <div className="space-y-3">
                <h3 className="font-semibold flex items-center gap-2">
                  <Award className="h-4 w-4" />
                  Achievement Improvements ({comprehensiveOptimization.achievement_improvements.length})
                </h3>
                <div className="space-y-3 max-h-64 overflow-y-auto">
                  {comprehensiveOptimization.achievement_improvements.map((improvement, index) => (
                    <div key={index} className="border rounded-lg p-4 space-y-2">
                      <div className="flex items-center justify-between">
                        <span className="px-2 py-1 bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300 text-xs rounded-md">
                          {improvement.improvement_type}
                        </span>
                        <div className="flex items-center gap-2">
                          {improvement.xyz_structure_applied && (
                            <span className="px-2 py-1 bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 text-xs rounded-md">
                              X-Y-Z Applied
                            </span>
                          )}
                          <span className="text-sm font-medium">
                            +{improvement.impact_score.toFixed(1)}%
                          </span>
                        </div>
                      </div>
                      <div className="grid gap-2">
                        <div>
                          <div className="text-xs font-medium text-red-600 dark:text-red-400 mb-1">Original:</div>
                          <div className="text-sm bg-red-50 dark:bg-red-900/20 p-2 rounded border-l-2 border-red-200 dark:border-red-800">
                            {improvement.original_bullet}
                          </div>
                        </div>
                        <div>
                          <div className="text-xs font-medium text-green-600 dark:text-green-400 mb-1">Improved:</div>
                          <div className="text-sm bg-green-50 dark:bg-green-900/20 p-2 rounded border-l-2 border-green-200 dark:border-green-800">
                            {improvement.improved_bullet}
                          </div>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {/* Hide Comprehensive Analysis */}
            <div className="flex justify-center">
              <Button
                variant="outline"
                size="sm"
                onClick={() => setShowComprehensiveAnalysis(false)}
              >
                Hide Comprehensive Analysis
              </Button>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Optimization Results */}
      {optimizationResult && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center justify-between">
              <span className="flex items-center gap-2">
                <TrendingUp className="h-5 w-5" />
                Optimization Results
              </span>
              <div className="flex items-center gap-4">
                <div className="text-right">
                  <div className="text-sm text-muted-foreground">Before</div>
                  <div className="text-lg font-bold">{optimizationResult.before_score.toFixed(1)}%</div>
                </div>
                <div className="text-2xl text-muted-foreground">→</div>
                <div className="text-right">
                  <div className="text-sm text-muted-foreground">After</div>
                  <div className="text-lg font-bold text-green-600">{optimizationResult.after_score.toFixed(1)}%</div>
                </div>
                <div className="text-right">
                  <div className="text-sm text-muted-foreground">Improvement</div>
                  <div className="text-lg font-bold text-green-600">
                    +{optimizationResult.improvement_percentage.toFixed(1)}%
                  </div>
                </div>
              </div>
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-6">
            <div className="space-y-4">
              <h3 className="font-semibold">Changes Made</h3>
              <div className="space-y-3">
                {optimizationResult.changes_made.map((change, index) => (
                  <div key={index} className="border rounded-lg p-4 space-y-2">
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-2">
                        <span className="px-2 py-1 bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 text-xs rounded-md">
                          {change.section}
                        </span>
                        <span className="px-2 py-1 bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300 text-xs rounded-md">
                          {change.change_type}
                        </span>
                      </div>
                      <div className="text-sm font-medium">
                        Impact: {change.impact_score.toFixed(1)}%
                      </div>
                    </div>
                    <div className="grid gap-2 md:grid-cols-2">
                      <div>
                        <div className="text-xs font-medium text-red-600 dark:text-red-400 mb-1">Original:</div>
                        <div className="text-sm bg-red-50 dark:bg-red-900/20 p-2 rounded border-l-2 border-red-200 dark:border-red-800">
                          {change.original}
                        </div>
                      </div>
                      <div>
                        <div className="text-xs font-medium text-green-600 dark:text-green-400 mb-1">Optimized:</div>
                        <div className="text-sm bg-green-50 dark:bg-green-900/20 p-2 rounded border-l-2 border-green-200 dark:border-green-800">
                          {change.optimized}
                        </div>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Current Score Display */}
      {currentScore && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <BarChart3 className="h-5 w-5" />
              Current Score Analysis
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* Category Scores */}
            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
              {Object.entries(currentScore.category_scores).map(([category, score]) => (
                <div key={category} className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span className="capitalize font-medium">{category}</span>
                    <span>{score.toFixed(1)}%</span>
                  </div>
                  <Progress value={score} />
                </div>
              ))}
            </div>

            {/* Missing Keywords */}
            {currentScore.missing_keywords.length > 0 && (
              <div className="space-y-3">
                <h3 className="font-semibold">Missing Keywords</h3>
                <div className="flex flex-wrap gap-2">
                  {currentScore.missing_keywords.slice(0, 10).map((keyword, index) => (
                    <span
                      key={index}
                      className="px-2 py-1 bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300 text-xs rounded-md"
                    >
                      {keyword}
                    </span>
                  ))}
                  {currentScore.missing_keywords.length > 10 && (
                    <span className="px-2 py-1 bg-muted text-muted-foreground text-xs rounded-md">
                      +{currentScore.missing_keywords.length - 10} more
                    </span>
                  )}
                </div>
              </div>
            )}

            {/* Recommendations */}
            {currentScore.recommendations.length > 0 && (
              <div className="space-y-3">
                <h3 className="font-semibold">Recommendations</h3>
                <ul className="space-y-2">
                  {currentScore.recommendations.slice(0, 5).map((rec, index) => (
                    <li key={index} className="flex items-start gap-2 text-sm">
                      <span className="text-primary mt-1">•</span>
                      <span>{rec}</span>
                    </li>
                  ))}
                </ul>
              </div>
            )}
          </CardContent>
        </Card>
      )}
    </div>
  );
}