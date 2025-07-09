use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Resume {
    pub id: String,
    pub filename: String,
    pub content: String,
    pub file_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct Analysis {
    pub id: String,
    pub resume_id: String,
    pub job_description_id: String,
    pub model_used: String,
    pub overall_score: f64,
    pub skills_score: f64,
    pub experience_score: f64,
    pub education_score: f64,
    pub keywords_score: f64,
    pub format_score: f64,
    pub detailed_feedback: String,
    pub missing_keywords: String,
    pub recommendations: String,
    pub processing_time_ms: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaModel {
    pub name: String,
    pub size: i64,
    pub digest: String,
    pub modified_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisRequest {
    pub resume_content: String,
    pub job_description: String,
    pub model_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub overall_score: f64,
    pub category_scores: CategoryScores,
    pub detailed_feedback: String,
    pub missing_keywords: Vec<String>,
    pub recommendations: Vec<String>,
    pub processing_time_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryScores {
    pub skills: f64,
    pub experience: f64,
    pub education: f64,
    pub keywords: f64,
    pub format: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentInfo {
    pub id: String,
    pub filename: String,
    pub file_type: String,
    pub size: usize,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRequest {
    pub resume_content: String,
    pub job_description: String,
    pub model_name: String,
    pub optimization_level: OptimizationLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationLevel {
    Conservative,
    Balanced,
    Aggressive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub optimized_content: String,
    pub changes_made: Vec<OptimizationChange>,
    pub before_score: f64,
    pub after_score: f64,
    pub improvement_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationChange {
    pub section: String,
    pub change_type: String,
    pub original: String,
    pub optimized: String,
    pub impact_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPerformance {
    pub model_name: String,
    pub avg_processing_time_ms: f64,
    pub total_analyses: i64,
    pub avg_accuracy_score: f64,
    pub last_used: DateTime<Utc>,
}

impl Resume {
    pub fn new(filename: String, content: String, file_type: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            filename,
            content,
            file_type,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Analysis {
    pub fn new(
        resume_id: String,
        job_description_id: String,
        model_used: String,
        result: &AnalysisResult,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            resume_id,
            job_description_id,
            model_used,
            overall_score: result.overall_score,
            skills_score: result.category_scores.skills,
            experience_score: result.category_scores.experience,
            education_score: result.category_scores.education,
            keywords_score: result.category_scores.keywords,
            format_score: result.category_scores.format,
            detailed_feedback: result.detailed_feedback.clone(),
            missing_keywords: serde_json::to_string(&result.missing_keywords).unwrap_or_default(),
            recommendations: serde_json::to_string(&result.recommendations).unwrap_or_default(),
            processing_time_ms: result.processing_time_ms,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserPreferences {
    pub id: String,
    pub user_id: String, // Future user system support

    // Ollama Settings
    pub ollama_host: String,
    pub ollama_port: i32,
    pub default_model: Option<String>,
    pub connection_timeout_seconds: i32,
    pub auto_connect_on_startup: bool,

    // Analysis Settings
    pub default_optimization_level: OptimizationLevel,
    pub auto_save_analyses: bool,
    pub analysis_history_retention_days: i32,

    // UI Preferences
    pub theme: ThemePreference,
    pub language: String,
    pub sidebar_collapsed: bool,
    pub show_advanced_features: bool,
    pub animation_speed: AnimationSpeed,

    // Data & Privacy
    pub data_storage_location: Option<String>,
    pub auto_backup_enabled: bool,
    pub backup_frequency_hours: i32,
    pub telemetry_enabled: bool,

    // Notifications
    pub desktop_notifications: bool,
    pub sound_notifications: bool,
    pub email_notifications: bool,
    pub notification_email: Option<String>,

    // Performance
    pub max_concurrent_analyses: i32,
    pub cache_size_mb: i32,
    pub enable_gpu_acceleration: bool,

    // Export Settings
    pub default_export_format: ExportFormat,
    pub include_metadata_in_exports: bool,
    pub compress_exports: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThemePreference {
    Light,
    Dark,
    System,
    HighContrast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationSpeed {
    None,
    Reduced,
    Normal,
    Fast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    Json,
    Csv,
    Pdf,
    Html,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferencesUpdate {
    pub ollama_host: Option<String>,
    pub ollama_port: Option<i32>,
    pub default_model: Option<String>,
    pub connection_timeout_seconds: Option<i32>,
    pub auto_connect_on_startup: Option<bool>,
    pub default_optimization_level: Option<OptimizationLevel>,
    pub auto_save_analyses: Option<bool>,
    pub analysis_history_retention_days: Option<i32>,
    pub theme: Option<ThemePreference>,
    pub language: Option<String>,
    pub sidebar_collapsed: Option<bool>,
    pub show_advanced_features: Option<bool>,
    pub animation_speed: Option<AnimationSpeed>,
    pub data_storage_location: Option<String>,
    pub auto_backup_enabled: Option<bool>,
    pub backup_frequency_hours: Option<i32>,
    pub telemetry_enabled: Option<bool>,
    pub desktop_notifications: Option<bool>,
    pub sound_notifications: Option<bool>,
    pub email_notifications: Option<bool>,
    pub notification_email: Option<String>,
    pub max_concurrent_analyses: Option<i32>,
    pub cache_size_mb: Option<i32>,
    pub enable_gpu_acceleration: Option<bool>,
    pub default_export_format: Option<ExportFormat>,
    pub include_metadata_in_exports: Option<bool>,
    pub compress_exports: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLPredictionResult {
    pub compatibility_prediction: f64,
    pub success_probability: f64,
    pub confidence_interval: (f64, f64),
    pub feature_importance: Vec<FeatureImportance>,
    pub model_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureImportance {
    pub feature_name: String,
    pub importance_score: f64,
    pub contribution: f64,
}

impl Default for UserPreferences {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            user_id: "default".to_string(),

            // Ollama Settings
            ollama_host: "http://localhost".to_string(),
            ollama_port: 11434,
            default_model: None,
            connection_timeout_seconds: 30,
            auto_connect_on_startup: true,

            // Analysis Settings
            default_optimization_level: OptimizationLevel::Balanced,
            auto_save_analyses: true,
            analysis_history_retention_days: 90,

            // UI Preferences
            theme: ThemePreference::Light,
            language: "en".to_string(),
            sidebar_collapsed: false,
            show_advanced_features: false,
            animation_speed: AnimationSpeed::Normal,

            // Data & Privacy
            data_storage_location: None,
            auto_backup_enabled: false,
            backup_frequency_hours: 24,
            telemetry_enabled: false,

            // Notifications
            desktop_notifications: true,
            sound_notifications: false,
            email_notifications: false,
            notification_email: None,

            // Performance
            max_concurrent_analyses: 3,
            cache_size_mb: 256,
            enable_gpu_acceleration: false,

            // Export Settings
            default_export_format: ExportFormat::Json,
            include_metadata_in_exports: true,
            compress_exports: false,

            created_at: now,
            updated_at: now,
        }
    }
}

// Phase 1 Enhanced Models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndustryKeyword {
    pub id: String,
    pub industry: String,
    pub keyword: String,
    pub weight: f64,
    pub category: String,
    pub synonyms: String, // JSON array as string
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ATSCompatibilityRule {
    pub id: String,
    pub ats_system: String,
    pub rule_type: String,
    pub rule_description: String,
    pub penalty_weight: f64,
    pub detection_pattern: String,
    pub suggestion: String,
    pub severity: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ScoringBenchmark {
    pub id: String,
    pub industry: String,
    pub job_level: String,
    pub experience_years: String,
    pub benchmark_type: String,
    pub score_threshold: f64,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserFeedback {
    pub id: String,
    pub analysis_id: String,
    pub user_id: String,
    pub feedback_type: String,
    pub rating: i32,
    pub comment: Option<String>,
    pub helpful_suggestions: String, // JSON array as string
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ModelPerformanceMetrics {
    pub id: String,
    pub model_name: String,
    pub analysis_id: String,
    pub processing_time_ms: i64,
    pub memory_usage_mb: f64,
    pub accuracy_score: f64,
    pub user_satisfaction: Option<f64>,
    pub error_count: i32,
    pub created_at: DateTime<Utc>,
}

// Enhanced Analysis Result with Phase 1 features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedAnalysisResult {
    pub base_result: AnalysisResult,
    pub industry_analysis: IndustryAnalysis,
    pub ats_compatibility: ATSCompatibilityAnalysis,
    pub benchmark_comparison: BenchmarkComparison,
    pub optimization_suggestions: Vec<OptimizationSuggestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryAnalysis {
    pub detected_industry: String,
    pub confidence_score: f64,
    pub keyword_matches: Vec<KeywordMatch>,
    pub industry_score: f64,
    pub trending_skills_present: Vec<String>,
    pub missing_trending_skills: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordMatch {
    pub keyword: String,
    pub found: bool,
    pub weight: f64,
    pub category: String,
    pub synonyms_found: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ATSCompatibilityAnalysis {
    pub overall_compatibility_score: f64,
    pub format_issues: Vec<FormatIssue>,
    pub parsing_warnings: Vec<String>,
    pub recommendations: Vec<String>,
    pub ats_friendly_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatIssue {
    pub issue_type: String,
    pub description: String,
    pub severity: String,
    pub suggestion: String,
    pub penalty_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkComparison {
    pub industry_benchmark: f64,
    pub experience_level_benchmark: f64,
    pub performance_percentile: f64,
    pub areas_above_benchmark: Vec<String>,
    pub areas_below_benchmark: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    pub category: String,
    pub priority: String,
    pub description: String,
    pub expected_impact: f64,
    pub implementation_difficulty: String,
    pub specific_examples: Vec<String>,
}

// Configuration Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub ollama_config: OllamaConfig,
    pub analysis_config: AnalysisConfig,
    pub performance_config: PerformanceConfig,
    pub logging_config: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    pub host: String,
    pub port: u16,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub default_model: String,
    pub models: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub enable_industry_analysis: bool,
    pub enable_ats_compatibility: bool,
    pub enable_benchmark_comparison: bool,
    pub default_optimization_level: OptimizationLevel,
    pub max_suggestions: usize,
    pub confidence_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub max_concurrent_analyses: usize,
    pub cache_size_mb: usize,
    pub enable_gpu_acceleration: bool,
    pub memory_limit_mb: usize,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file_path: Option<String>,
    pub enable_telemetry: bool,
    pub enable_performance_metrics: bool,
}
