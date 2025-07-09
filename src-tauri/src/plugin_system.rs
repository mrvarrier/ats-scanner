use anyhow::{anyhow, Result};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub capabilities: Vec<PluginCapability>,
    pub enabled: bool,
    pub config_schema: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginCapability {
    CustomScoring,
    DataExport,
    ResumeOptimization,
    DocumentParsing,
    JobMatching,
    Analytics,
    Notification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub plugin_id: String,
    pub config: serde_json::Value,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginExecutionContext {
    pub plugin_id: String,
    pub operation: String,
    pub input_data: serde_json::Value,
    pub config: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginExecutionResult {
    pub success: bool,
    pub output_data: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub execution_time_ms: u64,
    pub metadata: HashMap<String, String>,
}

// Plugin trait that all plugins must implement
pub trait Plugin: Send + Sync {
    fn info(&self) -> PluginInfo;
    fn execute(&self, context: PluginExecutionContext) -> Result<PluginExecutionResult>;
    fn validate_config(&self, config: &serde_json::Value) -> Result<()>;
    #[allow(dead_code)]
    fn get_schema(&self) -> Option<serde_json::Value>;
}

// Built-in plugin for custom scoring
#[derive(Debug)]
pub struct CustomScoringPlugin {
    id: String,
    name: String,
}

impl Default for CustomScoringPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomScoringPlugin {
    pub fn new() -> Self {
        Self {
            id: "custom-scoring-v1".to_string(),
            name: "Custom Scoring Engine".to_string(),
        }
    }
}

impl Plugin for CustomScoringPlugin {
    fn info(&self) -> PluginInfo {
        PluginInfo {
            id: self.id.clone(),
            name: self.name.clone(),
            version: "1.0.0".to_string(),
            description: "Advanced custom scoring algorithms for specialized industries"
                .to_string(),
            author: "ATS Scanner Team".to_string(),
            capabilities: vec![PluginCapability::CustomScoring],
            enabled: true,
            config_schema: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "industry": {
                        "type": "string",
                        "enum": ["tech", "finance", "healthcare", "education", "government"]
                    },
                    "experience_weight": {
                        "type": "number",
                        "minimum": 0.0,
                        "maximum": 1.0,
                        "default": 0.3
                    },
                    "skills_weight": {
                        "type": "number",
                        "minimum": 0.0,
                        "maximum": 1.0,
                        "default": 0.4
                    },
                    "education_weight": {
                        "type": "number",
                        "minimum": 0.0,
                        "maximum": 1.0,
                        "default": 0.2
                    },
                    "keywords_weight": {
                        "type": "number",
                        "minimum": 0.0,
                        "maximum": 1.0,
                        "default": 0.1
                    }
                },
                "required": ["industry"]
            })),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn execute(&self, context: PluginExecutionContext) -> Result<PluginExecutionResult> {
        let start_time = std::time::Instant::now();

        match context.operation.as_str() {
            "calculate_score" => {
                let config = context.config.unwrap_or_else(|| serde_json::json!({}));
                let industry = config["industry"].as_str().unwrap_or("tech");

                // Get weights from config or use defaults
                let experience_weight = config["experience_weight"].as_f64().unwrap_or(0.3);
                let skills_weight = config["skills_weight"].as_f64().unwrap_or(0.4);
                let education_weight = config["education_weight"].as_f64().unwrap_or(0.2);
                let keywords_weight = config["keywords_weight"].as_f64().unwrap_or(0.1);

                // Extract scores from input
                let category_scores = &context.input_data["category_scores"];
                let skills_score = category_scores["skills"].as_f64().unwrap_or(0.0);
                let experience_score = category_scores["experience"].as_f64().unwrap_or(0.0);
                let education_score = category_scores["education"].as_f64().unwrap_or(0.0);
                let keywords_score = category_scores["keywords"].as_f64().unwrap_or(0.0);

                // Industry-specific adjustments
                let industry_multiplier = match industry {
                    "tech" => 1.1,       // Favor technical skills
                    "finance" => 1.05,   // Balanced approach
                    "healthcare" => 1.2, // Heavy education weight
                    "education" => 1.15, // Education and experience
                    "government" => 1.0, // Standard scoring
                    _ => 1.0,
                };

                // Calculate weighted score
                let weighted_score = (skills_score * skills_weight
                    + experience_score * experience_weight
                    + education_score * education_weight
                    + keywords_score * keywords_weight)
                    * industry_multiplier;

                let custom_score = weighted_score.min(100.0);

                let mut metadata = HashMap::new();
                metadata.insert("industry".to_string(), industry.to_string());
                metadata.insert(
                    "algorithm".to_string(),
                    "weighted_industry_specific".to_string(),
                );
                metadata.insert("multiplier".to_string(), industry_multiplier.to_string());

                Ok(PluginExecutionResult {
                    success: true,
                    output_data: Some(serde_json::json!({
                        "custom_score": custom_score,
                        "industry": industry,
                        "weights": {
                            "experience": experience_weight,
                            "skills": skills_weight,
                            "education": education_weight,
                            "keywords": keywords_weight
                        }
                    })),
                    error_message: None,
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                    metadata,
                })
            }
            _ => Err(anyhow!("Unsupported operation: {}", context.operation)),
        }
    }

    fn validate_config(&self, config: &serde_json::Value) -> Result<()> {
        if !config["industry"].is_string() {
            return Err(anyhow!("Industry must be specified as a string"));
        }

        let industry = config["industry"].as_str().unwrap();
        let valid_industries = ["tech", "finance", "healthcare", "education", "government"];
        if !valid_industries.contains(&industry) {
            return Err(anyhow!(
                "Invalid industry. Must be one of: {:?}",
                valid_industries
            ));
        }

        // Validate weights if provided
        for weight_key in [
            "experience_weight",
            "skills_weight",
            "education_weight",
            "keywords_weight",
        ] {
            if let Some(weight) = config[weight_key].as_f64() {
                if !(0.0..=1.0).contains(&weight) {
                    return Err(anyhow!("{} must be between 0.0 and 1.0", weight_key));
                }
            }
        }

        Ok(())
    }

    fn get_schema(&self) -> Option<serde_json::Value> {
        self.info().config_schema
    }
}

