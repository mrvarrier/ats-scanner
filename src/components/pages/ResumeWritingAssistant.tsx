import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';
import { Progress } from '@/components/ui/progress';
import { Badge } from '@/components/ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import {
  PenTool,
  Lightbulb,
  Target,
  RefreshCw,
  CheckCircle,
  Sparkles,
  User,
  Briefcase,
  GraduationCap,
  Award,
  Zap,
} from 'lucide-react';
import { toast } from '@/hooks/use-toast';
import type { CommandResult, AchievementAnalysis } from '@/types';

interface LiveSuggestions {
  context_suggestions: ContextualSuggestion[];
  priority_improvements: PriorityImprovement[];
  real_time_score: number;
  score_change: number;
  section_strength: number;
  completion_percentage: number;
  next_recommended_action: string;
}

interface ContextualSuggestion {
  suggestion_id: string;
  type_: string;
  title: string;
  description: string;
  suggestion: string;
  confidence: number;
  applicable_text: string;
  replacement_text?: string;
  explanation: string;
  impact_score: number;
  urgency: string;
}

interface PriorityImprovement {
  improvement_id: string;
  category: string;
  title: string;
  description: string;
  current_issues: string[];
  suggested_fixes: string[];
  impact_score: number;
  implementation_effort: string;
  example_before?: string;
  example_after?: string;
}

interface WritingSuggestion {
  id: string;
  type: 'improvement' | 'addition' | 'removal' | 'format';
  priority: 'high' | 'medium' | 'low';
  section: string;
  original?: string;
  suggested: string;
  explanation: string;
  impact_score: number;
}

interface SectionContent {
  summary: string;
  experience: string;
  skills: string;
  education: string;
  achievements: string;
}

interface WritingMetrics {
  overall_score: number;
  section_scores: {
    summary: number;
    experience: number;
    skills: number;
    education: number;
    achievements: number;
  };
  word_count: number;
  readability_score: number;
  ats_score: number;
  keyword_density: number;
}

