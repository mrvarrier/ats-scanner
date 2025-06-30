use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::{info, warn};

use crate::semantic_analyzer::SemanticAnalysisResult;
use crate::industry_analyzer::IndustryAnalysisResult;
// Enhanced scoring types imported when needed

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub name: String,
    pub category: String,
    pub template: String,
    pub variables: Vec<String>,
    pub model_specific: HashMap<String, String>,
    pub context_window_size: usize,
    pub temperature: f64,
    pub max_tokens: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub model_name: String,
    pub max_context_length: usize,
    pub optimal_temperature: f64,
    pub supports_system_message: bool,
    pub instruction_format: String, // "alpaca", "chatML", "llama2", etc.
    pub stop_tokens: Vec<String>,
    pub special_tokens: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedPromptRequest {
    pub prompt_type: String,
    pub model_name: String,
    pub resume_content: String,
    pub job_description: String,
    pub industry_context: Option<IndustryAnalysisResult>,
    pub semantic_context: Option<SemanticAnalysisResult>,
    pub analysis_focus: Vec<String>, // ["skills", "experience", "ats_compatibility", etc.]
    pub output_format: String, // "json", "structured_text", "analysis"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedPromptResponse {
    pub formatted_prompt: String,
    pub model_config: ModelConfig,
    pub estimated_tokens: usize,
    pub prompt_strategy: String,
    pub context_summary: String,
}

pub struct EnhancedPromptEngine {
    model_configs: HashMap<String, ModelConfig>,
    prompt_templates: HashMap<String, PromptTemplate>,
    context_strategies: HashMap<String, ContextStrategy>,
}

#[derive(Debug, Clone)]
struct ContextStrategy {
    name: String,
    max_context_ratio: f64, // Ratio of context window to use
    prioritization: Vec<String>, // Order of content importance
    compression_technique: String,
}

impl EnhancedPromptEngine {
    pub fn new() -> Self {
        let model_configs = Self::build_model_configs();
        let prompt_templates = Self::build_prompt_templates();
        let context_strategies = Self::build_context_strategies();

        EnhancedPromptEngine {
            model_configs,
            prompt_templates,
            context_strategies,
        }
    }

    pub fn create_enhanced_prompt(
        &self,
        request: EnhancedPromptRequest,
    ) -> Result<EnhancedPromptResponse> {
        info!("Creating enhanced prompt for model: {} with type: {}", 
              request.model_name, request.prompt_type);

        // 1. Get model configuration
        let model_config = self.get_model_config(&request.model_name)?;

        // 2. Select appropriate prompt template
        let template = self.select_prompt_template(&request.prompt_type, &request.model_name)?;

        // 3. Determine context strategy
        let strategy = self.select_context_strategy(&request.model_name, &template);

        // 4. Prepare context with prioritization
        let context = self.prepare_prioritized_context(&request, &strategy, &model_config)?;

        // 5. Format the prompt according to model requirements
        let formatted_prompt = self.format_prompt_for_model(&template, &context, &model_config, &request)?;

        // 6. Estimate token count
        let estimated_tokens = self.estimate_token_count(&formatted_prompt, &model_config);

        // 7. Validate and optimize if needed
        let (final_prompt, optimization_applied) = self.optimize_prompt_if_needed(
            formatted_prompt, 
            estimated_tokens, 
            &model_config, 
            &strategy
        )?;

        Ok(EnhancedPromptResponse {
            formatted_prompt: final_prompt,
            model_config: model_config.clone(),
            estimated_tokens,
            prompt_strategy: format!("{} with {}", template.name, strategy.name),
            context_summary: format!("Context prepared with {} strategy{}",
                strategy.name,
                if optimization_applied { " (optimized)" } else { "" }
            ),
        })
    }

