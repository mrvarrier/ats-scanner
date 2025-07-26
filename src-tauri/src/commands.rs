use anyhow::Result;
use log::{error, info};
use serde::Serialize;
use std::path::Path;
use tauri::{Manager, State};

use crate::models::{
    ATSCompatibilityRule, Analysis, AnalysisRequest, AnalysisResult, DocumentInfo, IndustryKeyword,
    JobAnalytics, JobComparisonRequest, JobComparisonResult, JobDescription, JobSearchRequest,
    JobSearchResult, JobUrlExtractionRequest, JobUrlExtractionResult, ModelPerformance,
    ModelPerformanceMetrics, OptimizationRequest, OptimizationResult, Resume, ScoringBenchmark,
    UserFeedback, UserPreferences, UserPreferencesUpdate,
};
// Phase 2 imports
use crate::ats_simulator::{ATSSimulationResult, ATSSimulator};
use crate::enhanced_prompts::{
    EnhancedPromptEngine, EnhancedPromptRequest, EnhancedPromptResponse,
};
use crate::enhanced_scoring::{EnhancedAnalysisResult, EnhancedScoringEngine};
use crate::industry_analyzer::{IndustryAnalysisResult, IndustryAnalyzer};
use crate::semantic_analyzer::{SemanticAnalysisResult, SemanticAnalyzer};
// Phase 3 imports
use crate::format_checker::{FormatCompatibilityChecker, FormatCompatibilityReport};
use crate::format_issue_detector::{FormatIssueDetector, FormatIssueReport};
use crate::testing_framework::{ATSTestingFramework, ValidationReport};
// Phase 4 imports
use crate::achievement_analyzer::{AchievementAnalysis, AchievementAnalyzer};
use crate::realtime_optimizer::{LiveSuggestions, RealtimeOptimizer};
use crate::smart_optimizer::{
    ComprehensiveOptimization, OptimizationLevel, SmartOptimizationEngine,
};
// Phase 5 imports
use crate::competitive_analyzer::{CompetitiveAnalysis, CompetitiveAnalyzer};
// Phase 6 imports
use crate::document::DocumentParser;
use crate::ml_insights::{MLInsights, MLInsightsEngine};
use crate::modern_keyword_extractor::ExtractionResult;
use crate::ollama::OllamaClient;
use crate::plugin_system::{PluginExecutionResult, PluginInfo, PluginManager};
use crate::scoring::AnalysisEngine;
use crate::utils::{export_data, security};
use crate::AppState;
// Advanced Scoring Engine
use crate::advanced_scoring::{
    AdvancedScoringEngine, EnhancedAnalysisResult as AdvancedAnalysisResult,
};

// Frontend-compatible achievement analysis structures
#[derive(Debug, Serialize)]
pub struct FrontendAchievementAnalysis {
    pub bullet_points: Vec<FrontendBulletAnalysis>,
    pub xyz_formula_usage: f64,
    pub achievement_density: f64,
    pub quantification_score: f64,
    pub action_verb_strength: f64,
    pub overall_achievement_score: f64,
    pub suggestions: Vec<FrontendAchievementSuggestion>,
}

#[derive(Debug, Serialize)]
pub struct FrontendBulletAnalysis {
    pub text: String,
    pub section: String,
    pub has_xyz_structure: bool,
    pub action_verb: String,
    pub quantification: String,
    pub impact: String,
    pub strength_score: f64,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct FrontendAchievementSuggestion {
    pub bullet_point: String,
    pub suggestion_type: String,
    pub original: String,
    pub improved: String,
    pub explanation: String,
    pub impact_score: f64,
}

fn calculate_achievement_density(analysis: &AchievementAnalysis) -> f64 {
    let total_bullets = analysis.achievement_distribution.total_bullets as f64;
    if total_bullets == 0.0 {
        return 0.0;
    }
    (analysis.achievement_distribution.strong_achievements as f64 / total_bullets) * 100.0
}

#[derive(Debug, Serialize)]
pub struct CommandResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> CommandResult<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_ollama_models() -> CommandResult<Vec<crate::models::OllamaModel>> {
    info!("Getting Ollama models");

    let ollama_client = match OllamaClient::new(None) {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to create Ollama client: {}", e);
            return CommandResult::error(format!("Failed to create Ollama client: {}", e));
        }
    };

    match ollama_client.list_models().await {
        Ok(models) => {
            info!("Successfully retrieved {} models", models.len());
            CommandResult::success(models)
        }
        Err(e) => {
            error!("Failed to get Ollama models: {}", e);
            CommandResult::error(format!("Failed to get Ollama models: {}", e))
        }
    }
}

#[tauri::command]
pub async fn test_ollama_connection() -> CommandResult<bool> {
    info!("Testing Ollama connection");

    let ollama_client = match OllamaClient::new(None) {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to create Ollama client: {}", e);
            return CommandResult::error(format!("Failed to create Ollama client: {}", e));
        }
    };

    match ollama_client.test_connection().await {
        Ok(connected) => {
            info!("Ollama connection test result: {}", connected);
            CommandResult::success(connected)
        }
        Err(e) => {
            error!("Ollama connection test failed: {}", e);
            CommandResult::error(format!("Connection test failed: {}", e))
        }
    }
}

#[tauri::command]
pub async fn ollama_health_check() -> CommandResult<bool> {
    let ollama_client = match OllamaClient::new(None) {
        Ok(client) => client,
        Err(_) => return CommandResult::success(false), // Silent failure for health checks
    };

    match ollama_client.health_check().await {
        Ok(healthy) => CommandResult::success(healthy),
        Err(_) => CommandResult::success(false), // Silent failure for health checks
    }
}

#[tauri::command]
pub async fn parse_document(file_path: String) -> CommandResult<DocumentInfo> {
    info!("Parsing document: {}", file_path);

    // SECURITY: Validate file path to prevent path traversal attacks
    if let Err(e) = security::validate_file_path(&file_path, None) {
        error!(
            "Security violation: Invalid file path '{}': {}",
            file_path, e
        );
        return CommandResult::error("Invalid file path".to_string());
    }

    if !Path::new(&file_path).exists() {
        return CommandResult::error("File does not exist".to_string());
    }

    match DocumentParser::parse_file(&file_path).await {
        Ok(document_info) => {
            info!("Successfully parsed document: {}", document_info.filename);
            CommandResult::success(document_info)
        }
        Err(e) => {
            error!("Failed to parse document: {}", e);
            CommandResult::error(format!("Failed to parse document: {}", e))
        }
    }
}

#[tauri::command]
pub async fn parse_document_with_metadata(file_path: String) -> CommandResult<DocumentInfo> {
    info!("Parsing document with full metadata: {}", file_path);

    // SECURITY: Validate file path to prevent path traversal attacks
    if let Err(e) = security::validate_file_path(&file_path, None) {
        error!(
            "Security violation: Invalid file path '{}': {}",
            file_path, e
        );
        return CommandResult::error("Invalid file path".to_string());
    }

    if !Path::new(&file_path).exists() {
        return CommandResult::error("File does not exist".to_string());
    }

    match DocumentParser::parse_file(&file_path).await {
        Ok(document_info) => {
            info!(
                "Successfully parsed document with metadata: {} (Quality Score: {:.1})",
                document_info.filename,
                document_info
                    .quality_metrics
                    .as_ref()
                    .map(|q| q.overall_quality_score)
                    .unwrap_or(0.0)
            );
            CommandResult::success(document_info)
        }
        Err(e) => {
            error!("Failed to parse document with metadata: {}", e);
            CommandResult::error(format!("Failed to parse document with metadata: {}", e))
        }
    }
}

#[tauri::command]
pub async fn extract_document_structure(
    file_path: String,
) -> CommandResult<crate::models::DocumentStructure> {
    info!("Extracting document structure: {}", file_path);

    // SECURITY: Validate file path to prevent path traversal attacks
    if let Err(e) = security::validate_file_path(&file_path, None) {
        error!(
            "Security violation: Invalid file path '{}': {}",
            file_path, e
        );
        return CommandResult::error("Invalid file path".to_string());
    }

    if !Path::new(&file_path).exists() {
        return CommandResult::error("File does not exist".to_string());
    }

    match DocumentParser::parse_file(&file_path).await {
        Ok(document_info) => {
            if let Some(structure) = document_info.structure {
                info!(
                    "Successfully extracted structure: {} sections, {} headings",
                    structure.total_sections,
                    structure.headings.len()
                );
                CommandResult::success(structure)
            } else {
                CommandResult::error("No document structure could be extracted".to_string())
            }
        }
        Err(e) => {
            error!("Failed to extract document structure: {}", e);
            CommandResult::error(format!("Failed to extract document structure: {}", e))
        }
    }
}

#[tauri::command]
pub async fn analyze_document_quality(
    file_path: String,
) -> CommandResult<crate::models::DocumentQualityMetrics> {
    info!("Analyzing document quality: {}", file_path);

    // SECURITY: Validate file path to prevent path traversal attacks
    if let Err(e) = security::validate_file_path(&file_path, None) {
        error!(
            "Security violation: Invalid file path '{}': {}",
            file_path, e
        );
        return CommandResult::error("Invalid file path".to_string());
    }

    if !Path::new(&file_path).exists() {
        return CommandResult::error("File does not exist".to_string());
    }

    match DocumentParser::parse_file(&file_path).await {
        Ok(document_info) => {
            if let Some(quality_metrics) = document_info.quality_metrics {
                info!(
                    "Document quality analysis completed - Overall Score: {:.1}, ATS Score: {:.1}",
                    quality_metrics.overall_quality_score, quality_metrics.ats_compatibility_score
                );
                CommandResult::success(quality_metrics)
            } else {
                CommandResult::error("No quality metrics could be calculated".to_string())
            }
        }
        Err(e) => {
            error!("Failed to analyze document quality: {}", e);
            CommandResult::error(format!("Failed to analyze document quality: {}", e))
        }
    }
}

#[tauri::command]
pub async fn get_document_metadata(
    file_path: String,
) -> CommandResult<crate::models::DocumentMetadata> {
    info!("Extracting document metadata: {}", file_path);

    // SECURITY: Validate file path to prevent path traversal attacks
    if let Err(e) = security::validate_file_path(&file_path, None) {
        error!(
            "Security violation: Invalid file path '{}': {}",
            file_path, e
        );
        return CommandResult::error("Invalid file path".to_string());
    }

    if !Path::new(&file_path).exists() {
        return CommandResult::error("File does not exist".to_string());
    }

    match DocumentParser::parse_file(&file_path).await {
        Ok(document_info) => {
            info!(
                "Successfully extracted metadata for: {} (Created: {:?})",
                document_info.filename, document_info.metadata.creation_date
            );
            CommandResult::success(document_info.metadata)
        }
        Err(e) => {
            error!("Failed to extract document metadata: {}", e);
            CommandResult::error(format!("Failed to extract document metadata: {}", e))
        }
    }
}

