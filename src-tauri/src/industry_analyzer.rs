use anyhow::{Context, Result};
use log::{error, info};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::database::Database;
use crate::models::IndustryKeyword;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryAnalysisResult {
    pub detected_industry: String,
    pub confidence_score: f64,
    pub industry_keywords: Vec<IndustryKeywordMatch>,
    pub role_level_assessment: RoleLevelAssessment,
    pub required_certifications: Vec<CertificationCheck>,
    pub industry_trends: Vec<TrendAnalysis>,
    pub domain_expertise_score: f64,
    pub industry_specific_recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryKeywordMatch {
    pub keyword: String,
    pub category: String,
    pub found: bool,
    pub frequency: i32,
    pub context: Vec<String>,
    pub weight: f64,
    pub synonyms_found: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleLevelAssessment {
    pub detected_level: String,
    pub confidence: f64,
    pub experience_indicators: Vec<ExperienceIndicator>,
    pub leadership_indicators: Vec<LeadershipIndicator>,
    pub years_of_experience_estimate: Option<i32>,
    pub seniority_signals: Vec<SenioritySignal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceIndicator {
    pub indicator_type: String,
    pub description: String,
    pub weight: f64,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadershipIndicator {
    pub indicator_type: String,
    pub description: String,
    pub team_size: Option<i32>,
    pub scope: String,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenioritySignal {
    pub signal_type: String,
    pub description: String,
    pub strength: f64,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificationCheck {
    pub certification_name: String,
    pub found: bool,
    pub importance: f64,
    pub expiry_status: Option<String>,
    pub alternatives: Vec<String>,
    pub recommendation_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub trend_name: String,
    pub trend_type: String, // emerging, declining, stable, growing
    pub relevance_score: f64,
    pub found_in_resume: bool,
    pub importance_for_role: f64,
    pub learning_resources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryRules {
    pub industry_name: String,
    pub required_keywords: Vec<String>,
    pub preferred_keywords: Vec<String>,
    pub experience_levels: HashMap<String, ExperienceLevel>,
    pub certifications: Vec<IndustryCertification>,
    pub trends: Vec<IndustryTrend>,
    pub role_hierarchies: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceLevel {
    pub min_years: i32,
    pub max_years: Option<i32>,
    pub typical_responsibilities: Vec<String>,
    pub expected_skills: Vec<String>,
    pub leadership_expectations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryCertification {
    pub name: String,
    pub importance: f64,
    pub required_for_levels: Vec<String>,
    pub alternatives: Vec<String>,
    pub validity_years: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryTrend {
    pub name: String,
    pub trend_type: String,
    pub adoption_rate: f64,
    pub relevance_by_role: HashMap<String, f64>,
    pub keywords: Vec<String>,
}

pub struct IndustryAnalyzer {
    database: Database,
    _industry_rules: HashMap<String, IndustryRules>,
    industry_patterns: HashMap<String, Vec<Regex>>,
    experience_patterns: Vec<Regex>,
    leadership_patterns: Vec<Regex>,
}

impl IndustryAnalyzer {
    pub fn new(database: Database) -> Self {
        let industry_rules = Self::build_industry_rules();
        let industry_patterns = Self::build_industry_patterns();
        let experience_patterns = Self::build_experience_patterns();
        let leadership_patterns = Self::build_leadership_patterns();

        IndustryAnalyzer {
            database,
            _industry_rules: industry_rules,
            industry_patterns,
            experience_patterns,
            leadership_patterns,
        }
    }

    pub async fn analyze_for_industry(
        &self,
        resume_content: &str,
        job_description: &str,
        target_industry: &str,
    ) -> Result<IndustryAnalysisResult> {
        info!("Analyzing for industry: {}", target_industry);

        // 1. Detect actual industry from resume content
        let detected_industry = self.detect_industry_from_content(resume_content).await?;
        let confidence_score =
            self.calculate_industry_confidence(&detected_industry, target_industry);

        // 2. Analyze industry-specific keywords
        let industry_keywords = self
            .analyze_industry_keywords(resume_content, target_industry)
            .await?;

        // 3. Assess role level and experience
        let role_level_assessment = self.assess_role_level(resume_content, job_description);

        // 4. Check industry certifications
        let required_certifications =
            self.check_industry_certifications(resume_content, target_industry);

        // 5. Analyze industry trends alignment
        let industry_trends = self.analyze_industry_trends(resume_content, target_industry);

        // 6. Calculate domain expertise score
        let domain_expertise_score = self.calculate_domain_expertise_score(
            &industry_keywords,
            &required_certifications,
            &industry_trends,
        );

        // 7. Generate industry-specific recommendations
        let industry_specific_recommendations = self.generate_industry_recommendations(
            &industry_keywords,
            &required_certifications,
            &industry_trends,
            target_industry,
        );

        Ok(IndustryAnalysisResult {
            detected_industry,
            confidence_score,
            industry_keywords,
            role_level_assessment,
            required_certifications,
            industry_trends,
            domain_expertise_score,
            industry_specific_recommendations,
        })
    }

    async fn detect_industry_from_content(&self, resume_content: &str) -> Result<String> {
        let content_lower = resume_content.to_lowercase();
        let mut industry_scores: HashMap<String, f64> = HashMap::new();

        // Check patterns for each industry
        for (industry, patterns) in &self.industry_patterns {
            let mut score = 0.0;
            for pattern in patterns {
                let matches = pattern.find_iter(&content_lower).count();
                score += matches as f64;
            }

            if score > 0.0 {
                industry_scores.insert(industry.clone(), score);
            }
        }

        // Also check database keywords
        for industry in &[
            "technology",
            "healthcare",
            "finance",
            "education",
            "manufacturing",
            "retail",
            "consulting",
        ] {
            if let Ok(keywords) = self.database.get_industry_keywords(industry).await {
                let mut keyword_score = 0.0;
                for keyword in keywords {
                    if content_lower.contains(&keyword.keyword.to_lowercase()) {
                        keyword_score += keyword.weight;
                    }
                }

                let industry_key = industry.to_string();
                let existing_score = industry_scores.get(&industry_key).unwrap_or(&0.0);
                industry_scores.insert(industry_key, existing_score + keyword_score);
            }
        }

        // Return the industry with the highest score
        let detected_industry = industry_scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(industry, _)| industry.clone())
            .unwrap_or_else(|| "general".to_string());

        info!(
            "Detected industry: {} with scores: {:?}",
            detected_industry, industry_scores
        );
        Ok(detected_industry)
    }

    fn calculate_industry_confidence(&self, detected_industry: &str, target_industry: &str) -> f64 {
        if detected_industry.to_lowercase() == target_industry.to_lowercase() {
            0.95
        } else if self.are_related_industries(detected_industry, target_industry) {
            0.7
        } else {
            0.4
        }
    }

    fn are_related_industries(&self, industry1: &str, industry2: &str) -> bool {
        let related_groups = vec![
            vec![
                "technology",
                "software",
                "it",
                "computer science",
                "engineering",
            ],
            vec!["healthcare", "medical", "pharmaceutical", "biotech"],
            vec!["finance", "banking", "investment", "fintech", "accounting"],
            vec!["education", "academic", "research", "training"],
            vec!["retail", "e-commerce", "sales", "marketing"],
        ];

        for group in related_groups {
            if group.iter().any(|&i| i.contains(&industry1.to_lowercase()))
                && group.iter().any(|&i| i.contains(&industry2.to_lowercase()))
            {
                return true;
            }
        }
        false
    }

    async fn analyze_industry_keywords(
        &self,
        resume_content: &str,
        industry: &str,
    ) -> Result<Vec<IndustryKeywordMatch>> {
        info!(
            "IndustryAnalyzer: Starting analyze_industry_keywords for '{}'",
            industry
        );

        // Test database health before query
        match self.database.health_check().await {
            Ok(true) => {
                info!("IndustryAnalyzer: Database health check passed");
            }
            Ok(false) => {
                error!("IndustryAnalyzer: Database health check failed");
                return Err(anyhow::anyhow!(
                    "Database health check failed in IndustryAnalyzer"
                ));
            }
            Err(e) => {
                error!("IndustryAnalyzer: Database health check error: {}", e);
                return Err(anyhow::anyhow!(
                    "Database health check error in IndustryAnalyzer: {}",
                    e
                ));
            }
        }

        let keywords = self
            .database
            .get_industry_keywords(industry)
            .await
            .context(format!("Failed to load industry keywords for industry '{}'. Please check if the database is accessible and the industry is supported.", industry))?;

        info!(
            "Loaded {} keywords for industry analysis of '{}'",
            keywords.len(),
            industry
        );

        let content_lower = resume_content.to_lowercase();
        let mut keyword_matches = Vec::new();

        for keyword in keywords {
            let keyword_lower = keyword.keyword.to_lowercase();
            let found = content_lower.contains(&keyword_lower);
            let frequency = self.count_keyword_frequency(&content_lower, &keyword_lower);
            let context = self.extract_keyword_context(resume_content, &keyword.keyword);
            let synonyms_found = self.find_synonyms_in_content(&content_lower, &keyword);

            keyword_matches.push(IndustryKeywordMatch {
                keyword: keyword.keyword.clone(),
                category: keyword.category.clone(),
                found,
                frequency,
                context,
                weight: keyword.weight,
                synonyms_found,
            });
        }

        // Sort by relevance (found keywords with higher weight first)
        keyword_matches.sort_by(|a, b| {
            let score_a = if a.found {
                a.weight * (a.frequency as f64)
            } else {
                0.0
            };
            let score_b = if b.found {
                b.weight * (b.frequency as f64)
            } else {
                0.0
            };
            score_b
                .partial_cmp(&score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(keyword_matches)
    }

    fn assess_role_level(
        &self,
        resume_content: &str,
        job_description: &str,
    ) -> RoleLevelAssessment {
        let content_lower = resume_content.to_lowercase();
        let job_lower = job_description.to_lowercase();

        // Extract experience indicators
        let experience_indicators = self.extract_experience_indicators(&content_lower);

        // Extract leadership indicators
        let leadership_indicators = self.extract_leadership_indicators(&content_lower);

        // Estimate years of experience
        let years_of_experience_estimate = self.estimate_years_of_experience(&content_lower);

        // Analyze seniority signals
        let seniority_signals = self.analyze_seniority_signals(&content_lower);

        // Determine role level
        let (detected_level, confidence) = self.determine_role_level(
            &experience_indicators,
            &leadership_indicators,
            years_of_experience_estimate,
            &seniority_signals,
            &job_lower,
        );

        RoleLevelAssessment {
            detected_level,
            confidence,
            experience_indicators,
            leadership_indicators,
            years_of_experience_estimate,
            seniority_signals,
        }
    }

    fn extract_experience_indicators(&self, content: &str) -> Vec<ExperienceIndicator> {
        let mut indicators = Vec::new();

        for pattern in &self.experience_patterns {
            for mat in pattern.find_iter(content) {
                let matched_text = mat.as_str();
                let context = self.extract_surrounding_context(content, mat.start(), mat.end(), 50);

                indicators.push(ExperienceIndicator {
                    indicator_type: "experience_mention".to_string(),
                    description: matched_text.to_string(),
                    weight: 1.0,
                    context,
                });
            }
        }

        // Look for specific experience patterns
        let experience_keywords = [
            ("project management", 1.5),
            ("team lead", 2.0),
            ("senior", 1.8),
            ("principal", 2.5),
            ("architect", 2.2),
            ("director", 3.0),
            ("manager", 2.0),
            ("consultant", 1.5),
            ("specialist", 1.3),
            ("expert", 1.7),
        ];

        for (keyword, weight) in &experience_keywords {
            if content.contains(keyword) {
                let context = self.extract_keyword_context_simple(content, keyword, 100);
                indicators.push(ExperienceIndicator {
                    indicator_type: "role_keyword".to_string(),
                    description: keyword.to_string(),
                    weight: *weight,
                    context,
                });
            }
        }

        indicators
    }

    fn extract_leadership_indicators(&self, content: &str) -> Vec<LeadershipIndicator> {
        let mut indicators = Vec::new();

        for pattern in &self.leadership_patterns {
            for mat in pattern.find_iter(content) {
                let matched_text = mat.as_str();
                let context =
                    self.extract_surrounding_context(content, mat.start(), mat.end(), 100);

                // Try to extract team size if mentioned
                let team_size = self.extract_team_size(&context);

                indicators.push(LeadershipIndicator {
                    indicator_type: "leadership_mention".to_string(),
                    description: matched_text.to_string(),
                    team_size,
                    scope: self.determine_leadership_scope(&context),
                    context,
                });
            }
        }

        indicators
    }

    fn estimate_years_of_experience(&self, content: &str) -> Option<i32> {
        // Look for explicit year mentions
        let year_patterns = [
            Regex::new(r"(\d+)\+?\s*years?\s+(?:of\s+)?experience").unwrap(),
            Regex::new(r"(\d+)\+?\s*yrs?\s+(?:of\s+)?experience").unwrap(),
            Regex::new(r"over\s+(\d+)\s+years?").unwrap(),
            Regex::new(r"more than\s+(\d+)\s+years?").unwrap(),
        ];

        for pattern in &year_patterns {
            if let Some(captures) = pattern.captures(content) {
                if let Some(years_str) = captures.get(1) {
                    if let Ok(years) = years_str.as_str().parse::<i32>() {
                        return Some(years);
                    }
                }
            }
        }

        // Estimate based on role progression and job count
        let job_count = self.count_job_positions(content);
        let has_senior_roles =
            content.contains("senior") || content.contains("lead") || content.contains("principal");
        let has_management = content.contains("manager") || content.contains("director");

        match (job_count, has_senior_roles, has_management) {
            (1..=2, false, false) => Some(2),
            (1..=2, true, false) => Some(4),
            (3..=4, false, false) => Some(5),
            (3..=4, true, false) => Some(7),
            (3..=4, _, true) => Some(8),
            (5.., false, false) => Some(8),
            (5.., true, false) => Some(10),
            (5.., _, true) => Some(12),
            _ => None,
        }
    }

    fn analyze_seniority_signals(&self, content: &str) -> Vec<SenioritySignal> {
        let mut signals = Vec::new();

        let seniority_indicators = [
            ("mentoring", 1.5, "Mentoring experience indicates seniority"),
            (
                "architecture",
                2.0,
                "Architecture decisions show senior technical role",
            ),
            (
                "strategy",
                1.8,
                "Strategic involvement indicates senior position",
            ),
            (
                "budget",
                1.7,
                "Budget responsibility shows management seniority",
            ),
            (
                "hiring",
                1.6,
                "Hiring involvement indicates leadership role",
            ),
            ("stakeholder", 1.4, "Stakeholder management shows seniority"),
            (
                "cross-functional",
                1.3,
                "Cross-functional work indicates experience",
            ),
            (
                "implemented",
                1.2,
                "Implementation experience shows hands-on seniority",
            ),
            (
                "designed",
                1.4,
                "Design responsibility indicates senior technical role",
            ),
            (
                "led initiative",
                2.0,
                "Leading initiatives shows leadership seniority",
            ),
        ];

        for (indicator, strength, description) in &seniority_indicators {
            if content.contains(indicator) {
                let context = self.extract_keyword_context_simple(content, indicator, 80);
                signals.push(SenioritySignal {
                    signal_type: "seniority_keyword".to_string(),
                    description: description.to_string(),
                    strength: *strength,
                    context,
                });
            }
        }

        signals
    }

    fn determine_role_level(
        &self,
        experience_indicators: &[ExperienceIndicator],
        leadership_indicators: &[LeadershipIndicator],
        years_experience: Option<i32>,
        seniority_signals: &[SenioritySignal],
        job_description: &str,
    ) -> (String, f64) {
        let mut scores: HashMap<String, f64> = HashMap::new();

        // Initialize scores
        for level in &["entry", "mid", "senior", "lead", "executive"] {
            scores.insert(level.to_string(), 0.0);
        }

        // Years of experience scoring
        if let Some(years) = years_experience {
            match years {
                0..=2 => *scores.get_mut("entry").unwrap() += 2.0,
                3..=5 => *scores.get_mut("mid").unwrap() += 2.0,
                6..=8 => *scores.get_mut("senior").unwrap() += 2.0,
                9..=12 => *scores.get_mut("lead").unwrap() += 2.0,
                13.. => *scores.get_mut("executive").unwrap() += 2.0,
                _ => {} // Handle negative values (shouldn't happen in practice)
            }
        }

        // Experience indicators scoring
        for indicator in experience_indicators {
            match indicator.description.as_str() {
                desc if desc.contains("senior") => {
                    *scores.get_mut("senior").unwrap() += indicator.weight
                }
                desc if desc.contains("lead") || desc.contains("principal") => {
                    *scores.get_mut("lead").unwrap() += indicator.weight
                }
                desc if desc.contains("director") || desc.contains("vp") => {
                    *scores.get_mut("executive").unwrap() += indicator.weight
                }
                desc if desc.contains("manager") => {
                    *scores.get_mut("lead").unwrap() += indicator.weight * 0.8
                }
                _ => *scores.get_mut("mid").unwrap() += indicator.weight * 0.5,
            }
        }

        // Leadership indicators scoring
        for indicator in leadership_indicators {
            if let Some(team_size) = indicator.team_size {
                match team_size {
                    1..=3 => *scores.get_mut("senior").unwrap() += 1.0,
                    4..=10 => *scores.get_mut("lead").unwrap() += 1.5,
                    11.. => *scores.get_mut("executive").unwrap() += 2.0,
                    _ => {} // Handle zero or negative values
                }
            } else {
                *scores.get_mut("senior").unwrap() += 0.5;
            }
        }

        // Seniority signals scoring
        for signal in seniority_signals {
            let distribution_weight = signal.strength / 10.0; // Distribute across levels
            *scores.get_mut("senior").unwrap() += distribution_weight;
            *scores.get_mut("lead").unwrap() += distribution_weight * 0.8;
            *scores.get_mut("executive").unwrap() += distribution_weight * 0.6;
        }

        // Job description hints
        if job_description.contains("senior") {
            *scores.get_mut("senior").unwrap() += 1.0;
        }
        if job_description.contains("lead") || job_description.contains("principal") {
            *scores.get_mut("lead").unwrap() += 1.0;
        }
        if job_description.contains("director") || job_description.contains("manager") {
            *scores.get_mut("executive").unwrap() += 1.0;
        }

        // Find the highest scoring level
        let (detected_level, max_score) = scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(level, score)| (level.clone(), *score))
            .unwrap_or_else(|| ("mid".to_string(), 0.0));

        // Calculate confidence based on score separation
        let total_score: f64 = scores.values().sum();
        let confidence = if total_score > 0.0 {
            (max_score / total_score).min(1.0)
        } else {
            0.5
        };

        (detected_level, confidence)
    }

    // Helper methods
    fn count_keyword_frequency(&self, content: &str, keyword: &str) -> i32 {
        content.matches(keyword).count() as i32
    }

    fn extract_keyword_context(&self, content: &str, keyword: &str) -> Vec<String> {
        let mut contexts = Vec::new();
        let content_lower = content.to_lowercase();
        let keyword_lower = keyword.to_lowercase();

        let mut start = 0;
        while let Some(pos) = content_lower[start..].find(&keyword_lower) {
            let actual_pos = start + pos;
            let context_start = (actual_pos as i32 - 50).max(0) as usize;
            let context_end = (actual_pos + keyword.len() + 50).min(content.len());

            let context = content[context_start..context_end].trim().to_string();
            contexts.push(context);

            start = actual_pos + keyword.len();
        }

        contexts
    }

    fn extract_keyword_context_simple(
        &self,
        content: &str,
        keyword: &str,
        context_size: usize,
    ) -> String {
        if let Some(pos) = content.to_lowercase().find(&keyword.to_lowercase()) {
            let start = (pos as i32 - context_size as i32 / 2).max(0) as usize;
            let end = (pos + keyword.len() + context_size / 2).min(content.len());
            content[start..end].trim().to_string()
        } else {
            String::new()
        }
    }

    fn find_synonyms_in_content(&self, content: &str, keyword: &IndustryKeyword) -> Vec<String> {
        let mut found_synonyms = Vec::new();

        if let Ok(synonyms) = serde_json::from_str::<Vec<String>>(&keyword.synonyms) {
            for synonym in synonyms {
                if content.contains(&synonym.to_lowercase()) {
                    found_synonyms.push(synonym);
                }
            }
        }

        found_synonyms
    }

    fn extract_surrounding_context(
        &self,
        content: &str,
        start: usize,
        end: usize,
        context_size: usize,
    ) -> String {
        let context_start = (start as i32 - context_size as i32).max(0) as usize;
        let context_end = (end + context_size).min(content.len());
        content[context_start..context_end].trim().to_string()
    }

    fn extract_team_size(&self, context: &str) -> Option<i32> {
        let team_pattern =
            Regex::new(r"(?i)team\s+of\s+(\d+)|(\d+)\s+(?:person|people|member)").unwrap();

        if let Some(captures) = team_pattern.captures(context) {
            if let Some(size_str) = captures.get(1).or_else(|| captures.get(2)) {
                if let Ok(size) = size_str.as_str().parse::<i32>() {
                    return Some(size);
                }
            }
        }

        None
    }

    fn determine_leadership_scope(&self, context: &str) -> String {
        if context.to_lowercase().contains("department") || context.contains("division") {
            "department".to_string()
        } else if context.contains("team") {
            "team".to_string()
        } else if context.contains("project") {
            "project".to_string()
        } else {
            "individual".to_string()
        }
    }

    fn count_job_positions(&self, content: &str) -> usize {
        // Simple heuristic: count common job experience section patterns
        let job_patterns = [
            Regex::new(r"(?i)(?:^|\n)\s*\d{4}\s*[-–—]\s*(?:\d{4}|present)").unwrap(),
            Regex::new(r"(?i)(?:january|february|march|april|may|june|july|august|september|october|november|december)\s+\d{4}").unwrap(),
        ];

        let mut job_count = 0;
        for pattern in &job_patterns {
            job_count += pattern.find_iter(content).count();
        }

        // Remove duplicates and estimate
        (job_count / 2).max(1) // Rough estimate
    }

    // Placeholder implementations for certification and trend analysis
    fn check_industry_certifications(
        &self,
        resume_content: &str,
        industry: &str,
    ) -> Vec<CertificationCheck> {
        let mut certification_checks = Vec::new();
        let content_lower = resume_content.to_lowercase();

        if let Some(rules) = self._industry_rules.get(industry) {
            for cert in &rules.certifications {
                let cert_lower = cert.name.to_lowercase();
                let is_mentioned = content_lower.contains(&cert_lower);

                // Generate recommendation reason
                let recommendation_reason = if is_mentioned {
                    format!("Certification '{}' found in resume", cert.name)
                } else {
                    format!(
                        "Consider obtaining '{}' certification - importance: {:.1}%",
                        cert.name,
                        cert.importance * 100.0
                    )
                };

                certification_checks.push(CertificationCheck {
                    certification_name: cert.name.clone(),
                    found: is_mentioned,
                    importance: cert.importance,
                    expiry_status: if is_mentioned {
                        Some("Valid".to_string())
                    } else {
                        None
                    },
                    alternatives: cert.alternatives.clone(),
                    recommendation_reason,
                });
            }
        }

        certification_checks
    }

    fn analyze_industry_trends(
        &self,
        _resume_content: &str,
        _industry: &str,
    ) -> Vec<TrendAnalysis> {
        vec![] // Simplified implementation
    }

    fn calculate_domain_expertise_score(
        &self,
        industry_keywords: &[IndustryKeywordMatch],
        _certifications: &[CertificationCheck],
        _trends: &[TrendAnalysis],
    ) -> f64 {
        if industry_keywords.is_empty() {
            return 0.0;
        }

        let total_weight: f64 = industry_keywords.iter().map(|kw| kw.weight).sum();
        let matched_weight: f64 = industry_keywords
            .iter()
            .filter(|kw| kw.found)
            .map(|kw| kw.weight * kw.frequency as f64)
            .sum();

        if total_weight > 0.0 {
            (matched_weight / total_weight * 100.0).min(100.0)
        } else {
            0.0
        }
    }

    fn generate_industry_recommendations(
        &self,
        industry_keywords: &[IndustryKeywordMatch],
        certifications: &[CertificationCheck],
        trends: &[TrendAnalysis],
        industry: &str,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Analyze keyword coverage based on found field
        let found_keywords = industry_keywords.iter().filter(|k| k.found).count();
        let total_keywords = industry_keywords.len();

        if total_keywords > 0 {
            let coverage = (found_keywords as f64 / total_keywords as f64) * 100.0;

            if coverage < 60.0 {
                recommendations.push(format!(
                    "Industry keyword coverage is low ({:.1}%). Consider adding more relevant industry terms.",
                    coverage
                ));

                // Suggest specific missing keywords
                let missing_keywords: Vec<&str> = industry_keywords
                    .iter()
                    .filter(|k| !k.found && k.weight > 0.7)
                    .map(|k| k.keyword.as_str())
                    .take(5) // Limit to top 5 suggestions
                    .collect();

                if !missing_keywords.is_empty() {
                    recommendations.push(format!(
                        "Consider incorporating these high-impact terms: {}",
                        missing_keywords.join(", ")
                    ));
                }
            }
        }

        // Analyze certifications
        let present_certs = certifications.iter().filter(|c| c.found).count();
        let total_certs = certifications.len();

        if present_certs == 0 && total_certs > 0 {
            let high_value_certs: Vec<&str> = certifications
                .iter()
                .filter(|c| c.importance > 0.8)
                .map(|c| c.certification_name.as_str())
                .collect();

            if !high_value_certs.is_empty() {
                recommendations.push(format!(
                    "Consider pursuing industry certifications to strengthen your profile: {}",
                    high_value_certs.join(", ")
                ));
            }
        }

        // Industry-specific recommendations
        match industry {
            "technology" => {
                recommendations.push("Highlight specific programming languages and frameworks you've used in projects.".to_string());
                recommendations.push("Include quantifiable achievements (e.g., 'Improved system performance by 40%').".to_string());

                if !industry_keywords
                    .iter()
                    .any(|k| k.keyword.contains("agile") && k.found)
                {
                    recommendations.push(
                        "Consider mentioning experience with Agile/Scrum methodologies."
                            .to_string(),
                    );
                }
            }
            "finance" => {
                recommendations.push("Quantify your financial impact (e.g., 'Managed $2M budget', 'Reduced costs by 15%').".to_string());
                recommendations.push(
                    "Emphasize regulatory compliance and risk management experience.".to_string(),
                );

                if !certifications
                    .iter()
                    .any(|c| c.certification_name.contains("CPA") && c.found)
                {
                    recommendations.push(
                        "CPA certification would significantly strengthen your finance profile."
                            .to_string(),
                    );
                }
            }
            _ => {
                recommendations.push(
                    "Tailor your resume to include more industry-specific terminology.".to_string(),
                );
                recommendations.push(
                    "Research common skills and qualifications for your target roles.".to_string(),
                );
            }
        }

        // Add trend-based recommendations if available
        for trend in trends {
            if trend.relevance_score > 0.7 {
                recommendations.push(format!(
                    "Consider highlighting experience with trending topic: {}",
                    trend.trend_name
                ));
            }
        }

        recommendations
    }

    // Static data builders
    fn build_industry_rules() -> HashMap<String, IndustryRules> {
        let mut rules = HashMap::new();

        // Technology Industry Rules
        rules.insert(
            "technology".to_string(),
            IndustryRules {
                industry_name: "Technology".to_string(),
                required_keywords: vec![
                    "software".to_string(),
                    "programming".to_string(),
                    "development".to_string(),
                    "code".to_string(),
                    "technical".to_string(),
                    "system".to_string(),
                ],
                preferred_keywords: vec![
                    "agile".to_string(),
                    "scrum".to_string(),
                    "devops".to_string(),
                    "cloud".to_string(),
                    "api".to_string(),
                    "database".to_string(),
                ],
                experience_levels: HashMap::new(), // Can be expanded later
                certifications: vec![
                    IndustryCertification {
                        name: "AWS Certified".to_string(),
                        importance: 0.9,
                        required_for_levels: vec!["Senior".to_string(), "Lead".to_string()],
                        alternatives: vec![
                            "Azure Certified".to_string(),
                            "Google Cloud".to_string(),
                        ],
                        validity_years: Some(3),
                    },
                    IndustryCertification {
                        name: "Kubernetes".to_string(),
                        importance: 0.8,
                        required_for_levels: vec!["DevOps".to_string()],
                        alternatives: vec!["Docker Certified".to_string()],
                        validity_years: Some(2),
                    },
                ],
                trends: Vec::new(),               // Can be expanded later
                role_hierarchies: HashMap::new(), // Can be expanded later
            },
        );

        // Finance Industry Rules
        rules.insert(
            "finance".to_string(),
            IndustryRules {
                industry_name: "Finance".to_string(),
                required_keywords: vec![
                    "financial".to_string(),
                    "accounting".to_string(),
                    "budget".to_string(),
                    "analysis".to_string(),
                    "reporting".to_string(),
                    "compliance".to_string(),
                ],
                preferred_keywords: vec![
                    "risk management".to_string(),
                    "audit".to_string(),
                    "investment".to_string(),
                    "portfolio".to_string(),
                    "regulatory".to_string(),
                    "sox".to_string(),
                ],
                experience_levels: HashMap::new(),
                certifications: vec![
                    IndustryCertification {
                        name: "CPA".to_string(),
                        importance: 0.95,
                        required_for_levels: vec![
                            "Senior Accountant".to_string(),
                            "Finance Manager".to_string(),
                        ],
                        alternatives: vec!["CMA".to_string()],
                        validity_years: None, // Permanent with continuing education
                    },
                    IndustryCertification {
                        name: "CFA".to_string(),
                        importance: 0.9,
                        required_for_levels: vec!["Investment Analyst".to_string()],
                        alternatives: vec!["FRM".to_string()],
                        validity_years: None,
                    },
                ],
                trends: Vec::new(),
                role_hierarchies: HashMap::new(),
            },
        );

        rules
    }

    fn build_industry_patterns() -> HashMap<String, Vec<Regex>> {
        let mut patterns = HashMap::new();

        // Technology patterns
        patterns.insert("technology".to_string(), vec![
            Regex::new(r"(?i)\b(?:software|programming|development|coding|algorithm|database|api|framework|javascript|python|java|react|angular|vue|node|docker|kubernetes|aws|azure|gcp|devops|agile|scrum|git)\b").unwrap(),
        ]);

        // Healthcare patterns
        patterns.insert("healthcare".to_string(), vec![
            Regex::new(r"(?i)\b(?:medical|clinical|patient|healthcare|hospital|physician|nurse|pharmacy|medical device|fda|hipaa|ehr|emr|clinical trial)\b").unwrap(),
        ]);

        // Finance patterns
        patterns.insert("finance".to_string(), vec![
            Regex::new(r"(?i)\b(?:financial|banking|investment|portfolio|trading|risk management|compliance|audit|accounting|fintech|blockchain|cryptocurrency|loan|credit|insurance)\b").unwrap(),
        ]);

        patterns
    }

    fn build_experience_patterns() -> Vec<Regex> {
        vec![
            Regex::new(r"(?i)(\d+)\+?\s*years?\s+(?:of\s+)?(?:experience|exp)").unwrap(),
            Regex::new(r"(?i)over\s+(\d+)\s+years?").unwrap(),
            Regex::new(r"(?i)(\d+)\+?\s*yrs?\s+(?:of\s+)?(?:experience|exp)").unwrap(),
            Regex::new(r"(?i)more than\s+(\d+)\s+years?").unwrap(),
        ]
    }

    fn build_leadership_patterns() -> Vec<Regex> {
        vec![
            Regex::new(r"(?i)\b(?:led|managed|supervised|directed|coordinated|oversaw)\s+(?:a\s+)?(?:team|group|department|division)").unwrap(),
            Regex::new(r"(?i)\b(?:managed|supervised)\s+\d+\s+(?:people|employees|staff|members)").unwrap(),
            Regex::new(r"(?i)\b(?:team\s+lead|team\s+leader|project\s+manager|scrum\s+master|tech\s+lead)").unwrap(),
            Regex::new(r"(?i)\b(?:mentored|coached|trained)\s+(?:junior|new|team)").unwrap(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;

    async fn setup_test_analyzer() -> IndustryAnalyzer {
        let db = Database::new().await.unwrap();
        IndustryAnalyzer::new(db)
    }

    #[tokio::test]
    async fn test_industry_detection() {
        let analyzer = setup_test_analyzer().await;
        let resume_content = "Experienced software engineer with Python and React development";

        let detected = analyzer
            .detect_industry_from_content(resume_content)
            .await
            .unwrap();
        assert_eq!(detected, "technology");
    }

    #[tokio::test]
    async fn test_experience_estimation() {
        let analyzer = IndustryAnalyzer::new(Database::new().await.unwrap());
        let content = "Senior software engineer with 5 years of experience";

        let years = analyzer.estimate_years_of_experience(&content.to_lowercase());
        assert_eq!(years, Some(5));
    }

    #[tokio::test]
    async fn test_role_level_assessment() {
        let analyzer = IndustryAnalyzer::new(Database::new().await.unwrap());
        let resume_content = "Senior software engineer who led a team of 5 developers";
        let job_description = "Looking for a senior engineer";

        let assessment = analyzer.assess_role_level(resume_content, job_description);
        assert_eq!(assessment.detected_level, "senior");
        assert!(assessment.confidence > 0.5);
    }
}