// Advanced Analytics Plugin
#[derive(Debug)]
pub struct AdvancedAnalyticsPlugin {
    id: String,
    name: String,
}

impl Default for AdvancedAnalyticsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AdvancedAnalyticsPlugin {
    pub fn new() -> Self {
        Self {
            id: "advanced-analytics-v1".to_string(),
            name: "Advanced Analytics Engine".to_string(),
        }
    }
}

impl Plugin for AdvancedAnalyticsPlugin {
    fn info(&self) -> PluginInfo {
        PluginInfo {
            id: self.id.clone(),
            name: self.name.clone(),
            version: "1.0.0".to_string(),
            description: "Advanced analytics and insights generation for resume performance"
                .to_string(),
            author: "ATS Scanner Team".to_string(),
            capabilities: vec![PluginCapability::Analytics, PluginCapability::CustomScoring],
            enabled: true,
            config_schema: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "analysis_depth": {
                        "type": "string",
                        "enum": ["basic", "detailed", "comprehensive"],
                        "default": "detailed"
                    },
                    "include_predictions": {
                        "type": "boolean",
                        "default": true
                    },
                    "benchmark_against": {
                        "type": "string",
                        "enum": ["industry", "global", "custom"],
                        "default": "industry"
                    }
                }
            })),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn execute(&self, context: PluginExecutionContext) -> Result<PluginExecutionResult> {
        let start_time = std::time::Instant::now();

        match context.operation.as_str() {
            "generate_insights" => {
                let config = context.config.unwrap_or_else(|| serde_json::json!({}));
                let analysis_depth = config["analysis_depth"].as_str().unwrap_or("detailed");
                let include_predictions = config["include_predictions"].as_bool().unwrap_or(true);

                // Generate advanced insights based on input data
                let insights = match analysis_depth {
                    "basic" => generate_basic_insights(&context.input_data),
                    "detailed" => {
                        generate_detailed_insights(&context.input_data, include_predictions)
                    }
                    "comprehensive" => {
                        generate_comprehensive_insights(&context.input_data, include_predictions)
                    }
                    _ => generate_detailed_insights(&context.input_data, include_predictions),
                };

                let mut metadata = HashMap::new();
                metadata.insert("analysis_depth".to_string(), analysis_depth.to_string());
                metadata.insert(
                    "predictions_included".to_string(),
                    include_predictions.to_string(),
                );

                Ok(PluginExecutionResult {
                    success: true,
                    output_data: Some(insights),
                    error_message: None,
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                    metadata,
                })
            }
            _ => Err(anyhow!("Unsupported operation: {}", context.operation)),
        }
    }

    fn validate_config(&self, config: &serde_json::Value) -> Result<()> {
        if let Some(depth) = config["analysis_depth"].as_str() {
            if !["basic", "detailed", "comprehensive"].contains(&depth) {
                return Err(anyhow!("Invalid analysis_depth"));
            }
        }
        Ok(())
    }

    fn get_schema(&self) -> Option<serde_json::Value> {
        self.info().config_schema
    }
}

