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
    pub word_count: usize,
    pub character_count: usize,
    pub metadata: DocumentMetadata,
    pub structure: Option<DocumentStructure>,
    pub quality_metrics: Option<DocumentQualityMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentMetadata {
    pub creation_date: Option<DateTime<Utc>>,
    pub modification_date: Option<DateTime<Utc>>,
    pub author: Option<String>,
    pub title: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
    pub producer: Option<String>,
    pub creator: Option<String>,
    pub pages: Option<u32>,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentStructure {
    pub sections: Vec<DocumentSection>,
    pub contact_info: DocumentContactInfo,
    pub headings: Vec<DocumentHeading>,
    pub total_sections: usize,
    pub has_consistent_formatting: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentContactInfo {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub linkedin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSection {
    pub name: String,
    pub content: String,
    pub start_position: usize,
    pub end_position: usize,
    pub confidence: f64,
    pub bullet_points: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentHeading {
    pub text: String,
    pub level: u8,
    pub position: usize,
    pub formatting: HeadingFormatting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingFormatting {
    pub is_bold: bool,
    pub is_uppercase: bool,
    pub font_size_relative: Option<f32>,
    pub alignment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentQualityMetrics {
    pub ats_compatibility_score: f64,
    pub readability_score: f64,
    pub formatting_consistency_score: f64,
    pub keyword_density: f64,
    pub section_completeness_score: f64,
    pub contact_info_completeness: f64,
    pub overall_quality_score: f64,
    pub issues: Vec<DocumentIssue>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentIssue {
    pub issue_type: DocumentIssueType,
    pub description: String,
    pub severity: IssueSeverity,
    pub location: Option<String>,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentIssueType {
    Formatting,
    Structure,
    Content,
    AtsCompatibility,
    ContactInfo,
    Spelling,
    Grammar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Low,
    Medium,
    High,
    Critical,
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

// Job Description Management Models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct JobDescription {
    pub id: String,
    pub title: String,
    pub company: String,
    pub content: String,
    pub requirements: String, // JSON array as string
    pub preferred_qualifications: Option<String>, // JSON array as string
    pub salary_range_min: Option<i64>,
    pub salary_range_max: Option<i64>,
    pub salary_currency: Option<String>,
    pub location: String,
    pub remote_options: RemoteWorkType,
    pub employment_type: EmploymentType,
    pub experience_level: ExperienceLevel,
    pub posted_date: Option<DateTime<Utc>>,
    pub application_deadline: Option<DateTime<Utc>>,
    pub job_url: Option<String>,
    pub keywords: String, // JSON array as string
    pub industry: Option<String>,
    pub department: Option<String>,
    pub status: JobStatus,
    pub priority: JobPriority,
    pub notes: Option<String>,
    pub application_status: ApplicationStatus,
    pub application_date: Option<DateTime<Utc>>,
    pub interview_date: Option<DateTime<Utc>>,
    pub response_deadline: Option<DateTime<Utc>>,
    pub contact_person: Option<String>,
    pub contact_email: Option<String>,
    pub tags: String, // JSON array as string
    pub source: JobSource,
    pub is_archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum RemoteWorkType {
    #[default]
    OnSite,
    Remote,
    Hybrid,
    Flexible,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum EmploymentType {
    #[default]
    FullTime,
    PartTime,
    Contract,
    Temporary,
    Internship,
    Freelance,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ExperienceLevel {
    EntryLevel,
    Junior,
    #[default]
    MidLevel,
    Senior,
    Lead,
    Principal,
    Executive,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum JobStatus {
    #[default]
    Draft,
    Active,
    Applied,
    Interviewing,
    Offered,
    Rejected,
    Withdrawn,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum JobPriority {
    Low,
    #[default]
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ApplicationStatus {
    #[default]
    NotApplied,
    Applied,
    ApplicationReviewed,
    PhoneScreen,
    TechnicalInterview,
    OnSiteInterview,
    FinalRound,
    OfferReceived,
    OfferAccepted,
    OfferDeclined,
    Rejected,
    Withdrawn,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum JobSource {
    #[default]
    Manual,
    LinkedIn,
    Indeed,
    CompanyWebsite,
    Referral,
    Recruiter,
    JobBoard,
    URL,
}

impl JobDescription {
    pub fn new(title: String, company: String, content: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            company,
            content,
            requirements: "[]".to_string(),
            preferred_qualifications: None,
            salary_range_min: None,
            salary_range_max: None,
            salary_currency: Some("USD".to_string()),
            location: "".to_string(),
            remote_options: RemoteWorkType::OnSite,
            employment_type: EmploymentType::FullTime,
            experience_level: ExperienceLevel::MidLevel,
            posted_date: None,
            application_deadline: None,
            job_url: None,
            keywords: "[]".to_string(),
            industry: None,
            department: None,
            status: JobStatus::Draft,
            priority: JobPriority::Medium,
            notes: None,
            application_status: ApplicationStatus::NotApplied,
            application_date: None,
            interview_date: None,
            response_deadline: None,
            contact_person: None,
            contact_email: None,
            tags: "[]".to_string(),
            source: JobSource::Manual,
            is_archived: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn from_url(url: String, title: String, company: String, content: String) -> Self {
        let mut job = Self::new(title, company, content);
        job.job_url = Some(url);
        job.source = JobSource::URL;
        job
    }
}

// Job search and filter models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobSearchRequest {
    pub query: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
    pub remote_options: Option<Vec<RemoteWorkType>>,
    pub employment_type: Option<Vec<EmploymentType>>,
    pub experience_level: Option<Vec<ExperienceLevel>>,
    pub salary_min: Option<i64>,
    pub salary_max: Option<i64>,
    pub status: Option<Vec<JobStatus>>,
    pub priority: Option<Vec<JobPriority>>,
    pub application_status: Option<Vec<ApplicationStatus>>,
    pub industry: Option<String>,
    pub tags: Option<Vec<String>>,
    pub posted_after: Option<DateTime<Utc>>,
    pub posted_before: Option<DateTime<Utc>>,
    pub include_archived: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub sort_by: Option<JobSortOption>,
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobSortOption {
    CreatedAt,
    UpdatedAt,
    PostedDate,
    ApplicationDeadline,
    Priority,
    Title,
    Company,
    SalaryMin,
    SalaryMax,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobSearchResult {
    pub jobs: Vec<JobDescription>,
    pub total_count: i64,
    pub has_more: bool,
}

// Job URL extraction models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobUrlExtractionRequest {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobUrlExtractionResult {
    pub title: Option<String>,
    pub company: Option<String>,
    pub content: String,
    pub location: Option<String>,
    pub salary_range: Option<String>,
    pub employment_type: Option<String>,
    pub remote_options: Option<String>,
    pub requirements: Vec<String>,
    pub posted_date: Option<String>,
    pub application_deadline: Option<String>,
    pub success: bool,
    pub error: Option<String>,
}

// Job analytics and insights models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobAnalytics {
    pub total_jobs: i64,
    pub jobs_by_status: Vec<JobStatusCount>,
    pub jobs_by_priority: Vec<JobPriorityCount>,
    pub jobs_by_application_status: Vec<ApplicationStatusCount>,
    pub average_salary_range: Option<SalaryRangeStats>,
    pub top_companies: Vec<CompanyCount>,
    pub top_locations: Vec<LocationCount>,
    pub application_timeline: Vec<ApplicationTimelineEntry>,
    pub success_rate: f64,
    pub response_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobStatusCount {
    pub status: JobStatus,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobPriorityCount {
    pub priority: JobPriority,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationStatusCount {
    pub status: ApplicationStatus,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryRangeStats {
    pub min_avg: f64,
    pub max_avg: f64,
    pub median_min: f64,
    pub median_max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyCount {
    pub company: String,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationCount {
    pub location: String,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationTimelineEntry {
    pub date: DateTime<Utc>,
    pub applications_count: i64,
    pub responses_count: i64,
}

// Job comparison models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobComparisonRequest {
    pub job_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobComparisonResult {
    pub jobs: Vec<JobDescription>,
    pub comparison_matrix: JobComparisonMatrix,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobComparisonMatrix {
    pub salary_comparison: Vec<SalaryComparison>,
    pub location_comparison: Vec<LocationComparison>,
    pub requirements_comparison: RequirementsComparison,
    pub benefits_comparison: Vec<BenefitsComparison>,
    pub match_scores: Vec<JobMatchScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryComparison {
    pub job_id: String,
    pub min_salary: Option<i64>,
    pub max_salary: Option<i64>,
    pub currency: Option<String>,
    pub vs_average: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationComparison {
    pub job_id: String,
    pub location: String,
    pub remote_options: RemoteWorkType,
    pub commute_score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementsComparison {
    pub common_requirements: Vec<String>,
    pub unique_requirements: Vec<JobUniqueRequirements>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobUniqueRequirements {
    pub job_id: String,
    pub requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenefitsComparison {
    pub job_id: String,
    pub benefits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobMatchScore {
    pub job_id: String,
    pub match_score: f64,
    pub match_factors: Vec<MatchFactor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchFactor {
    pub factor: String,
    pub score: f64,
    pub weight: f64,
    pub explanation: String,
}
