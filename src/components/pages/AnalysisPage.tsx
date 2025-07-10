import { useState, useCallback } from 'react';
import { useDropzone } from 'react-dropzone';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';
import { Textarea } from '@/components/ui/textarea';
import { useAppStore } from '@/store/useAppStore';
import {
  Upload,
  Zap,
  AlertCircle,
  CheckCircle,
  RotateCcw,
  Eye,
  Sparkles,
} from 'lucide-react';
import { toast } from '@/hooks/use-toast';
import type {
  CommandResult,
  DocumentInfo,
  AnalysisResult,
  AchievementAnalysis,
  MLInsights,
} from '@/types';

// Extended File interface for drag-and-drop with path support
interface FileWithPath extends File {
  path?: string;
}

type AnalysisStep =
  | 'upload'
  | 'job-description'
  | 'model'
  | 'ready'
  | 'analyzing'
  | 'complete';

export function AnalysisPage() {
  const {
    models,
    selectedModel,
    setSelectedModel,
    setCurrentAnalysis,
    isAnalyzing,
    setIsAnalyzing,
    isOllamaConnected,
    setCurrentDetailedAnalysis,
    setActiveTab,
  } = useAppStore();

  const [currentStep, setCurrentStep] = useState<AnalysisStep>('upload');
  const [uploadedFile, setUploadedFile] = useState<DocumentInfo | null>(null);
  const [jobDescription, setJobDescription] = useState('');
  const [analysisProgress, setAnalysisProgress] = useState(0);
  const [_uploadProgress, setUploadProgress] = useState(0);
  const [completedAnalysis, setCompletedAnalysis] =
    useState<AnalysisResult | null>(null);

  // Drag and drop handler
  const onDrop = useCallback(async (acceptedFiles: FileWithPath[]) => {
    const file = acceptedFiles[0];
    if (!file) return;

    try {
      setUploadProgress(0);
      setUploadedFile(null);

      // Save file to temporary location and parse
      const result = await invoke<CommandResult<DocumentInfo>>(
        'parse_document',
        {
          filePath: file.path ?? file.name,
        }
      );

      if (result.success && result.data) {
        setUploadedFile(result.data);
        setUploadProgress(100);
        setCurrentStep('job-description');
        toast({
          title: 'File uploaded successfully',
          description: `Parsed ${result.data.filename} (${result.data.word_count} words)`,
        });
      } else {
        throw new Error(result.error ?? 'Failed to parse document');
      }
    } catch (error) {
      toast({
        title: 'Upload failed',
        description: `Error: ${error}`,
        variant: 'destructive',
      });
    }
  }, []);

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    accept: {
      'application/pdf': ['.pdf'],
      'application/vnd.openxmlformats-officedocument.wordprocessingml.document':
        ['.docx'],
      'text/plain': ['.txt'],
    },
    multiple: false,
    maxSize: 10 * 1024 * 1024, // 10MB
  });

  // File browser handler
  const handleBrowseFiles = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: 'Resume Files',
            extensions: ['pdf', 'docx', 'txt'],
          },
        ],
      });

      if (selected && typeof selected === 'string') {
        const result = await invoke<CommandResult<DocumentInfo>>(
          'parse_document',
          {
            filePath: selected,
          }
        );

        if (result.success && result.data) {
          setUploadedFile(result.data);
          setCurrentStep('job-description');
          toast({
            title: 'File loaded successfully',
            description: `Parsed ${result.data.filename} (${result.data.word_count} words)`,
          });
        } else {
          throw new Error(result.error ?? 'Failed to parse document');
        }
      }
    } catch (error) {
      toast({
        title: 'File loading failed',
        description: `Error: ${error}`,
        variant: 'destructive',
      });
    }
  };

  // Job description change handler
  const handleJobDescriptionChange = (value: string) => {
    setJobDescription(value);
    if (value.trim() && currentStep === 'job-description') {
      // Auto-progress after a short delay
      setTimeout(() => {
        if (jobDescription.trim()) {
          setCurrentStep('model');
        }
      }, 1000);
    }
  };

  // Model selection handler
  const handleModelSelect = (model: string) => {
    setSelectedModel(model);
    setCurrentStep('ready');
  };

  // Analysis handler
  const handleAnalyze = async () => {
    if (!uploadedFile || !jobDescription.trim() || !selectedModel) {
      return;
    }

    try {
      setCurrentStep('analyzing');
      setIsAnalyzing(true);
      setAnalysisProgress(0);

      // Simulate progress updates
      const progressInterval = setInterval(() => {
        setAnalysisProgress(prev => {
          if (prev >= 90) {
            return 90;
          }
          return prev + 10;
        });
      }, 500);

      try {
        const result = await invoke<CommandResult<AnalysisResult>>(
          'analyze_resume',
          {
            request: {
              resume_content: uploadedFile.content,
              job_description: jobDescription,
              model_name: selectedModel,
            },
          }
        );

        clearInterval(progressInterval);
        setAnalysisProgress(100);

        if (result.success && result.data) {
          setCurrentAnalysis(result.data);
          setCompletedAnalysis(result.data);

          // Run achievement analysis and ML insights in parallel
          const [achievementResult, mlInsightsResult] =
            await Promise.allSettled([
              runAchievementAnalysis(uploadedFile.content),
              runMLInsights(uploadedFile.content, jobDescription),
            ]);

          // Prepare detailed analysis data
          const detailedAnalysisData = {
            result: result.data,
            achievementAnalysis:
              achievementResult.status === 'fulfilled'
                ? achievementResult.value
                : undefined,
            mlInsights:
              mlInsightsResult.status === 'fulfilled'
                ? mlInsightsResult.value
                : undefined,
            resumeFilename: uploadedFile.filename,
            jobDescription: jobDescription,
            modelUsed: selectedModel,
            timestamp: new Date().toISOString(),
          };

          // Store the detailed analysis data
          setCurrentDetailedAnalysis(detailedAnalysisData);
          setCurrentStep('complete');

          toast({
            title: 'Analysis completed!',
            description: `Your resume scored ${result.data.overall_score.toFixed(1)}%`,
          });
        } else {
          clearInterval(progressInterval);
          throw new Error(result.error ?? 'Analysis failed');
        }
      } catch (analysisError) {
        clearInterval(progressInterval);
        throw analysisError;
      }
    } catch (error) {
      setCurrentStep('ready');
      toast({
        title: 'Analysis failed',
        description: `Error: ${error}`,
        variant: 'destructive',
      });
    } finally {
      setIsAnalyzing(false);
    }
  };

  // Achievement analysis handler
  const runAchievementAnalysis = async (
    resumeContent: string
  ): Promise<AchievementAnalysis | undefined> => {
    try {
      const result = await invoke<CommandResult<AchievementAnalysis>>(
        'analyze_achievements',
        {
          resumeContent,
        }
      );

      if (result.success && result.data) {
        return result.data;
      } else {
        // Achievement analysis failed - continue without it
        return undefined;
      }
    } catch {
      // Achievement analysis error - continue without it
      return undefined;
    }
  };

  // ML insights handler
  const runMLInsights = async (
    resumeContent: string,
    jobDesc: string
  ): Promise<MLInsights | undefined> => {
    try {
      const result = await invoke<CommandResult<MLInsights>>(
        'generate_ml_insights',
        {
          resumeContent,
          jobDescription: jobDesc,
        }
      );

      if (result.success && result.data) {
        return result.data;
      } else {
        // ML insights failed - continue without it
        return undefined;
      }
    } catch {
      // ML insights error - continue without it
      return undefined;
    }
  };

  // Reset form
  const handleReset = () => {
    setUploadedFile(null);
    setJobDescription('');
    setSelectedModel('');
    setCompletedAnalysis(null);
    setCurrentStep('upload');
    setAnalysisProgress(0);
    setUploadProgress(0);
  };

  const isStepComplete = (step: AnalysisStep): boolean => {
    switch (step) {
      case 'upload':
        return !!uploadedFile;
      case 'job-description':
        return !!jobDescription.trim();
      case 'model':
        return !!selectedModel;
      case 'ready':
        return !!(uploadedFile && jobDescription.trim() && selectedModel);
      default:
        return false;
    }
  };

  return (
    <div className="mx-auto max-w-4xl space-y-8">
      {/* Header */}
      <div className="space-y-3 text-center">
        <h1 className="text-4xl font-bold tracking-tight">Resume Analysis</h1>
        <p className="text-xl text-muted-foreground">
          Get instant ATS compatibility analysis with AI-powered insights
        </p>
      </div>

      {/* Ollama Connection Warning */}
      {!isOllamaConnected && (
        <Card className="border-yellow-200 bg-yellow-50 dark:border-yellow-800 dark:bg-yellow-900/20">
          <CardContent className="flex items-center gap-3 pt-6">
            <AlertCircle className="h-5 w-5 text-yellow-600" />
            <p className="text-sm text-yellow-700 dark:text-yellow-300">
              Ollama is not connected. Please check your Ollama installation and
              try again.
            </p>
          </CardContent>
        </Card>
      )}

      {/* Main Analysis Card */}
      {currentStep !== 'complete' && (
        <Card className="border-2">
          <CardHeader className="text-center">
            <CardTitle className="text-2xl">
              {currentStep === 'analyzing'
                ? 'Analyzing Your Resume...'
                : 'Start Your Analysis'}
            </CardTitle>
            <CardDescription>
              {currentStep === 'analyzing'
                ? 'Please wait while we analyze your resume against the job description'
                : 'Follow the steps below to analyze your resume'}
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-8">
            {/* Progress Steps */}
            {currentStep !== 'analyzing' && (
              <div className="flex items-center justify-center space-x-4">
                {['upload', 'job-description', 'model', 'ready'].map(
                  (step, index) => (
                    <div key={step} className="flex items-center">
                      <div
                        className={`flex h-8 w-8 items-center justify-center rounded-full border-2 text-sm font-medium ${
                          isStepComplete(step as AnalysisStep)
                            ? 'border-primary bg-primary text-primary-foreground'
                            : currentStep === step
                              ? 'border-primary text-primary'
                              : 'border-muted-foreground text-muted-foreground'
                        }`}
                      >
                        {isStepComplete(step as AnalysisStep) ? (
                          <CheckCircle className="h-4 w-4" />
                        ) : (
                          index + 1
                        )}
                      </div>
                      {index < 3 && (
                        <div
                          className={`mx-2 h-0.5 w-16 ${
                            isStepComplete(step as AnalysisStep)
                              ? 'bg-primary'
                              : 'bg-muted'
                          }`}
                        />
                      )}
                    </div>
                  )
                )}
              </div>
            )}

            {/* Step 1: File Upload */}
            {(currentStep === 'upload' || uploadedFile) &&
              currentStep !== 'analyzing' && (
                <div className="space-y-4">
                  <div className="flex items-center gap-2">
                    <div className="flex h-6 w-6 items-center justify-center rounded-full bg-primary text-sm font-medium text-primary-foreground">
                      {uploadedFile ? <CheckCircle className="h-4 w-4" /> : '1'}
                    </div>
                    <h3 className="text-lg font-semibold">
                      Upload Your Resume
                    </h3>
                  </div>

                  {!uploadedFile ? (
                    <div
                      {...getRootProps()}
                      className={`cursor-pointer rounded-lg border-2 border-dashed p-8 text-center transition-all ${
                        isDragActive
                          ? 'scale-105 border-primary bg-primary/5'
                          : 'border-muted-foreground/25 hover:border-primary/50 hover:bg-muted/20'
                      }`}
                    >
                      <input {...getInputProps()} />
                      <Upload className="mx-auto mb-4 h-12 w-12 text-primary" />
                      {isDragActive ? (
                        <p className="text-lg font-medium text-primary">
                          Drop your resume here...
                        </p>
                      ) : (
                        <>
                          <p className="mb-2 text-lg font-medium">
                            Drag & drop your resume here
                          </p>
                          <p className="mb-4 text-sm text-muted-foreground">
                            or click to browse files
                          </p>
                          <Button
                            size="lg"
                            onClick={e => {
                              e.stopPropagation();
                              void handleBrowseFiles();
                            }}
                          >
                            <Upload className="mr-2 h-4 w-4" />
                            Choose File
                          </Button>
                        </>
                      )}
                      <p className="mt-4 text-xs text-muted-foreground">
                        Supports PDF, DOCX, TXT files up to 10MB
                      </p>
                    </div>
                  ) : (
                    <div className="flex items-center gap-4 rounded-lg border border-green-200 bg-green-50 p-4 dark:border-green-800 dark:bg-green-900/20">
                      <CheckCircle className="h-6 w-6 flex-shrink-0 text-green-600" />
                      <div className="flex-1">
                        <p className="font-medium text-green-700 dark:text-green-300">
                          {uploadedFile.filename}
                        </p>
                        <p className="text-sm text-green-600 dark:text-green-400">
                          {uploadedFile.word_count} words •{' '}
                          {uploadedFile.file_type.toUpperCase()}
                        </p>
                      </div>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => {
                          setUploadedFile(null);
                          setCurrentStep('upload');
                        }}
                        className="text-green-700 hover:text-green-800"
                      >
                        Change File
                      </Button>
                    </div>
                  )}
                </div>
              )}

            {/* Step 2: Job Description */}
            {(currentStep === 'job-description' ||
              (uploadedFile && jobDescription)) &&
              currentStep !== 'analyzing' && (
                <div className="space-y-4">
                  <div className="flex items-center gap-2">
                    <div className="flex h-6 w-6 items-center justify-center rounded-full bg-primary text-sm font-medium text-primary-foreground">
                      {jobDescription.trim() ? (
                        <CheckCircle className="h-4 w-4" />
                      ) : (
                        '2'
                      )}
                    </div>
                    <h3 className="text-lg font-semibold">
                      Add Job Description
                    </h3>
                  </div>

                  <div className="space-y-3">
                    <Textarea
                      placeholder="Paste the job description here..."
                      value={jobDescription}
                      onChange={e => handleJobDescriptionChange(e.target.value)}
                      className="min-h-[150px] resize-none text-sm"
                      autoFocus={currentStep === 'job-description'}
                    />
                    <div className="flex items-center justify-between text-xs text-muted-foreground">
                      <span>{jobDescription.length} characters</span>
                      {jobDescription.trim() && (
                        <div className="flex items-center gap-1 text-green-600">
                          <CheckCircle className="h-3 w-3" />
                          Ready to proceed
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              )}

            {/* Step 3: Model Selection */}
            {(currentStep === 'model' ||
              (uploadedFile && jobDescription && selectedModel)) &&
              currentStep !== 'analyzing' && (
                <div className="space-y-4">
                  <div className="flex items-center gap-2">
                    <div className="flex h-6 w-6 items-center justify-center rounded-full bg-primary text-sm font-medium text-primary-foreground">
                      {selectedModel ? (
                        <CheckCircle className="h-4 w-4" />
                      ) : (
                        '3'
                      )}
                    </div>
                    <h3 className="text-lg font-semibold">Select AI Model</h3>
                  </div>

                  <div className="space-y-3">
                    <select
                      value={selectedModel ?? ''}
                      onChange={e => handleModelSelect(e.target.value)}
                      className="w-full rounded-lg border border-input bg-background px-4 py-3 text-sm focus:border-transparent focus:ring-2 focus:ring-primary"
                      disabled={models.length === 0}
                      autoFocus={currentStep === 'model'}
                    >
                      <option value="">Choose an AI model...</option>
                      {models.map(model => (
                        <option key={model.name} value={model.name}>
                          {model.name}
                        </option>
                      ))}
                    </select>
                    {models.length === 0 && (
                      <p className="text-xs text-muted-foreground">
                        No models available. Please check your Ollama
                        installation.
                      </p>
                    )}
                    {selectedModel && (
                      <div className="flex items-center gap-1 text-xs text-green-600">
                        <CheckCircle className="h-3 w-3" />
                        Model selected: {selectedModel}
                      </div>
                    )}
                  </div>
                </div>
              )}

            {/* Step 4: Ready to Analyze */}
            {currentStep === 'ready' && (
              <div className="space-y-6 text-center">
                <div className="space-y-2">
                  <div className="flex items-center justify-center gap-2">
                    <Sparkles className="h-6 w-6 text-primary" />
                    <h3 className="text-xl font-semibold">Ready to Analyze!</h3>
                  </div>
                  <p className="text-muted-foreground">
                    Your resume will be analyzed against the job description
                    using {selectedModel}
                  </p>
                </div>

                <Button
                  onClick={handleAnalyze}
                  size="lg"
                  className="px-8 py-6 text-lg"
                  disabled={isAnalyzing}
                >
                  <Zap className="mr-2 h-5 w-5" />
                  Start Analysis
                </Button>
              </div>
            )}

            {/* Analyzing State */}
            {currentStep === 'analyzing' && (
              <div className="space-y-6 text-center">
                <div className="space-y-4">
                  <div className="mx-auto h-16 w-16 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
                  <div className="space-y-2">
                    <h3 className="text-xl font-semibold">
                      Analyzing Your Resume
                    </h3>
                    <p className="text-muted-foreground">
                      This may take a few moments...
                    </p>
                  </div>
                </div>

                <div className="mx-auto max-w-md space-y-2">
                  <div className="flex justify-between text-sm">
                    <span>Progress</span>
                    <span>{analysisProgress}%</span>
                  </div>
                  <Progress value={analysisProgress} className="h-2" />
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Completion State */}
      {currentStep === 'complete' && completedAnalysis && (
        <Card className="border-2 border-green-500 bg-gradient-to-br from-green-50 to-emerald-50 dark:from-green-950/20 dark:to-emerald-950/20">
          <CardContent className="space-y-6 pt-8 text-center">
            <div className="space-y-3">
              <div className="mx-auto flex h-16 w-16 items-center justify-center rounded-full bg-green-500">
                <CheckCircle className="h-8 w-8 text-white" />
              </div>
              <h2 className="text-3xl font-bold text-green-700 dark:text-green-300">
                Analysis Complete!
              </h2>
              <p className="text-lg text-green-600 dark:text-green-400">
                Your resume scored {completedAnalysis.overall_score.toFixed(1)}%
              </p>
            </div>

            <div className="flex flex-col justify-center gap-4 sm:flex-row">
              <Button
                onClick={() => setActiveTab('analysis-result')}
                size="lg"
                className="px-8"
              >
                <Eye className="mr-2 h-5 w-5" />
                View Full Results
              </Button>
              <Button onClick={handleReset} variant="outline" size="lg">
                <RotateCcw className="mr-2 h-5 w-5" />
                Analyze Another Resume
              </Button>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
