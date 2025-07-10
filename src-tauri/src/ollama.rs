use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use log::{error, info, warn};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

use crate::models::OllamaModel;

#[derive(Debug, Clone)]
pub struct OllamaClient {
    client: Client,
    base_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: Option<OllamaOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaOptions {
    temperature: Option<f64>,
    top_p: Option<f64>,
    max_tokens: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaGenerateResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
    context: Option<Vec<i32>>,
    total_duration: Option<i64>,
    load_duration: Option<i64>,
    prompt_eval_count: Option<i32>,
    prompt_eval_duration: Option<i64>,
    eval_count: Option<i32>,
    eval_duration: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaListResponse {
    models: Vec<OllamaModelInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaModelInfo {
    name: String,
    size: i64,
    digest: String,
    modified_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaShowRequest {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaShowResponse {
    license: Option<String>,
    modelfile: Option<String>,
    parameters: Option<String>,
    template: Option<String>,
}

impl OllamaClient {
    pub fn new(base_url: Option<String>) -> Result<Self> {
        let base_url = base_url.unwrap_or_else(|| "http://localhost:11434".to_string());
        let client = Client::builder()
            .timeout(Duration::from_secs(120)) // 2 minutes timeout for AI processing
            .connect_timeout(Duration::from_secs(10)) // 10 seconds connection timeout
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create HTTP client: {}", e))?;

        Ok(Self { client, base_url })
    }

    pub async fn test_connection(&self) -> Result<bool> {
        info!("Testing Ollama connection");

        match self
            .client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    info!("Successfully connected to Ollama");
                    Ok(true)
                } else {
                    warn!("Ollama responded with status: {}", response.status());
                    Ok(false)
                }
            }
            Err(e) => {
                error!("Failed to connect to Ollama: {}", e);
                Ok(false)
            }
        }
    }

    /// Lightweight health check for periodic monitoring
    /// Uses a shorter timeout and simpler endpoint for efficiency
    pub async fn health_check(&self) -> Result<bool> {
        let health_client = Client::builder()
            .timeout(Duration::from_secs(5)) // Shorter timeout for health checks
            .connect_timeout(Duration::from_secs(2))
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create health check client: {}", e))?;

        match health_client.get(&self.base_url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false), // Silent failure for health checks
        }
    }

    pub async fn list_models(&self) -> Result<Vec<OllamaModel>> {
        info!("Fetching available Ollama models");

        let response = self
            .client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Ollama API returned status: {}", response.status()));
        }

        let list_response: OllamaListResponse = response.json().await?;

        let models: Vec<OllamaModel> = list_response
            .models
            .into_iter()
            .map(|model| {
                let modified_at = model
                    .modified_at
                    .parse::<DateTime<Utc>>()
                    .unwrap_or_else(|_| Utc::now());

                OllamaModel {
                    name: model.name,
                    size: model.size,
                    digest: model.digest,
                    modified_at,
                }
            })
            .collect();

        info!("Found {} Ollama models", models.len());
        Ok(models)
    }

    pub async fn generate_response(
        &self,
        model: &str,
        prompt: &str,
        temperature: Option<f64>,
    ) -> Result<(String, i64)> {
        info!("Generating response with model: {}", model);
        let start_time = Instant::now();

        // Model-specific optimizations
        let (optimized_temperature, top_p, max_tokens) =
            self.get_model_optimizations(model, temperature);

        let options = Some(OllamaOptions {
            temperature: Some(optimized_temperature),
            top_p: Some(top_p),
            max_tokens: Some(max_tokens),
        });

        let request = OllamaGenerateRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
            options,
        };

        let response = self
            .client
            .post(format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Ollama API error: {} - {}", status, error_text));
        }

        let generate_response: OllamaGenerateResponse = response.json().await?;
        let processing_time = start_time.elapsed().as_millis() as i64;

        info!("Response generated successfully in {}ms", processing_time);

