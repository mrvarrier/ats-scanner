import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import {
  Award,
  TrendingUp,
  Lightbulb,
  Target,
  CheckCircle,
  AlertCircle,
  ArrowRight,
  RefreshCw,
  Zap,
} from 'lucide-react';
import { CommandResult } from '@/types/api';

interface XYZComponents {
  x_accomplishment: string | null;
  y_measurement: string | null;
  z_method: string | null;
  completeness_score: number;
}

interface BulletAnalysis {
  original_text: string;
  section: string;
  has_action_verb: boolean;
  action_verb: string | null;
  action_verb_strength: string;
  has_quantification: boolean;
  quantifications: string[];
  has_outcome: boolean;
  outcome_description: string | null;
  has_xyz_formula: boolean;
  xyz_components: XYZComponents;
  strength_score: number;
  improvement_suggestions: string[];
}

interface XYZSuggestion {
  original: string;
  section: string;
  weakness_type: string;
  suggested_x: string | null;
  suggested_y: string | null;
  suggested_z: string | null;
  example_improvement: string;
  reasoning: string;
  expected_score_improvement: number;
}

interface SectionSuggestions {
  section_name: string;
  bullets: BulletAnalysis[];
  suggestions: XYZSuggestion[];
  section_score: number;
  improvement_potential: number;
}

interface AchievementSuggestionsProps {
  resumeContent: string;
  sectionName?: string;
  onSuggestionApply?: (_suggestion: XYZSuggestion) => void;
}

