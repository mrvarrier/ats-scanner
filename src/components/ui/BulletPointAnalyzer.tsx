import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Card, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { CheckCircle, XCircle, AlertTriangle, Lightbulb } from 'lucide-react';
import { CommandResult } from '@/types/api';

interface XYZValidationResult {
  has_xyz_formula: boolean;
  xyz_components: {
    action: string | null;
    quantification: string | null;
    impact: string | null;
  };
  strength_score: number;
  suggestions: string[];
}

interface BulletPointAnalyzerProps {
  text: string;
  onAnalysisComplete?: (_result: XYZValidationResult) => void;
}

export function BulletPointAnalyzer({
  text,
  onAnalysisComplete,
}: BulletPointAnalyzerProps) {
  const [analysis, setAnalysis] = useState<XYZValidationResult | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState(false);

  useEffect(() => {
    if (!text.trim()) {
      setAnalysis(null);
      return;
    }

    const analyzeText = async () => {
      setIsAnalyzing(true);
      try {
        const result = await invoke<CommandResult<XYZValidationResult>>(
          'validate_xyz_formula',
          { bulletText: text }
        );

        if (result.success && result.data) {
          setAnalysis(result.data);
          onAnalysisComplete?.(result.data);
        }
      } catch {
        // XYZ validation failed - continue without analysis
      } finally {
        setIsAnalyzing(false);
      }
    };

    const timeoutId = setTimeout(analyzeText, 500); // Debounce analysis
    return () => clearTimeout(timeoutId);
  }, [text, onAnalysisComplete]);

  if (!text.trim()) return null;

  if (isAnalyzing) {
    return (
      <Card className="border-blue-200 bg-blue-50">
        <CardContent className="p-3">
          <div className="flex items-center gap-2 text-blue-600">
            <div className="h-4 w-4 animate-spin rounded-full border-2 border-blue-600 border-t-transparent"></div>
            <span className="text-sm">Analyzing bullet point...</span>
          </div>
        </CardContent>
      </Card>
    );
  }

  if (!analysis) return null;

  const getStatusIcon = () => {
    if (analysis.has_xyz_formula && analysis.strength_score >= 80) {
      return <CheckCircle className="h-4 w-4 text-green-600" />;
    } else if (analysis.strength_score >= 60) {
      return <AlertTriangle className="h-4 w-4 text-yellow-600" />;
    } else {
      return <XCircle className="h-4 w-4 text-red-600" />;
    }
  };

  const getStatusColor = () => {
    if (analysis.has_xyz_formula && analysis.strength_score >= 80) {
      return 'border-green-200 bg-green-50';
    } else if (analysis.strength_score >= 60) {
      return 'border-yellow-200 bg-yellow-50';
    } else {
      return 'border-red-200 bg-red-50';
    }
  };

  const getStatusText = () => {
    if (analysis.has_xyz_formula && analysis.strength_score >= 80) {
      return 'Strong X-Y-Z Formula';
    } else if (analysis.has_xyz_formula) {
      return 'X-Y-Z Formula Detected';
    } else if (analysis.strength_score >= 60) {
      return 'Good Achievement Statement';
    } else {
      return 'Needs Improvement';
    }
  };

  return (
    <Card className={`${getStatusColor()} mb-2`}>
      <CardContent className="p-3">
        {/* Status Header */}
        <div className="mb-2 flex items-center justify-between">
          <div className="flex items-center gap-2">
            {getStatusIcon()}
            <span className="text-sm font-medium">{getStatusText()}</span>
          </div>
          <Badge variant="outline" className="text-xs">
            Score: {Math.round(analysis.strength_score)}%
          </Badge>
        </div>

        {/* XYZ Components */}
        {analysis.has_xyz_formula && (
          <div className="mb-2 grid grid-cols-3 gap-2 text-xs">
            <div className="flex flex-col">
              <span className="font-medium text-gray-600">Action</span>
              <span
                className={
                  analysis.xyz_components.action
                    ? 'text-green-600'
                    : 'text-gray-400'
                }
              >
                {analysis.xyz_components.action ?? 'Missing'}
              </span>
            </div>
            <div className="flex flex-col">
              <span className="font-medium text-gray-600">Quantification</span>
              <span
                className={
                  analysis.xyz_components.quantification
                    ? 'text-green-600'
                    : 'text-gray-400'
                }
              >
                {analysis.xyz_components.quantification ?? 'Missing'}
              </span>
            </div>
            <div className="flex flex-col">
              <span className="font-medium text-gray-600">Impact</span>
              <span
                className={
                  analysis.xyz_components.impact
                    ? 'text-green-600'
                    : 'text-gray-400'
                }
              >
                {analysis.xyz_components.impact ?? 'Missing'}
              </span>
            </div>
          </div>
        )}

        {/* Suggestions */}
        {analysis.suggestions.length > 0 && (
          <div className="mt-2 border-t pt-2">
            <div className="mb-1 flex items-center gap-1">
              <Lightbulb className="h-3 w-3 text-blue-600" />
              <span className="text-xs font-medium text-blue-600">
                Suggestions:
              </span>
            </div>
            <ul className="space-y-1 text-xs text-gray-600">
              {analysis.suggestions.map((suggestion, index) => (
                <li key={index} className="flex items-start gap-1">
                  <span className="mt-0.5 text-blue-400">•</span>
                  {suggestion}
                </li>
              ))}
            </ul>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