        Ok((generate_response.response, processing_time))
    }

    pub async fn analyze_resume_compatibility(
        &self,
        model: &str,
        resume_content: &str,
        job_description: &str,
    ) -> Result<(String, i64)> {
        let prompt = self.create_analysis_prompt(model, resume_content, job_description);
        let temperature = self.get_analysis_temperature(model);
        self.generate_response(model, &prompt, Some(temperature))
            .await
    }

    pub async fn optimize_resume(
        &self,
        model: &str,
        resume_content: &str,
        job_description: &str,
        optimization_level: &str,
    ) -> Result<(String, i64)> {
        let prompt = self.create_optimization_prompt(
            model,
            resume_content,
            job_description,
            optimization_level,
        );
        let temperature = self.get_optimization_temperature(model);
        self.generate_response(model, &prompt, Some(temperature))
            .await
    }

    #[allow(dead_code)]
    pub async fn extract_job_requirements(
        &self,
        model: &str,
        job_description: &str,
    ) -> Result<(String, i64)> {
        let prompt = self.create_job_analysis_prompt(model, job_description);
        let temperature = self.get_extraction_temperature(model);
        self.generate_response(model, &prompt, Some(temperature))
            .await
    }

    fn create_analysis_prompt(
        &self,
        model: &str,
        resume_content: &str,
        job_description: &str,
    ) -> String {
        let model_lower = model.to_lowercase();

        if model_lower.contains("mistral") {
            // Mistral-optimized prompt with clear structure and step-by-step guidance
            format!(
                r#"<s>[INST] You are an expert ATS (Applicant Tracking System) analyzer with deep knowledge of recruitment processes.

Task: Analyze the compatibility between the provided resume and job description.

Analysis Framework:
1. Calculate overall compatibility score (0-100)
2. Evaluate category-specific scores
3. Identify missing keywords and skills gaps
4. Provide actionable recommendations

Output Format (JSON only):
{{
    "overall_score": <number 0-100>,
    "category_scores": {{
        "skills": <number 0-100>,
        "experience": <number 0-100>,
        "education": <number 0-100>,
        "keywords": <number 0-100>,
        "format": <number 0-100>
    }},
    "detailed_feedback": "<comprehensive analysis with specific examples>",
    "missing_keywords": ["keyword1", "keyword2"],
    "recommendations": ["actionable suggestion 1", "actionable suggestion 2"]
}}

Resume Content:
{resume_content}

Job Description:
{job_description}

Provide only the JSON response: [/INST]"#,
                resume_content = resume_content,
                job_description = job_description
            )
        } else if model_lower.contains("qwen") {
            // Qwen-optimized prompt with clear instructions and structured thinking
            format!(
                r#"# ATS Resume Analysis Task

## Role
You are an expert ATS analyzer specializing in resume-job compatibility assessment.

## Instructions
Analyze the resume against the job description and provide a comprehensive compatibility assessment.

## Analysis Criteria
- Skills alignment and technical competencies
- Experience relevance and career progression
- Educational background match
- Keyword optimization for ATS systems
- Resume format and structure quality

## Required Output Format
Respond with ONLY a valid JSON object:

{{
    "overall_score": <integer 0-100>,
    "category_scores": {{
        "skills": <integer 0-100>,
        "experience": <integer 0-100>,
        "education": <integer 0-100>,
        "keywords": <integer 0-100>,
        "format": <integer 0-100>
    }},
    "detailed_feedback": "<specific, actionable analysis>",
    "missing_keywords": ["term1", "term2"],
    "recommendations": ["improvement1", "improvement2"]
}}

## Input Data

### Resume:
{resume_content}

### Job Description:
{job_description}

## Response
Provide the JSON analysis now:"#,
                resume_content = resume_content,
                job_description = job_description
            )
        } else {
            // Default prompt for other models
            format!(
                r#"You are an expert ATS (Applicant Tracking System) analyzer. Analyze the compatibility between the following resume and job description. Provide a detailed analysis in JSON format with the following structure:

{{
    "overall_score": <number between 0-100>,
    "category_scores": {{
        "skills": <number between 0-100>,
        "experience": <number between 0-100>,
        "education": <number between 0-100>,
        "keywords": <number between 0-100>,
        "format": <number between 0-100>
    }},
    "detailed_feedback": "<detailed explanation of the analysis>",
    "missing_keywords": ["<keyword1>", "<keyword2>", ...],
    "recommendations": ["<recommendation1>", "<recommendation2>", ...]
}}

Focus on:
1. Skills match between resume and job requirements
2. Experience level and relevance
3. Educational qualifications alignment
4. Keyword density and ATS-friendly terms
5. Resume format and structure quality

Be specific and actionable in your recommendations.

RESUME:
{resume_content}

JOB DESCRIPTION:
{job_description}

Provide only the JSON response without any additional text:"#,
                resume_content = resume_content,
                job_description = job_description
            )
        }
    }

    fn create_optimization_prompt(
        &self,
        model: &str,
        resume_content: &str,
        job_description: &str,
        optimization_level: &str,
    ) -> String {
        let model_lower = model.to_lowercase();
        let level_instructions = match optimization_level {
            "conservative" => {
                "Make minimal, safe changes that maintain the original content integrity"
            }
            "balanced" => {
                "Make moderate improvements balancing ATS optimization with natural language"
            }
            "aggressive" => {
                "Make significant changes to maximize ATS compatibility and keyword matching"
            }
            _ => "Make balanced improvements to optimize for ATS systems",
        };

        if model_lower.contains("mistral") {
            // Mistral-optimized optimization prompt
            format!(
                r#"<s>[INST] You are an expert resume optimizer specializing in ATS optimization.

Task: Optimize the provided resume for the target job description using a {optimization_level} approach.

Optimization Strategy: {level_instructions}

Key Optimization Areas:
1. Keyword integration and density optimization
2. ATS-friendly formatting improvements
3. Action verb strengthening
4. Quantifiable achievement enhancement
5. Section header optimization

Output Format (JSON only):
{{
    "optimized_content": "<complete optimized resume text>",
    "changes_made": [
        {{
            "section": "<section name>",
            "change_type": "<modification type>",
            "original": "<original text>",
            "optimized": "<improved text>",
            "impact_score": <integer 0-100>
        }}
    ],
    "improvement_summary": "<concise summary of all improvements>"
}}

Optimization Principles:
- Preserve factual accuracy at all costs
- Integrate keywords naturally into existing content
- Enhance readability while maintaining ATS compatibility
- Strengthen weak action verbs with powerful alternatives
- Add quantifiable metrics where appropriate

Original Resume:
{resume_content}

Target Job Description:
{job_description}

Provide only the JSON response: [/INST]"#,
                optimization_level = optimization_level,
                level_instructions = level_instructions,
                resume_content = resume_content,
                job_description = job_description
            )
        } else if model_lower.contains("qwen") {
            // Qwen-optimized optimization prompt
            format!(
                r#"# Resume Optimization Task

## Objective
Optimize the provided resume for the target job description using a **{optimization_level}** optimization approach.

## Optimization Guidelines
{level_instructions}

## Focus Areas
1. **Keyword Optimization**: Integrate relevant job-specific terms naturally
2. **ATS Formatting**: Ensure machine-readable structure and formatting
3. **Action Verbs**: Replace weak verbs with impactful alternatives
4. **Quantification**: Add measurable achievements where possible
5. **Section Enhancement**: Improve headers and organization

## Required Output Format
Respond with ONLY a valid JSON object:

```json
{{
    "optimized_content": "<full optimized resume text>",
    "changes_made": [
        {{
            "section": "<section name>",
            "change_type": "<type of change>",
            "original": "<original text>",
            "optimized": "<optimized text>",
            "impact_score": <integer 0-100>
        }}
    ],
    "improvement_summary": "<brief summary of improvements>"
}}
```

## Constraints
- Maintain 100% factual accuracy
- Preserve professional tone and authenticity
- Ensure natural language flow
- Keep original structure where beneficial

## Input Data

### Original Resume:
{resume_content}

### Target Job Description:
{job_description}

## Response
Provide the optimized resume JSON now:"#,
                optimization_level = optimization_level,
                level_instructions = level_instructions,
                resume_content = resume_content,
                job_description = job_description
            )
        } else {
            // Default optimization prompt for other models
            format!(
                r#"You are an expert resume optimizer. Optimize the following resume for the given job description using a {optimization_level} approach. {level_instructions}.

Provide your response in JSON format:
{{
    "optimized_content": "<the full optimized resume text>",
    "changes_made": [
        {{
            "section": "<section name>",
            "change_type": "<type of change>",
            "original": "<original text>",
            "optimized": "<optimized text>",
            "impact_score": <number between 0-100>
        }}
    ],
    "improvement_summary": "<brief summary of improvements made>"
}}

Guidelines:
- Maintain factual accuracy
- Improve keyword density naturally
- Enhance ATS-friendly formatting
- Strengthen action verbs and quantifiable achievements
- Ensure proper section headers and bullet points

ORIGINAL RESUME:
{resume_content}

TARGET JOB DESCRIPTION:
{job_description}

Provide only the JSON response:"#,
                optimization_level = optimization_level,
                level_instructions = level_instructions,
                resume_content = resume_content,
                job_description = job_description
            )
        }
    }

    #[allow(dead_code)]
    fn create_job_analysis_prompt(&self, model: &str, job_description: &str) -> String {
        let model_lower = model.to_lowercase();

        if model_lower.contains("mistral") {
            // Mistral-optimized job analysis prompt
            format!(
                r#"<s>[INST] You are an expert HR analyst specializing in job requirement extraction and analysis.

Task: Analyze the provided job description and extract all key requirements, skills, and qualifications in a structured format.

Extraction Framework:
1. Identify required vs preferred qualifications
2. Categorize technical and soft skills
3. Determine experience level and education requirements
4. Extract company culture and industry indicators
5. Compile comprehensive keyword list for ATS optimization

Output Format (JSON only):
{{
    "required_skills": ["<skill1>", "<skill2>"],
    "preferred_skills": ["<skill1>", "<skill2>"],
    "experience_level": "<entry/mid/senior>",
    "education_requirements": ["<requirement1>", "<requirement2>"],
    "key_responsibilities": ["<responsibility1>", "<responsibility2>"],
    "company_culture": "<culture description>",
    "industry": "<industry name>",
    "keywords": ["<keyword1>", "<keyword2>"],
    "role_level": "<role level>",
    "location_requirements": "<location info>"
}}

Analysis Focus:
- Technical skills (programming languages, frameworks, tools)
- Soft skills (communication, leadership, problem-solving)
- Experience requirements (years, specific domains)
- Educational qualifications (degrees, certifications)
- Industry-specific terminology and jargon
- Company values and cultural indicators

Job Description:
{job_description}

Provide only the JSON response: [/INST]"#,
                job_description = job_description
            )
        } else if model_lower.contains("qwen") {
            // Qwen-optimized job analysis prompt
            format!(
                r#"# Job Description Analysis Task

## Objective
Analyze the job description and extract comprehensive requirements, skills, and qualifications for ATS optimization.

## Analysis Framework
Systematically extract and categorize:
- **Required Skills**: Must-have technical and soft skills
- **Preferred Skills**: Nice-to-have additional qualifications
- **Experience Level**: Entry, mid-level, or senior position
- **Education**: Degree requirements and certifications
- **Responsibilities**: Key job duties and expectations
- **Company Culture**: Values, work environment, team dynamics
- **Industry Context**: Sector-specific requirements and terminology
- **Keywords**: ATS-relevant terms and phrases

## Required Output Format
Respond with ONLY a valid JSON object:

{{
    "required_skills": ["<essential skill 1>", "<essential skill 2>"],
    "preferred_skills": ["<preferred skill 1>", "<preferred skill 2>"],
    "experience_level": "<entry/mid/senior>",
    "education_requirements": ["<education req 1>", "<education req 2>"],
    "key_responsibilities": ["<responsibility 1>", "<responsibility 2>"],
    "company_culture": "<culture and values summary>",
    "industry": "<industry sector>",
    "keywords": ["<ats keyword 1>", "<ats keyword 2>"],
    "role_level": "<position level description>",
    "location_requirements": "<location specifics or remote options>"
}}

## Input Data

### Job Description:
{job_description}

## Response
Provide the structured JSON analysis now:"#,
                job_description = job_description
            )
        } else {
            // Default prompt for other models
            format!(
                r#"Analyze the following job description and extract key requirements, skills, and qualifications. Provide a structured analysis in JSON format:

{{
    "required_skills": ["<skill1>", "<skill2>", ...],
    "preferred_skills": ["<skill1>", "<skill2>", ...],
    "experience_level": "<entry/mid/senior>",
    "education_requirements": ["<requirement1>", "<requirement2>", ...],
    "key_responsibilities": ["<responsibility1>", "<responsibility2>", ...],
    "company_culture": "<brief description>",
    "industry": "<industry name>",
    "keywords": ["<keyword1>", "<keyword2>", ...],
    "role_level": "<role level description>",
    "location_requirements": "<location info if specified>"
}}

Focus on extracting:
- Technical and soft skills mentioned
- Years of experience required
- Educational background needed
- Key job responsibilities
- Industry-specific terms and keywords
- Company culture indicators

JOB DESCRIPTION:
{job_description}

Provide only the JSON response:"#,
                job_description = job_description
            )
        }
    }

    #[allow(dead_code)]
    pub async fn get_model_info(&self, model_name: &str) -> Result<String> {
        info!("Getting model info for: {}", model_name);

        let request = OllamaShowRequest {
            name: model_name.to_string(),
        };

        let response = self
            .client
            .post(format!("{}/api/show", self.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to get model info: {}", response.status()));
        }

        let show_response: OllamaShowResponse = response.json().await?;

        Ok(serde_json::to_string_pretty(&show_response)?)
    }

    #[allow(dead_code)]
    pub async fn check_model_availability(&self, model_name: &str) -> Result<bool> {
        let models = self.list_models().await?;
        Ok(models.iter().any(|m| m.name == model_name))
    }

    /// Get model-specific optimizations for generation parameters
    fn get_model_optimizations(
        &self,
        model: &str,
        base_temperature: Option<f64>,
    ) -> (f64, f64, i32) {
        let model_lower = model.to_lowercase();

        match () {
            _ if model_lower.contains("mistral") => {
                // Mistral models work best with higher temperature and top_p for creativity
                // but lower for analytical tasks
                let temp = base_temperature.unwrap_or(0.4);
                (temp, 0.95, 6000) // Higher token limit for detailed responses
            }
            _ if model_lower.contains("qwen") => {
                // Qwen models are excellent at following instructions with moderate settings
                let temp = base_temperature.unwrap_or(0.3);
                (temp, 0.85, 5000) // Good balance for structured outputs
            }
            _ => {
                // Default settings for other models
                let temp = base_temperature.unwrap_or(0.3);
                (temp, 0.9, 4000)
            }
        }
    }

    /// Get model-specific temperature for analysis tasks
    fn get_analysis_temperature(&self, model: &str) -> f64 {
        let model_lower = model.to_lowercase();
        match () {
            _ if model_lower.contains("mistral") => 0.2, // Lower for precise analysis
            _ if model_lower.contains("qwen") => 0.25,   // Slightly higher for nuanced insights
            _ => 0.3,
        }
    }

    /// Generate ML analysis with optimized prompts for structured data extraction
    pub async fn generate_ml_analysis(
        &self,
        model: &str,
        prompt: &str,
        analysis_type: &str,
    ) -> Result<String> {
        info!(
            "Generating ML analysis of type: {} with model: {}",
            analysis_type, model
        );

        let optimized_prompt = self.optimize_prompt_for_model(model, prompt, analysis_type);
        let temperature = self.get_analysis_temperature(model);

        let (response, _) = self
            .generate_response(model, &optimized_prompt, Some(temperature))
            .await?;
        Ok(response.trim().to_string())
    }

    /// Optimize prompts based on model capabilities
    fn optimize_prompt_for_model(&self, model: &str, prompt: &str, analysis_type: &str) -> String {
        let model_lower = model.to_lowercase();

        let prefix = if model_lower.contains("mistral") {
            // Mistral performs better with clear instructions and examples
            match analysis_type {
                "skill_extraction" => "[INST] You are an expert technical recruiter. Extract skills precisely and return ONLY a valid JSON array. [/INST]\n\n",
                "keyword_analysis" => "[INST] You are a resume optimization expert. Compare the texts and return ONLY missing keywords as a JSON array. [/INST]\n\n",
                "xyz_analysis" => "[INST] You are a professional writing coach. Analyze bullet points for X-Y-Z formula and return ONLY valid JSON. [/INST]\n\n",
                "industry_detection" => "[INST] You are an industry classification expert. Classify the industry and return ONLY the category name. [/INST]\n\n",
                "success_prediction" => "[INST] You are a career counselor with data analysis expertise. Predict success probability and return ONLY valid JSON. [/INST]\n\n",
                _ => "[INST] You are an expert analyst. Provide accurate analysis and return structured data as requested. [/INST]\n\n"
            }
        } else if model_lower.contains("qwen") {
            // Qwen models work well with role-based prompting
            match analysis_type {
                "skill_extraction" => "You are a skilled technical analyst specializing in resume parsing. Your task is to extract technical skills accurately.\n\n",
                "keyword_analysis" => "You are a resume optimization specialist. Your task is to identify missing keywords that would improve ATS compatibility.\n\n", 
                "xyz_analysis" => "You are an expert in professional communication and achievement writing. Your task is to analyze accomplishment statements.\n\n",
                "industry_detection" => "You are an industry classification specialist with deep knowledge of various sectors. Your task is to categorize professional profiles.\n\n",
                "success_prediction" => "You are a career success analyst with expertise in job market trends. Your task is to predict application outcomes.\n\n",
                _ => "You are a professional analyst with expertise in the requested domain. Your task is to provide accurate insights.\n\n"
            }
        } else {
            // Default prompting for other models
            "You are an expert analyst. Please provide accurate analysis based on the following request:\n\n"
        };

        let suffix = if model_lower.contains("mistral") || model_lower.contains("qwen") {
            match analysis_type {
                "skill_extraction" | "keyword_analysis" => "\n\nIMPORTANT: Return ONLY a valid JSON array. No explanations, no markdown formatting, no additional text.",
                "xyz_analysis" | "success_prediction" => "\n\nIMPORTANT: Return ONLY valid JSON with the exact structure requested. No explanations, no markdown formatting.",
                "industry_detection" => "\n\nIMPORTANT: Return ONLY the industry category name. No explanations, no additional text.",
                _ => "\n\nIMPORTANT: Return only the requested format. Be precise and concise."
            }
        } else {
            "\n\nPlease provide a clear and accurate response."
        };

        format!("{}{}{}", prefix, prompt, suffix)
    }

    /// Get model-specific temperature for optimization tasks
    fn get_optimization_temperature(&self, model: &str) -> f64 {
        let model_lower = model.to_lowercase();
        match () {
            _ if model_lower.contains("mistral") => 0.4, // Moderate creativity for suggestions
            _ if model_lower.contains("qwen") => 0.35,   // Conservative creativity
            _ => 0.5,
        }
    }

    /// Get model-specific temperature for extraction tasks
    #[allow(dead_code)]
    fn get_extraction_temperature(&self, model: &str) -> f64 {
        let model_lower = model.to_lowercase();
        match () {
            _ if model_lower.contains("mistral") => 0.1, // Very precise for extraction
            _ if model_lower.contains("qwen") => 0.15,   // Slightly more flexible
            _ => 0.2,
        }
    }
}
