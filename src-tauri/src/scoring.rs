use anyhow::{anyhow, Result};
use log::info;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

use crate::models::{AnalysisResult, CategoryScores, OptimizationChange, OptimizationResult};
use crate::ollama::OllamaClient;

pub struct AnalysisEngine {
    ollama_client: OllamaClient,
}

impl AnalysisEngine {
    pub fn new(ollama_client: OllamaClient) -> Self {
        Self { ollama_client }
    }

    pub async fn analyze_resume(
        &self,
        resume_content: &str,
        job_description: &str,
        model_name: &str,
    ) -> Result<AnalysisResult> {
        info!("Starting resume analysis with model: {}", model_name);

        // First, let the AI do the main analysis
        let (ai_response, processing_time) = self
            .ollama_client
            .analyze_resume_compatibility(model_name, resume_content, job_description)
            .await?;

        // Parse AI response
        let mut analysis_result = self.parse_ai_analysis(&ai_response)?;
        analysis_result.processing_time_ms = processing_time;

        // Enhance with our own scoring algorithms
        self.enhance_analysis(&mut analysis_result, resume_content, job_description)?;

        info!("Resume analysis completed with score: {:.1}", analysis_result.overall_score);
        Ok(analysis_result)
    }

    pub async fn optimize_resume(
        &self,
        resume_content: &str,
        job_description: &str,
        model_name: &str,
        optimization_level: &str,
    ) -> Result<OptimizationResult> {
        info!("Starting resume optimization with level: {}", optimization_level);

        // Get original score
        let original_analysis = self
            .analyze_resume(resume_content, job_description, model_name)
            .await?;

        // Get AI optimization
        let (ai_response, _) = self
            .ollama_client
            .optimize_resume(model_name, resume_content, job_description, optimization_level)
            .await?;

        // Parse optimization response
        let optimization_data = self.parse_optimization_response(&ai_response)?;

        // Analyze optimized content
        let optimized_analysis = self
            .analyze_resume(&optimization_data.optimized_content, job_description, model_name)
            .await?;

        let improvement_percentage = if original_analysis.overall_score > 0.0 {
            ((optimized_analysis.overall_score - original_analysis.overall_score)
                / original_analysis.overall_score)
                * 100.0
        } else {
            0.0
        };

        Ok(OptimizationResult {
            optimized_content: optimization_data.optimized_content,
            changes_made: optimization_data.changes_made,
            before_score: original_analysis.overall_score,
            after_score: optimized_analysis.overall_score,
            improvement_percentage,
        })
    }