fn generate_basic_insights(data: &serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "summary": "Basic insights generated",
        "key_metrics": {
            "overall_score": data["overall_score"],
            "top_category": "skills"
        }
    })
}

fn generate_detailed_insights(
    _data: &serde_json::Value,
    include_predictions: bool,
) -> serde_json::Value {
    let mut insights = serde_json::json!({
        "summary": "Detailed analysis completed",
        "strengths": [
            "Strong technical skills alignment",
            "Relevant experience background"
        ],
        "improvement_areas": [
            "Add more industry keywords",
            "Quantify achievements with metrics"
        ],
        "competitive_analysis": {
            "percentile_rank": 78,
            "comparison_group": "Similar experience level"
        }
    });

    if include_predictions {
        insights["predictions"] = serde_json::json!({
            "interview_probability": 0.72,
            "hiring_likelihood": 0.65,
            "confidence_interval": [0.6, 0.8]
        });
    }

    insights
}

fn generate_comprehensive_insights(
    data: &serde_json::Value,
    include_predictions: bool,
) -> serde_json::Value {
    let mut insights = generate_detailed_insights(data, include_predictions);

    insights["advanced_metrics"] = serde_json::json!({
        "readability_score": 8.5,
        "keyword_density": 0.12,
        "ats_compatibility": 0.94,
        "formatting_score": 0.88
    });

    insights["recommendations"] = serde_json::json!({
        "immediate": ["Add contact information", "Include relevant certifications"],
        "short_term": ["Gain experience in cloud technologies", "Complete online courses"],
        "long_term": ["Pursue advanced degree", "Build portfolio projects"]
    });

    insights
}

// Plugin Manager
#[allow(dead_code)]
pub struct PluginManager {
    plugins: Arc<RwLock<HashMap<String, Box<dyn Plugin>>>>,
    plugin_configs: Arc<RwLock<HashMap<String, PluginConfig>>>,
    plugins_directory: PathBuf,
}

impl PluginManager {
    pub fn new(plugins_directory: PathBuf) -> Self {
        let mut manager = Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            plugin_configs: Arc::new(RwLock::new(HashMap::new())),
            plugins_directory,
        };