    fn get_model_config(&self, model_name: &str) -> Result<&ModelConfig> {
        // Try exact match first
        if let Some(config) = self.model_configs.get(model_name) {
            return Ok(config);
        }

        // Try partial matches for model families
        let model_lower = model_name.to_lowercase();
        
        if model_lower.contains("llama") {
            return self.model_configs.get("llama2").context("Llama family config not found");
        } else if model_lower.contains("mistral") {
            return self.model_configs.get("mistral").context("Mistral family config not found");
        } else if model_lower.contains("code") || model_lower.contains("starcoder") {
            return self.model_configs.get("codellama").context("Code model config not found");
        } else if model_lower.contains("neural") || model_lower.contains("chat") {
            return self.model_configs.get("neural-chat").context("Chat model config not found");
        }

        // Default fallback
        self.model_configs.get("default").context("No suitable model config found")
    }

    fn select_prompt_template(&self, prompt_type: &str, model_name: &str) -> Result<&PromptTemplate> {
        // First try model-specific template
        let model_specific_key = format!("{}_{}", prompt_type, model_name);
        if let Some(template) = self.prompt_templates.get(&model_specific_key) {
            return Ok(template);
        }

        // Fallback to general template
        self.prompt_templates.get(prompt_type)
            .with_context(|| format!("No template found for prompt type: {}", prompt_type))
    }

    fn select_context_strategy(&self, model_name: &str, template: &PromptTemplate) -> &ContextStrategy {
        let model_lower = model_name.to_lowercase();
        
        // Select strategy based on model capabilities and template requirements
        if template.context_window_size > 8000 {
            self.context_strategies.get("comprehensive").unwrap_or(
                self.context_strategies.get("default").unwrap()
            )
        } else if model_lower.contains("code") {
            self.context_strategies.get("technical_focused").unwrap_or(
                self.context_strategies.get("default").unwrap()
            )
        } else if template.category == "analysis" {
            self.context_strategies.get("analytical").unwrap_or(
                self.context_strategies.get("default").unwrap()
            )
        } else {
            self.context_strategies.get("default").unwrap()
        }
    }

    fn prepare_prioritized_context(
        &self,
        request: &EnhancedPromptRequest,
        strategy: &ContextStrategy,
        model_config: &ModelConfig,
    ) -> Result<HashMap<String, String>> {
        let mut context = HashMap::new();
        let available_tokens = (model_config.max_context_length as f64 * strategy.max_context_ratio) as usize;

        // Base content (always included)
        context.insert("resume_content".to_string(), request.resume_content.clone());
        context.insert("job_description".to_string(), request.job_description.clone());

        // Calculate remaining space after base content
        let base_tokens = self.estimate_text_tokens(&request.resume_content) + 
                         self.estimate_text_tokens(&request.job_description);
        let remaining_tokens = available_tokens.saturating_sub(base_tokens);

        // Add contextual information based on priority
        let mut used_tokens = 0;
        
        for priority_item in &strategy.prioritization {
            if used_tokens >= remaining_tokens {
                break;
            }

            match priority_item.as_str() {
                "industry_analysis" => {
                    if let Some(industry_ctx) = &request.industry_context {
                        let industry_summary = self.summarize_industry_analysis(industry_ctx);
                        let tokens = self.estimate_text_tokens(&industry_summary);
                        
                        if used_tokens + tokens <= remaining_tokens {
                            context.insert("industry_analysis".to_string(), industry_summary);
                            used_tokens += tokens;
                        }
                    }
                }
                "semantic_analysis" => {
                    if let Some(semantic_ctx) = &request.semantic_context {
                        let semantic_summary = self.summarize_semantic_analysis(semantic_ctx);
                        let tokens = self.estimate_text_tokens(&semantic_summary);
                        
                        if used_tokens + tokens <= remaining_tokens {
                            context.insert("semantic_analysis".to_string(), semantic_summary);
                            used_tokens += tokens;
                        }
                    }
                }
                "focus_areas" => {
                    if !request.analysis_focus.is_empty() {
                        let focus_summary = request.analysis_focus.join(", ");
                        let tokens = self.estimate_text_tokens(&focus_summary);
                        
                        if used_tokens + tokens <= remaining_tokens {
                            context.insert("analysis_focus".to_string(), focus_summary);
                            used_tokens += tokens;
                        }
                    }
                }
                _ => {
                    warn!("Unknown priority item in context strategy: {}", priority_item);
                }
            }
        }

        // Add output format specification
        context.insert("output_format".to_string(), request.output_format.clone());

        info!("Context prepared: {} tokens used out of {} available", 
              used_tokens + base_tokens, available_tokens);

        Ok(context)
    }

