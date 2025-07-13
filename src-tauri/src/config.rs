use anyhow::{Context, Result};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::models::{
    AnalysisConfig, AppConfig, LoggingConfig, OllamaConfig, OptimizationLevel, PerformanceConfig,
};

#[derive(Debug, Clone)]
pub struct ConfigManager {
    config_path: PathBuf,
    config: AppConfig,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        let config = Self::load_or_create_default_config(&config_path)?;

        Ok(ConfigManager {
            config_path,
            config,
        })
    }

    pub fn new_with_path(config_path: PathBuf) -> Result<Self> {
        let config = Self::load_or_create_default_config(&config_path)?;

        Ok(ConfigManager {
            config_path,
            config,
        })
    }

    fn get_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
            .context("Could not determine config directory")?;

        let app_config_dir = config_dir.join("ats-scanner");

        // Create config directory if it doesn't exist
        if !app_config_dir.exists() {
            fs::create_dir_all(&app_config_dir).context("Failed to create config directory")?;
            info!("Created config directory: {:?}", app_config_dir);
        }

        Ok(app_config_dir.join("config.json"))
    }

    fn load_or_create_default_config(config_path: &PathBuf) -> Result<AppConfig> {
        if config_path.exists() {
            info!("Loading configuration from: {:?}", config_path);
            Self::load_config(config_path)
        } else {
            info!("Creating default configuration at: {:?}", config_path);
            let config = Self::default_config();
            Self::save_config_to_path(&config, config_path)?;
            Ok(config)
        }
    }

    fn load_config(config_path: &PathBuf) -> Result<AppConfig> {
        let config_str = fs::read_to_string(config_path).context("Failed to read config file")?;

        let config: AppConfig =
            serde_json::from_str(&config_str).context("Failed to parse config file")?;

        info!("Configuration loaded successfully");
        Ok(config)
    }

    fn save_config_to_path(config: &AppConfig, config_path: &PathBuf) -> Result<()> {
        let config_str =
            serde_json::to_string_pretty(config).context("Failed to serialize config")?;

        fs::write(config_path, config_str).context("Failed to write config file")?;

        info!("Configuration saved to: {:?}", config_path);
        Ok(())
    }

    fn default_config() -> AppConfig {
        let default_db_path = if let Some(home_dir) = dirs::home_dir() {
            home_dir.join(".ats-scanner").join("ats_scanner.db")
        } else {
            PathBuf::from("./data/ats_scanner.db")
        };
        let database_url = format!("sqlite:{}", default_db_path.to_string_lossy());

        AppConfig {
            database_url,
            ollama_config: OllamaConfig {
                host: "localhost".to_string(),
                port: 11434,
                timeout_seconds: 30,
                max_retries: 3,
                default_model: "llama2".to_string(),
                models: vec![
                    "llama2".to_string(),
                    "codellama".to_string(),
                    "mistral".to_string(),
                    "neural-chat".to_string(),
                ],
            },
            analysis_config: AnalysisConfig {
                enable_industry_analysis: true,
                enable_ats_compatibility: true,
                enable_benchmark_comparison: true,
                default_optimization_level: OptimizationLevel::Balanced,
                max_suggestions: 10,
                confidence_threshold: 0.7,
            },
            performance_config: PerformanceConfig {
                max_concurrent_analyses: 3,
                cache_size_mb: 256,
                enable_gpu_acceleration: false,
                memory_limit_mb: 1024,
                timeout_seconds: 300,
            },
            logging_config: LoggingConfig {
                level: "info".to_string(),
                file_path: None,
                enable_telemetry: false,
                enable_performance_metrics: true,
            },
        }
    }

    // Getter methods
    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }

    pub fn get_database_url(&self) -> &str {
        &self.config.database_url
    }

    pub fn get_ollama_config(&self) -> &OllamaConfig {
        &self.config.ollama_config
    }

    pub fn get_analysis_config(&self) -> &AnalysisConfig {
        &self.config.analysis_config
    }

    pub fn get_performance_config(&self) -> &PerformanceConfig {
        &self.config.performance_config
    }

    pub fn get_logging_config(&self) -> &LoggingConfig {
        &self.config.logging_config
    }

    // Update methods
    pub fn update_ollama_config(&mut self, ollama_config: OllamaConfig) -> Result<()> {
        self.config.ollama_config = ollama_config;
        self.save_config()
    }

    pub fn update_analysis_config(&mut self, analysis_config: AnalysisConfig) -> Result<()> {
        self.config.analysis_config = analysis_config;
        self.save_config()
    }

    pub fn update_performance_config(
        &mut self,
        performance_config: PerformanceConfig,
    ) -> Result<()> {
        self.config.performance_config = performance_config;
        self.save_config()
    }

    pub fn update_logging_config(&mut self, logging_config: LoggingConfig) -> Result<()> {
        self.config.logging_config = logging_config;
        self.save_config()
    }

    pub fn update_database_url(&mut self, database_url: String) -> Result<()> {
        self.config.database_url = database_url;
        self.save_config()
    }

    // Save current config to file
    pub fn save_config(&self) -> Result<()> {
        Self::save_config_to_path(&self.config, &self.config_path)
    }

    // Validate configuration
    pub fn validate_config(&self) -> Result<Vec<String>> {
        let mut warnings = Vec::new();

        // Validate database URL
        if let Some(db_warnings) = self.validate_database_url() {
            warnings.extend(db_warnings);
        }

        // Validate Ollama config
        if self.config.ollama_config.port == 0 {
            warnings.push("Ollama port cannot be 0".to_string());
        }

        if self.config.ollama_config.default_model.is_empty() {
            warnings.push("Default Ollama model must be specified".to_string());
        }

        // Validate performance config
        if self.config.performance_config.max_concurrent_analyses == 0 {
            warnings.push("Max concurrent analyses must be at least 1".to_string());
        }

        if self.config.performance_config.cache_size_mb < 64 {
            warnings.push("Cache size should be at least 64MB for optimal performance".to_string());
        }

        if self.config.performance_config.memory_limit_mb < 512 {
            warnings.push("Memory limit should be at least 512MB".to_string());
        }

        // Validate analysis config
        if self.config.analysis_config.max_suggestions == 0 {
            warnings.push("Max suggestions should be at least 1".to_string());
        }

        if self.config.analysis_config.confidence_threshold < 0.0
            || self.config.analysis_config.confidence_threshold > 1.0
        {
            warnings.push("Confidence threshold must be between 0.0 and 1.0".to_string());
        }

        // Log warnings
        for warning in &warnings {
            warn!("Configuration warning: {}", warning);
        }

        Ok(warnings)
    }

    // Validate and sanitize database URL
    fn validate_database_url(&self) -> Option<Vec<String>> {
        let mut warnings = Vec::new();
        let db_url = &self.config.database_url;

        // Check if database URL is valid
        if db_url.is_empty() {
            warnings.push("Database URL cannot be empty".to_string());
            return Some(warnings);
        }

        // Validate SQLite URLs
        if db_url.starts_with("sqlite:") {
            let db_path_str = db_url.strip_prefix("sqlite:").unwrap_or(db_url);

            // Check for in-memory database
            if db_path_str.contains(":memory:") {
                info!("Using in-memory database (data will not persist)");
                return None;
            }

            // Validate file path
            let db_path = PathBuf::from(db_path_str);

            // Check for relative paths that might cause issues
            if db_path_str.starts_with("./") || db_path_str.starts_with("../") {
                warnings.push(format!(
                    "Relative database path '{}' may cause issues. Consider using absolute paths.",
                    db_path_str
                ));
            }

            // Check parent directory accessibility
            if let Some(parent) = db_path.parent() {
                if parent.to_string_lossy().is_empty() {
                    warnings.push("Database path has no parent directory".to_string());
                } else {
                    // Note: Directory creation will be handled async during database initialization
                    info!("Database directory will be created: {:?}", parent);
                }
            }

            // Check for potentially problematic characters
            if db_path_str.contains(' ') && !db_path_str.starts_with('"') {
                warnings.push(
                    "Database path contains spaces. Consider using quotes or avoiding spaces."
                        .to_string(),
                );
            }

            // Check for very long paths
            if db_path_str.len() > 260 {
                warnings.push(
                    "Database path is very long and may cause issues on some systems".to_string(),
                );
            }
        } else {
            warnings.push(format!("Unsupported database URL format: '{}'", db_url));
        }

        if warnings.is_empty() {
            None
        } else {
            Some(warnings)
        }
    }

    // Sanitize database URL
    pub fn sanitize_database_url(&mut self) -> Result<()> {
        let db_url = &self.config.database_url;

        // Skip sanitization for in-memory databases
        if db_url.contains(":memory:") {
            return Ok(());
        }

        if db_url.starts_with("sqlite:") {
            let db_path_str = db_url.strip_prefix("sqlite:").unwrap_or(db_url);
            let db_path = PathBuf::from(db_path_str);

            // Convert relative paths to absolute when possible
            if db_path.is_relative() {
                if let Ok(current_dir) = std::env::current_dir() {
                    let absolute_path = current_dir.join(&db_path);
                    let sanitized_url = format!("sqlite:{}", absolute_path.to_string_lossy());
                    self.config.database_url = sanitized_url;
                    info!(
                        "Sanitized database URL to absolute path: {}",
                        self.config.database_url
                    );
                }
            }

            // Normalize path separators
            let normalized_path = db_path.to_string_lossy().replace('\\', "/");
            if normalized_path != db_path.to_string_lossy() {
                self.config.database_url = format!("sqlite:{}", normalized_path);
                info!(
                    "Normalized database URL path separators: {}",
                    self.config.database_url
                );
            }
        }

        Ok(())
    }

    // Reset to default configuration
    pub fn reset_to_default(&mut self) -> Result<()> {
        self.config = Self::default_config();
        self.save_config()
    }

    // Environment variable overrides
    pub fn apply_env_overrides(&mut self) -> Result<()> {
        if let Ok(db_url) = std::env::var("ATS_DATABASE_URL") {
            self.config.database_url = db_url;
            info!("Database URL overridden from environment");
        }

        if let Ok(ollama_host) = std::env::var("ATS_OLLAMA_HOST") {
            self.config.ollama_config.host = ollama_host;
            info!("Ollama host overridden from environment");
        }

        if let Ok(ollama_port) = std::env::var("ATS_OLLAMA_PORT") {
            if let Ok(port) = ollama_port.parse::<u16>() {
                self.config.ollama_config.port = port;
                info!("Ollama port overridden from environment");
            }
        }

        if let Ok(default_model) = std::env::var("ATS_DEFAULT_MODEL") {
            self.config.ollama_config.default_model = default_model;
            info!("Default model overridden from environment");
        }

        if let Ok(log_level) = std::env::var("ATS_LOG_LEVEL") {
            self.config.logging_config.level = log_level;
            info!("Log level overridden from environment");
        }

        if let Ok(max_concurrent) = std::env::var("ATS_MAX_CONCURRENT") {
            if let Ok(concurrent) = max_concurrent.parse::<usize>() {
                self.config.performance_config.max_concurrent_analyses = concurrent;
                info!("Max concurrent analyses overridden from environment");
            }
        }

        Ok(())
    }

    // Export configuration for debugging
    pub fn export_config(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.config).context("Failed to export configuration")
    }

    // Check if configuration file exists
    pub fn config_exists(&self) -> bool {
        self.config_path.exists()
    }

    // Get configuration file path
    pub fn get_config_file_path(&self) -> &PathBuf {
        &self.config_path
    }
}

