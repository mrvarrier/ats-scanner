// Library exports for integration testing

pub mod commands;
pub mod config;
pub mod database;
pub mod document;
pub mod models;
pub mod ollama;
pub mod plugin_system;
pub mod scoring;
pub mod utils;
// Advanced Scoring Engine
pub mod advanced_scoring;
// Phase 2 Enhanced Analysis Modules
pub mod ats_simulator;
pub mod enhanced_prompts;
pub mod enhanced_scoring;
pub mod industry_analyzer;
pub mod semantic_analyzer;
// Phase 3 ATS Format & Testing Modules
pub mod format_checker;
pub mod format_issue_detector;
pub mod testing_framework;
// Phase 4 Advanced Optimization Modules
pub mod achievement_analyzer;
pub mod realtime_optimizer;
pub mod smart_optimizer;
// Phase 5 Competitive Features
pub mod competitive_analyzer;
// Phase 6 Advanced AI Integration & Machine Learning
pub mod ml_insights;
// Modern NLP-Based Keyword Extraction (2024-2025)
pub mod modern_keyword_extractor;

use config::ConfigManager;
use database::Database;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Database>>,
    pub config: Arc<Mutex<ConfigManager>>,
}