    fn format_prompt_for_model(
        &self,
        template: &PromptTemplate,
        context: &HashMap<String, String>,
        model_config: &ModelConfig,
        request: &EnhancedPromptRequest,
    ) -> Result<String> {
        // Get the appropriate template for this model
        let base_template = if let Some(model_specific) = template.model_specific.get(&request.model_name) {
            model_specific
        } else {
            &template.template
        };

        // Replace template variables
        let mut formatted = base_template.clone();
        for (key, value) in context {
            let placeholder = format!("{{{}}}", key);
            formatted = formatted.replace(&placeholder, value);
        }

        // Apply model-specific formatting
        let final_prompt = match model_config.instruction_format.as_str() {
            "alpaca" => self.format_alpaca_style(&formatted, model_config),
            "chatML" => self.format_chatml_style(&formatted, model_config),
            "llama2" => self.format_llama2_style(&formatted, model_config),
            "mistral" => self.format_mistral_style(&formatted, model_config),
            _ => formatted, // Use as-is for unknown formats
        };

        Ok(final_prompt)
    }

    fn format_alpaca_style(&self, prompt: &str, _model_config: &ModelConfig) -> String {
        format!(
            "### Instruction:\n{}\n\n### Response:\n",
            prompt
        )
    }

    fn format_chatml_style(&self, prompt: &str, _model_config: &ModelConfig) -> String {
        format!(
            "<|im_start|>system\nYou are an expert resume analyzer and career consultant.<|im_end|>\n<|im_start|>user\n{}<|im_end|>\n<|im_start|>assistant\n",
            prompt
        )
    }

    fn format_llama2_style(&self, prompt: &str, _model_config: &ModelConfig) -> String {
        format!(
            "[INST] <<SYS>>\nYou are an expert resume analyzer. Provide detailed, actionable analysis.\n<</SYS>>\n\n{} [/INST]",
            prompt
        )
    }

    fn format_mistral_style(&self, prompt: &str, _model_config: &ModelConfig) -> String {
        format!(
            "<s>[INST] {} [/INST]",
            prompt
        )
    }

    fn estimate_token_count(&self, text: &str, model_config: &ModelConfig) -> usize {
        // Rough estimation: 1 token ≈ 4 characters for most models
        let char_count = text.chars().count();
        let estimated_tokens = (char_count as f64 / 4.0).ceil() as usize;
        
        // Apply model-specific adjustments
        let adjustment_factor = match model_config.model_name.as_str() {
            name if name.contains("llama") => 1.1, // Llama tends to use slightly more tokens
            name if name.contains("mistral") => 0.9, // Mistral is more efficient
            name if name.contains("code") => 1.2, // Code models use more tokens for symbols
            _ => 1.0,
        };

        (estimated_tokens as f64 * adjustment_factor) as usize
    }

    fn estimate_text_tokens(&self, text: &str) -> usize {
        // Simple heuristic for token estimation
        (text.chars().count() as f64 / 4.0).ceil() as usize
    }

    fn optimize_prompt_if_needed(
        &self,
        prompt: String,
        estimated_tokens: usize,
        model_config: &ModelConfig,
        strategy: &ContextStrategy,
    ) -> Result<(String, bool)> {
        let max_tokens = (model_config.max_context_length as f64 * strategy.max_context_ratio) as usize;
        
        if estimated_tokens <= max_tokens {
            return Ok((prompt, false));
        }

        info!("Prompt too long ({} tokens), optimizing for {} max tokens", 
              estimated_tokens, max_tokens);

        // Apply compression technique based on strategy
        let optimized_prompt = match strategy.compression_technique.as_str() {
            "truncate_context" => self.truncate_context(&prompt, max_tokens),
            "summarize_sections" => self.summarize_sections(&prompt, max_tokens),
            "remove_examples" => self.remove_examples(&prompt, max_tokens),
            _ => self.truncate_context(&prompt, max_tokens), // Default fallback
        };

        Ok((optimized_prompt, true))
    }