// Configuration update structures for partial updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfigUpdate {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub timeout_seconds: Option<u64>,
    pub max_retries: Option<u32>,
    pub default_model: Option<String>,
    pub models: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfigUpdate {
    pub enable_industry_analysis: Option<bool>,
    pub enable_ats_compatibility: Option<bool>,
    pub enable_benchmark_comparison: Option<bool>,
    pub default_optimization_level: Option<OptimizationLevel>,
    pub max_suggestions: Option<usize>,
    pub confidence_threshold: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfigUpdate {
    pub max_concurrent_analyses: Option<usize>,
    pub cache_size_mb: Option<usize>,
    pub enable_gpu_acceleration: Option<bool>,
    pub memory_limit_mb: Option<usize>,
    pub timeout_seconds: Option<u64>,
}

impl ConfigManager {
    // Partial update methods
    pub fn partial_update_ollama(&mut self, update: OllamaConfigUpdate) -> Result<()> {
        if let Some(host) = update.host {
            self.config.ollama_config.host = host;
        }
        if let Some(port) = update.port {
            self.config.ollama_config.port = port;
        }
        if let Some(timeout) = update.timeout_seconds {
            self.config.ollama_config.timeout_seconds = timeout;
        }
        if let Some(retries) = update.max_retries {
            self.config.ollama_config.max_retries = retries;
        }
        if let Some(model) = update.default_model {
            self.config.ollama_config.default_model = model;
        }
        if let Some(models) = update.models {
            self.config.ollama_config.models = models;
        }

        self.save_config()
    }

    pub fn partial_update_analysis(&mut self, update: AnalysisConfigUpdate) -> Result<()> {
        if let Some(industry_analysis) = update.enable_industry_analysis {
            self.config.analysis_config.enable_industry_analysis = industry_analysis;
        }
        if let Some(ats_compatibility) = update.enable_ats_compatibility {
            self.config.analysis_config.enable_ats_compatibility = ats_compatibility;
        }
        if let Some(benchmark_comparison) = update.enable_benchmark_comparison {
            self.config.analysis_config.enable_benchmark_comparison = benchmark_comparison;
        }
        if let Some(opt_level) = update.default_optimization_level {
            self.config.analysis_config.default_optimization_level = opt_level;
        }
        if let Some(max_suggestions) = update.max_suggestions {
            self.config.analysis_config.max_suggestions = max_suggestions;
        }
        if let Some(confidence) = update.confidence_threshold {
            self.config.analysis_config.confidence_threshold = confidence;
        }

        self.save_config()
    }

    pub fn partial_update_performance(&mut self, update: PerformanceConfigUpdate) -> Result<()> {
        if let Some(max_concurrent) = update.max_concurrent_analyses {
            self.config.performance_config.max_concurrent_analyses = max_concurrent;
        }
        if let Some(cache_size) = update.cache_size_mb {
            self.config.performance_config.cache_size_mb = cache_size;
        }
        if let Some(gpu_acceleration) = update.enable_gpu_acceleration {
            self.config.performance_config.enable_gpu_acceleration = gpu_acceleration;
        }
        if let Some(memory_limit) = update.memory_limit_mb {
            self.config.performance_config.memory_limit_mb = memory_limit;
        }
        if let Some(timeout) = update.timeout_seconds {
            self.config.performance_config.timeout_seconds = timeout;
        }

        self.save_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_config_creation() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.json");

        let config_manager = ConfigManager::new_with_path(config_path.clone()).unwrap();

        assert!(config_path.exists());
        assert_eq!(config_manager.get_ollama_config().port, 11434);
    }

    #[test]
    fn test_config_validation() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.json");

        let mut config_manager = ConfigManager::new_with_path(config_path).unwrap();

        // Test valid config
        let warnings = config_manager.validate_config().unwrap();
        assert!(warnings.is_empty());

        // Test invalid config
        config_manager.config.ollama_config.port = 0;
        config_manager
            .config
            .performance_config
            .max_concurrent_analyses = 0;

        let warnings = config_manager.validate_config().unwrap();
        assert!(warnings.len() >= 2);
    }

    #[test]
    fn test_partial_updates() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.json");

        let mut config_manager = ConfigManager::new_with_path(config_path).unwrap();

        // Test ollama config update
        let ollama_update = OllamaConfigUpdate {
            host: Some("new_host".to_string()),
            port: Some(8080),
            timeout_seconds: None,
            max_retries: None,
            default_model: None,
            models: None,
        };

        config_manager.partial_update_ollama(ollama_update).unwrap();

        assert_eq!(config_manager.get_ollama_config().host, "new_host");
        assert_eq!(config_manager.get_ollama_config().port, 8080);
        // Unchanged values should remain the same
        assert_eq!(config_manager.get_ollama_config().timeout_seconds, 30);
    }
}
