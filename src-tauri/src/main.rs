// Prevents additional console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod database;
mod document;
mod models;
mod ollama;
mod plugin_system;
mod scoring;
mod utils;
// Advanced Scoring Engine
mod advanced_scoring;
// Phase 2 Enhanced Analysis Modules
mod ats_simulator;
mod enhanced_prompts;
mod enhanced_scoring;
mod industry_analyzer;
mod semantic_analyzer;
// Phase 3 ATS Simulation & Format Checking
mod format_checker;
mod format_issue_detector;
mod testing_framework;
// Phase 4 Advanced Optimization Engine
mod achievement_analyzer;
mod realtime_optimizer;
mod smart_optimizer;
// Phase 5 Competitive Features
mod competitive_analyzer;
// Phase 6 Advanced AI Integration & Machine Learning
mod ml_insights;

use crate::config::ConfigManager;
use crate::database::Database;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: std::sync::Arc<tokio::sync::Mutex<Database>>,
    pub config: std::sync::Arc<tokio::sync::Mutex<ConfigManager>>,
}
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting ATS Scanner application");

    // Initialize configuration
    let mut config_manager = ConfigManager::new()?;
    config_manager.apply_env_overrides()?;

    // Sanitize configuration
    config_manager.sanitize_database_url()?;

    // Validate configuration
    let warnings = config_manager.validate_config()?;
    if !warnings.is_empty() {
        for warning in warnings {
            log::warn!("Configuration warning: {}", warning);
        }
    }

    // Initialize database with config
    let database_url = config_manager.get_database_url();
    let database = if database_url == "sqlite::memory:" {
        Database::new_with_url(database_url).await?
    } else {
        Database::new_with_url(database_url).await?
    };

    // Perform initial health check
    match database.health_check().await {
        Ok(true) => {
            info!("Database health check passed");
        }
        Ok(false) => {
            log::warn!("Database health check failed - database may not be functioning correctly");
        }
        Err(e) => {
            log::error!("Database health check error: {}", e);
            return Err(e.into());
        }
    }

    let app_state = AppState {
        db: std::sync::Arc::new(tokio::sync::Mutex::new(database)),
        config: std::sync::Arc::new(tokio::sync::Mutex::new(config_manager)),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::get_ollama_models,
            commands::test_ollama_connection,
            commands::ollama_health_check,
            commands::parse_document,
            commands::save_resume,
            commands::get_all_resumes,
            commands::get_resume,
            commands::delete_resume,
            commands::analyze_resume,
            commands::get_analysis_history,
            commands::delete_analysis,
            commands::export_results,
            commands::optimize_resume,
            commands::get_model_performance,
            commands::get_analysis_stats,
            commands::get_score_distribution,
            commands::get_improvement_trends,
            commands::get_user_preferences,
            commands::update_user_preferences,
            commands::reset_user_preferences,
            commands::export_user_preferences,
            commands::import_user_preferences,
            commands::list_plugins,
            commands::get_plugin_info,
            commands::execute_plugin,
            commands::update_plugin_config,
            // Phase 1 Enhanced Commands
            commands::get_industry_keywords,
            commands::get_all_industries,
            commands::save_industry_keyword,
            commands::get_ats_rules,
            commands::save_ats_rule,
            commands::get_scoring_benchmarks,
            commands::save_scoring_benchmark,
            commands::save_user_feedback,
            commands::get_feedback_by_analysis,
            commands::get_feedback_stats,
            commands::save_model_performance,
            commands::get_model_performance_stats,
            commands::get_all_model_performance,
            commands::get_app_config,
            commands::validate_app_config,
            // Phase 2 Enhanced Analysis Commands
            commands::semantic_analysis,
            commands::comprehensive_analysis,
            commands::industry_analysis,
            commands::create_enhanced_prompt,
            commands::simulate_ats_processing,
            // Phase 3 ATS Format Compatibility Commands
            commands::check_format_compatibility,
            commands::analyze_format_issues,
            commands::detect_advanced_format_issues,
            commands::run_ats_validation_suite,
            commands::simulate_multiple_ats_systems,
            // Phase 4 Advanced Optimization Commands
            commands::analyze_achievements,
            commands::generate_comprehensive_optimization,
            commands::get_realtime_suggestions,
            commands::validate_xyz_formula,
            commands::get_achievement_suggestions,
            // Phase 5 Competitive Features Commands
            commands::generate_competitive_analysis,
            commands::get_market_position_analysis,
            commands::get_salary_insights,
            commands::get_hiring_probability,
            // Phase 6 Advanced AI Integration & Machine Learning Commands
            commands::generate_ml_insights,
            commands::predict_application_success,
            commands::get_career_path_suggestions,
            commands::get_salary_prediction_ml,
            commands::get_ml_recommendations,
        ])
        .setup(|_app| {
            info!("Application setup completed");
            // Note: Database upgrade to use app handle is handled by the Database methods
            // which include fallback strategies for path resolution
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
