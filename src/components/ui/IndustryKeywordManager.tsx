import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Input } from '@/components/ui/input';
import {
  Target,
  Plus,
  X,
  RefreshCw,
  TrendingUp,
  CheckCircle,
  AlertTriangle,
  Brain,
} from 'lucide-react';
import { CommandResult } from '@/types/api';

interface IndustryKeywordMatch {
  keyword: string;
  category: string;
  found: boolean;
  frequency: number;
  context: string[];
  weight: number;
  synonyms_found: string[];
}

interface IndustryAnalysisResult {
  detected_industry: string;
  confidence_score: number;
  industry_keywords: IndustryKeywordMatch[];
  domain_expertise_score: number;
  industry_specific_recommendations: string[];
}

interface IndustryKeywordManagerProps {
  resumeContent: string;
  jobDescription: string;
  onAnalysisComplete?: (_result: IndustryAnalysisResult) => void;
}

const SUPPORTED_INDUSTRIES = [
  'technology',
  'finance',
  'healthcare',
  'marketing',
  'sales',
  'consulting',
  'engineering',
  'design',
  'education',
  'legal',
];

export function IndustryKeywordManager({
  resumeContent,
  jobDescription,
  onAnalysisComplete,
}: IndustryKeywordManagerProps) {
  const [selectedIndustry, setSelectedIndustry] = useState('technology');
  const [analysis, setAnalysis] = useState<IndustryAnalysisResult | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [customKeywords, setCustomKeywords] = useState<string[]>([]);
  const [newKeyword, setNewKeyword] = useState('');

  const performAnalysis = useCallback(async () => {
    if (!resumeContent.trim() || !jobDescription.trim()) return;

    try {
      setIsAnalyzing(true);
      const result = await invoke<CommandResult<IndustryAnalysisResult>>(
        'industry_analysis',
        {
          resumeContent,
          jobDescription,
          targetIndustry: selectedIndustry,
        }
      );

      if (result.success && result.data) {
        setAnalysis(result.data);
        onAnalysisComplete?.(result.data);
      }
    } catch {
      // Industry analysis failed - continue without analysis
    } finally {
      setIsAnalyzing(false);
    }
  }, [resumeContent, jobDescription, selectedIndustry, onAnalysisComplete]);

  useEffect(() => {
    if (resumeContent.trim() && jobDescription.trim()) {
      void performAnalysis();
    }
  }, [selectedIndustry, resumeContent, jobDescription, performAnalysis]);

  const addCustomKeyword = () => {
    if (newKeyword.trim() && !customKeywords.includes(newKeyword.trim())) {
      setCustomKeywords([...customKeywords, newKeyword.trim()]);
      setNewKeyword('');
    }
  };

  const removeCustomKeyword = (keyword: string) => {
    setCustomKeywords(customKeywords.filter(k => k !== keyword));
  };

  const getKeywordStatusIcon = (keyword: IndustryKeywordMatch) => {
    if (keyword.found && keyword.frequency >= 2) {
      return <CheckCircle className="h-4 w-4 text-green-600" />;
    } else if (keyword.found) {
      return <AlertTriangle className="h-4 w-4 text-yellow-600" />;
    } else {
      return <X className="h-4 w-4 text-red-600" />;
    }
  };

  const getKeywordStatusColor = (keyword: IndustryKeywordMatch) => {
    if (keyword.found && keyword.frequency >= 2) {
      return 'border-green-200 bg-green-50';
    } else if (keyword.found) {
      return 'border-yellow-200 bg-yellow-50';
    } else {
      return 'border-red-200 bg-red-50';
    }
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Target className="h-5 w-5" />
          Industry Keyword Analysis
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Industry Selection */}
        <div className="flex items-center gap-4">
          <div className="flex-1">
            <label className="text-sm font-medium">Target Industry</label>
            <select
              value={selectedIndustry}
              onChange={e => setSelectedIndustry(e.target.value)}
              className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
            >
              {SUPPORTED_INDUSTRIES.map(industry => (
                <option key={industry} value={industry}>
                  {industry.charAt(0).toUpperCase() + industry.slice(1)}
                </option>
              ))}
            </select>
          </div>
          <Button
            onClick={performAnalysis}
            disabled={
              !resumeContent.trim() || !jobDescription.trim() || isAnalyzing
            }
            size="sm"
          >
            {isAnalyzing ? (
              <>
                <RefreshCw className="mr-2 h-4 w-4 animate-spin" />
                Analyzing...
              </>
            ) : (
              <>
                <Brain className="mr-2 h-4 w-4" />
                Analyze
              </>
            )}
          </Button>
        </div>

        {/* Analysis Results */}
        {analysis && (
          <div className="space-y-4">
            {/* Industry Detection */}
            <div className="rounded-lg border p-4">
              <div className="mb-2 flex items-center justify-between">
                <h3 className="font-semibold">Industry Detection</h3>
                <Badge variant="outline">
                  {Math.round(analysis.confidence_score * 100)}% confidence
                </Badge>
              </div>
              <p className="text-sm text-muted-foreground">
                Detected Industry: <strong>{analysis.detected_industry}</strong>
              </p>
              <div className="mt-2">
                <div className="flex items-center gap-2 text-sm">
                  <TrendingUp className="h-4 w-4" />
                  Domain Expertise Score:{' '}
                  <span className="font-medium">
                    {Math.round(analysis.domain_expertise_score * 100)}%
                  </span>
                </div>
              </div>
            </div>

            {/* Keyword Analysis */}
            <div className="space-y-3">
              <h3 className="font-semibold">Keyword Analysis</h3>
              <div className="grid gap-3 sm:grid-cols-2">
                {analysis.industry_keywords.map((keyword, index) => (
                  <div
                    key={index}
                    className={`rounded-lg border p-3 ${getKeywordStatusColor(keyword)}`}
                  >
                    <div className="mb-1 flex items-center justify-between">
                      <div className="flex items-center gap-2">
                        {getKeywordStatusIcon(keyword)}
                        <span className="text-sm font-medium">
                          {keyword.keyword}
                        </span>
                      </div>
                      <div className="flex items-center gap-2">
                        <Badge variant="secondary" className="text-xs">
                          {keyword.category}
                        </Badge>
                        {keyword.found && (
                          <Badge variant="outline" className="text-xs">
                            {keyword.frequency}x
                          </Badge>
                        )}
                      </div>
                    </div>
                    {keyword.synonyms_found.length > 0 && (
                      <div className="text-xs text-muted-foreground">
                        Found variants: {keyword.synonyms_found.join(', ')}
                      </div>
                    )}
                    {keyword.context.length > 0 && (
                      <div className="mt-1 text-xs text-muted-foreground">
                        Context: {keyword.context[0]}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            </div>

            {/* Recommendations */}
            {analysis.industry_specific_recommendations.length > 0 && (
              <div className="space-y-2">
                <h3 className="font-semibold">
                  Industry-Specific Recommendations
                </h3>
                <ul className="space-y-2">
                  {analysis.industry_specific_recommendations.map(
                    (rec, index) => (
                      <li
                        key={index}
                        className="flex items-start gap-2 rounded-lg border border-blue-200 bg-blue-50 p-3 text-sm"
                      >
                        <span className="mt-1 text-blue-600">•</span>
                        <span>{rec}</span>
                      </li>
                    )
                  )}
                </ul>
              </div>
            )}
          </div>
        )}

        {/* Custom Keywords Management */}
        <div className="space-y-3">
          <h3 className="font-semibold">Custom Keywords</h3>
          <div className="flex gap-2">
            <Input
              placeholder="Add custom keyword..."
              value={newKeyword}
              onChange={e => setNewKeyword(e.target.value)}
              onKeyDown={e => e.key === 'Enter' && addCustomKeyword()}
              className="flex-1"
            />
            <Button onClick={addCustomKeyword} size="sm">
              <Plus className="h-4 w-4" />
            </Button>
          </div>
          {customKeywords.length > 0 && (
            <div className="flex flex-wrap gap-2">
              {customKeywords.map(keyword => (
                <Badge
                  key={keyword}
                  variant="secondary"
                  className="flex items-center gap-1"
                >
                  {keyword}
                  <button
                    onClick={() => removeCustomKeyword(keyword)}
                    className="ml-1 text-muted-foreground hover:text-foreground"
                  >
                    <X className="h-3 w-3" />
                  </button>
                </Badge>
              ))}
            </div>
          )}
        </div>
      </CardContent>
    </Card>
  );
}