#[tauri::command]
pub async fn save_resume(
    state: State<'_, AppState>,
    filename: String,
    content: String,
    file_type: String,
) -> Result<CommandResult<String>, String> {
    info!("Saving resume: {}", filename);

    let resume = Resume::new(filename, content, file_type);
    let resume_id = resume.id.clone();
    let db = state.db.lock().await;

    match db.save_resume(&resume).await {
        Ok(()) => {
            info!("Resume saved with ID: {}", resume_id);
            Ok(CommandResult::success(resume_id))
        }
        Err(e) => {
            error!("Failed to save resume: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to save resume: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_all_resumes(
    state: State<'_, AppState>,
) -> Result<CommandResult<Vec<Resume>>, String> {
    info!("Getting all resumes");

    let db = state.db.lock().await;

    match db.get_all_resumes().await {
        Ok(resumes) => {
            info!("Retrieved {} resumes", resumes.len());
            Ok(CommandResult::success(resumes))
        }
        Err(e) => {
            error!("Failed to get resumes: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get resumes: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_resume(
    state: State<'_, AppState>,
    id: String,
) -> Result<CommandResult<Option<Resume>>, String> {
    info!("Getting resume with ID: {}", id);

    let db = state.db.lock().await;

    match db.get_resume(&id).await {
        Ok(resume) => {
            info!("Retrieved resume: {:?}", resume.is_some());
            Ok(CommandResult::success(resume))
        }
        Err(e) => {
            error!("Failed to get resume: {}", e);
            Ok(CommandResult::error(format!("Failed to get resume: {}", e)))
        }
    }
}

#[tauri::command]
pub async fn delete_resume(
    state: State<'_, AppState>,
    id: String,
) -> Result<CommandResult<bool>, String> {
    info!("Deleting resume with ID: {}", id);

    let db = state.db.lock().await;

    match db.delete_resume(&id).await {
        Ok(()) => {
            info!("Resume deleted successfully");
            Ok(CommandResult::success(true))
        }
        Err(e) => {
            error!("Failed to delete resume: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to delete resume: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn analyze_resume(
    request: AnalysisRequest,
    state: State<'_, AppState>,
) -> Result<CommandResult<AnalysisResult>, String> {
    info!("Analyzing resume with model: {}", request.model_name);

    let ollama_client = match OllamaClient::new(None) {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to create Ollama client: {}", e);
            return Ok(CommandResult::error(format!(
                "Failed to create Ollama client: {}",
                e
            )));
        }
    };
    let analysis_engine = AnalysisEngine::new(ollama_client);

    match analysis_engine
        .analyze_resume(
            &request.resume_content,
            &request.job_description,
            &request.model_name,
        )
        .await
    {
        Ok(result) => {
            info!(
                "Resume analysis completed with score: {:.1}",
                result.overall_score
            );

            // Save to database
            let db = state.db.lock().await;

            // Create and save resume
            let resume = Resume::new(
                "temp_resume.txt".to_string(),
                request.resume_content,
                "txt".to_string(),
            );
            if let Err(e) = db.save_resume(&resume).await {
                error!("Failed to save resume: {}", e);
            }

            // Create and save analysis
            let analysis = Analysis::new(
                resume.id,
                "temp_job_id".to_string(),
                request.model_name,
                &result,
            );
            if let Err(e) = db.save_analysis(&analysis).await {
                error!("Failed to save analysis: {}", e);
            }

            Ok(CommandResult::success(result))
        }
        Err(e) => {
            error!("Resume analysis failed: {}", e);
            Ok(CommandResult::error(format!("Analysis failed: {}", e)))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_analysis_history(
    limit: Option<i64>,
    state: State<'_, AppState>,
) -> Result<CommandResult<Vec<Analysis>>, String> {
    info!("Getting analysis history with limit: {:?}", limit);

    let db = state.db.lock().await;

    match db.get_analysis_history(limit).await {
        Ok(analyses) => {
            info!("Retrieved {} analyses from history", analyses.len());
            Ok(CommandResult::success(analyses))
        }
        Err(e) => {
            error!("Failed to get analysis history: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get history: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn delete_analysis(
    state: State<'_, AppState>,
    id: String,
) -> Result<CommandResult<bool>, String> {
    info!("Deleting analysis with ID: {}", id);

    let db = state.db.lock().await;

    match db.delete_analysis(&id).await {
        Ok(()) => {
            info!("Analysis deleted successfully");
            Ok(CommandResult::success(true))
        }
        Err(e) => {
            error!("Failed to delete analysis: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to delete analysis: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn optimize_resume(request: OptimizationRequest) -> CommandResult<OptimizationResult> {
    info!(
        "Optimizing resume with level: {:?}",
        request.optimization_level
    );

    let ollama_client = match OllamaClient::new(None) {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to create Ollama client: {}", e);
            return CommandResult::error(format!("Failed to create Ollama client: {}", e));
        }
    };
    let analysis_engine = AnalysisEngine::new(ollama_client);

    let optimization_level = match request.optimization_level {
        crate::models::OptimizationLevel::Conservative => "conservative",
        crate::models::OptimizationLevel::Balanced => "balanced",
        crate::models::OptimizationLevel::Aggressive => "aggressive",
    };

    match analysis_engine
        .optimize_resume(
            &request.resume_content,
            &request.job_description,
            &request.model_name,
            optimization_level,
        )
        .await
    {
        Ok(result) => {
            info!(
                "Resume optimization completed: {:.1}% improvement",
                result.improvement_percentage
            );
            CommandResult::success(result)
        }
        Err(e) => {
            error!("Resume optimization failed: {}", e);
            CommandResult::error(format!("Optimization failed: {}", e))
        }
    }
}

#[tauri::command]
pub async fn export_results(
    analysis_ids: Vec<String>,
    format: String,
    state: State<'_, AppState>,
) -> Result<CommandResult<String>, String> {
    info!(
        "Exporting {} analyses in {} format",
        analysis_ids.len(),
        format
    );

    let db = state.db.lock().await;
    let mut analyses = Vec::new();

    for analysis_id in analysis_ids {
        // Note: You'd need to implement get_analysis_by_id in the database module
        // For now, we'll get all analyses and filter (not efficient, but works for demo)
        if let Ok(all_analyses) = db.get_analysis_history(None).await {
            if let Some(analysis) = all_analyses.into_iter().find(|a| a.id == analysis_id) {
                analyses.push(analysis);
            }
        }
    }

    match export_data(&analyses, &format).await {
        Ok(file_path) => {
            info!("Successfully exported results to: {}", file_path);
            Ok(CommandResult::success(file_path))
        }
        Err(e) => {
            error!("Failed to export results: {}", e);
            Ok(CommandResult::error(format!("Export failed: {}", e)))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_model_performance(
    state: State<'_, AppState>,
) -> Result<CommandResult<Vec<ModelPerformance>>, String> {
    info!("Getting model performance statistics");

    let db = state.db.lock().await;

    match db.get_analysis_history(None).await {
        Ok(analyses) => {
            let mut model_stats = std::collections::HashMap::new();

            for analysis in analyses {
                let entry = model_stats.entry(analysis.model_used.clone()).or_insert((
                    0,
                    0i64,
                    0.0,
                    analysis.created_at,
                ));
                entry.0 += 1; // count
                entry.1 += analysis.processing_time_ms; // total time
                entry.2 += analysis.overall_score; // total score
                if analysis.created_at > entry.3 {
                    entry.3 = analysis.created_at; // last used
                }
            }

            let performance: Vec<ModelPerformance> = model_stats
                .into_iter()
                .map(
                    |(model_name, (count, total_time, total_score, last_used))| ModelPerformance {
                        model_name,
                        avg_processing_time_ms: total_time as f64 / count as f64,
                        total_analyses: count,
                        avg_accuracy_score: total_score / count as f64,
                        last_used,
                    },
                )
                .collect();

            info!(
                "Retrieved performance data for {} models",
                performance.len()
            );
            Ok(CommandResult::success(performance))
        }
        Err(e) => {
            error!("Failed to get model performance: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get performance data: {}",
                e
            )))
        }
    }
}

// Analytics Commands

#[allow(dead_code)]
#[tauri::command]
pub async fn get_analysis_stats(
    state: State<'_, AppState>,
    days: Option<i32>,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!("Getting analysis stats for {} days", days.unwrap_or(30));

    match state.db.lock().await.get_analysis_stats(days).await {
        Ok(stats) => {
            info!("Retrieved analysis stats successfully");
            Ok(CommandResult::success(stats))
        }
        Err(e) => {
            error!("Failed to get analysis stats: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get analysis stats: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_score_distribution(
    state: State<'_, AppState>,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!("Getting score distribution");

    match state.db.lock().await.get_score_distribution().await {
        Ok(distribution) => {
            info!("Retrieved score distribution successfully");
            Ok(CommandResult::success(distribution))
        }
        Err(e) => {
            error!("Failed to get score distribution: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get score distribution: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_improvement_trends(
    state: State<'_, AppState>,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!("Getting improvement trends");

    match state.db.lock().await.get_improvement_trends().await {
        Ok(trends) => {
            info!("Retrieved improvement trends successfully");
            Ok(CommandResult::success(trends))
        }
        Err(e) => {
            error!("Failed to get improvement trends: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get improvement trends: {}",
                e
            )))
        }
    }
}

// User Preferences Commands

#[allow(dead_code)]
#[tauri::command]
pub async fn get_user_preferences(
    state: State<'_, AppState>,
    user_id: Option<String>,
) -> Result<CommandResult<UserPreferences>, ()> {
    let user_id = user_id.unwrap_or_else(|| "default".to_string());
    info!("Getting user preferences for user: {}", user_id);

    match state
        .db
        .lock()
        .await
        .get_or_create_user_preferences(&user_id)
        .await
    {
        Ok(preferences) => {
            info!("Retrieved user preferences successfully");
            Ok(CommandResult::success(preferences))
        }
        Err(e) => {
            error!("Failed to get user preferences: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get user preferences: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn update_user_preferences(
    state: State<'_, AppState>,
    user_id: Option<String>,
    updates: UserPreferencesUpdate,
) -> Result<CommandResult<UserPreferences>, ()> {
    let user_id = user_id.unwrap_or_else(|| "default".to_string());
    info!("Updating user preferences for user: {}", user_id);

    match state
        .db
        .lock()
        .await
        .update_user_preferences(&user_id, &updates)
        .await
    {
        Ok(_) => match state.db.lock().await.get_user_preferences(&user_id).await {
            Ok(Some(preferences)) => {
                info!("User preferences updated successfully");
                Ok(CommandResult::success(preferences))
            }
            Ok(None) => {
                error!("Failed to retrieve updated preferences");
                Ok(CommandResult::error(
                    "Failed to retrieve updated preferences".to_string(),
                ))
            }
            Err(e) => {
                error!("Failed to retrieve updated preferences: {}", e);
                Ok(CommandResult::error(format!(
                    "Failed to retrieve updated preferences: {}",
                    e
                )))
            }
        },
        Err(e) => {
            error!("Failed to update user preferences: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to update user preferences: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn reset_user_preferences(
    state: State<'_, AppState>,
    user_id: Option<String>,
) -> Result<CommandResult<UserPreferences>, ()> {
    let user_id = user_id.unwrap_or_else(|| "default".to_string());
    info!("Resetting user preferences for user: {}", user_id);

    let defaults = UserPreferences {
        user_id: user_id.clone(),
        ..Default::default()
    };

    match state.db.lock().await.save_user_preferences(&defaults).await {
        Ok(_) => {
            info!("User preferences reset successfully");
            Ok(CommandResult::success(defaults))
        }
        Err(e) => {
            error!("Failed to reset user preferences: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to reset user preferences: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn export_user_preferences(
    state: State<'_, AppState>,
    user_id: Option<String>,
) -> Result<CommandResult<String>, ()> {
    let user_id = user_id.unwrap_or_else(|| "default".to_string());
    info!("Exporting user preferences for user: {}", user_id);

    match state.db.lock().await.get_user_preferences(&user_id).await {
        Ok(Some(preferences)) => match serde_json::to_string_pretty(&preferences) {
            Ok(json_string) => {
                info!("User preferences exported successfully");
                Ok(CommandResult::success(json_string))
            }
            Err(e) => {
                error!("Failed to serialize preferences: {}", e);
                Ok(CommandResult::error(format!(
                    "Failed to serialize preferences: {}",
                    e
                )))
            }
        },
        Ok(None) => {
            error!("User preferences not found");
            Ok(CommandResult::error(
                "User preferences not found".to_string(),
            ))
        }
        Err(e) => {
            error!("Failed to export user preferences: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to export user preferences: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn import_user_preferences(
    state: State<'_, AppState>,
    user_id: Option<String>,
    preferences_json: String,
) -> Result<CommandResult<UserPreferences>, ()> {
    let user_id = user_id.unwrap_or_else(|| "default".to_string());
    info!("Importing user preferences for user: {}", user_id);

    match serde_json::from_str::<UserPreferences>(&preferences_json) {
        Ok(mut preferences) => {
            preferences.user_id = user_id.clone();
            preferences.updated_at = chrono::Utc::now();

            match state
                .db
                .lock()
                .await
                .save_user_preferences(&preferences)
                .await
            {
                Ok(_) => {
                    info!("User preferences imported successfully");
                    Ok(CommandResult::success(preferences))
                }
                Err(e) => {
                    error!("Failed to save imported preferences: {}", e);
                    Ok(CommandResult::error(format!(
                        "Failed to save imported preferences: {}",
                        e
                    )))
                }
            }
        }
        Err(e) => {
            error!("Failed to parse preferences JSON: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to parse preferences JSON: {}",
                e
            )))
        }
    }
}

// Plugin System Commands

#[tauri::command]
pub async fn list_plugins(
    _state: State<'_, AppState>,
) -> Result<CommandResult<Vec<PluginInfo>>, ()> {
    info!("Listing available plugins");

    let plugins_dir = match std::env::current_dir() {
        Ok(dir) => dir.join("plugins"),
        Err(e) => {
            error!("Failed to get current directory: {}", e);
            return Ok(CommandResult::error(
                "Failed to access plugins directory".to_string(),
            ));
        }
    };
    let plugin_manager = PluginManager::new(plugins_dir).await;

    let plugins = plugin_manager.list_plugins().await;
    info!("Found {} plugins", plugins.len());

    Ok(CommandResult::success(plugins))
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_plugin_info(
    _state: State<'_, AppState>,
    plugin_id: String,
) -> Result<CommandResult<Option<PluginInfo>>, ()> {
    info!("Getting plugin info for: {}", plugin_id);

    let plugins_dir = match std::env::current_dir() {
        Ok(dir) => dir.join("plugins"),
        Err(e) => {
            error!("Failed to get current directory: {}", e);
            return Ok(CommandResult::error(
                "Failed to access plugins directory".to_string(),
            ));
        }
    };
    let plugin_manager = PluginManager::new(plugins_dir).await;

    let plugin_info = plugin_manager.get_plugin_info(&plugin_id).await;

    Ok(CommandResult::success(plugin_info))
}

#[tauri::command]
pub async fn execute_plugin(
    _state: State<'_, AppState>,
    plugin_id: String,
    operation: String,
    input_data: serde_json::Value,
) -> Result<CommandResult<PluginExecutionResult>, ()> {
    info!(
        "Executing plugin {} with operation {}",
        plugin_id, operation
    );

    let plugins_dir = match std::env::current_dir() {
        Ok(dir) => dir.join("plugins"),
        Err(e) => {
            error!("Failed to get current directory: {}", e);
            return Ok(CommandResult::error(
                "Failed to access plugins directory".to_string(),
            ));
        }
    };
    let plugin_manager = PluginManager::new(plugins_dir).await;

    match plugin_manager
        .execute_plugin(&plugin_id, &operation, input_data)
        .await
    {
        Ok(result) => {
            info!("Plugin execution completed successfully");
            Ok(CommandResult::success(result))
        }
        Err(e) => {
            error!("Plugin execution failed: {}", e);
            Ok(CommandResult::error(format!(
                "Plugin execution failed: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn update_plugin_config(
    _state: State<'_, AppState>,
    plugin_id: String,
    config: serde_json::Value,
) -> Result<CommandResult<String>, ()> {
    info!("Updating config for plugin: {}", plugin_id);

    let plugins_dir = match std::env::current_dir() {
        Ok(dir) => dir.join("plugins"),
        Err(e) => {
            error!("Failed to get current directory: {}", e);
            return Ok(CommandResult::error(
                "Failed to access plugins directory".to_string(),
            ));
        }
    };
    let plugin_manager = PluginManager::new(plugins_dir).await;

    match plugin_manager
        .update_plugin_config(&plugin_id, config)
        .await
    {
        Ok(_) => {
            info!("Plugin config updated successfully");
            Ok(CommandResult::success(
                "Config updated successfully".to_string(),
            ))
        }
        Err(e) => {
            error!("Failed to update plugin config: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to update plugin config: {}",
                e
            )))
        }
    }
}

// === PHASE 1 ENHANCED COMMANDS ===

#[allow(dead_code)]
#[tauri::command]
pub async fn get_industry_keywords(
    state: State<'_, AppState>,
    industry: String,
) -> Result<CommandResult<Vec<IndustryKeyword>>, ()> {
    info!("Getting industry keywords for: {}", industry);

    let db = state.db.lock().await;
    match db.get_industry_keywords(&industry).await {
        Ok(keywords) => Ok(CommandResult::success(keywords)),
        Err(e) => {
            error!("Failed to get industry keywords: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get industry keywords: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_all_industries(
    state: State<'_, AppState>,
) -> Result<CommandResult<Vec<String>>, ()> {
    info!("Getting all industries");

    let db = state.db.lock().await;
    match db.get_all_industries().await {
        Ok(industries) => Ok(CommandResult::success(industries)),
        Err(e) => {
            error!("Failed to get industries: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get industries: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn save_industry_keyword(
    state: State<'_, AppState>,
    keyword: IndustryKeyword,
) -> Result<CommandResult<String>, ()> {
    info!("Saving industry keyword: {}", keyword.keyword);

    let db = state.db.lock().await;
    match db.save_industry_keyword(&keyword).await {
        Ok(_) => Ok(CommandResult::success(
            "Keyword saved successfully".to_string(),
        )),
        Err(e) => {
            error!("Failed to save industry keyword: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to save keyword: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_ats_rules(
    state: State<'_, AppState>,
    ats_system: Option<String>,
) -> Result<CommandResult<Vec<ATSCompatibilityRule>>, ()> {
    info!("Getting ATS rules for system: {:?}", ats_system);

    let db = state.db.lock().await;
    match db.get_ats_rules(ats_system.as_deref()).await {
        Ok(rules) => Ok(CommandResult::success(rules)),
        Err(e) => {
            error!("Failed to get ATS rules: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get ATS rules: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn save_ats_rule(
    state: State<'_, AppState>,
    rule: ATSCompatibilityRule,
) -> Result<CommandResult<String>, ()> {
    info!("Saving ATS rule: {}", rule.rule_type);

    let db = state.db.lock().await;
    match db.save_ats_rule(&rule).await {
        Ok(_) => Ok(CommandResult::success(
            "ATS rule saved successfully".to_string(),
        )),
        Err(e) => {
            error!("Failed to save ATS rule: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to save ATS rule: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_scoring_benchmarks(
    state: State<'_, AppState>,
    industry: String,
    job_level: String,
) -> Result<CommandResult<Vec<ScoringBenchmark>>, ()> {
    info!(
        "Getting scoring benchmarks for {} level in {}",
        job_level, industry
    );

    let db = state.db.lock().await;
    match db.get_scoring_benchmarks(&industry, &job_level).await {
        Ok(benchmarks) => Ok(CommandResult::success(benchmarks)),
        Err(e) => {
            error!("Failed to get scoring benchmarks: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get benchmarks: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn save_scoring_benchmark(
    state: State<'_, AppState>,
    benchmark: ScoringBenchmark,
) -> Result<CommandResult<String>, ()> {
    info!("Saving scoring benchmark: {}", benchmark.benchmark_type);

    let db = state.db.lock().await;
    match db.save_scoring_benchmark(&benchmark).await {
        Ok(_) => Ok(CommandResult::success(
            "Benchmark saved successfully".to_string(),
        )),
        Err(e) => {
            error!("Failed to save benchmark: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to save benchmark: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn save_user_feedback(
    state: State<'_, AppState>,
    feedback: UserFeedback,
) -> Result<CommandResult<String>, ()> {
    info!(
        "Saving user feedback for analysis: {}",
        feedback.analysis_id
    );

    let db = state.db.lock().await;
    match db.save_user_feedback(&feedback).await {
        Ok(_) => Ok(CommandResult::success(
            "Feedback saved successfully".to_string(),
        )),
        Err(e) => {
            error!("Failed to save feedback: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to save feedback: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_feedback_by_analysis(
    state: State<'_, AppState>,
    analysis_id: String,
) -> Result<CommandResult<Vec<UserFeedback>>, ()> {
    info!("Getting feedback for analysis: {}", analysis_id);

    let db = state.db.lock().await;
    match db.get_feedback_by_analysis(&analysis_id).await {
        Ok(feedback) => Ok(CommandResult::success(feedback)),
        Err(e) => {
            error!("Failed to get feedback: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get feedback: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_feedback_stats(
    state: State<'_, AppState>,
    days: Option<i32>,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!("Getting feedback stats for {} days", days.unwrap_or(30));

    let db = state.db.lock().await;
    match db.get_feedback_stats(days).await {
        Ok(stats) => Ok(CommandResult::success(stats)),
        Err(e) => {
            error!("Failed to get feedback stats: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get feedback stats: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn save_model_performance(
    state: State<'_, AppState>,
    metrics: ModelPerformanceMetrics,
) -> Result<CommandResult<String>, ()> {
    info!(
        "Saving performance metrics for model: {}",
        metrics.model_name
    );

    let db = state.db.lock().await;
    match db.save_model_performance(&metrics).await {
        Ok(_) => Ok(CommandResult::success(
            "Performance metrics saved".to_string(),
        )),
        Err(e) => {
            error!("Failed to save performance metrics: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to save metrics: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_model_performance_stats(
    state: State<'_, AppState>,
    model_name: String,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!("Getting performance stats for model: {}", model_name);

    let db = state.db.lock().await;
    match db.get_model_performance_stats(&model_name).await {
        Ok(stats) => Ok(CommandResult::success(stats)),
        Err(e) => {
            error!("Failed to get model performance stats: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get performance stats: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_all_model_performance(
    state: State<'_, AppState>,
) -> Result<CommandResult<Vec<serde_json::Value>>, ()> {
    info!("Getting performance stats for all models");

    let db = state.db.lock().await;
    match db.get_all_model_performance().await {
        Ok(stats) => Ok(CommandResult::success(stats)),
        Err(e) => {
            error!("Failed to get all model performance: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get all performance stats: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_app_config(
    state: State<'_, AppState>,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!("Getting application configuration");

    let config = state.config.lock().await;
    match config.export_config() {
        Ok(config_json) => match serde_json::from_str::<serde_json::Value>(&config_json) {
            Ok(config_value) => Ok(CommandResult::success(config_value)),
            Err(e) => {
                error!("Failed to parse config JSON: {}", e);
                Ok(CommandResult::error(format!(
                    "Failed to parse config: {}",
                    e
                )))
            }
        },
        Err(e) => {
            error!("Failed to export config: {}", e);
            Ok(CommandResult::error(format!("Failed to get config: {}", e)))
        }
    }
}

#[tauri::command]
pub async fn validate_app_config(
    state: State<'_, AppState>,
) -> Result<CommandResult<Vec<String>>, ()> {
    info!("Validating application configuration");

    let config = state.config.lock().await;
    match config.validate_config() {
        Ok(warnings) => Ok(CommandResult::success(warnings)),
        Err(e) => {
            error!("Failed to validate config: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to validate config: {}",
                e
            )))
        }
    }
}

// ============================================================================
// PHASE 2: Enhanced Analysis Commands
// ============================================================================

#[tauri::command]
pub async fn semantic_analysis(
    state: State<'_, AppState>,
    resume_content: String,
    job_description: String,
    industry: String,
) -> Result<CommandResult<SemanticAnalysisResult>, ()> {
    info!("Performing semantic analysis for industry: {}", industry);

    // Clone the database first, then release the lock
    let db_clone = {
        let db = state.db.lock().await;
        info!("Database lock acquired for semantic analysis");

        // Test database connectivity before creating analyzer
        match db.health_check().await {
            Ok(true) => {
                info!("Database health check passed in semantic analysis");
            }
            Ok(false) => {
                error!("Database health check failed in semantic analysis");
                return Ok(CommandResult::error(
                    "Database health check failed".to_string(),
                ));
            }
            Err(e) => {
                error!("Database health check error in semantic analysis: {}", e);
                return Ok(CommandResult::error(format!(
                    "Database health check error: {}",
                    e
                )));
            }
        }

        // Test industry keywords access directly
        match db.get_industry_keywords(&industry).await {
            Ok(keywords) => {
                info!(
                    "Successfully loaded {} keywords for industry '{}' in command",
                    keywords.len(),
                    industry
                );
            }
            Err(e) => {
                error!(
                    "Failed to load industry keywords directly in command: {}",
                    e
                );
                return Ok(CommandResult::error(format!(
                    "Failed to load industry keywords: {}",
                    e
                )));
            }
        }

        // Clone the database before releasing the lock
        db.clone()
    }; // Lock is released here

    info!("Database lock released, creating SemanticAnalyzer");
    let analyzer = SemanticAnalyzer::new(db_clone);
    info!("SemanticAnalyzer created successfully");

    match analyzer
        .analyze_semantic_keywords(&resume_content, &job_description, &industry)
        .await
    {
        Ok(result) => {
            info!("Semantic analysis completed successfully");
            Ok(CommandResult::success(result))
        }
        Err(e) => {
            error!("Failed to perform semantic analysis: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to perform semantic analysis: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn comprehensive_analysis(
    state: State<'_, AppState>,
    resume_content: String,
    job_description: String,
    target_industry: String,
    target_role_level: String,
) -> Result<CommandResult<EnhancedAnalysisResult>, ()> {
    info!(
        "Performing comprehensive analysis for {} level position in {}",
        target_role_level, target_industry
    );

    // Clone the database first, then release the lock
    let db_clone = {
        let db = state.db.lock().await;
        info!("Database lock acquired for comprehensive analysis");

        // Test database connectivity before creating scoring engine
        match db.health_check().await {
            Ok(true) => {
                info!("Database health check passed in comprehensive analysis");
            }
            Ok(false) => {
                error!("Database health check failed in comprehensive analysis");
                return Ok(CommandResult::error(
                    "Database health check failed".to_string(),
                ));
            }
            Err(e) => {
                error!(
                    "Database health check error in comprehensive analysis: {}",
                    e
                );
                return Ok(CommandResult::error(format!(
                    "Database health check error: {}",
                    e
                )));
            }
        }

        // Test industry keywords access directly in comprehensive analysis
        match db.get_industry_keywords(&target_industry).await {
            Ok(keywords) => {
                info!(
                    "Successfully loaded {} keywords for industry '{}' in comprehensive analysis",
                    keywords.len(),
                    target_industry
                );
            }
            Err(e) => {
                error!(
                    "Failed to load industry keywords directly in comprehensive analysis: {}",
                    e
                );
                return Ok(CommandResult::error(format!(
                    "Failed to load industry keywords in comprehensive analysis: {}",
                    e
                )));
            }
        }

        // Clone the database before releasing the lock
        db.clone()
    }; // Lock is released here

    info!("Database lock released, creating EnhancedScoringEngine");
    let scoring_engine = EnhancedScoringEngine::new(db_clone);
    info!("EnhancedScoringEngine created successfully");

    match scoring_engine
        .comprehensive_analysis(
            &resume_content,
            &job_description,
            &target_industry,
            &target_role_level,
        )
        .await
    {
        Ok(result) => {
            info!("Comprehensive analysis completed successfully");
            Ok(CommandResult::success(result))
        }
        Err(e) => {
            error!("Failed to perform comprehensive analysis: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to perform comprehensive analysis: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn industry_analysis(
    state: State<'_, AppState>,
    resume_content: String,
    job_description: String,
    target_industry: String,
) -> Result<CommandResult<IndustryAnalysisResult>, ()> {
    info!("Performing industry analysis for: {}", target_industry);

    let db = state.db.lock().await;
    let analyzer = IndustryAnalyzer::new(db.clone());

    match analyzer
        .analyze_for_industry(&resume_content, &job_description, &target_industry)
        .await
    {
        Ok(result) => Ok(CommandResult::success(result)),
        Err(e) => {
            error!("Failed to perform industry analysis: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to perform industry analysis: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn create_enhanced_prompt(
    prompt_request: EnhancedPromptRequest,
) -> Result<CommandResult<EnhancedPromptResponse>, ()> {
    info!(
        "Creating enhanced prompt for model: {} with type: {}",
        prompt_request.model_name, prompt_request.prompt_type
    );

    let prompt_engine = EnhancedPromptEngine::new();

    match prompt_engine.create_enhanced_prompt(prompt_request) {
        Ok(result) => Ok(CommandResult::success(result)),
        Err(e) => {
            error!("Failed to create enhanced prompt: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to create enhanced prompt: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn simulate_ats_processing(
    state: State<'_, AppState>,
    resume_content: String,
    target_job_keywords: Vec<String>,
) -> Result<CommandResult<ATSSimulationResult>, ()> {
    info!(
        "Simulating ATS processing for resume with {} target keywords",
        target_job_keywords.len()
    );

    let db = state.db.lock().await;
    let simulator = ATSSimulator::new(db.clone());

    match simulator
        .simulate_ats_processing(&resume_content, &target_job_keywords)
        .await
    {
        Ok(result) => Ok(CommandResult::success(result)),
        Err(e) => {
            error!("Failed to simulate ATS processing: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to simulate ATS processing: {}",
                e
            )))
        }
    }
}

// Phase 3 Commands - ATS Format Compatibility and Testing

#[tauri::command]
pub async fn check_format_compatibility(
    resume_content: String,
) -> Result<CommandResult<FormatCompatibilityReport>, ()> {
    info!("Checking format compatibility for resume");

    let format_checker = FormatCompatibilityChecker::new();

    match format_checker.check_comprehensive_compatibility(&resume_content) {
        Ok(report) => Ok(CommandResult::success(report)),
        Err(e) => {
            error!("Failed to check format compatibility: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to check format compatibility: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn analyze_format_issues(
    resume_content: String,
) -> Result<CommandResult<FormatIssueReport>, ()> {
    info!("Analyzing format issues for resume");

    let format_checker = FormatCompatibilityChecker::new();
    let issue_detector = FormatIssueDetector::new();

    match format_checker.check_comprehensive_compatibility(&resume_content) {
        Ok(compatibility_report) => {
            match issue_detector.analyze_format_issues(&resume_content, &compatibility_report) {
                Ok(issue_report) => Ok(CommandResult::success(issue_report)),
                Err(e) => {
                    error!("Failed to analyze format issues: {}", e);
                    Ok(CommandResult::error(format!(
                        "Failed to analyze format issues: {}",
                        e
                    )))
                }
            }
        }
        Err(e) => {
            error!("Failed to check format compatibility: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to check format compatibility: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn detect_advanced_format_issues(
    resume_content: String,
) -> Result<CommandResult<Vec<crate::format_checker::FormatIssue>>, ()> {
    info!("Detecting advanced format issues for resume");

    let issue_detector = FormatIssueDetector::new();

    match issue_detector.detect_advanced_issues(&resume_content) {
        Ok(issues) => Ok(CommandResult::success(issues)),
        Err(e) => {
            error!("Failed to detect advanced format issues: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to detect advanced format issues: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn run_ats_validation_suite(
    state: State<'_, AppState>,
) -> Result<CommandResult<ValidationReport>, ()> {
    info!("Running comprehensive ATS validation suite");

    let db = state.db.lock().await;
    let testing_framework = ATSTestingFramework::new(db.clone());

    match testing_framework.run_comprehensive_validation().await {
        Ok(report) => Ok(CommandResult::success(report)),
        Err(e) => {
            error!("Failed to run ATS validation suite: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to run validation suite: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn simulate_multiple_ats_systems(
    state: State<'_, AppState>,
    resume_content: String,
    target_keywords: Vec<String>,
) -> Result<CommandResult<ATSSimulationResult>, ()> {
    info!("Simulating multiple ATS systems for resume processing");

    let db = state.db.lock().await;
    let simulator = ATSSimulator::new(db.clone());

    match simulator
        .simulate_multiple_ats_systems(&resume_content, &target_keywords)
        .await
    {
        Ok(result) => Ok(CommandResult::success(result)),
        Err(e) => {
            error!("Failed to simulate multiple ATS systems: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to simulate multiple ATS systems: {}",
                e
            )))
        }
    }
}

// Phase 4 Commands - Advanced Optimization Engine

#[tauri::command]
pub async fn analyze_achievements(
    resume_content: String,
) -> Result<CommandResult<FrontendAchievementAnalysis>, ()> {
    info!("Analyzing achievements with X-Y-Z formula detection");

    let analyzer = AchievementAnalyzer::new();

    match analyzer.analyze_achievements(&resume_content) {
        Ok(analysis) => {
            info!("Achievement analysis completed with {} strong achievements and {} improvement opportunities", 
                  analysis.strong_achievements.len(), analysis.improvement_opportunities.len());

            // Transform to frontend-expected structure
            let frontend_analysis = FrontendAchievementAnalysis {
                bullet_points: analysis
                    .strong_achievements
                    .clone()
                    .into_iter()
                    .map(|bullet| FrontendBulletAnalysis {
                        text: bullet.original_text,
                        section: bullet.section,
                        has_xyz_structure: bullet.has_xyz_formula,
                        action_verb: bullet.action_verb.unwrap_or_default(),
                        quantification: bullet.quantifications.join(", "),
                        impact: bullet.outcome_description.unwrap_or_default(),
                        strength_score: bullet.strength_score,
                        suggestions: bullet.improvement_suggestions,
                    })
                    .collect(),
                xyz_formula_usage: analysis.xyz_formula_compliance,
                achievement_density: calculate_achievement_density(&analysis),
                quantification_score: analysis.quantification_rate,
                action_verb_strength: analysis.action_verb_strength,
                overall_achievement_score: analysis.overall_achievement_score,
                suggestions: analysis
                    .improvement_opportunities
                    .clone()
                    .into_iter()
                    .map(|sugg| FrontendAchievementSuggestion {
                        bullet_point: sugg.original.clone(),
                        suggestion_type: sugg.weakness_type,
                        original: sugg.original,
                        improved: sugg.improved_version,
                        explanation: sugg.explanation,
                        impact_score: sugg.improvement_impact,
                    })
                    .collect(),
            };

            Ok(CommandResult::success(frontend_analysis))
        }
        Err(e) => {
            error!("Failed to analyze achievements: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to analyze achievements: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn generate_comprehensive_optimization(
    state: State<'_, AppState>,
    resume_content: String,
    job_description: String,
    optimization_level: String,
) -> Result<CommandResult<ComprehensiveOptimization>, ()> {
    info!(
        "Generating comprehensive optimization with level: {}",
        optimization_level
    );

    let db = state.db.lock().await;
    let optimizer = SmartOptimizationEngine::new(db.clone());

    let level = match optimization_level.as_str() {
        "conservative" => OptimizationLevel::Conservative,
        "balanced" => OptimizationLevel::Balanced,
        "aggressive" => OptimizationLevel::Aggressive,
        _ => OptimizationLevel::Balanced,
    };

    match optimizer
        .generate_comprehensive_optimization(&resume_content, &job_description, level)
        .await
    {
        Ok(optimization) => {
            info!(
                "Comprehensive optimization completed. Score improvement: {:.1} -> {:.1}",
                optimization.before_score, optimization.projected_after_score
            );
            Ok(CommandResult::success(optimization))
        }
        Err(e) => {
            error!("Failed to generate comprehensive optimization: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to generate optimization: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_realtime_suggestions(
    state: State<'_, AppState>,
    resume_content: String,
    job_description: String,
    cursor_position: usize,
) -> Result<CommandResult<LiveSuggestions>, ()> {
    info!(
        "Getting real-time suggestions for position: {}",
        cursor_position
    );

    let db = state.db.lock().await;
    let mut optimizer = RealtimeOptimizer::new(db.clone());

    match optimizer
        .get_live_suggestions(&resume_content, &job_description, cursor_position)
        .await
    {
        Ok(suggestions) => {
            info!(
                "Generated {} real-time suggestions with score: {:.1}",
                suggestions.context_suggestions.len(),
                suggestions.real_time_score
            );
            Ok(CommandResult::success(suggestions))
        }
        Err(e) => {
            error!("Failed to get real-time suggestions: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get real-time suggestions: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn validate_xyz_formula(
    bullet_text: String,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!("Validating X-Y-Z formula for bullet point");

    let analyzer = AchievementAnalyzer::new();

    // Create a temporary resume content with just this bullet
    let temp_content = format!("Experience\n• {}", bullet_text);

    match analyzer.analyze_achievements(&temp_content) {
        Ok(analysis) => {
            let result = if let Some(bullet_analysis) = analysis.strong_achievements.first() {
                serde_json::json!({
                    "has_xyz_formula": bullet_analysis.has_xyz_formula,
                    "xyz_components": bullet_analysis.xyz_components,
                    "strength_score": bullet_analysis.strength_score,
                    "improvement_suggestions": bullet_analysis.improvement_suggestions,
                    "action_verb_strength": bullet_analysis.action_verb_strength,
                    "has_quantification": bullet_analysis.has_quantification,
                    "has_outcome": bullet_analysis.has_outcome
                })
            } else if let Some(improvement) = analysis.improvement_opportunities.first() {
                serde_json::json!({
                    "has_xyz_formula": false,
                    "weakness_type": improvement.weakness_type,
                    "improved_version": improvement.improved_version,
                    "explanation": improvement.explanation,
                    "impact_score": improvement.improvement_impact,
                    "suggested_x": improvement.suggested_x,
                    "suggested_y": improvement.suggested_y,
                    "suggested_z": improvement.suggested_z
                })
            } else {
                serde_json::json!({
                    "has_xyz_formula": false,
                    "message": "Unable to analyze bullet point"
                })
            };

            Ok(CommandResult::success(result))
        }
        Err(e) => {
            error!("Failed to validate X-Y-Z formula: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to validate X-Y-Z formula: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_achievement_suggestions(
    _state: State<'_, AppState>,
    resume_content: String,
    section_name: String,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!(
        "Getting achievement suggestions for section: {}",
        section_name
    );

    let analyzer = AchievementAnalyzer::new();

    match analyzer.analyze_achievements(&resume_content) {
        Ok(analysis) => {
            // Filter suggestions for the specific section
            let section_suggestions: Vec<_> = analysis
                .improvement_opportunities
                .into_iter()
                .filter(|suggestion| {
                    suggestion.section.to_lowercase() == section_name.to_lowercase()
                })
                .collect();

            let result = serde_json::json!({
                "section": section_name,
                "suggestions": section_suggestions,
                "section_score": analysis.section_scores.get(&section_name).unwrap_or(&0.0),
                "xyz_compliance": analysis.xyz_formula_compliance,
                "overall_achievement_score": analysis.overall_achievement_score
            });

            info!(
                "Generated {} achievement suggestions for {}",
                section_suggestions.len(),
                section_name
            );
            Ok(CommandResult::success(result))
        }
        Err(e) => {
            error!("Failed to get achievement suggestions: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get achievement suggestions: {}",
                e
            )))
        }
    }
}

// =============================================================================
// Phase 5: Competitive Features Commands
// =============================================================================

#[tauri::command]
pub async fn generate_competitive_analysis(
    state: State<'_, AppState>,
    resume_content: String,
    job_description: String,
    target_companies: Vec<String>,
) -> Result<CommandResult<CompetitiveAnalysis>, ()> {
    info!(
        "Generating competitive analysis for {} target companies",
        target_companies.len()
    );

    let db = state.db.lock().await;
    let competitive_analyzer = CompetitiveAnalyzer::new(db.clone());

    match competitive_analyzer
        .generate_competitive_analysis(&resume_content, &job_description, target_companies)
        .await
    {
        Ok(analysis) => {
            info!("Successfully generated competitive analysis");
            Ok(CommandResult::success(analysis))
        }
        Err(e) => {
            error!("Failed to generate competitive analysis: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to generate competitive analysis: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_market_position_analysis(
    state: State<'_, AppState>,
    resume_content: String,
    job_description: String,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!("Getting market position analysis");

    let db = state.db.lock().await;
    let competitive_analyzer = CompetitiveAnalyzer::new(db.clone());

    match competitive_analyzer
        .calculate_market_position(&resume_content, &job_description)
        .await
    {
        Ok(market_position) => {
            let result = serde_json::json!({
                "market_position": market_position,
                "generated_at": chrono::Utc::now()
            });

            info!("Successfully generated market position analysis");
            Ok(CommandResult::success(result))
        }
        Err(e) => {
            error!("Failed to get market position analysis: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get market position analysis: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_salary_insights(
    state: State<'_, AppState>,
    resume_content: String,
    job_description: String,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!("Getting salary insights");

    let db = state.db.lock().await;
    let competitive_analyzer = CompetitiveAnalyzer::new(db.clone());

    match competitive_analyzer
        .generate_salary_insights(&resume_content, &job_description)
        .await
    {
        Ok(salary_insights) => {
            let result = serde_json::json!({
                "salary_insights": salary_insights,
                "generated_at": chrono::Utc::now()
            });

            info!("Successfully generated salary insights");
            Ok(CommandResult::success(result))
        }
        Err(e) => {
            error!("Failed to get salary insights: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get salary insights: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_hiring_probability(
    state: State<'_, AppState>,
    resume_content: String,
    job_description: String,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!("Calculating hiring probability");

    let db = state.db.lock().await;
    let competitive_analyzer = CompetitiveAnalyzer::new(db.clone());

    match competitive_analyzer
        .calculate_hiring_probability(&resume_content, &job_description)
        .await
    {
        Ok(hiring_probability) => {
            let result = serde_json::json!({
                "hiring_probability": hiring_probability,
                "generated_at": chrono::Utc::now()
            });

            info!("Successfully calculated hiring probability");
            Ok(CommandResult::success(result))
        }
        Err(e) => {
            error!("Failed to calculate hiring probability: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to calculate hiring probability: {}",
                e
            )))
        }
    }
}

// ==================== Phase 6: Advanced AI Integration & Machine Learning ====================

#[tauri::command]
pub async fn generate_ml_insights(
    state: State<'_, AppState>,
    resume_content: String,
    job_description: String,
) -> Result<CommandResult<MLInsights>, ()> {
    info!("Generating ML insights for resume");

    let db = state.db.lock().await;
    let ml_engine = MLInsightsEngine::new(db.clone());

    // Get user history for better predictions
    let user_history = vec![]; // In a real implementation, we'd fetch from database

    match ml_engine
        .generate_ml_insights(&resume_content, &job_description, &user_history)
        .await
    {
        Ok(insights) => {
            info!("Successfully generated ML insights");
            Ok(CommandResult::success(insights))
        }
        Err(e) => {
            error!("Failed to generate ML insights: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to generate ML insights: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn predict_application_success(
    state: State<'_, AppState>,
    resume_content: String,
    job_description: String,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!("Predicting application success probability");

    let db = state.db.lock().await;
    let ml_engine = MLInsightsEngine::new(db.clone());
    let user_history = vec![];

    match ml_engine
        .generate_ml_insights(&resume_content, &job_description, &user_history)
        .await
    {
        Ok(insights) => {
            let result = serde_json::json!({
                "success_prediction": insights.success_prediction,
                "confidence_metrics": insights.confidence_metrics,
                "generated_at": insights.generated_at
            });

            info!("Successfully predicted application success");
            Ok(CommandResult::success(result))
        }
        Err(e) => {
            error!("Failed to predict application success: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to predict application success: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_career_path_suggestions(
    state: State<'_, AppState>,
    resume_content: String,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!("Generating career path suggestions");

    let db = state.db.lock().await;
    let ml_engine = MLInsightsEngine::new(db.clone());
    let user_history = vec![];

    match ml_engine
        .generate_ml_insights(&resume_content, "General technology roles", &user_history)
        .await
    {
        Ok(insights) => {
            let result = serde_json::json!({
                "career_path_suggestions": insights.career_path_suggestions,
                "skill_demand_forecast": insights.skill_demand_forecast,
                "generated_at": insights.generated_at
            });

            info!("Successfully generated career path suggestions");
            Ok(CommandResult::success(result))
        }
        Err(e) => {
            error!("Failed to generate career path suggestions: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to generate career path suggestions: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_salary_prediction_ml(
    state: State<'_, AppState>,
    resume_content: String,
    job_description: String,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!("Generating ML-based salary prediction");

    let db = state.db.lock().await;
    let ml_engine = MLInsightsEngine::new(db.clone());
    let user_history = vec![];

    match ml_engine
        .generate_ml_insights(&resume_content, &job_description, &user_history)
        .await
    {
        Ok(insights) => {
            let result = serde_json::json!({
                "salary_prediction": insights.salary_prediction,
                "confidence_metrics": insights.confidence_metrics,
                "generated_at": insights.generated_at
            });

            info!("Successfully generated ML salary prediction");
            Ok(CommandResult::success(result))
        }
        Err(e) => {
            error!("Failed to generate ML salary prediction: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to generate ML salary prediction: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_ml_recommendations(
    state: State<'_, AppState>,
    resume_content: String,
    job_description: String,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!("Generating ML-based recommendations");

    let db = state.db.lock().await;
    let ml_engine = MLInsightsEngine::new(db.clone());
    let user_history = vec![];

    match ml_engine
        .generate_ml_insights(&resume_content, &job_description, &user_history)
        .await
    {
        Ok(insights) => {
            let result = serde_json::json!({
                "recommendations": insights.recommendation_engine,
                "optimization_prioritization": insights.optimization_prioritization,
                "confidence_metrics": insights.confidence_metrics,
                "generated_at": insights.generated_at
            });

            info!("Successfully generated ML recommendations");
            Ok(CommandResult::success(result))
        }
        Err(e) => {
            error!("Failed to generate ML recommendations: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to generate ML recommendations: {}",
                e
            )))
        }
    }
}

// ==================== Advanced Scoring Engine Commands ====================

#[allow(dead_code)]
#[tauri::command]
pub async fn analyze_resume_advanced(
    state: State<'_, AppState>,
    resume_content: String,
    job_description: String,
    industry: String,
    experience_level: String,
) -> Result<CommandResult<AdvancedAnalysisResult>, ()> {
    info!(
        "Starting advanced analysis for {} industry, {} level",
        industry, experience_level
    );

    let db = state.db.clone();
    let advanced_engine = AdvancedScoringEngine::new(db);

    match advanced_engine
        .analyze_comprehensive(
            &resume_content,
            &job_description,
            &industry,
            &experience_level,
        )
        .await
    {
        Ok(result) => {
            info!(
                "Advanced analysis completed with enhanced score: {:.1}",
                result.base_analysis.overall_score
            );
            Ok(CommandResult::success(result))
        }
        Err(e) => {
            error!("Advanced analysis failed: {}", e);
            Ok(CommandResult::error(format!(
                "Advanced analysis failed: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_keyword_analysis_detailed(
    state: State<'_, AppState>,
    resume_content: String,
    job_description: String,
    industry: String,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!(
        "Getting detailed keyword analysis for {} industry",
        industry
    );

    let db = state.db.clone();
    let advanced_engine = AdvancedScoringEngine::new(db);

    match advanced_engine
        .analyze_comprehensive(&resume_content, &job_description, &industry, "mid-level")
        .await
    {
        Ok(result) => {
            let keyword_analysis = serde_json::json!({
                "exact_matches": result.keyword_analysis.exact_matches,
                "stemmed_matches": result.keyword_analysis.stemmed_matches,
                "contextual_matches": result.keyword_analysis.contextual_matches,
                "synonym_matches": result.keyword_analysis.synonym_matches,
                "overall_score": result.keyword_analysis.overall_score,
                "match_density": result.keyword_analysis.match_density,
                "section_distribution": result.keyword_analysis.section_distribution
            });

            info!("Detailed keyword analysis completed");
            Ok(CommandResult::success(keyword_analysis))
        }
        Err(e) => {
            error!("Detailed keyword analysis failed: {}", e);
            Ok(CommandResult::error(format!(
                "Detailed keyword analysis failed: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_ats_compatibility_scores(
    state: State<'_, AppState>,
    resume_content: String,
    job_description: String,
    industry: String,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!("Getting ATS compatibility scores for {} industry", industry);

    let db = state.db.clone();
    let advanced_engine = AdvancedScoringEngine::new(db);

    match advanced_engine
        .analyze_comprehensive(&resume_content, &job_description, &industry, "mid-level")
        .await
    {
        Ok(result) => {
            let ats_scores = serde_json::json!({
                "compatibility_scores": result.ats_compatibility,
                "format_analysis": result.format_analysis,
                "parsing_issues": result.format_analysis.parsing_issues,
                "overall_compatibility": result.format_analysis.ats_compatibility_score
            });

            info!("ATS compatibility analysis completed");
            Ok(CommandResult::success(ats_scores))
        }
        Err(e) => {
            error!("ATS compatibility analysis failed: {}", e);
            Ok(CommandResult::error(format!(
                "ATS compatibility analysis failed: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_benchmark_comparison(
    state: State<'_, AppState>,
    resume_content: String,
    job_description: String,
    industry: String,
    experience_level: String,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!(
        "Getting benchmark comparison for {} industry, {} level",
        industry, experience_level
    );

    let db = state.db.clone();
    let advanced_engine = AdvancedScoringEngine::new(db);

    match advanced_engine
        .analyze_comprehensive(
            &resume_content,
            &job_description,
            &industry,
            &experience_level,
        )
        .await
    {
        Ok(result) => {
            let benchmark_data = serde_json::json!({
                "industry_percentile": result.benchmark_comparison.industry_percentile,
                "experience_level_percentile": result.benchmark_comparison.experience_level_percentile,
                "overall_percentile": result.benchmark_comparison.overall_percentile,
                "top_performers_gap": result.benchmark_comparison.top_performers_gap,
                "industry_alignment": result.industry_alignment
            });

            info!("Benchmark comparison completed");
            Ok(CommandResult::success(benchmark_data))
        }
        Err(e) => {
            error!("Benchmark comparison failed: {}", e);
            Ok(CommandResult::error(format!(
                "Benchmark comparison failed: {}",
                e
            )))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
pub async fn get_optimization_suggestions_prioritized(
    state: State<'_, AppState>,
    resume_content: String,
    job_description: String,
    industry: String,
    experience_level: String,
) -> Result<CommandResult<serde_json::Value>, ()> {
    info!(
        "Getting prioritized optimization suggestions for {} industry, {} level",
        industry, experience_level
    );

    let db = state.db.clone();
    let advanced_engine = AdvancedScoringEngine::new(db);

    match advanced_engine
        .analyze_comprehensive(
            &resume_content,
            &job_description,
            &industry,
            &experience_level,
        )
        .await
    {
        Ok(result) => {
            let mut suggestions = result.improvement_suggestions;
            // Sort by impact score (highest first)
            suggestions.sort_by(|a, b| {
                b.impact_score
                    .partial_cmp(&a.impact_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            let optimization_data = serde_json::json!({
                "suggestions": suggestions,
                "total_potential_improvement": suggestions.iter().map(|s| s.impact_score).sum::<f64>(),
                "high_impact_count": suggestions.iter().filter(|s| s.impact_score > 10.0).count(),
                "quick_wins": suggestions.iter().filter(|s| s.difficulty == "Easy").count()
            });

            info!("Prioritized optimization suggestions completed");
            Ok(CommandResult::success(optimization_data))
        }
        Err(e) => {
            error!("Prioritized optimization suggestions failed: {}", e);
            Ok(CommandResult::error(format!(
                "Prioritized optimization suggestions failed: {}",
                e
            )))
        }
    }
}

// Job Description Management Commands

#[tauri::command]
pub async fn save_job_description(
    state: State<'_, AppState>,
    job: JobDescription,
) -> Result<CommandResult<String>, ()> {
    info!("Saving job description: {}", job.title);

    let db = state.db.lock().await;
    match db.save_job_description(&job).await {
        Ok(_) => {
            info!("Job description saved successfully with ID: {}", job.id);
            Ok(CommandResult::success(job.id))
        }
        Err(e) => {
            error!("Failed to save job description: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to save job description: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn get_job_description(
    state: State<'_, AppState>,
    id: String,
) -> Result<CommandResult<Option<JobDescription>>, ()> {
    info!("Getting job description with ID: {}", id);

    let db = state.db.lock().await;
    match db.get_job_description(&id).await {
        Ok(job) => {
            info!("Job description retrieved successfully");
            Ok(CommandResult::success(job))
        }
        Err(e) => {
            error!("Failed to get job description: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get job description: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn update_job_description(
    state: State<'_, AppState>,
    job: JobDescription,
) -> Result<CommandResult<String>, ()> {
    info!("Updating job description: {}", job.title);

    let db = state.db.lock().await;
    match db.update_job_description(&job).await {
        Ok(_) => {
            info!("Job description updated successfully with ID: {}", job.id);
            Ok(CommandResult::success(job.id))
        }
        Err(e) => {
            error!("Failed to update job description: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to update job description: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn delete_job_description(
    state: State<'_, AppState>,
    id: String,
) -> Result<CommandResult<String>, ()> {
    info!("Deleting job description with ID: {}", id);

    let db = state.db.lock().await;
    match db.delete_job_description(&id).await {
        Ok(_) => {
            info!("Job description deleted successfully");
            Ok(CommandResult::success(
                "Job description deleted".to_string(),
            ))
        }
        Err(e) => {
            error!("Failed to delete job description: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to delete job description: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn get_job_descriptions(
    state: State<'_, AppState>,
    include_archived: Option<bool>,
) -> Result<CommandResult<Vec<JobDescription>>, ()> {
    info!("Getting all job descriptions");

    let db = state.db.lock().await;
    let include_archived = include_archived.unwrap_or(false);

    match db.get_all_job_descriptions(include_archived).await {
        Ok(jobs) => {
            info!("Retrieved {} job descriptions", jobs.len());
            Ok(CommandResult::success(jobs))
        }
        Err(e) => {
            error!("Failed to get job descriptions: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get job descriptions: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn search_job_descriptions(
    state: State<'_, AppState>,
    request: JobSearchRequest,
) -> Result<CommandResult<JobSearchResult>, ()> {
    info!("Searching job descriptions with filters");

    let db = state.db.lock().await;
    match db.search_job_descriptions(&request).await {
        Ok(result) => {
            info!("Found {} job descriptions", result.jobs.len());
            Ok(CommandResult::success(result))
        }
        Err(e) => {
            error!("Failed to search job descriptions: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to search job descriptions: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn get_job_analytics(
    state: State<'_, AppState>,
) -> Result<CommandResult<JobAnalytics>, ()> {
    info!("Getting job analytics");

    let db = state.db.lock().await;
    match db.get_job_analytics().await {
        Ok(analytics) => {
            info!("Job analytics retrieved successfully");
            Ok(CommandResult::success(analytics))
        }
        Err(e) => {
            error!("Failed to get job analytics: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get job analytics: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn extract_job_from_url(
    _state: State<'_, AppState>,
    request: JobUrlExtractionRequest,
) -> Result<CommandResult<JobUrlExtractionResult>, ()> {
    info!("Extracting job details from URL: {}", request.url);

    // Basic implementation - in a real application, you would implement proper web scraping
    // For now, return a basic structure that indicates the feature is not fully implemented
    let result = JobUrlExtractionResult {
        title: Some("Job Title (Extracted from URL)".to_string()),
        company: Some("Company Name".to_string()),
        content: format!("Job description extracted from: {}\n\nThis is a placeholder implementation. Full URL extraction would require web scraping capabilities.", request.url),
        location: Some("Location TBD".to_string()),
        salary_range: None,
        employment_type: Some("Full-time".to_string()),
        remote_options: None,
        requirements: vec!["Requirements extraction not implemented".to_string()],
        posted_date: None,
        application_deadline: None,
        success: true,
        error: None,
    };

    info!("Job URL extraction completed (basic implementation)");
    Ok(CommandResult::success(result))
}

#[tauri::command]
pub async fn compare_job_descriptions(
    state: State<'_, AppState>,
    request: JobComparisonRequest,
) -> Result<CommandResult<JobComparisonResult>, ()> {
    info!("Comparing {} job descriptions", request.job_ids.len());

    let db = state.db.lock().await;

    // Get all jobs to compare
    let mut jobs = Vec::new();
    for job_id in &request.job_ids {
        match db.get_job_description(job_id).await {
            Ok(Some(job)) => jobs.push(job),
            Ok(None) => {
                return Ok(CommandResult::error(format!(
                    "Job description not found: {}",
                    job_id
                )));
            }
            Err(e) => {
                return Ok(CommandResult::error(format!(
                    "Failed to get job description {}: {}",
                    job_id, e
                )));
            }
        }
    }

    // Build comparison matrix (basic implementation)
    use crate::models::{
        BenefitsComparison, JobComparisonMatrix, JobMatchScore, JobUniqueRequirements,
        LocationComparison, MatchFactor, RequirementsComparison, SalaryComparison,
    };

    let salary_comparison: Vec<SalaryComparison> = jobs
        .iter()
        .map(|job| SalaryComparison {
            job_id: job.id.clone(),
            min_salary: job.salary_range_min,
            max_salary: job.salary_range_max,
            currency: job.salary_currency.clone(),
            vs_average: None, // Could calculate vs average
        })
        .collect();

    let location_comparison: Vec<LocationComparison> = jobs
        .iter()
        .map(|job| LocationComparison {
            job_id: job.id.clone(),
            location: job.location.clone(),
            remote_options: job.remote_options.clone(),
            commute_score: None, // Could calculate based on user location
        })
        .collect();

    let requirements_comparison = RequirementsComparison {
        common_requirements: vec!["Common requirement analysis not implemented".to_string()],
        unique_requirements: jobs
            .iter()
            .map(|job| JobUniqueRequirements {
                job_id: job.id.clone(),
                requirements: vec!["Requirements parsing not implemented".to_string()],
            })
            .collect(),
    };

    let benefits_comparison: Vec<BenefitsComparison> = jobs
        .iter()
        .map(|job| BenefitsComparison {
            job_id: job.id.clone(),
            benefits: vec!["Benefits parsing not implemented".to_string()],
        })
        .collect();

    let match_scores: Vec<JobMatchScore> = jobs
        .iter()
        .map(|job| JobMatchScore {
            job_id: job.id.clone(),
            match_score: 75.0, // Placeholder score
            match_factors: vec![MatchFactor {
                factor: "Overall compatibility".to_string(),
                score: 75.0,
                weight: 1.0,
                explanation: "Basic compatibility assessment".to_string(),
            }],
        })
        .collect();

    let comparison_matrix = JobComparisonMatrix {
        salary_comparison,
        location_comparison,
        requirements_comparison,
        benefits_comparison,
        match_scores,
    };

    let result = JobComparisonResult {
        jobs,
        comparison_matrix,
    };

    info!("Job comparison completed");
    Ok(CommandResult::success(result))
}

// Modern Keyword Extraction Command (Phase 1 of 2024-2025 upgrade)
#[tauri::command]
pub async fn analyze_resume_modern_nlp(
    app: tauri::AppHandle,
    resume_content: String,
    job_description: String,
    model_name: String,
    target_industry: Option<String>,
) -> Result<CommandResult<(AnalysisResult, ExtractionResult)>, String> {
    info!("Starting modern NLP-based resume analysis");

    let state = app.state::<AppState>();
    let db_guard = state.db.lock().await;
    let database = (*db_guard).clone();
    drop(db_guard);

    let ollama_client = match OllamaClient::new(None) {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to create Ollama client: {}", e);
            return Ok(CommandResult::error(format!(
                "Ollama connection failed: {}",
                e
            )));
        }
    };

    let analysis_engine =
        match AnalysisEngine::new_with_modern_extraction(ollama_client, database).await {
            Ok(engine) => engine,
            Err(e) => {
                error!("Failed to create modern analysis engine: {}", e);
                return Ok(CommandResult::error(format!(
                    "Modern analysis engine initialization failed: {}",
                    e
                )));
            }
        };

    match analysis_engine
        .analyze_resume_modern(
            &resume_content,
            &job_description,
            &model_name,
            target_industry.as_deref(),
        )
        .await
    {
        Ok(result) => {
            info!(
                "Modern NLP analysis completed successfully with {} keywords extracted",
                result.1.keyword_matches.len()
            );
            Ok(CommandResult::success(result))
        }
        Err(e) => {
            error!("Modern resume analysis failed: {}", e);
            Ok(CommandResult::error(format!("Analysis failed: {}", e)))
        }
    }
}

// Phase 3: Context-Aware Matching Engine Commands (2024-2025 upgrade)
#[tauri::command]
pub async fn analyze_context_aware_match(
    app: tauri::AppHandle,
    resume_content: String,
    job_description: String,
    target_industry: String,
) -> Result<CommandResult<crate::context_aware_matcher::ContextAwareMatchResult>, String> {
    info!(
        "Starting context-aware match analysis for industry: {}",
        target_industry
    );

    let state = app.state::<AppState>();
    let db_guard = state.db.lock().await;
    let database = (*db_guard).clone();
    drop(db_guard);

    // First, perform modern keyword extraction to get extraction results
    let ollama_client = match OllamaClient::new(None) {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to create Ollama client: {}", e);
            return Ok(CommandResult::error(format!(
                "Ollama connection failed: {}",
                e
            )));
        }
    };

    let analysis_engine =
        match AnalysisEngine::new_with_modern_extraction(ollama_client, database.clone()).await {
            Ok(engine) => engine,
            Err(e) => {
                error!("Failed to create modern analysis engine: {}", e);
                return Ok(CommandResult::error(format!(
                    "Modern analysis engine initialization failed: {}",
                    e
                )));
            }
        };

    // Extract keywords first using modern analysis
    let (_analysis_result, extraction_result) = match analysis_engine
        .analyze_resume_modern(
            &resume_content,
            &job_description,
            "qwen2.5:14b", // Default model
            Some(&target_industry),
        )
        .await
    {
        Ok(result) => result,
        Err(e) => {
            error!("Modern resume analysis failed: {}", e);
            return Ok(CommandResult::error(format!(
                "Modern resume analysis failed: {}",
                e
            )));
        }
    };

    // Now perform context-aware matching
    match crate::context_aware_matcher::ContextAwareMatcher::new(database).await {
        Ok(matcher) => {
            match matcher
                .analyze_match(
                    &resume_content,
                    &job_description,
                    &extraction_result,
                    &target_industry,
                )
                .await
            {
                Ok(result) => {
                    info!(
                        "Context-aware analysis completed with overall match score: {:.2}%",
                        result.overall_match_score * 100.0
                    );
                    Ok(CommandResult::success(result))
                }
                Err(e) => {
                    error!("Context-aware match analysis failed: {}", e);
                    Ok(CommandResult::error(format!(
                        "Context-aware analysis failed: {}",
                        e
                    )))
                }
            }
        }
        Err(e) => {
            error!("Failed to create context-aware matcher: {}", e);
            Ok(CommandResult::error(format!(
                "Context-aware matcher initialization failed: {}",
                e
            )))
        }
    }
}

// Phase 4: AI-Powered Skill Relationship Mapping Commands (2024-2025 upgrade)
#[tauri::command]
pub async fn analyze_skill_relationships(
    app: tauri::AppHandle,
    resume_skills: Vec<String>,
    job_requirements: Vec<String>,
    target_industry: String,
    career_goals: Option<String>,
) -> Result<CommandResult<crate::skill_relationship_mapper::SkillRelationshipResult>, String> {
    info!(
        "Starting skill relationship analysis for {} skills in {} industry",
        resume_skills.len(),
        target_industry
    );

    let state = app.state::<AppState>();
    let db_guard = state.db.lock().await;
    let database = (*db_guard).clone();
    drop(db_guard);

    match crate::skill_relationship_mapper::SkillRelationshipMapper::new(database).await {
        Ok(mapper) => {
            match mapper
                .analyze_skill_relationships(
                    &resume_skills,
                    &job_requirements,
                    &target_industry,
                    career_goals.as_deref(),
                )
                .await
            {
                Ok(result) => {
                    info!(
                        "Skill relationship analysis completed with {} career paths and {} learning recommendations",
                        result.career_progression_paths.len(),
                        result.learning_recommendations.len()
                    );
                    Ok(CommandResult::success(result))
                }
                Err(e) => {
                    error!("Skill relationship analysis failed: {}", e);
                    Ok(CommandResult::error(format!(
                        "Skill relationship analysis failed: {}",
                        e
                    )))
                }
            }
        }
        Err(e) => {
            error!("Failed to create skill relationship mapper: {}", e);
            Ok(CommandResult::error(format!(
                "Skill relationship mapper initialization failed: {}",
                e
            )))
        }
    }
}

// Phase 5: Machine Learning-Based Optimization Commands (2024-2025 upgrade)
#[tauri::command]
pub async fn optimize_ml_parameters(
    app: tauri::AppHandle,
    user_id: String,
    session_id: String,
    optimization_goals: Vec<String>,
) -> Result<CommandResult<crate::ml_optimization_engine::MLOptimizationResult>, String> {
    info!(
        "Starting ML parameter optimization for user: {} with {} goals",
        user_id,
        optimization_goals.len()
    );

    let state = app.state::<AppState>();
    let db_guard = state.db.lock().await;
    let database = (*db_guard).clone();
    drop(db_guard);

    match crate::ml_optimization_engine::MLOptimizationEngine::new(database).await {
        Ok(mut engine) => {
            let optimization_context = crate::ml_optimization_engine::OptimizationContext {
                user_id: user_id.clone(),
                session_id,
                data_source: "user_interaction".to_string(),
                optimization_goals,
                constraints: std::collections::HashMap::new(),
                timestamp: chrono::Utc::now(),
            };

            match engine.optimize_ml_parameters(&optimization_context).await {
                Ok(result) => {
                    info!(
                        "ML optimization completed for user: {} with {} insights and {} improvements",
                        user_id,
                        result.predictive_insights.len(),
                        result.recommendation_improvements.len()
                    );
                    Ok(CommandResult::success(result))
                }
                Err(e) => {
                    error!("ML optimization failed for user {}: {}", user_id, e);
                    Ok(CommandResult::error(format!(
                        "ML optimization failed: {}",
                        e
                    )))
                }
            }
        }
        Err(e) => {
            error!("Failed to create ML optimization engine: {}", e);
            Ok(CommandResult::error(format!(
                "ML optimization engine initialization failed: {}",
                e
            )))
        }
    }
}

// Phase 2: Dynamic Keyword Database Commands (2024-2025 upgrade)
#[tauri::command]
pub async fn get_trending_keywords(
    app: tauri::AppHandle,
    limit: Option<usize>,
) -> Result<CommandResult<Vec<crate::dynamic_keyword_db::TrendingKeywordData>>, String> {
    info!("Getting trending keywords with limit: {:?}", limit);

    let state = app.state::<AppState>();
    let db_guard = state.db.lock().await;
    let database = (*db_guard).clone();
    drop(db_guard);

    match crate::dynamic_keyword_db::DynamicKeywordDatabase::new(database).await {
        Ok(dynamic_db) => {
            let trending = dynamic_db.get_trending_keywords(limit);
            let trending_data: Vec<_> = trending.into_iter().cloned().collect();

            info!("Retrieved {} trending keywords", trending_data.len());
            Ok(CommandResult::success(trending_data))
        }
        Err(e) => {
            error!("Failed to access dynamic keyword database: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to get trending keywords: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn get_market_demand_data(
    app: tauri::AppHandle,
    skill: String,
) -> Result<CommandResult<Option<crate::dynamic_keyword_db::MarketDemandData>>, String> {
    info!("Getting market demand data for skill: {}", skill);

    let state = app.state::<AppState>();
    let db_guard = state.db.lock().await;
    let database = (*db_guard).clone();
    drop(db_guard);

    match crate::dynamic_keyword_db::DynamicKeywordDatabase::new(database).await {
        Ok(dynamic_db) => match dynamic_db.get_market_demand(&skill).await {
            Ok(market_data) => {
                info!(
                    "Retrieved market demand data for skill '{}': {:?}",
                    skill,
                    market_data.is_some()
                );
                Ok(CommandResult::success(market_data))
            }
            Err(e) => {
                error!(
                    "Failed to get market demand data for skill '{}': {}",
                    skill, e
                );
                Ok(CommandResult::error(format!(
                    "Failed to get market demand data: {}",
                    e
                )))
            }
        },
        Err(e) => {
            error!("Failed to access dynamic keyword database: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to access dynamic keyword database: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn get_industry_keywords_dynamic(
    app: tauri::AppHandle,
    industry: String,
) -> Result<CommandResult<Vec<crate::dynamic_keyword_db::DynamicKeyword>>, String> {
    info!("Getting dynamic keywords for industry: {}", industry);

    let state = app.state::<AppState>();
    let db_guard = state.db.lock().await;
    let database = (*db_guard).clone();
    drop(db_guard);

    match crate::dynamic_keyword_db::DynamicKeywordDatabase::new(database).await {
        Ok(mut dynamic_db) => match dynamic_db.get_industry_keywords(&industry).await {
            Ok(keywords) => {
                info!(
                    "Retrieved {} dynamic keywords for industry '{}'",
                    keywords.len(),
                    industry
                );
                Ok(CommandResult::success(keywords))
            }
            Err(e) => {
                error!(
                    "Failed to get dynamic keywords for industry '{}': {}",
                    industry, e
                );
                Ok(CommandResult::error(format!(
                    "Failed to get industry keywords: {}",
                    e
                )))
            }
        },
        Err(e) => {
            error!("Failed to access dynamic keyword database: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to access dynamic keyword database: {}",
                e
            )))
        }
    }
}

#[tauri::command]
pub async fn submit_keyword_feedback(
    app: tauri::AppHandle,
    keyword: String,
    industry: String,
    rating: u8,
    comment: Option<String>,
    context: Option<String>,
) -> Result<CommandResult<()>, String> {
    info!(
        "Submitting feedback for keyword '{}' in industry '{}' with rating: {}",
        keyword, industry, rating
    );

    let state = app.state::<AppState>();
    let db_guard = state.db.lock().await;
    let database = (*db_guard).clone();
    drop(db_guard);

    let feedback = crate::dynamic_keyword_db::UserFeedback {
        keyword: keyword.clone(),
        industry: industry.clone(),
        rating,
        comment,
        context,
    };

    match crate::dynamic_keyword_db::DynamicKeywordDatabase::new(database).await {
        Ok(mut dynamic_db) => {
            match dynamic_db
                .add_user_feedback(&keyword, &industry, feedback)
                .await
            {
                Ok(()) => {
                    info!(
                        "Successfully submitted feedback for keyword '{}' in industry '{}'",
                        keyword, industry
                    );
                    Ok(CommandResult::success(()))
                }
                Err(e) => {
                    error!("Failed to submit feedback for keyword '{}': {}", keyword, e);
                    Ok(CommandResult::error(format!(
                        "Failed to submit keyword feedback: {}",
                        e
                    )))
                }
            }
        }
        Err(e) => {
            error!("Failed to access dynamic keyword database: {}", e);
            Ok(CommandResult::error(format!(
                "Failed to access dynamic keyword database: {}",
                e
            )))
        }
    }
}