export function ResumeWritingAssistant() {
  const [jobDescription, setJobDescription] = useState('');
  const [activeSection, setActiveSection] = useState('summary');
  const [sectionContent, setSectionContent] = useState<SectionContent>({
    summary: '',
    experience: '',
    skills: '',
    education: '',
    achievements: '',
  });
  const [suggestions, setSuggestions] = useState<WritingSuggestion[]>([]);
  const [metrics, setMetrics] = useState<WritingMetrics | null>(null);
  const [isGenerating, setIsGenerating] = useState(false);
  const [_isAnalyzing, setIsAnalyzing] = useState(false);

  // Section configuration
  const sections = [
    {
      id: 'summary',
      name: 'Professional Summary',
      icon: User,
      description: 'Compelling professional overview',
    },
    {
      id: 'experience',
      name: 'Work Experience',
      icon: Briefcase,
      description: 'Professional experience and achievements',
    },
    {
      id: 'skills',
      name: 'Skills',
      icon: Zap,
      description: 'Technical and soft skills',
    },
    {
      id: 'education',
      name: 'Education',
      icon: GraduationCap,
      description: 'Educational background',
    },
    {
      id: 'achievements',
      name: 'Key Achievements',
      icon: Award,
      description: 'Notable accomplishments and awards',
    },
  ];

  // Generate AI-powered content suggestions
  const generateSuggestions = useCallback(async () => {
    if (!jobDescription.trim()) {
      toast({
        title: 'Job description required',
        description:
          'Please provide a job description to get personalized suggestions.',
        variant: 'destructive',
      });
      return;
    }

    try {
      setIsGenerating(true);

      const currentContent =
        sectionContent[activeSection as keyof SectionContent];

      // Use existing real-time suggestions command
      const result = await invoke<CommandResult<LiveSuggestions>>(
        'get_realtime_suggestions',
        {
          resume_content: currentContent,
          job_description: jobDescription,
          cursor_position: currentContent.length,
        }
      );

      if (result.success && result.data) {
        // Transform live suggestions to writing suggestions
        const contextualSuggestions: WritingSuggestion[] =
          result.data.context_suggestions.map(cs => ({
            id: cs.suggestion_id,
            type: 'improvement' as const,
            priority:
              cs.urgency === 'immediate'
                ? ('high' as const)
                : cs.urgency === 'soon'
                  ? ('medium' as const)
                  : ('low' as const),
            section: activeSection,
            original: cs.applicable_text,
            suggested: cs.replacement_text ?? cs.suggestion,
            explanation: cs.explanation,
            impact_score: cs.impact_score,
          }));

        const prioritySuggestions: WritingSuggestion[] =
          result.data.priority_improvements.map(pi => ({
            id: pi.improvement_id,
            type: 'improvement' as const,
            priority:
              pi.impact_score > 80
                ? ('high' as const)
                : pi.impact_score > 50
                  ? ('medium' as const)
                  : ('low' as const),
            section: activeSection,
            original: pi.example_before,
            suggested: pi.example_after ?? pi.suggested_fixes.join('; '),
            explanation: pi.description,
            impact_score: pi.impact_score,
          }));

        setSuggestions([...contextualSuggestions, ...prioritySuggestions]);

        // Update metrics with real-time data
        setMetrics(prevMetrics => ({
          overall_score: result.data?.real_time_score ?? 0,
          section_scores: {
            summary: 70,
            experience: 70,
            skills: 70,
            education: 70,
            achievements: 70,
            ...prevMetrics?.section_scores,
            [activeSection]: result.data?.section_strength ?? 0,
          },
          word_count: prevMetrics?.word_count ?? 0,
          readability_score: prevMetrics?.readability_score ?? 0,
          keyword_density: prevMetrics?.keyword_density ?? 0,
          ats_score: result.data?.real_time_score ?? 0,
        }));
      } else {
        throw new Error(result.error ?? 'Failed to generate suggestions');
      }
    } catch (error) {
      toast({
        title: 'Error generating suggestions',
        description: `Failed to generate writing suggestions: ${error}`,
        variant: 'destructive',
      });
    } finally {
      setIsGenerating(false);
    }
  }, [jobDescription, activeSection, sectionContent]);

  // Analyze content metrics
  const analyzeContent = useCallback(async () => {
    if (!sectionContent[activeSection as keyof SectionContent].trim()) {
      return;
    }

    try {
      setIsAnalyzing(true);

      // Use existing analysis command
      const result = await invoke<CommandResult<AchievementAnalysis>>(
        'analyze_achievements',
        {
          resume_content: sectionContent[activeSection as keyof SectionContent],
          job_description: jobDescription,
        }
      );

      if (result.success && result.data) {
        // Transform achievement analysis to writing metrics
        const achievementScore = result.data.overall_achievement_score;
        const writingMetrics: WritingMetrics = {
          overall_score: achievementScore,
          section_scores: {
            summary: activeSection === 'summary' ? achievementScore : 70,
            experience: activeSection === 'experience' ? achievementScore : 70,
            skills: activeSection === 'skills' ? achievementScore : 70,
            education: activeSection === 'education' ? achievementScore : 70,
            achievements:
              activeSection === 'achievements' ? achievementScore : 70,
          },
          word_count:
            sectionContent[activeSection as keyof SectionContent].split(' ')
              .length,
          readability_score: 85, // Placeholder
          ats_score: result.data.quantification_score ?? 75,
          keyword_density: result.data.bullet_points?.length ?? 0,
        };

        setMetrics(writingMetrics);
      }
    } catch (_error) {
      // Silent failure for metrics
    } finally {
      setIsAnalyzing(false);
    }
  }, [activeSection, sectionContent, jobDescription]);

  // Apply suggestion to content
  const applySuggestion = (suggestion: WritingSuggestion) => {
    const currentContent =
      sectionContent[activeSection as keyof SectionContent];

    let newContent: string;
    if (suggestion.type === 'addition') {
      newContent = currentContent + '\n\n' + suggestion.suggested;
    } else if (suggestion.original) {
      newContent = currentContent.replace(
        suggestion.original,
        suggestion.suggested
      );
    } else {
      newContent = suggestion.suggested;
    }

    setSectionContent(prev => ({
      ...prev,
      [activeSection]: newContent,
    }));

    // Remove applied suggestion
    setSuggestions(prev => prev.filter(s => s.id !== suggestion.id));

    toast({
      title: 'Suggestion applied',
      description: 'Content has been updated with the AI suggestion.',
    });
  };

  // Handle content change
  const handleContentChange = (content: string) => {
    setSectionContent(prev => ({
      ...prev,
      [activeSection]: content,
    }));
  };

  // Auto-analyze content when it changes
  useEffect(() => {
    const timer = setTimeout(() => {
      void analyzeContent();
    }, 1000);

    return () => clearTimeout(timer);
  }, [analyzeContent]);

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="flex items-center gap-2 text-3xl font-bold tracking-tight">
            <PenTool className="h-7 w-7" />
            AI Resume Writing Assistant
          </h1>
          <p className="text-muted-foreground">
            Get personalized AI-powered suggestions to improve your resume
            content
          </p>
        </div>
      </div>

      {/* Job Description Input */}
      <Card>
        <CardHeader>
          <CardTitle>Target Job Description</CardTitle>
          <CardDescription>
            Paste the job description to get personalized writing suggestions
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Textarea
            placeholder="Paste the job description here..."
            value={jobDescription}
            onChange={e => setJobDescription(e.target.value)}
            className="min-h-[120px]"
          />
        </CardContent>
      </Card>

      <div className="grid gap-6 lg:grid-cols-3">
        {/* Section Selection and Content Editor */}
        <div className="lg:col-span-2">
          <Card>
            <CardHeader>
              <CardTitle>Resume Content Editor</CardTitle>
              <CardDescription>
                Select a section and start writing. Get AI suggestions as you
                work.
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                {/* Section Tabs */}
                <Tabs value={activeSection} onValueChange={setActiveSection}>
                  <TabsList className="grid w-full grid-cols-5">
                    {sections.map(section => {
                      const Icon = section.icon;
                      return (
                        <TabsTrigger
                          key={section.id}
                          value={section.id}
                          className="flex items-center gap-1"
                        >
                          <Icon className="h-4 w-4" />
                          <span className="hidden sm:inline">
                            {section.name}
                          </span>
                        </TabsTrigger>
                      );
                    })}
                  </TabsList>

                  {sections.map(section => (
                    <TabsContent
                      key={section.id}
                      value={section.id}
                      className="space-y-4"
                    >
                      <div className="flex items-center justify-between">
                        <div>
                          <h3 className="text-lg font-semibold">
                            {section.name}
                          </h3>
                          <p className="text-sm text-muted-foreground">
                            {section.description}
                          </p>
                        </div>
                        <Button
                          onClick={generateSuggestions}
                          disabled={isGenerating}
                        >
                          {isGenerating ? (
                            <RefreshCw className="mr-2 h-4 w-4 animate-spin" />
                          ) : (
                            <Sparkles className="mr-2 h-4 w-4" />
                          )}
                          Get AI Suggestions
                        </Button>
                      </div>

                      <Textarea
                        placeholder={`Write your ${section.name.toLowerCase()} content here...`}
                        value={
                          sectionContent[section.id as keyof SectionContent]
                        }
                        onChange={e => handleContentChange(e.target.value)}
                        className="min-h-[300px]"
                      />

                      {/* Content Metrics */}
                      {metrics && activeSection === section.id && (
                        <div className="rounded-lg border bg-muted/50 p-4">
                          <div className="grid gap-4 md:grid-cols-2">
                            <div className="space-y-2">
                              <div className="flex items-center justify-between">
                                <span className="text-sm font-medium">
                                  Section Score
                                </span>
                                <span className="text-sm">
                                  {
                                    metrics.section_scores[
                                      section.id as keyof typeof metrics.section_scores
                                    ]
                                  }
                                  %
                                </span>
                              </div>
                              <Progress
                                value={
                                  metrics.section_scores[
                                    section.id as keyof typeof metrics.section_scores
                                  ]
                                }
                                className="h-2"
                              />
                            </div>
                            <div className="space-y-2">
                              <div className="flex items-center justify-between">
                                <span className="text-sm font-medium">
                                  ATS Score
                                </span>
                                <span className="text-sm">
                                  {metrics.ats_score}%
                                </span>
                              </div>
                              <Progress
                                value={metrics.ats_score}
                                className="h-2"
                              />
                            </div>
                          </div>
                          <div className="mt-2 flex gap-4 text-xs text-muted-foreground">
                            <span>Words: {metrics.word_count}</span>
                            <span>Keywords: {metrics.keyword_density}</span>
                            <span>
                              Readability: {metrics.readability_score}%
                            </span>
                          </div>
                        </div>
                      )}
                    </TabsContent>
                  ))}
                </Tabs>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* AI Suggestions Panel */}
        <div>
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Lightbulb className="h-5 w-5" />
                AI Suggestions
              </CardTitle>
              <CardDescription>
                Personalized writing suggestions for the current section
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                {suggestions.length === 0 ? (
                  <div className="py-8 text-center">
                    <Target className="mx-auto h-12 w-12 text-muted-foreground/50" />
                    <p className="mt-2 text-sm text-muted-foreground">
                      {jobDescription.trim()
                        ? "Click 'Get AI Suggestions' to receive personalized recommendations"
                        : 'Add a job description and content to get AI suggestions'}
                    </p>
                  </div>
                ) : (
                  suggestions.map(suggestion => (
                    <Card
                      key={suggestion.id}
                      className="border-l-4 border-l-blue-500"
                    >
                      <CardContent className="pt-4">
                        <div className="space-y-3">
                          <div className="flex items-center justify-between">
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
                            <div className="flex items-center gap-1 text-sm text-muted-foreground">
                              <Target className="h-3 w-3" />
                              Impact: {suggestion.impact_score}%
                            </div>
                          </div>

                          <div className="space-y-2">
                            <p className="text-sm text-muted-foreground">
                              {suggestion.explanation}
                            </p>

                            {suggestion.original && (
                              <div className="rounded bg-red-50 p-2 dark:bg-red-950/20">
                                <p className="text-xs font-medium text-red-700 dark:text-red-300">
                                  Current:
                                </p>
                                <p className="text-sm text-red-600 dark:text-red-400">
                                  {suggestion.original}
                                </p>
                              </div>
                            )}

                            <div className="rounded bg-green-50 p-2 dark:bg-green-950/20">
                              <p className="text-xs font-medium text-green-700 dark:text-green-300">
                                Suggested:
                              </p>
                              <p className="text-sm text-green-600 dark:text-green-400">
                                {suggestion.suggested}
                              </p>
                            </div>
                          </div>

                          <Button
                            size="sm"
                            onClick={() => applySuggestion(suggestion)}
                            className="w-full"
                          >
                            <CheckCircle className="mr-2 h-4 w-4" />
                            Apply Suggestion
                          </Button>
                        </div>
                      </CardContent>
                    </Card>
                  ))
                )}
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}
