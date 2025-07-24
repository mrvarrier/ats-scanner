import { useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
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
  Zap,
  AlertCircle,
  CheckCircle,
  RotateCcw,
  Eye,
  Sparkles,
} from 'lucide-react';
import { toast } from '@/hooks/use-toast';
import { FileUpload } from '@/components/ui/FileUpload';
import type {
  CommandResult,
  DocumentInfo,
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
  const [selectedIndustry, setSelectedIndustry] = useState<string>('');

  // File upload handler
  const handleFileUploaded = useCallback((document: DocumentInfo) => {
    setUploadedFile(document);
    setCurrentStep('job-description');
    toast({
      title: 'File uploaded successfully',
      description: `Parsed ${document.filename} (${document.content.split(/\s+/).length} words)`,
    });
  }, []);

  const handleContentExtracted = useCallback((_content: string) => {
    // Content is already handled in handleFileUploaded
    // This callback is available for future use if needed
  }, []);

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

          // Run achievement analysis, ML insights, semantic analysis, format compatibility, industry analysis, ATS testing, comprehensive analysis, competitive analysis, and individual ML commands in parallel
          const [
            achievementResult,
            mlInsightsResult,
            semanticResult,
            formatResult,
            industryResult,
            atsValidationResult,
            atsSimulationResult,
            comprehensiveResult,
            competitiveAnalysisResult,
            marketPositionResult,
            salaryInsightsResult,
            hiringProbabilityResult,
            applicationSuccessResult,
            careerPathResult,
            salaryPredictionMLResult,
            mlRecommendationsResult,
          ] = await Promise.allSettled([
            runAchievementAnalysis(uploadedFile.content),
            runMLInsights(uploadedFile.content, jobDescription),
            runSemanticAnalysis(
              uploadedFile.content,
              jobDescription,
              selectedIndustry || 'technology'
            ),
            runFormatCompatibilityCheck(uploadedFile.content),
            runIndustryAnalysis(
              uploadedFile.content,
              jobDescription,
              selectedIndustry || 'technology'
            ),
            runATSValidationSuite(),
            runATSSimulation(uploadedFile.content, jobDescription),
            runComprehensiveAnalysis(
              uploadedFile.content,
              jobDescription,
              selectedIndustry || 'technology'
            ),
            runCompetitiveAnalysis(uploadedFile.content, jobDescription),
            runMarketPositionAnalysis(uploadedFile.content, jobDescription),
            runSalaryInsights(uploadedFile.content, jobDescription),
            runHiringProbabilityAnalysis(uploadedFile.content, jobDescription),
            runApplicationSuccessPrediction(
              uploadedFile.content,
              jobDescription
            ),
            runCareerPathSuggestions(uploadedFile.content),
            runSalaryPredictionML(uploadedFile.content, jobDescription),
            runMLRecommendations(uploadedFile.content, jobDescription),
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
            semanticAnalysis:
              semanticResult.status === 'fulfilled'
                ? semanticResult.value
                : undefined,
            formatCompatibility:
              formatResult.status === 'fulfilled'
                ? formatResult.value
                : undefined,
            industryAnalysis:
              industryResult.status === 'fulfilled'
                ? industryResult.value
                : undefined,
            atsValidation:
              atsValidationResult.status === 'fulfilled'
                ? atsValidationResult.value
                : undefined,
            atsSimulation:
              atsSimulationResult.status === 'fulfilled'
                ? atsSimulationResult.value
                : undefined,
            comprehensiveAnalysis:
              comprehensiveResult.status === 'fulfilled'
                ? comprehensiveResult.value
                : undefined,
            competitiveAnalysis:
              competitiveAnalysisResult.status === 'fulfilled'
                ? competitiveAnalysisResult.value
                : undefined,
            marketPositionAnalysis:
              marketPositionResult.status === 'fulfilled'
                ? marketPositionResult.value
                : undefined,
            salaryInsights:
              salaryInsightsResult.status === 'fulfilled'
                ? salaryInsightsResult.value
                : undefined,
            hiringProbabilityAnalysis:
              hiringProbabilityResult.status === 'fulfilled'
                ? hiringProbabilityResult.value
                : undefined,
            applicationSuccessPrediction:
              applicationSuccessResult.status === 'fulfilled'
                ? applicationSuccessResult.value
                : undefined,
            careerPathSuggestions:
              careerPathResult.status === 'fulfilled'
                ? careerPathResult.value
                : undefined,
            salaryPredictionML:
              salaryPredictionMLResult.status === 'fulfilled'
                ? salaryPredictionMLResult.value
                : undefined,
            mlRecommendations:
              mlRecommendationsResult.status === 'fulfilled'
                ? mlRecommendationsResult.value
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

  // Semantic analysis handler
  const runSemanticAnalysis = async (
    resumeContent: string,
    jobDesc: string,
    industry: string
  ): Promise<SemanticAnalysisResult | undefined> => {
    try {
      const result = await invoke<CommandResult<SemanticAnalysisResult>>(
        'semantic_analysis',
        {
          resumeContent,
          jobDescription: jobDesc,
          industry,
        }
      );

      if (result.success && result.data) {
        return result.data;
      } else {
        // Semantic analysis failed - continue without it
        return undefined;
      }
    } catch {
      // Semantic analysis error - continue without it
      return undefined;
    }
  };

  // Format compatibility check handler
  const runFormatCompatibilityCheck = async (
    resumeContent: string
  ): Promise<FormatCompatibilityReport | undefined> => {
    try {
      const result = await invoke<CommandResult<FormatCompatibilityReport>>(
        'check_format_compatibility',
        {
          resumeContent,
        }
      );

      if (result.success && result.data) {
        return result.data;
      } else {
        // Format compatibility check failed - continue without it
        return undefined;
      }
    } catch {
      // Format compatibility check error - continue without it
      return undefined;
    }
  };

  // Industry analysis handler
  const runIndustryAnalysis = async (
    resumeContent: string,
    jobDesc: string,
    targetIndustry: string
  ): Promise<IndustryAnalysisResult | undefined> => {
    try {
      const result = await invoke<CommandResult<IndustryAnalysisResult>>(
        'industry_analysis',
        {
          resumeContent,
          jobDescription: jobDesc,
          targetIndustry,
        }
      );

      if (result.success && result.data) {
        return result.data;
      } else {
        // Industry analysis failed - continue without it
        return undefined;
      }
    } catch {
      // Industry analysis error - continue without it
      return undefined;
    }
  };

  // ATS validation suite handler
  const runATSValidationSuite = async (): Promise<
    ValidationReport | undefined
  > => {
    try {
      const result = await invoke<CommandResult<ValidationReport>>(
        'run_ats_validation_suite'
      );

      if (result.success && result.data) {
        return result.data;
      } else {
        // ATS validation suite failed - continue without it
        return undefined;
      }
    } catch {
      // ATS validation suite error - continue without it
      return undefined;
    }
  };

  // ATS simulation handler
  const runATSSimulation = async (
    resumeContent: string,
    jobDesc: string
  ): Promise<ATSSimulationResult | undefined> => {
    try {
      const result = await invoke<CommandResult<ATSSimulationResult>>(
        'simulate_multiple_ats_systems',
        {
          resumeContent,
          targetKeywords: jobDesc
            .split(/\s+/)
            .filter(word => word.length > 3)
            .slice(0, 20),
        }
      );

      if (result.success && result.data) {
        return result.data;
      } else {
        // ATS simulation failed - continue without it
        return undefined;
      }
    } catch {
      // ATS simulation error - continue without it
      return undefined;
    }
  };

  // Comprehensive analysis handler
  const runComprehensiveAnalysis = async (
    resumeContent: string,
    jobDescription: string,
    targetIndustry: string
  ): Promise<EnhancedAnalysisResult | undefined> => {
    try {
      const result = await invoke<CommandResult<EnhancedAnalysisResult>>(
        'comprehensive_analysis',
        {
          resumeContent,
          jobDescription,
          targetIndustry,
          targetRoleLevel: 'mid', // Default to mid-level, could be made configurable
        }
      );

      if (result.success && result.data) {
        return result.data;
      } else {
        // Comprehensive analysis failed - continue without it
        return undefined;
      }
    } catch {
      // Comprehensive analysis error - continue without it
      return undefined;
    }
  };

  // Competitive Analysis handlers
  const runCompetitiveAnalysis = async (
    resumeContent: string,
    jobDesc: string
  ): Promise<CompetitiveAnalysis | undefined> => {
    try {
      // Default target companies for comprehensive analysis
      const targetCompanies = [
        'Google',
        'Microsoft',
        'Amazon',
        'Apple',
        'Meta',
      ];

      const result = await invoke<CommandResult<CompetitiveAnalysis>>(
        'generate_competitive_analysis',
        {
          resumeContent,
          jobDescription: jobDesc,
          targetCompanies,
        }
      );

      if (result.success && result.data) {
        return result.data;
      } else {
        // Competitive analysis failed - continue without it
        return undefined;
      }
    } catch {
      // Competitive analysis error - continue without it
      return undefined;
    }
  };

  const runMarketPositionAnalysis = async (
    resumeContent: string,
    jobDesc: string
  ): Promise<MarketPositionAnalysis | undefined> => {
    try {
      const result = await invoke<CommandResult<MarketPositionAnalysis>>(
        'get_market_position_analysis',
        {
          resumeContent,
          jobDescription: jobDesc,
        }
      );

      if (result.success && result.data) {
        return result.data;
      } else {
        // Market position analysis failed - continue without it
        return undefined;
      }
    } catch {
      // Market position analysis error - continue without it
      return undefined;
    }
  };

  const runSalaryInsights = async (
    resumeContent: string,
    jobDesc: string
  ): Promise<SalaryInsightsResponse | undefined> => {
    try {
      const result = await invoke<CommandResult<SalaryInsightsResponse>>(
        'get_salary_insights',
        {
          resumeContent,
          jobDescription: jobDesc,
        }
      );

      if (result.success && result.data) {
        return result.data;
      } else {
        // Salary insights failed - continue without it
        return undefined;
      }
    } catch {
      // Salary insights error - continue without it
      return undefined;
    }
  };

  const runHiringProbabilityAnalysis = async (
    resumeContent: string,
    jobDesc: string
  ): Promise<HiringProbabilityResponse | undefined> => {
    try {
      const result = await invoke<CommandResult<HiringProbabilityResponse>>(
        'get_hiring_probability',
        {
          resumeContent,
          jobDescription: jobDesc,
        }
      );

      if (result.success && result.data) {
        return result.data;
      } else {
        // Hiring probability analysis failed - continue without it
        return undefined;
      }
    } catch {
      // Hiring probability analysis error - continue without it
      return undefined;
    }
  };

  // Individual ML command handlers
  const runApplicationSuccessPrediction = async (
    resumeContent: string,
    jobDesc: string
  ): Promise<ApplicationSuccessResponse | undefined> => {
    try {
      const result = await invoke<CommandResult<ApplicationSuccessResponse>>(
        'predict_application_success',
        {
          resumeContent,
          jobDescription: jobDesc,
        }
      );

      if (result.success && result.data) {
        return result.data;
      } else {
        // Application success prediction failed - continue without it
        return undefined;
      }
    } catch {
      // Application success prediction error - continue without it
      return undefined;
    }
  };

  const runCareerPathSuggestions = async (
    resumeContent: string
  ): Promise<CareerPathSuggestionsResponse | undefined> => {
    try {
      const result = await invoke<CommandResult<CareerPathSuggestionsResponse>>(
        'get_career_path_suggestions',
        {
          resumeContent,
        }
      );

      if (result.success && result.data) {
        return result.data;
      } else {
        // Career path suggestions failed - continue without it
        return undefined;
      }
    } catch {
      // Career path suggestions error - continue without it
      return undefined;
    }
  };

  const runSalaryPredictionML = async (
    resumeContent: string,
    jobDesc: string
  ): Promise<SalaryPredictionMLResponse | undefined> => {
    try {
      const result = await invoke<CommandResult<SalaryPredictionMLResponse>>(
        'get_salary_prediction_ml',
        {
          resumeContent,
          jobDescription: jobDesc,
        }
      );

      if (result.success && result.data) {
        return result.data;
      } else {
        // ML salary prediction failed - continue without it
        return undefined;
      }
    } catch {
      // ML salary prediction error - continue without it
      return undefined;
    }
  };

  const runMLRecommendations = async (
    resumeContent: string,
    jobDesc: string
  ): Promise<MLRecommendationsResponse | undefined> => {
    try {
      const result = await invoke<CommandResult<MLRecommendationsResponse>>(
        'get_ml_recommendations',
        {
          resumeContent,
          jobDescription: jobDesc,
        }
      );

      if (result.success && result.data) {
        return result.data;
      } else {
        // ML recommendations failed - continue without it
        return undefined;
      }
    } catch {
      // ML recommendations error - continue without it
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
                    <FileUpload
                      onFileUploaded={handleFileUploaded}
                      onContentExtracted={handleContentExtracted}
                      accept={['.pdf', '.docx', '.doc', '.txt']}
                      maxSize={10}
                      className="border-0 shadow-none"
                    />
                  ) : (
                    <div className="flex items-center gap-4 rounded-lg border border-green-200 bg-green-50 p-4 dark:border-green-800 dark:bg-green-900/20">
                      <CheckCircle className="h-6 w-6 flex-shrink-0 text-green-600" />
                      <div className="flex-1">
                        <p className="font-medium text-green-700 dark:text-green-300">
                          {uploadedFile.filename}
                        </p>
                        <p className="text-sm text-green-600 dark:text-green-400">
                          {uploadedFile.content.split(/\s+/).length} words •{' '}
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

                    {/* Industry Selection */}
                    <div className="space-y-2">
                      <label className="text-sm font-medium">
                        Target Industry (Optional)
                      </label>
                      <select
                        value={selectedIndustry}
                        onChange={e => setSelectedIndustry(e.target.value)}
                        className="w-full rounded-lg border border-input bg-background px-3 py-2 text-sm focus:border-transparent focus:ring-2 focus:ring-primary"
                      >
                        <option value="">
                          Auto-detect from job description
                        </option>
                        <option value="technology">Technology</option>
                        <option value="finance">Finance</option>
                        <option value="healthcare">Healthcare</option>
                        <option value="marketing">Marketing</option>
                        <option value="sales">Sales</option>
                        <option value="consulting">Consulting</option>
                        <option value="education">Education</option>
                        <option value="government">Government</option>
                        <option value="nonprofit">Non-profit</option>
                        <option value="retail">Retail</option>
                        <option value="manufacturing">Manufacturing</option>
                        <option value="legal">Legal</option>
                      </select>
                    </div>

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