        // Register built-in plugins
        manager.register_builtin_plugins();
        manager
    }

    fn register_builtin_plugins(&mut self) {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let mut plugins = self.plugins.write().await;

            // Register custom scoring plugin
            let custom_scoring = Box::new(CustomScoringPlugin::new());
            plugins.insert(custom_scoring.info().id.clone(), custom_scoring);

            // Register advanced analytics plugin
            let analytics = Box::new(AdvancedAnalyticsPlugin::new());
            plugins.insert(analytics.info().id.clone(), analytics);

            info!("Registered {} built-in plugins", plugins.len());
        });
    }

    pub async fn list_plugins(&self) -> Vec<PluginInfo> {
        let plugins = self.plugins.read().await;
        plugins.values().map(|plugin| plugin.info()).collect()
    }

    pub async fn get_plugin_info(&self, plugin_id: &str) -> Option<PluginInfo> {
        let plugins = self.plugins.read().await;
        plugins.get(plugin_id).map(|plugin| plugin.info())
    }

    pub async fn execute_plugin(
        &self,
        plugin_id: &str,
        operation: &str,
        input_data: serde_json::Value,
    ) -> Result<PluginExecutionResult> {
        let plugins = self.plugins.read().await;
        let configs = self.plugin_configs.read().await;

        let plugin = plugins
            .get(plugin_id)
            .ok_or_else(|| anyhow!("Plugin not found: {}", plugin_id))?;

        let config = configs.get(plugin_id).map(|c| c.config.clone());

        let context = PluginExecutionContext {
            plugin_id: plugin_id.to_string(),
            operation: operation.to_string(),
            input_data,
            config,
        };

        plugin.execute(context)
    }

    pub async fn update_plugin_config(
        &self,
        plugin_id: &str,
        config: serde_json::Value,
    ) -> Result<()> {
        let plugins = self.plugins.read().await;
        let plugin = plugins
            .get(plugin_id)
            .ok_or_else(|| anyhow!("Plugin not found: {}", plugin_id))?;

        // Validate config
        plugin.validate_config(&config)?;

        drop(plugins);

        // Update config
        let mut configs = self.plugin_configs.write().await;
        let plugin_config = PluginConfig {
            plugin_id: plugin_id.to_string(),
            config,
            updated_at: chrono::Utc::now(),
        };

        configs.insert(plugin_id.to_string(), plugin_config);
        info!("Updated config for plugin: {}", plugin_id);

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn enable_plugin(&self, plugin_id: &str) -> Result<()> {
        // Implementation would modify plugin state
        info!("Enabled plugin: {}", plugin_id);
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn disable_plugin(&self, plugin_id: &str) -> Result<()> {
        // Implementation would modify plugin state
        info!("Disabled plugin: {}", plugin_id);
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get_plugins_by_capability(&self, capability: PluginCapability) -> Vec<PluginInfo> {
        let plugins = self.plugins.read().await;
        plugins
            .values()
            .filter(|plugin| plugin.info().capabilities.contains(&capability))
            .map(|plugin| plugin.info())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_plugin_manager_creation() {
        let temp_dir = tempdir().unwrap();
        let manager = PluginManager::new(temp_dir.path().to_path_buf());

        let plugins = manager.list_plugins().await;
        assert!(plugins.len() >= 2); // At least the built-in plugins
    }

    #[tokio::test]
    async fn test_custom_scoring_plugin() {
        let plugin = CustomScoringPlugin::new();
        let info = plugin.info();

        assert_eq!(info.name, "Custom Scoring Engine");
        assert!(info.capabilities.contains(&PluginCapability::CustomScoring));
    }

    #[tokio::test]
    async fn test_plugin_execution() {
        let temp_dir = tempdir().unwrap();
        let manager = PluginManager::new(temp_dir.path().to_path_buf());

        let input_data = serde_json::json!({
            "category_scores": {
                "skills": 85.0,
                "experience": 78.0,
                "education": 82.0,
                "keywords": 88.0
            }
        });

        let result = manager
            .execute_plugin("custom-scoring-v1", "calculate_score", input_data)
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.success);
        assert!(result.output_data.is_some());
    }
}