export function AchievementSuggestions({
  resumeContent,
  sectionName = 'experience',
  onSuggestionApply,
}: AchievementSuggestionsProps) {
  const [suggestions, setSuggestions] = useState<SectionSuggestions | null>(
    null
  );
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [selectedBullet, setSelectedBullet] = useState<string | null>(null);

  const fetchSuggestions = useCallback(async () => {
    if (!resumeContent.trim()) return;

    try {
      setIsAnalyzing(true);
      const result = await invoke<CommandResult<SectionSuggestions>>(
        'get_achievement_suggestions',
        {
          resumeContent,
          sectionName,
        }
      );

      if (result.success && result.data) {
        setSuggestions(result.data);
      }
    } catch {
      // Achievement suggestions failed - continue without suggestions
    } finally {
      setIsAnalyzing(false);
    }
  }, [resumeContent, sectionName]);

  useEffect(() => {
    if (resumeContent.trim()) {
      void fetchSuggestions();
    }
  }, [resumeContent, sectionName, fetchSuggestions]);

  const getStrengthIcon = (score: number) => {
    if (score >= 80) return <CheckCircle className="h-4 w-4 text-green-600" />;
    if (score >= 60) return <AlertCircle className="h-4 w-4 text-yellow-600" />;
    return <AlertCircle className="h-4 w-4 text-red-600" />;
  };

  const getWeaknessTypeLabel = (type: string) => {
    switch (type) {
      case 'missing_quantification':
        return 'Add Numbers';
      case 'weak_action_verb':
        return 'Stronger Verb';
      case 'no_outcome':
        return 'Show Impact';
      case 'incomplete_xyz':
        return 'Complete X-Y-Z';
      default:
        return 'Improve';
    }
  };

  const getWeaknessTypeColor = (type: string) => {
    switch (type) {
      case 'missing_quantification':
        return 'bg-blue-100 text-blue-800';
      case 'weak_action_verb':
        return 'bg-purple-100 text-purple-800';
      case 'no_outcome':
        return 'bg-orange-100 text-orange-800';
      case 'incomplete_xyz':
        return 'bg-red-100 text-red-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  if (!resumeContent.trim()) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Award className="h-5 w-5" />
            Achievement Suggestions
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="py-8 text-center text-muted-foreground">
            <Award className="mx-auto mb-4 h-12 w-12 opacity-50" />
            <p>Upload a resume to get contextual achievement suggestions</p>
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
            <Award className="h-5 w-5" />
            Achievement Suggestions
            {sectionName && (
              <Badge variant="outline" className="capitalize">
                {sectionName}
              </Badge>
            )}
          </span>
          <Button
            onClick={fetchSuggestions}
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
        {/* Section Overview */}
        {suggestions && (
          <div className="grid gap-4 md:grid-cols-3">
            <div className="rounded-lg border p-4 text-center">
              <div className="mb-2 flex items-center justify-center">
                <Target className="h-8 w-8 text-blue-600" />
              </div>
              <div className="text-2xl font-bold">
                {Math.round(suggestions.section_score)}%
              </div>
              <div className="text-sm text-muted-foreground">Section Score</div>
            </div>

            <div className="rounded-lg border p-4 text-center">
              <div className="mb-2 flex items-center justify-center">
                <TrendingUp className="h-8 w-8 text-green-600" />
              </div>
              <div className="text-2xl font-bold">
                +{Math.round(suggestions.improvement_potential)}%
              </div>
              <div className="text-sm text-muted-foreground">
                Improvement Potential
              </div>
            </div>

            <div className="rounded-lg border p-4 text-center">
              <div className="mb-2 flex items-center justify-center">
                <Lightbulb className="h-8 w-8 text-yellow-600" />
              </div>
              <div className="text-2xl font-bold">
                {suggestions.suggestions.length}
              </div>
              <div className="text-sm text-muted-foreground">Suggestions</div>
            </div>
          </div>
        )}

        {/* Current Bullets Analysis */}
        {suggestions && suggestions.bullets.length > 0 && (
          <div className="space-y-4">
            <h3 className="font-semibold">Current Bullet Points</h3>
            <div className="space-y-3">
              {suggestions.bullets.slice(0, 5).map((bullet, index) => (
                <div
                  key={index}
                  className={`cursor-pointer rounded-lg border p-4 transition-colors ${
                    selectedBullet === bullet.original_text
                      ? 'bg-blue-50 ring-2 ring-primary'
                      : 'hover:bg-gray-50'
                  }`}
                  onClick={() =>
                    setSelectedBullet(
                      selectedBullet === bullet.original_text
                        ? null
                        : bullet.original_text
                    )
                  }
                >
                  <div className="mb-2 flex items-start justify-between">
                    <div className="flex items-center gap-2">
                      {getStrengthIcon(bullet.strength_score)}
                      <span className="text-sm font-medium">
                        Score: {Math.round(bullet.strength_score)}%
                      </span>
                    </div>
                    <div className="flex items-center gap-2">
                      {bullet.has_xyz_formula && (
                        <Badge variant="secondary" className="text-xs">
                          X-Y-Z Formula
                        </Badge>
                      )}
                      <ArrowRight className="h-4 w-4 text-muted-foreground" />
                    </div>
                  </div>
                  <p className="mb-2 text-sm text-muted-foreground">
                    {bullet.original_text}
                  </p>
                  <div className="flex flex-wrap gap-2">
                    {bullet.has_action_verb && (
                      <Badge variant="outline" className="text-xs">
                        Action: {bullet.action_verb}
                      </Badge>
                    )}
                    {bullet.has_quantification && (
                      <Badge variant="outline" className="text-xs">
                        Quantified: {bullet.quantifications.length}
                      </Badge>
                    )}
                    {bullet.has_outcome && (
                      <Badge variant="outline" className="text-xs">
                        Impact Shown
                      </Badge>
                    )}
                  </div>

                  {/* Expanded Details */}
                  {selectedBullet === bullet.original_text && (
                    <div className="mt-4 space-y-3 border-t pt-4">
                      {/* X-Y-Z Components */}
                      {bullet.has_xyz_formula && (
                        <div className="grid grid-cols-3 gap-3 text-xs">
                          <div className="text-center">
                            <div className="font-medium text-blue-600">
                              X - Action
                            </div>
                            <div className="text-muted-foreground">
                              {bullet.xyz_components.x_accomplishment ??
                                'Missing'}
                            </div>
                          </div>
                          <div className="text-center">
                            <div className="font-medium text-green-600">
                              Y - Numbers
                            </div>
                            <div className="text-muted-foreground">
                              {bullet.xyz_components.y_measurement ?? 'Missing'}
                            </div>
                          </div>
                          <div className="text-center">
                            <div className="font-medium text-purple-600">
                              Z - Impact
                            </div>
                            <div className="text-muted-foreground">
                              {bullet.xyz_components.z_method ?? 'Missing'}
                            </div>
                          </div>
                        </div>
                      )}

                      {/* Improvement Suggestions */}
                      {bullet.improvement_suggestions.length > 0 && (
                        <div className="space-y-2">
                          <div className="text-xs font-medium">
                            Quick improvements:
                          </div>
                          <ul className="space-y-1">
                            {bullet.improvement_suggestions.map(
                              (suggestion, suggIndex) => (
                                <li
                                  key={suggIndex}
                                  className="flex items-start gap-2 text-xs text-muted-foreground"
                                >
                                  <span className="mt-0.5 text-blue-400">
                                    •
                                  </span>
                                  {suggestion}
                                </li>
                              )
                            )}
                          </ul>
                        </div>
                      )}
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Contextual Suggestions */}
        {suggestions && suggestions.suggestions.length > 0 && (
          <div className="space-y-4">
            <h3 className="font-semibold">Targeted Improvements</h3>
            <div className="space-y-4">
              {suggestions.suggestions.slice(0, 6).map((suggestion, index) => (
                <div key={index} className="rounded-lg border p-4">
                  <div className="mb-3 flex items-start justify-between">
                    <div className="flex items-center gap-2">
                      <Badge
                        className={getWeaknessTypeColor(
                          suggestion.weakness_type
                        )}
                      >
                        {getWeaknessTypeLabel(suggestion.weakness_type)}
                      </Badge>
                      <span className="text-sm font-medium">
                        +{suggestion.expected_score_improvement.toFixed(1)}%
                        improvement
                      </span>
                    </div>
                    {onSuggestionApply && (
                      <Button
                        onClick={() => onSuggestionApply(suggestion)}
                        size="sm"
                        variant="outline"
                      >
                        Apply
                      </Button>
                    )}
                  </div>

                  <div className="space-y-3">
                    <div>
                      <div className="mb-1 text-xs font-medium text-red-600">
                        Current:
                      </div>
                      <div className="rounded border-l-2 border-red-200 bg-red-50 p-2 text-sm">
                        {suggestion.original}
                      </div>
                    </div>

                    <div>
                      <div className="mb-1 text-xs font-medium text-green-600">
                        Improved:
                      </div>
                      <div className="rounded border-l-2 border-green-200 bg-green-50 p-2 text-sm">
                        {suggestion.example_improvement}
                      </div>
                    </div>

                    <div className="text-xs text-muted-foreground">
                      <strong>Why this helps:</strong> {suggestion.reasoning}
                    </div>

                    {/* X-Y-Z Breakdown */}
                    {/* eslint-disable @typescript-eslint/prefer-nullish-coalescing */}
                    {(suggestion.suggested_x ||
                      suggestion.suggested_y ||
                      suggestion.suggested_z) && (
                      /* eslint-enable @typescript-eslint/prefer-nullish-coalescing */
                      <div className="grid grid-cols-3 gap-2 border-t pt-3 text-xs">
                        <div className="text-center">
                          <div className="font-medium text-blue-600">
                            X - Action
                          </div>
                          <div className="text-muted-foreground">
                            {suggestion.suggested_x ?? '✓ Good'}
                          </div>
                        </div>
                        <div className="text-center">
                          <div className="font-medium text-green-600">
                            Y - Numbers
                          </div>
                          <div className="text-muted-foreground">
                            {suggestion.suggested_y ?? '✓ Good'}
                          </div>
                        </div>
                        <div className="text-center">
                          <div className="font-medium text-purple-600">
                            Z - Impact
                          </div>
                          <div className="text-muted-foreground">
                            {suggestion.suggested_z ?? '✓ Good'}
                          </div>
                        </div>
                      </div>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* No Suggestions Message */}
        {suggestions && suggestions.suggestions.length === 0 && (
          <div className="py-8 text-center">
            <CheckCircle className="mx-auto mb-4 h-12 w-12 text-green-600" />
            <h3 className="mb-2 text-lg font-semibold text-green-600">
              Excellent Achievement Statements!
            </h3>
            <p className="text-muted-foreground">
              Your {sectionName} section already uses strong action verbs,
              quantification, and impact statements effectively.
            </p>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
