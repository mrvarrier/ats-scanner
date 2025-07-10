import {
  BarChart3,
  FileText,
  Clock,
  TrendingUp,
  Upload,
  Zap,
  Target,
  Award,
  Activity,
  CheckCircle,
  XCircle,
} from 'lucide-react';
import { useAppStore } from '@/store/useAppStore';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';

export function Dashboard() {
  const { analysisHistory, models, isOllamaConnected, setActiveTab } =
    useAppStore();

  // Calculate dashboard statistics
  const totalAnalyses = analysisHistory.length;
  const avgScore =
    totalAnalyses > 0
      ? analysisHistory.reduce(
          (sum, analysis) => sum + analysis.overall_score,
          0
        ) / totalAnalyses
      : 0;

  const recentAnalyses = analysisHistory.slice(0, 5);
  const highScoreAnalyses = analysisHistory.filter(a => a.overall_score >= 80);

  // Get the latest analysis for recent activity
  const lastWeekAnalyses = analysisHistory.filter(a => {
    const date = new Date(a.created_at);
    const weekAgo = new Date();
    weekAgo.setDate(weekAgo.getDate() - 7);
    return date > weekAgo;
  });

  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <h1 className="text-3xl font-bold tracking-tight">Dashboard</h1>
        <p className="text-muted-foreground">
          Welcome to ATS Scanner. Get insights into your resume optimization
          journey.
        </p>
      </div>

      {/* Connection Status */}
      <Card
        className={`${isOllamaConnected ? 'border-green-200 bg-green-50 dark:border-green-800 dark:bg-green-900/20' : 'border-yellow-200 bg-yellow-50 dark:border-yellow-800 dark:bg-yellow-900/20'}`}
      >
        <CardContent className="flex items-center gap-2 pt-6">
          {isOllamaConnected ? (
            <>
              <CheckCircle className="h-4 w-4 text-green-600" />
              <p className="text-sm text-green-700 dark:text-green-300">
                System ready - Ollama is connected and {models.length} model
                {models.length !== 1 ? 's' : ''} available
              </p>
            </>
          ) : (
            <>
              <XCircle className="h-4 w-4 text-yellow-600" />
              <p className="text-sm text-yellow-700 dark:text-yellow-300">
                Ollama is not connected. Please check your Ollama installation
                to start analyzing resumes.
              </p>
            </>
          )}
        </CardContent>
      </Card>

      {/* Overview Cards */}
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              Total Analyses
            </CardTitle>
            <FileText className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{totalAnalyses}</div>
            <p className="text-xs text-muted-foreground">
              {lastWeekAnalyses.length} this week
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Average Score</CardTitle>
            <BarChart3 className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{avgScore.toFixed(1)}%</div>
            <p className="text-xs text-muted-foreground">Across all analyses</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">High Scores</CardTitle>
            <Award className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{highScoreAnalyses.length}</div>
            <p className="text-xs text-muted-foreground">80%+ compatibility</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              Models Available
            </CardTitle>
            <Activity className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{models.length}</div>
            <p className="text-xs text-muted-foreground">AI models ready</p>
          </CardContent>
        </Card>
      </div>

      {/* Quick Actions */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Target className="h-5 w-5" />
            Quick Actions
          </CardTitle>
          <CardDescription>
            Get started with analyzing and optimizing your resume
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid gap-4 md:grid-cols-3">
            <Button
              onClick={() => setActiveTab('analysis')}
              className="flex h-20 flex-col gap-2"
              variant="outline"
            >
              <Upload className="h-6 w-6" />
              <span>Analyze Resume</span>
            </Button>

            <Button
              onClick={() => setActiveTab('optimization')}
              className="flex h-20 flex-col gap-2"
              variant="outline"
            >
              <Zap className="h-6 w-6" />
              <span>Optimize Content</span>
            </Button>

            <Button
              onClick={() => setActiveTab('results')}
              className="flex h-20 flex-col gap-2"
              variant="outline"
            >
              <BarChart3 className="h-6 w-6" />
              <span>View Results</span>
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Recent Activity */}
      {recentAnalyses.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Clock className="h-5 w-5" />
              Recent Activity
            </CardTitle>
            <CardDescription>Your latest resume analyses</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              {recentAnalyses.map((analysis, index) => (
                <div
                  key={analysis.id || index}
                  className="flex items-center justify-between rounded-lg border p-3"
                >
                  <div className="flex items-center gap-3">
                    <FileText className="h-4 w-4 text-muted-foreground" />
                    <div>
                      <p className="font-medium">
                        Analysis {analysis.id?.slice(0, 8) || `#${index + 1}`}
                      </p>
                      <p className="text-sm text-muted-foreground">
                        {new Date(analysis.created_at).toLocaleDateString()}
                      </p>
                    </div>
                  </div>
                  <div className="flex items-center gap-3">
                    <div className="text-right">
                      <div className="text-lg font-bold">
                        {analysis.overall_score.toFixed(1)}%
                      </div>
                      <div className="text-xs text-muted-foreground">
                        {analysis.model_used}
                      </div>
                    </div>
                    <Progress value={analysis.overall_score} className="w-20" />
                  </div>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      {/* Welcome Message for New Users */}
      {totalAnalyses === 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <TrendingUp className="h-5 w-5" />
              Get Started
            </CardTitle>
          </CardHeader>
          <CardContent className="py-8 text-center">
            <FileText className="mx-auto mb-4 h-16 w-16 text-muted-foreground" />
            <h3 className="mb-2 text-lg font-semibold">
              Welcome to ATS Scanner
            </h3>
            <p className="mx-auto mb-6 max-w-md text-muted-foreground">
              Upload and analyze your first resume to see detailed compatibility
              analysis and optimization suggestions.
            </p>
            <Button onClick={() => setActiveTab('analysis')} size="lg">
              <Upload className="mr-2 h-4 w-4" />
              Start Your First Analysis
            </Button>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
