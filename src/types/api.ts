// Shared type definitions for API responses and common data structures

export interface CommandResult<T> {
  success: boolean;
  data?: T;
  error?: string;
}

export interface DocumentInfo {
  filename: string;
  content: string;
  file_type: string;
  word_count: number;
  character_count: number;
}

export interface OllamaModel {
  name: string;
  size: number;
  digest: string;
  modified_at: string;
}

export interface CategoryScores {
  [category: string]: number;
}

export interface AnalysisResult {
  overall_score: number;
  category_scores: CategoryScores;
  detailed_feedback: string;
  missing_keywords: string[];
  recommendations: string[];
  processing_time_ms: number;
}

// Achievement Analysis Types
export interface AchievementAnalysis {
  bullet_points: BulletAnalysis[];
  xyz_formula_usage: number;
  achievement_density: number;
  quantification_score: number;
  action_verb_strength: number;
  overall_achievement_score: number;
  suggestions: AchievementSuggestion[];
}

export interface BulletAnalysis {
  text: string;
  section: string;
  has_xyz_structure: boolean;
  action_verb: string;
  quantification: string;
  impact: string;
  strength_score: number;
  suggestions: string[];
}

export interface AchievementSuggestion {
  bullet_point: string;
  suggestion_type: string;
  original: string;
  improved: string;
  explanation: string;
  impact_score: number;
}

// ML Insights Types
export interface MLInsights {
  success_prediction: SuccessPrediction;
  career_path_suggestions: CareerPathSuggestions;
  salary_prediction: SalaryPrediction;
  ml_recommendations: MLRecommendation[];
  feature_analysis: FeatureAnalysis;
  confidence_score: number;
}

export interface SuccessPrediction {
  application_success_probability: number;
  interview_likelihood: number;
  hiring_probability: number;
  factors_analysis: PredictionFactor[];
  risk_factors: string[];
  success_factors: string[];
}

export interface CareerPathSuggestions {
  current_level: string;
  suggested_roles: SuggestedRole[];
  skill_gaps: SkillGap[];
  growth_trajectory: GrowthPath[];
  industry_alignment: number;
}

export interface SalaryPrediction {
  predicted_salary_range: SalaryRange;
  market_percentile: number;
  factors_affecting_salary: SalaryFactor[];
  improvement_potential: number;
  location_adjustments: LocationAdjustment[];
}

export interface SuggestedRole {
  title: string;
  match_score: number;
  requirements_met: number;
  missing_skills: string[];
  salary_range: SalaryRange;
  growth_potential: number;
}

export interface SalaryRange {
  min: number;
  max: number;
  median: number;
}

export interface SkillGap {
  skill: string;
  current_level: number;
  required_level: number;
  learning_resources: string[];
}

export interface GrowthPath {
  role: string;
  timeline: string;
  requirements: string[];
  salary_increase: number;
}

export interface PredictionFactor {
  factor: string;
  impact: number;
  explanation: string;
}

export interface SalaryFactor {
  factor: string;
  impact_percentage: number;
  description: string;
}

export interface LocationAdjustment {
  location: string;
  adjustment_factor: number;
  cost_of_living: number;
}

export interface MLRecommendation {
  category: string;
  priority: string;
  recommendation: string;
  expected_outcome: string;
  implementation_difficulty: string;
  impact_score: number;
}

export interface FeatureAnalysis {
  feature_importance: FeatureImportance[];
  model_accuracy: number;
  prediction_confidence: number;
}

export interface FeatureImportance {
  feature: string;
  importance: number;
  description: string;
}

// User Preferences Types
export interface UserPreferences {
  theme: 'Light' | 'Dark' | 'HighContrast';
  auto_connect_on_startup: boolean;
  default_model?: string;
  analysis_history_retention_days: number;
  notification_preferences: NotificationPreferences;
  privacy_settings: PrivacySettings;
  performance_settings: PerformanceSettings;
}

export interface NotificationPreferences {
  enable_analysis_complete: boolean;
  enable_error_notifications: boolean;
  enable_update_notifications: boolean;
}

export interface PrivacySettings {
  store_analysis_history: boolean;
  anonymous_usage_statistics: boolean;
  auto_clear_uploaded_files: boolean;
}

export interface PerformanceSettings {
  max_concurrent_analyses: number;
  cache_analysis_results: boolean;
  auto_optimize_memory: boolean;
}

// Error Types
export interface AppError {
  code: string;
  message: string;
  details?: Record<string, unknown>;
  timestamp: string;
}

// Analysis Request Types
export interface AnalysisRequest {
  resume_content: string;
  job_description: string;
  model_name: string;
}

export interface OptimizationRequest {
  resume_content: string;
  job_description: string;
  model_name: string;
  optimization_level: 'Basic' | 'Intermediate' | 'Advanced';
}

// Analysis Database Record
export interface Analysis {
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
}