    fn parse_ai_analysis(&self, response: &str) -> Result<AnalysisResult> {
        // Try to extract JSON from the response
        let json_str = self.extract_json_from_response(response)?;
        let parsed: Value = serde_json::from_str(&json_str)?;

        let overall_score = parsed["overall_score"]
            .as_f64()
            .ok_or_else(|| anyhow!("Missing overall_score in AI response"))?;

        let category_scores = CategoryScores {
            skills: parsed["category_scores"]["skills"]
                .as_f64()
                .unwrap_or(50.0),
            experience: parsed["category_scores"]["experience"]
                .as_f64()
                .unwrap_or(50.0),
            education: parsed["category_scores"]["education"]
                .as_f64()
                .unwrap_or(50.0),
            keywords: parsed["category_scores"]["keywords"]
                .as_f64()
                .unwrap_or(50.0),
            format: parsed["category_scores"]["format"]
                .as_f64()
                .unwrap_or(50.0),
        };

        let detailed_feedback = parsed["detailed_feedback"]
            .as_str()
            .unwrap_or("No detailed feedback provided")
            .to_string();

        let missing_keywords = parsed["missing_keywords"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let recommendations = parsed["recommendations"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        Ok(AnalysisResult {
            overall_score,
            category_scores,
            detailed_feedback,
            missing_keywords,
            recommendations,
            processing_time_ms: 0, // Will be set by caller
        })
    }

    fn parse_optimization_response(&self, response: &str) -> Result<OptimizationData> {
        let json_str = self.extract_json_from_response(response)?;
        let parsed: Value = serde_json::from_str(&json_str)?;

        let optimized_content = parsed["optimized_content"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing optimized_content in AI response"))?
            .to_string();

        let changes_made = parsed["changes_made"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|change| {
                        Some(OptimizationChange {
                            section: change["section"].as_str()?.to_string(),
                            change_type: change["change_type"].as_str()?.to_string(),
                            original: change["original"].as_str()?.to_string(),
                            optimized: change["optimized"].as_str()?.to_string(),
                            impact_score: change["impact_score"].as_f64().unwrap_or(50.0),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(OptimizationData {
            optimized_content,
            changes_made,
        })
    }

    fn extract_json_from_response(&self, response: &str) -> Result<String> {
        // Look for JSON content between curly braces
        let json_regex = Regex::new(r"\{.*\}").unwrap();
        
        if let Some(json_match) = json_regex.find(response) {
            return Ok(json_match.as_str().to_string());
        }

        // Try to find JSON with line breaks
        let multiline_json_regex = Regex::new(r"(?s)\{.*\}").unwrap();
        if let Some(json_match) = multiline_json_regex.find(response) {
            return Ok(json_match.as_str().to_string());
        }

        // If no JSON found, try the entire response
        if response.trim().starts_with('{') && response.trim().ends_with('}') {
            return Ok(response.trim().to_string());
        }

        Err(anyhow!("Could not extract JSON from AI response: {}", response))
    }

    fn enhance_analysis(
        &self,
        analysis: &mut AnalysisResult,
        resume_content: &str,
        job_description: &str,
    ) -> Result<()> {
        // Perform additional keyword analysis
        let keyword_analysis = self.analyze_keywords(resume_content, job_description)?;
        
        // Adjust keyword score based on our analysis
        if keyword_analysis.missing_count > 10 {
            analysis.category_scores.keywords = (analysis.category_scores.keywords * 0.8).max(20.0);
        }

        // Perform format analysis
        let format_score = self.analyze_format(resume_content)?;
        analysis.category_scores.format = (analysis.category_scores.format + format_score) / 2.0;

        // Recalculate overall score
        analysis.overall_score = self.calculate_weighted_score(&analysis.category_scores);

        // Enhance recommendations
        self.enhance_recommendations(analysis, &keyword_analysis, format_score)?;

        Ok(())
    }

    pub fn analyze_keywords(&self, resume_content: &str, job_description: &str) -> Result<KeywordAnalysis> {
        let resume_lower = resume_content.to_lowercase();
        let job_lower = job_description.to_lowercase();

        // Extract key terms from job description
        let job_keywords = self.extract_keywords(&job_lower);
        let mut found_keywords = Vec::new();
        let mut missing_keywords = Vec::new();

        for keyword in &job_keywords {
            if resume_lower.contains(keyword) {
                found_keywords.push(keyword.clone());
            } else {
                missing_keywords.push(keyword.clone());
            }
        }

        let match_rate = if !job_keywords.is_empty() {
            (found_keywords.len() as f64 / job_keywords.len() as f64) * 100.0
        } else {
            100.0
        };

        Ok(KeywordAnalysis {
            total_keywords: job_keywords.len(),
            found_count: found_keywords.len(),
            missing_count: missing_keywords.len(),
            match_rate,
            found_keywords,
            missing_keywords,
        })
    }

    fn extract_keywords(&self, text: &str) -> Vec<String> {
        let mut keywords = Vec::new();
        let text_lower = text.to_lowercase();
        
        // Technical skills patterns (case-insensitive)
        let tech_patterns = [
            r"\b(?:python|java|javascript|c\+\+|rust|go|kotlin|swift)\b",
            r"\b(?:react|angular|vue|node\.?js|django|flask|spring|laravel)\b",
            r"\b(?:aws|azure|gcp|docker|kubernetes|jenkins|git|ci/cd)\b",
            r"\b(?:sql|mongodb|postgresql|mysql|redis|elasticsearch)\b",
            r"\b(?:machine learning|ai|data science|analytics|big data)\b",
            r"\b(?:agile|scrum|devops|microservices|api|rest|graphql)\b",
        ];

        for pattern in &tech_patterns {
            let regex = Regex::new(pattern).unwrap();
            for mat in regex.find_iter(&text_lower) {
                keywords.push(mat.as_str().to_string());
            }
        }

        // Experience-related keywords
        let exp_regex = Regex::new(r"\b\d+\+?\s*(?:years?|yrs?)\s*(?:of\s*)?(?:experience|exp)\b").unwrap();
        for mat in exp_regex.find_iter(&text_lower) {
            keywords.push(mat.as_str().to_string());
        }

        // Education keywords
        let edu_regex = Regex::new(r"\b(?:bachelor|master|phd|degree|certification|certified)\b").unwrap();
        for mat in edu_regex.find_iter(&text_lower) {
            keywords.push(mat.as_str().to_string());
        }

        // Deduplicate and clean
        keywords.sort();
        keywords.dedup();
        keywords.into_iter().filter(|k| k.len() > 2).collect()
    }

    pub fn analyze_format(&self, resume_content: &str) -> Result<f64> {
        let mut score: f64 = 100.0;
        let lines: Vec<&str> = resume_content.lines().collect();

        // Check for contact information at the top
        let first_lines = lines.iter().take(5).map(|s| *s).collect::<Vec<&str>>().join("\n");
        let email_regex = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
        let phone_regex = Regex::new(r"(\+?1[-.\s]?)?\(?([0-9]{3})\)?[-.\s]?([0-9]{3})[-.\s]?([0-9]{4})").unwrap();
        
        if !email_regex.is_match(&first_lines) {
            score -= 10.0;
        }
        if !phone_regex.is_match(&first_lines) {
            score -= 5.0;
        }

        // Check for proper section headers
        let section_headers = [
            "experience", "education", "skills", "summary", "objective",
            "work experience", "professional experience", "technical skills"
        ];
        
        let mut found_sections = 0;
        for line in &lines {
            let line_lower = line.to_lowercase();
            for header in &section_headers {
                if line_lower.contains(header) && line.len() < 50 {
                    found_sections += 1;
                    break;
                }
            }
        }

        if found_sections < 3 {
            score -= 15.0;
        }

        // Check for bullet points or structured formatting
        let bullet_regex = Regex::new(r"^\s*[•\-\*\+]").unwrap();
        let bullet_count = lines.iter().filter(|line| bullet_regex.is_match(line)).count();
        
        if bullet_count < 5 {
            score -= 10.0;
        }

        // Check for reasonable line lengths (ATS-friendly)
        let long_lines = lines.iter().filter(|line| line.len() > 120).count();
        if long_lines > lines.len() / 4 {
            score -= 5.0;
        }

        Ok(score.max(0.0f64))
    }

    pub fn calculate_weighted_score(&self, scores: &CategoryScores) -> f64 {
        // Weighted scoring based on ATS importance
        let weights = HashMap::from([
            ("keywords", 0.25),
            ("skills", 0.25),
            ("experience", 0.20),
            ("format", 0.15),
            ("education", 0.15),
        ]);

        scores.keywords * weights["keywords"]
            + scores.skills * weights["skills"]
            + scores.experience * weights["experience"]
            + scores.format * weights["format"]
            + scores.education * weights["education"]
    }

    fn enhance_recommendations(
        &self,
        analysis: &mut AnalysisResult,
        keyword_analysis: &KeywordAnalysis,
        format_score: f64,
    ) -> Result<()> {
        // Add keyword-specific recommendations
        if keyword_analysis.missing_count > 5 {
            let display_count = keyword_analysis.missing_count.min(5);
            let keywords_to_show = keyword_analysis.missing_keywords
                .iter()
                .take(display_count)
                .cloned()
                .collect::<Vec<_>>()
                .join(", ");
            
            analysis.recommendations.push(format!(
                "Add {} missing keywords naturally throughout your resume: {}",
                display_count,
                keywords_to_show
            ));
        }

        // Add format recommendations
        if format_score < 80.0 {
            analysis.recommendations.push(
                "Improve resume formatting with clear section headers, bullet points, and contact information at the top".to_string()
            );
        }

        // Add score-specific recommendations
        if analysis.category_scores.skills < 70.0 {
            analysis.recommendations.push(
                "Strengthen your skills section with more relevant technical and soft skills".to_string()
            );
        }

        if analysis.category_scores.experience < 70.0 {
            analysis.recommendations.push(
                "Enhance experience descriptions with quantifiable achievements and relevant responsibilities".to_string()
            );
        }

        Ok(())
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct KeywordAnalysis {
    pub total_keywords: usize,
    pub found_count: usize,
    pub missing_count: usize,
    pub match_rate: f64,
    pub found_keywords: Vec<String>,
    pub missing_keywords: Vec<String>,
}

#[derive(Debug)]
struct OptimizationData {
    optimized_content: String,
    changes_made: Vec<OptimizationChange>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ollama::OllamaClient;

    const SAMPLE_RESUME: &str = r#"
John Doe
Software Engineer
john.doe@email.com
(555) 123-4567

EXPERIENCE
Senior Software Engineer - Tech Corp (2020-2023)
• Developed web applications using React and Node.js
• Led team of 4 developers
• Improved system performance by 40%

Software Engineer - StartupCo (2018-2020)
• Built REST APIs using Python and Django
• Implemented CI/CD pipelines

EDUCATION
Bachelor of Computer Science - State University (2018)

SKILLS
Python, JavaScript, React, Node.js, Docker, AWS
"#;

    const SAMPLE_JOB_DESCRIPTION: &str = r#"
Software Engineer Position
Tech Company Inc.

Requirements:
• 3+ years of experience in software development
• Proficiency in Python and JavaScript
• Experience with React and Node.js
• Knowledge of cloud platforms (AWS, Azure)
• Bachelor's degree in Computer Science or related field
• Experience with Docker and containerization
• Strong problem-solving skills
• Experience with agile development methodologies
"#;

    fn create_mock_ollama_client() -> OllamaClient {
        OllamaClient::new(Some("http://localhost:11434".to_string())).expect("Failed to create test Ollama client")
    }

    #[test]
    fn test_extract_keywords() {
        let engine = AnalysisEngine::new(create_mock_ollama_client());
        let keywords = engine.extract_keywords(SAMPLE_JOB_DESCRIPTION);
        
        assert!(!keywords.is_empty());
        assert!(keywords.iter().any(|k| k.contains("python")));
        assert!(keywords.iter().any(|k| k.contains("javascript")));
        assert!(keywords.iter().any(|k| k.contains("react")));
        assert!(keywords.iter().any(|k| k.contains("aws")));
    }

    #[test]
    fn test_analyze_keywords() {
        let engine = AnalysisEngine::new(create_mock_ollama_client());
        let result = engine.analyze_keywords(SAMPLE_RESUME, SAMPLE_JOB_DESCRIPTION);
        
        assert!(result.is_ok());
        let analysis = result.unwrap();
        
        assert!(analysis.total_keywords > 0);
        assert!(analysis.found_count > 0);
        assert!(analysis.match_rate >= 0.0 && analysis.match_rate <= 100.0);
        assert_eq!(analysis.found_count + analysis.missing_count, analysis.total_keywords);
    }

    #[test]
    fn test_analyze_format() {
        let engine = AnalysisEngine::new(create_mock_ollama_client());
        
        // Test well-formatted resume
        let good_format_score = engine.analyze_format(SAMPLE_RESUME);
        assert!(good_format_score.is_ok());
        let good_score = good_format_score.unwrap();
        assert!(good_score > 50.0);
        
        // Test poorly formatted resume
        let poor_resume = "John Doe some text without proper formatting or structure";
        let poor_format_score = engine.analyze_format(poor_resume);
        assert!(poor_format_score.is_ok());
        let poor_score = poor_format_score.unwrap();
        assert!(poor_score < good_score);
    }

    #[test]
    fn test_calculate_weighted_score() {
        let engine = AnalysisEngine::new(create_mock_ollama_client());
        let scores = CategoryScores {
            skills: 80.0,
            experience: 85.0,
            education: 75.0,
            keywords: 90.0,
            format: 95.0,
        };
        
        let weighted_score = engine.calculate_weighted_score(&scores);
        
        // Should be a weighted average
        assert!(weighted_score > 0.0 && weighted_score <= 100.0);
        // Keywords and skills have highest weight (0.25 each), so should be closer to their average
        let expected_range = 80.0..90.0;
        assert!(expected_range.contains(&weighted_score));
    }

    #[test]
    fn test_extract_json_from_response() {
        let engine = AnalysisEngine::new(create_mock_ollama_client());
        
        // Test clean JSON
        let clean_json = r#"{"overall_score": 85.5, "detailed_feedback": "Good resume"}"#;
        let result = engine.extract_json_from_response(clean_json);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), clean_json);
        
        // Test JSON with surrounding text
        let surrounded_json = r#"Here is the analysis: {"overall_score": 85.5, "detailed_feedback": "Good resume"} End of analysis."#;
        let result = engine.extract_json_from_response(surrounded_json);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("overall_score"));
        
        // Test invalid response
        let invalid_response = "This is not JSON at all";
        let result = engine.extract_json_from_response(invalid_response);
        assert!(result.is_err());
    }

    #[test]
    fn test_keyword_analysis_edge_cases() {
        let engine = AnalysisEngine::new(create_mock_ollama_client());
        
        // Test empty resume
        let empty_analysis = engine.analyze_keywords("", SAMPLE_JOB_DESCRIPTION);
        assert!(empty_analysis.is_ok());
        let analysis = empty_analysis.unwrap();
        assert_eq!(analysis.found_count, 0);
        assert_eq!(analysis.match_rate, 0.0);
        
        // Test empty job description
        let empty_job_analysis = engine.analyze_keywords(SAMPLE_RESUME, "");
        assert!(empty_job_analysis.is_ok());
        let analysis = empty_job_analysis.unwrap();
        assert_eq!(analysis.total_keywords, 0);
        assert_eq!(analysis.match_rate, 100.0); // No keywords to match
    }

    #[test]
    fn test_format_analysis_components() {
        let engine = AnalysisEngine::new(create_mock_ollama_client());
        
        // Test resume with good contact info
        let good_contact_resume = r#"
John Doe
john.doe@email.com
(555) 123-4567

EXPERIENCE
Software Engineer
"#;
        let score = engine.analyze_format(good_contact_resume).unwrap();
        
        // Test resume without contact info
        let no_contact_resume = r#"
EXPERIENCE
Software Engineer
"#;
        let no_contact_score = engine.analyze_format(no_contact_resume).unwrap();
        
        assert!(score > no_contact_score);
    }

    #[test]
    fn test_enhance_recommendations() {
        let engine = AnalysisEngine::new(create_mock_ollama_client());
        let mut analysis = AnalysisResult {
            overall_score: 75.0,
            category_scores: CategoryScores {
                skills: 60.0,
                experience: 65.0,
                education: 80.0,
                keywords: 70.0,
                format: 85.0,
            },
            detailed_feedback: "Initial feedback".to_string(),
            missing_keywords: vec!["kubernetes".to_string(), "microservices".to_string()],
            recommendations: vec![],
            processing_time_ms: 1000,
        };
        
        let keyword_analysis = KeywordAnalysis {
            total_keywords: 10,
            found_count: 4,
            missing_count: 6,
            match_rate: 40.0,
            found_keywords: vec!["python".to_string(), "react".to_string()],
            missing_keywords: vec!["kubernetes".to_string(), "microservices".to_string()],
        };
        
        let result = engine.enhance_recommendations(&mut analysis, &keyword_analysis, 75.0);
        assert!(result.is_ok());
        
        // Should have added recommendations
        assert!(!analysis.recommendations.is_empty());
        
        // Should have keyword recommendations due to missing keywords
        assert!(analysis.recommendations.iter().any(|r| r.contains("keywords")));
        
        // Should have skills recommendations due to low skills score
        assert!(analysis.recommendations.iter().any(|r| r.contains("skills")));
    }

    #[test]
    fn test_extract_keywords_patterns() {
        let engine = AnalysisEngine::new(create_mock_ollama_client());
        
        // Test technical skills
        let tech_text = "Experience with Python, JavaScript, React, Node.js, AWS, Docker";
        let keywords = engine.extract_keywords(tech_text);
        assert!(keywords.contains(&"python".to_string()));
        assert!(keywords.contains(&"javascript".to_string()));
        assert!(keywords.contains(&"react".to_string()));
        assert!(keywords.contains(&"aws".to_string()));
        assert!(keywords.contains(&"docker".to_string()));
        
        // Test experience patterns
        let exp_text = "5+ years of experience in software development";
        let keywords = engine.extract_keywords(exp_text);
        assert!(keywords.iter().any(|k| k.contains("years") && k.contains("experience")));
        
        // Test education patterns
        let edu_text = "Bachelor's degree in Computer Science, AWS certified";
        let keywords = engine.extract_keywords(edu_text);
        assert!(keywords.contains(&"bachelor".to_string()));
        assert!(keywords.contains(&"certified".to_string()));
    }

    #[test]
    fn test_keyword_deduplication() {
        let engine = AnalysisEngine::new(create_mock_ollama_client());
        
        // Text with duplicate keywords
        let duplicate_text = "Python developer with Python experience. Python Python Python.";
        let keywords = engine.extract_keywords(duplicate_text);
        
        // Should only have one instance of "python"
        let python_count = keywords.iter().filter(|&k| k == "python").count();
        assert_eq!(python_count, 1);
    }
}