    fn truncate_context(&self, prompt: &str, max_tokens: usize) -> String {
        let target_chars = max_tokens * 4; // Rough conversion back to characters
        
        if prompt.len() <= target_chars {
            return prompt.to_string();
        }

        // Try to truncate at sentence boundaries
        let truncated = &prompt[..target_chars];
        if let Some(last_period) = truncated.rfind('.') {
            format!("{}.", &truncated[..last_period])
        } else if let Some(last_space) = truncated.rfind(' ') {
            truncated[..last_space].to_string()
        } else {
            truncated.to_string()
        }
    }

    fn summarize_sections(&self, prompt: &str, max_tokens: usize) -> String {
        // Simple implementation: keep the instruction and truncate content sections
        let sections: Vec<&str> = prompt.split("\n\n").collect();
        let mut result = String::new();
        let mut current_tokens = 0;
        let target_tokens = max_tokens;

        for section in sections {
            let section_tokens = self.estimate_text_tokens(section);
            
            if current_tokens + section_tokens <= target_tokens {
                if !result.is_empty() {
                    result.push_str("\n\n");
                }
                result.push_str(section);
                current_tokens += section_tokens;
            } else {
                // Add truncated version if space allows
                let remaining_tokens = target_tokens.saturating_sub(current_tokens);
                if remaining_tokens > 50 { // Only if we have meaningful space left
                    let truncated_section = self.truncate_context(section, remaining_tokens);
                    if !result.is_empty() {
                        result.push_str("\n\n");
                    }
                    result.push_str(&truncated_section);
                }
                break;
            }
        }

        result
    }

    fn remove_examples(&self, prompt: &str, max_tokens: usize) -> String {
        // Remove example sections to save space
        let without_examples = prompt
            .lines()
            .filter(|line| {
                let line_lower = line.to_lowercase();
                !line_lower.contains("example:") && 
                !line_lower.contains("for example") &&
                !line_lower.contains("e.g.")
            })
            .collect::<Vec<_>>()
            .join("\n");

        if self.estimate_text_tokens(&without_examples) <= max_tokens {
            without_examples
        } else {
            self.truncate_context(&without_examples, max_tokens)
        }
    }

    fn summarize_industry_analysis(&self, analysis: &IndustryAnalysisResult) -> String {
        format!(
            "Industry Analysis:\n- Detected Industry: {} (confidence: {:.1}%)\n- Role Level: {} (confidence: {:.1}%)\n- Domain Expertise Score: {:.1}%\n- Key Strengths: {}\n- Experience Estimate: {} years",
            analysis.detected_industry,
            analysis.confidence_score * 100.0,
            analysis.role_level_assessment.detected_level,
            analysis.role_level_assessment.confidence * 100.0,
            analysis.domain_expertise_score,
            analysis.role_level_assessment.experience_indicators.iter()
                .take(3)
                .map(|ei| &ei.description)
                .cloned()
                .collect::<Vec<_>>()
                .join(", "),
            analysis.role_level_assessment.years_of_experience_estimate.unwrap_or(0)
        )
    }

