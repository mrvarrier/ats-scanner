use anyhow::Result;
use log::{error, info};
use serde::Serialize;
use std::path::Path;
use tauri::State;

use crate::models::{
    ATSCompatibilityRule, Analysis, AnalysisRequest, AnalysisResult, DocumentInfo, IndustryKeyword,
    ModelPerformance, ModelPerformanceMetrics, OptimizationRequest, OptimizationResult, Resume,
    ScoringBenchmark, UserFeedback, UserPreferences, UserPreferencesUpdate,
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
use crate::ollama::OllamaClient;
use crate::plugin_system::{PluginExecutionResult, PluginInfo, PluginManager};
use crate::scoring::AnalysisEngine;
use crate::utils::export_data;
use crate::AppState;

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
pub async fn parse_document(file_path: String) -> CommandResult<DocumentInfo> {
    info!("Parsing document: {}", file_path);

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

    let plugins_dir = std::env::current_dir().unwrap().join("plugins");
    let plugin_manager = PluginManager::new(plugins_dir);

    let plugins = plugin_manager.list_plugins().await;
    info!("Found {} plugins", plugins.len());

    Ok(CommandResult::success(plugins))
}

#[tauri::command]
pub async fn get_plugin_info(
    _state: State<'_, AppState>,
    plugin_id: String,
) -> Result<CommandResult<Option<PluginInfo>>, ()> {
    info!("Getting plugin info for: {}", plugin_id);

    let plugins_dir = std::env::current_dir().unwrap().join("plugins");
    let plugin_manager = PluginManager::new(plugins_dir);

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

    let plugins_dir = std::env::current_dir().unwrap().join("plugins");
    let plugin_manager = PluginManager::new(plugins_dir);

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

    let plugins_dir = std::env::current_dir().unwrap().join("plugins");
    let plugin_manager = PluginManager::new(plugins_dir);

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

    let db = state.db.lock().await;
    let analyzer = SemanticAnalyzer::new(db.clone());

    match analyzer
        .analyze_semantic_keywords(&resume_content, &job_description, &industry)
        .await
    {
        Ok(result) => Ok(CommandResult::success(result)),
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

    let db = state.db.lock().await;
    let scoring_engine = EnhancedScoringEngine::new(db.clone());

    match scoring_engine
        .comprehensive_analysis(
            &resume_content,
            &job_description,
            &target_industry,
            &target_role_level,
        )
        .await
    {
        Ok(result) => Ok(CommandResult::success(result)),
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