    fn summarize_semantic_analysis(&self, analysis: &SemanticAnalysisResult) -> String {
        let top_keywords: Vec<String> = analysis.keyword_matches.iter()
            .filter(|km| km.found_in_resume)
            .take(5)
            .map(|km| format!("{} ({:.1})", km.keyword, km.relevance_score))
            .collect();

        format!(
            "Semantic Analysis:\n- Industry Relevance: {:.1}%\n- Semantic Similarity: {:.1}%\n- Confidence: {:.1}%\n- Top Keywords Found: {}\n- Skill Gaps: {}",
            analysis.industry_relevance_score * 100.0,
            analysis.semantic_similarity_score * 100.0,
            analysis.confidence_score * 100.0,
            top_keywords.join(", "),
            analysis.skill_gaps.iter()
                .take(3)
                .map(|sg| &sg.missing_skill)
                .cloned()
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    // Static builders for configurations and templates
    fn build_model_configs() -> HashMap<String, ModelConfig> {
        let mut configs = HashMap::new();

        configs.insert("llama2".to_string(), ModelConfig {
            model_name: "llama2".to_string(),
            max_context_length: 4096,
            optimal_temperature: 0.1,
            supports_system_message: true,
            instruction_format: "llama2".to_string(),
            stop_tokens: vec!["</s>".to_string(), "[/INST]".to_string()],
            special_tokens: [
                ("bos".to_string(), "<s>".to_string()),
                ("eos".to_string(), "</s>".to_string()),
            ].iter().cloned().collect(),
        });

        configs.insert("mistral".to_string(), ModelConfig {
            model_name: "mistral".to_string(),
            max_context_length: 8192,
            optimal_temperature: 0.1,
            supports_system_message: false,
            instruction_format: "mistral".to_string(),
            stop_tokens: vec!["</s>".to_string()],
            special_tokens: HashMap::new(),
        });

        configs.insert("codellama".to_string(), ModelConfig {
            model_name: "codellama".to_string(),
            max_context_length: 4096,
            optimal_temperature: 0.05,
            supports_system_message: true,
            instruction_format: "llama2".to_string(),
            stop_tokens: vec!["</s>".to_string(), "[/INST]".to_string()],
            special_tokens: HashMap::new(),
        });

        configs.insert("neural-chat".to_string(), ModelConfig {
            model_name: "neural-chat".to_string(),
            max_context_length: 4096,
            optimal_temperature: 0.1,
            supports_system_message: true,
            instruction_format: "chatML".to_string(),
            stop_tokens: vec!["<|im_end|>".to_string()],
            special_tokens: HashMap::new(),
        });

        configs.insert("default".to_string(), ModelConfig {
            model_name: "default".to_string(),
            max_context_length: 2048,
            optimal_temperature: 0.1,
            supports_system_message: false,
            instruction_format: "alpaca".to_string(),
            stop_tokens: vec!["###".to_string()],
            special_tokens: HashMap::new(),
        });

        configs
    }

    fn build_prompt_templates() -> HashMap<String, PromptTemplate> {
        let mut templates = HashMap::new();

        templates.insert("comprehensive_analysis".to_string(), PromptTemplate {
            name: "Comprehensive Resume Analysis".to_string(),
            category: "analysis".to_string(),
            template: r#"Analyze the following resume against the job description with enhanced AI analysis.

RESUME CONTENT:
{resume_content}

JOB DESCRIPTION:
{job_description}

CONTEXT ANALYSIS:
{industry_analysis}
{semantic_analysis}

ANALYSIS FOCUS:
Please focus your analysis on: {analysis_focus}

Please provide a comprehensive analysis covering:
1. Skills alignment and gaps
2. Experience relevance and level assessment
3. Industry-specific insights
4. ATS compatibility considerations
5. Specific recommendations for improvement

Output format: {output_format}

Provide detailed, actionable insights that will help improve the candidate's resume effectiveness."#.to_string(),
            variables: vec![
                "resume_content".to_string(),
                "job_description".to_string(),
                "industry_analysis".to_string(),
                "semantic_analysis".to_string(),
                "analysis_focus".to_string(),
                "output_format".to_string(),
            ],
            model_specific: HashMap::new(),
            context_window_size: 6000,
            temperature: 0.1,
            max_tokens: Some(2048),
        });

        templates.insert("skills_analysis".to_string(), PromptTemplate {
            name: "Skills-Focused Analysis".to_string(),
            category: "analysis".to_string(),
            template: r#"Perform a detailed skills analysis of this resume against the job requirements.

RESUME:
{resume_content}

JOB REQUIREMENTS:
{job_description}

SEMANTIC CONTEXT:
{semantic_analysis}

Focus on:
- Technical skills matching
- Experience level appropriateness  
- Missing critical skills
- Skill development recommendations

Provide specific, actionable recommendations in {output_format} format."#.to_string(),
            variables: vec![
                "resume_content".to_string(),
                "job_description".to_string(),
                "semantic_analysis".to_string(),
                "output_format".to_string(),
            ],
            model_specific: HashMap::new(),
            context_window_size: 4000,
            temperature: 0.05,
            max_tokens: Some(1024),
        });

        templates.insert("ats_optimization".to_string(), PromptTemplate {
            name: "ATS Optimization Analysis".to_string(),
            category: "optimization".to_string(),
            template: r#"Analyze this resume for ATS (Applicant Tracking System) optimization.

RESUME:
{resume_content}

TARGET JOB:
{job_description}

Provide specific recommendations for:
1. Keyword optimization
2. Format improvements for ATS parsing
3. Section organization
4. Content structure optimization

Output in {output_format} format with specific, implementable suggestions."#.to_string(),
            variables: vec![
                "resume_content".to_string(),
                "job_description".to_string(),
                "output_format".to_string(),
            ],
            model_specific: HashMap::new(),
            context_window_size: 3000,
            temperature: 0.05,
            max_tokens: Some(1024),
        });

        templates
    }

    fn build_context_strategies() -> HashMap<String, ContextStrategy> {
        let mut strategies = HashMap::new();

        strategies.insert("comprehensive".to_string(), ContextStrategy {
            name: "Comprehensive Analysis".to_string(),
            max_context_ratio: 0.8,
            prioritization: vec![
                "industry_analysis".to_string(),
                "semantic_analysis".to_string(),
                "focus_areas".to_string(),
            ],
            compression_technique: "summarize_sections".to_string(),
        });

        strategies.insert("technical_focused".to_string(), ContextStrategy {
            name: "Technical Focus".to_string(),
            max_context_ratio: 0.7,
            prioritization: vec![
                "semantic_analysis".to_string(),
                "focus_areas".to_string(),
                "industry_analysis".to_string(),
            ],
            compression_technique: "remove_examples".to_string(),
        });

        strategies.insert("analytical".to_string(), ContextStrategy {
            name: "Analytical Deep Dive".to_string(),
            max_context_ratio: 0.85,
            prioritization: vec![
                "industry_analysis".to_string(),
                "semantic_analysis".to_string(),
                "focus_areas".to_string(),
            ],
            compression_technique: "summarize_sections".to_string(),
        });

        strategies.insert("default".to_string(), ContextStrategy {
            name: "Balanced Approach".to_string(),
            max_context_ratio: 0.75,
            prioritization: vec![
                "focus_areas".to_string(),
                "semantic_analysis".to_string(),
                "industry_analysis".to_string(),
            ],
            compression_technique: "truncate_context".to_string(),
        });

        strategies
    }
}

impl Default for EnhancedPromptEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_template_creation() {
        let engine = EnhancedPromptEngine::new();
        
        let request = EnhancedPromptRequest {
            prompt_type: "skills_analysis".to_string(),
            model_name: "llama2".to_string(),
            resume_content: "Software engineer with Python experience".to_string(),
            job_description: "Looking for Python developer".to_string(),
            industry_context: None,
            semantic_context: None,
            analysis_focus: vec!["skills".to_string()],
            output_format: "json".to_string(),
        };

        let result = engine.create_enhanced_prompt(request).unwrap();
        assert!(result.formatted_prompt.contains("Python"));
        assert!(result.formatted_prompt.contains("[INST]")); // Llama2 format
    }

    #[test]
    fn test_token_estimation() {
        let engine = EnhancedPromptEngine::new();
        let config = engine.model_configs.get("default").unwrap();
        
        let text = "This is a test string for token estimation.";
        let tokens = engine.estimate_token_count(text, config);
        
        assert!(tokens > 0);
        assert!(tokens < text.len()); // Should be less than character count
    }

    #[test]
    fn test_model_config_selection() {
        let engine = EnhancedPromptEngine::new();
        
        // Test exact match
        let config = engine.get_model_config("llama2").unwrap();
        assert_eq!(config.model_name, "llama2");
        
        // Test family match
        let config = engine.get_model_config("llama2-7b-chat").unwrap();
        assert_eq!(config.model_name, "llama2");
        
        // Test fallback
        let config = engine.get_model_config("unknown-model").unwrap();
        assert_eq!(config.model_name, "default");
    }
}