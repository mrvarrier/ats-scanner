use anyhow::{Context, Result};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::database::Database;
use crate::models::{AnalysisResult, CategoryScores};
use crate::semantic_analyzer::{SemanticAnalysisResult, SemanticAnalyzer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedAnalysisResult {
    pub base_analysis: AnalysisResult,
    pub semantic_analysis: SemanticAnalysisResult,
    pub industry_analysis: IndustryAnalysisResult,
    pub ats_compatibility: ATSCompatibilityResult,
    pub scoring_breakdown: ScoringBreakdown,
    pub optimization_suggestions: Vec<OptimizationSuggestion>,
    pub benchmarks_comparison: BenchmarkComparison,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryAnalysisResult {
    pub detected_industry: String,
    pub confidence_score: f64,
    pub role_level_assessment: RoleLevelAssessment,
    pub industry_keywords_score: f64,
    pub required_certifications: Vec<CertificationCheck>,
    pub industry_trends_alignment: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleLevelAssessment {
    pub detected_level: String, // entry, mid, senior, lead, executive
    pub confidence: f64,
    pub experience_indicators: Vec<String>,
    pub leadership_indicators: Vec<String>,
    pub years_of_experience_estimate: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificationCheck {
    pub certification_name: String,
    pub found: bool,
    pub importance: f64,
    pub alternatives: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ATSCompatibilityResult {
    pub overall_compatibility_score: f64,
    pub system_specific_scores: HashMap<String, f64>,
    pub format_issues: Vec<FormatIssue>,
    pub parsing_warnings: Vec<String>,
    pub ats_optimization_suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatIssue {
    pub issue_type: String,
    pub description: String,
    pub severity: String, // low, medium, high, critical
    pub suggestion: String,
    pub ats_systems_affected: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringBreakdown {
    pub weighted_scores: HashMap<String, WeightedScore>,
    pub industry_adjustments: HashMap<String, f64>,
    pub role_level_multipliers: HashMap<String, f64>,
    pub final_calculations: FinalCalculations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightedScore {
    pub raw_score: f64,
    pub weight: f64,
    pub adjusted_score: f64,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalCalculations {
    pub base_score: f64,
    pub industry_bonus: f64,
    pub role_level_bonus: f64,
    pub semantic_bonus: f64,
    pub ats_penalty: f64,
    pub final_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    pub category: String,
    pub priority: String, // high, medium, low
    pub title: String,
    pub description: String,
    pub expected_impact: f64,
    pub implementation_difficulty: String, // easy, medium, hard
    pub specific_actions: Vec<String>,
    pub ats_systems_helped: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkComparison {
    pub industry_benchmark: f64,
    pub role_level_benchmark: f64,
    pub percentile_ranking: f64,
    pub peer_comparison: PeerComparison,
    pub improvement_potential: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerComparison {
    pub above_average_areas: Vec<String>,
    pub below_average_areas: Vec<String>,
    pub standout_strengths: Vec<String>,
    pub critical_gaps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringWeights {
    pub technical_skills: f64,
    pub experience: f64,
    pub education: f64,
    pub keywords: f64,
    pub format: f64,
    pub leadership: f64,
    pub certifications: f64,
    pub industry_specific: f64,
}

impl Default for ScoringWeights {
    fn default() -> Self {
        ScoringWeights {
            technical_skills: 0.25,
            experience: 0.20,
            education: 0.15,
            keywords: 0.15,
            format: 0.10,
            leadership: 0.05,
            certifications: 0.05,
            industry_specific: 0.05,
        }
    }
}

pub struct EnhancedScoringEngine {
    database: Database,
    semantic_analyzer: SemanticAnalyzer,
}

impl EnhancedScoringEngine {
    pub fn new(database: Database) -> Self {
        let semantic_analyzer = SemanticAnalyzer::new(database.clone());

        EnhancedScoringEngine {
            database,
            semantic_analyzer,
        }
    }

    pub async fn comprehensive_analysis(
        &self,
        resume_content: &str,
        job_description: &str,
        target_industry: &str,
        target_role_level: &str,
    ) -> Result<EnhancedAnalysisResult> {
        info!(
            "Starting comprehensive analysis for {} level position in {}",
            target_role_level, target_industry
        );

        // 1. Run semantic analysis
        let semantic_result = self
            .semantic_analyzer
            .analyze_semantic_keywords(resume_content, job_description, target_industry)
            .await
            .context("Failed to perform semantic analysis")?;

        // 2. Industry-specific analysis
        let industry_result = self
            .analyze_for_industry(resume_content, job_description, target_industry)
            .await?;

        // 3. ATS compatibility analysis
        let ats_result = self.analyze_ats_compatibility(resume_content).await?;

        // 4. Generate dynamic scoring weights
        let scoring_weights = self.calculate_dynamic_weights(target_industry, target_role_level);

        // 5. Calculate enhanced scores
        let scoring_breakdown = self
            .calculate_enhanced_scores(
                resume_content,
                job_description,
                &semantic_result,
                &industry_result,
                &ats_result,
                &scoring_weights,
            )
            .await?;

        // 6. Generate optimization suggestions
        let optimization_suggestions = self.generate_optimization_suggestions(
            &semantic_result,
            &industry_result,
            &ats_result,
            &scoring_breakdown,
        );

        // 7. Benchmark comparison
        let benchmarks_comparison = self
            .compare_with_benchmarks(target_industry, target_role_level, &scoring_breakdown)
            .await?;

        // 8. Create base analysis result for compatibility
        let base_analysis = self.create_base_analysis_result(&scoring_breakdown);

        Ok(EnhancedAnalysisResult {
            base_analysis,
            semantic_analysis: semantic_result,
            industry_analysis: industry_result,
            ats_compatibility: ats_result,
            scoring_breakdown,
            optimization_suggestions,
            benchmarks_comparison,
        })
    }

    async fn analyze_for_industry(
        &self,
        resume_content: &str,
        job_description: &str,
        industry: &str,
    ) -> Result<IndustryAnalysisResult> {
        // Detect industry from content
        let detected_industry = self.detect_industry_from_content(resume_content).await?;
        let confidence_score = self.calculate_industry_confidence(&detected_industry, industry);

        // Assess role level
        let role_level_assessment = self.assess_role_level(resume_content, job_description);

        // Check industry keywords
        let industry_keywords = self.database.get_industry_keywords(industry).await?;
        let industry_keywords_score =
            self.calculate_industry_keywords_score(resume_content, &industry_keywords);

        // Check certifications
        let required_certifications = self.check_industry_certifications(resume_content, industry);

        // Calculate trends alignment
        let industry_trends_alignment = self
            .calculate_trends_alignment(resume_content, industry)
            .await?;

        Ok(IndustryAnalysisResult {
            detected_industry,
            confidence_score,
            role_level_assessment,
            industry_keywords_score,
            required_certifications,
            industry_trends_alignment,
        })
    }

    async fn analyze_ats_compatibility(
        &self,
        resume_content: &str,
    ) -> Result<ATSCompatibilityResult> {
        let mut system_specific_scores = HashMap::new();
        let mut format_issues = Vec::new();
        let mut parsing_warnings = Vec::new();
        let mut ats_optimization_suggestions = Vec::new();

        // Get ATS rules from database
        let ats_rules = self.database.get_ats_rules(None).await?;

        // Analyze for each ATS system
        let ats_systems = vec!["greenhouse", "lever", "workday", "taleo", "icims"];

        for system in &ats_systems {
            let system_rules = ats_rules.iter().filter(|rule| rule.ats_system == *system);
            let mut system_score = 100.0;

            for rule in system_rules {
                if self.check_ats_rule_violation(resume_content, rule) {
                    system_score -= rule.penalty_weight * 10.0; // Convert to percentage points

                    format_issues.push(FormatIssue {
                        issue_type: rule.rule_type.clone(),
                        description: rule.rule_description.clone(),
                        severity: rule.severity.clone(),
                        suggestion: rule.suggestion.clone(),
                        ats_systems_affected: vec![system.to_string()],
                    });
                }
            }

            system_specific_scores.insert(system.to_string(), system_score.max(0.0));
        }

        // Calculate overall compatibility
        let overall_compatibility_score = if system_specific_scores.is_empty() {
            85.0 // Default score if no rules available
        } else {
            system_specific_scores.values().sum::<f64>() / system_specific_scores.len() as f64
        };

        // Generate parsing warnings
        parsing_warnings.extend(self.detect_parsing_issues(resume_content));

        // Generate optimization suggestions
        ats_optimization_suggestions
            .extend(self.generate_ats_optimization_suggestions(&format_issues));

        Ok(ATSCompatibilityResult {
            overall_compatibility_score,
            system_specific_scores,
            format_issues,
            parsing_warnings,
            ats_optimization_suggestions,
        })
    }

    fn calculate_dynamic_weights(&self, industry: &str, role_level: &str) -> ScoringWeights {
        let mut weights = ScoringWeights::default();

        // Adjust weights based on industry
        match industry.to_lowercase().as_str() {
            "technology" | "software" | "it" => {
                weights.technical_skills = 0.35;
                weights.experience = 0.25;
                weights.education = 0.10;
                weights.keywords = 0.15;
                weights.certifications = 0.10;
                weights.industry_specific = 0.05;
            }
            "healthcare" | "medical" => {
                weights.certifications = 0.30;
                weights.education = 0.25;
                weights.experience = 0.20;
                weights.technical_skills = 0.10;
                weights.keywords = 0.10;
                weights.industry_specific = 0.05;
            }
            "finance" | "banking" => {
                weights.certifications = 0.25;
                weights.experience = 0.25;
                weights.education = 0.20;
                weights.technical_skills = 0.15;
                weights.keywords = 0.10;
                weights.industry_specific = 0.05;
            }
            "education" | "academic" => {
                weights.education = 0.35;
                weights.experience = 0.20;
                weights.certifications = 0.15;
                weights.technical_skills = 0.10;
                weights.keywords = 0.15;
                weights.industry_specific = 0.05;
            }
            _ => {
                // Keep default weights for unknown industries
            }
        }

        // Adjust weights based on role level
        match role_level.to_lowercase().as_str() {
            "senior" | "lead" | "principal" => {
                weights.leadership += 0.10;
                weights.experience += 0.05;
                weights.technical_skills -= 0.05;
                weights.education -= 0.05;
                weights.keywords -= 0.05;
            }
            "executive" | "director" | "vp" | "cto" | "ceo" => {
                weights.leadership += 0.20;
                weights.experience += 0.10;
                weights.technical_skills -= 0.10;
                weights.education -= 0.10;
                weights.keywords -= 0.10;
            }
            "entry" | "junior" | "intern" => {
                weights.education += 0.10;
                weights.technical_skills += 0.05;
                weights.experience -= 0.10;
                weights.leadership -= 0.05;
            }
            _ => {
                // Keep adjusted weights for mid-level roles
            }
        }

        // Ensure weights sum to approximately 1.0
        let total_weight: f64 = weights.technical_skills
            + weights.experience
            + weights.education
            + weights.keywords
            + weights.format
            + weights.leadership
            + weights.certifications
            + weights.industry_specific;

        if total_weight > 0.0 {
            let normalization_factor = 1.0 / total_weight;
            weights.technical_skills *= normalization_factor;
            weights.experience *= normalization_factor;
            weights.education *= normalization_factor;
            weights.keywords *= normalization_factor;
            weights.format *= normalization_factor;
            weights.leadership *= normalization_factor;
            weights.certifications *= normalization_factor;
            weights.industry_specific *= normalization_factor;
        }

        weights
    }

    async fn calculate_enhanced_scores(
        &self,
        resume_content: &str,
        _job_description: &str,
        semantic_result: &SemanticAnalysisResult,
        industry_result: &IndustryAnalysisResult,
        ats_result: &ATSCompatibilityResult,
        weights: &ScoringWeights,
    ) -> Result<ScoringBreakdown> {
        let mut weighted_scores = HashMap::new();

        // Calculate individual scores
        let technical_score =
            self.calculate_technical_skills_score(resume_content, semantic_result);
        let experience_score =
            self.calculate_experience_score(resume_content, &industry_result.role_level_assessment);
        let education_score = self.calculate_education_score(resume_content);
        let keywords_score = semantic_result.industry_relevance_score * 100.0;
        let format_score = ats_result.overall_compatibility_score;
        let leadership_score =
            self.calculate_leadership_score(resume_content, &industry_result.role_level_assessment);
        let certifications_score =
            self.calculate_certifications_score(&industry_result.required_certifications);
        let industry_specific_score = industry_result.industry_keywords_score * 100.0;

        // Create weighted scores
        weighted_scores.insert(
            "technical_skills".to_string(),
            WeightedScore {
                raw_score: technical_score,
                weight: weights.technical_skills,
                adjusted_score: technical_score * weights.technical_skills,
                explanation: "Technical skills relevance based on semantic analysis".to_string(),
            },
        );

        weighted_scores.insert(
            "experience".to_string(),
            WeightedScore {
                raw_score: experience_score,
                weight: weights.experience,
                adjusted_score: experience_score * weights.experience,
                explanation: "Experience level and relevance assessment".to_string(),
            },
        );

        weighted_scores.insert(
            "education".to_string(),
            WeightedScore {
                raw_score: education_score,
                weight: weights.education,
                adjusted_score: education_score * weights.education,
                explanation: "Educational background relevance".to_string(),
            },
        );

        weighted_scores.insert(
            "keywords".to_string(),
            WeightedScore {
                raw_score: keywords_score,
                weight: weights.keywords,
                adjusted_score: keywords_score * weights.keywords,
                explanation: "Industry keyword matching and semantic relevance".to_string(),
            },
        );

        weighted_scores.insert(
            "format".to_string(),
            WeightedScore {
                raw_score: format_score,
                weight: weights.format,
                adjusted_score: format_score * weights.format,
                explanation: "ATS compatibility and format optimization".to_string(),
            },
        );

        weighted_scores.insert(
            "leadership".to_string(),
            WeightedScore {
                raw_score: leadership_score,
                weight: weights.leadership,
                adjusted_score: leadership_score * weights.leadership,
                explanation: "Leadership experience and potential".to_string(),
            },
        );

        weighted_scores.insert(
            "certifications".to_string(),
            WeightedScore {
                raw_score: certifications_score,
                weight: weights.certifications,
                adjusted_score: certifications_score * weights.certifications,
                explanation: "Industry-relevant certifications and credentials".to_string(),
            },
        );

        weighted_scores.insert(
            "industry_specific".to_string(),
            WeightedScore {
                raw_score: industry_specific_score,
                weight: weights.industry_specific,
                adjusted_score: industry_specific_score * weights.industry_specific,
                explanation: "Industry-specific knowledge and terminology".to_string(),
            },
        );

        // Calculate adjustments and bonuses
        let industry_adjustments = self.calculate_industry_adjustments(industry_result);
        let role_level_multipliers =
            self.calculate_role_level_multipliers(&industry_result.role_level_assessment);

        // Calculate final score
        let base_score = weighted_scores
            .values()
            .map(|ws| ws.adjusted_score)
            .sum::<f64>();
        let industry_bonus = industry_adjustments.values().sum::<f64>();
        let role_level_bonus = role_level_multipliers.values().sum::<f64>();
        let semantic_bonus = (semantic_result.confidence_score - 0.5).max(0.0) * 10.0; // Up to 5 point bonus
        let ats_penalty = (85.0 - ats_result.overall_compatibility_score).max(0.0) * 0.1; // Penalty for poor ATS compatibility

        let final_score = (base_score + industry_bonus + role_level_bonus + semantic_bonus
            - ats_penalty)
            .clamp(0.0, 100.0);

        let final_calculations = FinalCalculations {
            base_score,
            industry_bonus,
            role_level_bonus,
            semantic_bonus,
            ats_penalty,
            final_score,
        };

        Ok(ScoringBreakdown {
            weighted_scores,
            industry_adjustments,
            role_level_multipliers,
            final_calculations,
        })
    }

    // Helper methods for scoring calculations
    fn calculate_technical_skills_score(
        &self,
        _resume_content: &str,
        semantic_result: &SemanticAnalysisResult,
    ) -> f64 {
        let technical_keywords = semantic_result
            .keyword_matches
            .iter()
            .filter(|km| {
                matches!(
                    km.category.as_str(),
                    "programming_language" | "framework" | "technology" | "tool"
                )
            })
            .collect::<Vec<_>>();

        if technical_keywords.is_empty() {
            return 50.0; // Default score if no technical keywords found
        }

        let total_relevance: f64 = technical_keywords
            .iter()
            .map(|km| km.relevance_score * km.weight)
            .sum();

        let average_relevance = total_relevance / technical_keywords.len() as f64;
        (average_relevance * 100.0).min(100.0)
    }

    fn calculate_experience_score(
        &self,
        _resume_content: &str,
        role_assessment: &RoleLevelAssessment,
    ) -> f64 {
        let mut score = 50.0; // Base score

        // Years of experience component
        if let Some(years) = role_assessment.years_of_experience_estimate {
            score += (years as f64 * 2.0).min(30.0); // Up to 30 points for 15+ years
        }

        // Role level component
        let role_bonus = match role_assessment.detected_level.as_str() {
            "entry" => 5.0,
            "mid" => 15.0,
            "senior" => 25.0,
            "lead" | "principal" => 35.0,
            "executive" => 40.0,
            _ => 10.0,
        };

        score += role_bonus;

        // Experience indicators component
        score += (role_assessment.experience_indicators.len() as f64 * 2.0).min(20.0);

        score.min(100.0)
    }

    fn calculate_education_score(&self, resume_content: &str) -> f64 {
        let content_lower = resume_content.to_lowercase();
        let mut score: f64 = 0.0;

        // Education levels
        if content_lower.contains("phd") || content_lower.contains("doctorate") {
            score += 40.0;
        } else if content_lower.contains("master") || content_lower.contains("mba") {
            score += 30.0;
        } else if content_lower.contains("bachelor") || content_lower.contains("degree") {
            score += 20.0;
        } else if content_lower.contains("associate") || content_lower.contains("diploma") {
            score += 10.0;
        }

        // Relevant institutions or programs
        if content_lower.contains("computer science") || content_lower.contains("engineering") {
            score += 15.0;
        }

        // Ongoing education
        if content_lower.contains("continuing education")
            || content_lower.contains("professional development")
        {
            score += 10.0;
        }

        score.min(100.0)
    }

    fn calculate_leadership_score(
        &self,
        resume_content: &str,
        role_assessment: &RoleLevelAssessment,
    ) -> f64 {
        let mut score = 0.0;

        // Leadership indicators from role assessment
        score += (role_assessment.leadership_indicators.len() as f64 * 10.0).min(40.0);

        // Leadership keywords in content
        let leadership_keywords = [
            "led",
            "managed",
            "supervised",
            "directed",
            "coordinated",
            "mentored",
            "team lead",
            "project manager",
            "scrum master",
            "architect",
            "principal",
        ];

        let content_lower = resume_content.to_lowercase();
        for keyword in &leadership_keywords {
            if content_lower.contains(keyword) {
                score += 5.0;
            }
        }

        // Role level bonus
        let role_bonus = match role_assessment.detected_level.as_str() {
            "lead" | "principal" => 20.0,
            "senior" => 15.0,
            "executive" => 30.0,
            _ => 0.0,
        };

        score += role_bonus;
        score.min(100.0)
    }

    fn calculate_certifications_score(&self, certifications: &[CertificationCheck]) -> f64 {
        if certifications.is_empty() {
            return 50.0; // Default score if no certifications required
        }

        let found_count = certifications.iter().filter(|cert| cert.found).count();
        let total_importance: f64 = certifications.iter().map(|cert| cert.importance).sum();
        let found_importance: f64 = certifications
            .iter()
            .filter(|cert| cert.found)
            .map(|cert| cert.importance)
            .sum();

        if total_importance > 0.0 {
            (found_importance / total_importance * 100.0).min(100.0)
        } else {
            (found_count as f64 / certifications.len() as f64 * 100.0).min(100.0)
        }
    }

    // More helper methods would continue here...
    // [Additional implementation methods would be added here for brevity]

    fn create_base_analysis_result(&self, scoring_breakdown: &ScoringBreakdown) -> AnalysisResult {
        let final_score = scoring_breakdown.final_calculations.final_score;

        AnalysisResult {
            overall_score: final_score,
            category_scores: CategoryScores {
                skills: scoring_breakdown
                    .weighted_scores
                    .get("technical_skills")
                    .map(|ws| ws.raw_score)
                    .unwrap_or(0.0),
                experience: scoring_breakdown
                    .weighted_scores
                    .get("experience")
                    .map(|ws| ws.raw_score)
                    .unwrap_or(0.0),
                education: scoring_breakdown
                    .weighted_scores
                    .get("education")
                    .map(|ws| ws.raw_score)
                    .unwrap_or(0.0),
                keywords: scoring_breakdown
                    .weighted_scores
                    .get("keywords")
                    .map(|ws| ws.raw_score)
                    .unwrap_or(0.0),
                format: scoring_breakdown
                    .weighted_scores
                    .get("format")
                    .map(|ws| ws.raw_score)
                    .unwrap_or(0.0),
            },
            detailed_feedback: format!(
                "Enhanced analysis completed with {} confidence",
                scoring_breakdown.final_calculations.final_score
            ),
            missing_keywords: Vec::new(),
            recommendations: Vec::new(),
            processing_time_ms: 0, // Will be set by caller
        }
    }

    // ML-powered industry detection and confidence scoring
    async fn detect_industry_from_content(&self, resume_content: &str) -> Result<String> {
        use crate::ollama::OllamaClient;

        let industry_classification_prompt = format!(
            "Classify this resume content into the most appropriate industry category.

Resume Content:
{}

Industry Categories (choose exactly one):
- Technology/Software
- Healthcare/Medical
- Finance/Banking
- Education/Academic
- Manufacturing/Engineering
- Retail/Consumer
- Government/Public
- Non-profit/NGO
- Media/Entertainment
- Consulting/Professional
- Real Estate
- Transportation/Logistics
- Energy/Utilities
- Legal/Law
- Construction

Analyze the content for:
1. Technical skills and tools mentioned
2. Job titles and roles described
3. Company types and industries referenced
4. Projects and accomplishments context
5. Educational background relevance
6. Certifications and specializations
7. Industry-specific terminology used

Return ONLY the single most appropriate industry category from the list above.",
            resume_content
        );

        let ollama_client = OllamaClient::new(None)?;
        let response = ollama_client
            .generate_ml_analysis(
                "qwen2.5:14b",
                &industry_classification_prompt,
                "industry_detection",
            )
            .await?;

        // Clean and validate response
        let detected_industry = self.validate_and_normalize_industry(&response);

        if detected_industry != "unknown" {
            info!("ML detected industry: {}", detected_industry);
            Ok(detected_industry)
        } else {
            log::warn!("ML industry detection failed or returned invalid category, using fallback");
            self.fallback_industry_detection(resume_content)
        }
    }

    fn validate_and_normalize_industry(&self, ml_response: &str) -> String {
        let response_lower = ml_response.trim().to_lowercase();

        // Map of valid industries with their variations
        let industry_mappings = vec![
            (
                "technology/software",
                vec![
                    "technology",
                    "software",
                    "tech",
                    "it",
                    "information technology",
                ],
            ),
            (
                "healthcare/medical",
                vec!["healthcare", "medical", "health", "medicine", "clinical"],
            ),
            (
                "finance/banking",
                vec!["finance", "banking", "financial", "fintech", "investment"],
            ),
            (
                "education/academic",
                vec!["education", "academic", "university", "school", "teaching"],
            ),
            (
                "manufacturing/engineering",
                vec![
                    "manufacturing",
                    "engineering",
                    "industrial",
                    "mechanical",
                    "production",
                ],
            ),
            (
                "retail/consumer",
                vec!["retail", "consumer", "sales", "ecommerce", "commerce"],
            ),
            (
                "government/public",
                vec!["government", "public", "federal", "state", "municipal"],
            ),
            (
                "non-profit/ngo",
                vec!["non-profit", "ngo", "nonprofit", "charity", "foundation"],
            ),
            (
                "media/entertainment",
                vec![
                    "media",
                    "entertainment",
                    "broadcasting",
                    "journalism",
                    "creative",
                ],
            ),
            (
                "consulting/professional",
                vec!["consulting", "professional", "advisory", "services"],
            ),
            (
                "real estate",
                vec!["real estate", "property", "realty", "real-estate"],
            ),
            (
                "transportation/logistics",
                vec!["transportation", "logistics", "shipping", "supply chain"],
            ),
            (
                "energy/utilities",
                vec!["energy", "utilities", "power", "renewable", "oil", "gas"],
            ),
            (
                "legal/law",
                vec!["legal", "law", "attorney", "lawyer", "juridical"],
            ),
            (
                "construction",
                vec!["construction", "building", "architecture", "contractor"],
            ),
        ];

        // Find best match
        for (canonical_industry, variations) in &industry_mappings {
            for variation in variations {
                if response_lower.contains(variation) {
                    return canonical_industry.to_string();
                }
            }
        }

        "unknown".to_string()
    }

    fn fallback_industry_detection(&self, resume_content: &str) -> Result<String> {
        let content_lower = resume_content.to_lowercase();

        // Comprehensive keyword-based industry detection
        let industry_keyword_weights = vec![
            (
                "technology/software",
                vec![
                    ("software", 3.0),
                    ("programming", 3.0),
                    ("developer", 3.0),
                    ("engineer", 2.0),
                    ("code", 2.0),
                    ("api", 2.0),
                    ("database", 2.0),
                    ("web", 2.0),
                    ("mobile", 2.0),
                    ("application", 1.5),
                    ("javascript", 3.0),
                    ("python", 3.0),
                    ("java", 2.5),
                    ("react", 2.5),
                    ("node", 2.0),
                    ("aws", 2.5),
                    ("cloud", 2.0),
                    ("devops", 3.0),
                    ("agile", 1.5),
                    ("scrum", 1.5),
                    ("git", 2.0),
                    ("docker", 2.5),
                    ("kubernetes", 2.5),
                ],
            ),
            (
                "healthcare/medical",
                vec![
                    ("medical", 3.0),
                    ("healthcare", 3.0),
                    ("patient", 3.0),
                    ("clinical", 3.0),
                    ("hospital", 2.5),
                    ("doctor", 2.5),
                    ("nurse", 2.5),
                    ("physician", 2.5),
                    ("treatment", 2.0),
                    ("diagnosis", 2.0),
                    ("pharmaceutical", 2.5),
                    ("biomedical", 2.5),
                    ("surgery", 2.0),
                    ("therapy", 2.0),
                    ("medical device", 2.5),
                    ("hipaa", 2.0),
                    ("epic", 1.5),
                    ("emr", 2.0),
                    ("ehr", 2.0),
                    ("fda", 2.0),
                ],
            ),
            (
                "finance/banking",
                vec![
                    ("financial", 3.0),
                    ("banking", 3.0),
                    ("investment", 3.0),
                    ("trading", 2.5),
                    ("accounting", 2.5),
                    ("finance", 3.0),
                    ("portfolio", 2.0),
                    ("risk", 2.0),
                    ("compliance", 2.0),
                    ("audit", 2.0),
                    ("credit", 2.0),
                    ("loan", 2.0),
                    ("mortgage", 2.0),
                    ("derivatives", 2.5),
                    ("securities", 2.5),
                    ("fintech", 3.0),
                    ("blockchain", 2.0),
                    ("cryptocurrency", 2.0),
                    ("cfa", 2.5),
                    ("cpa", 2.5),
                ],
            ),
            (
                "education/academic",
                vec![
                    ("education", 3.0),
                    ("teaching", 3.0),
                    ("professor", 2.5),
                    ("university", 2.5),
                    ("school", 2.0),
                    ("curriculum", 2.0),
                    ("student", 2.0),
                    ("research", 2.5),
                    ("academic", 3.0),
                    ("learning", 2.0),
                    ("instruction", 2.0),
                    ("pedagogy", 2.5),
                    ("classroom", 2.0),
                    ("course", 1.5),
                    ("degree", 1.5),
                    ("scholarship", 2.0),
                ],
            ),
            (
                "manufacturing/engineering",
                vec![
                    ("manufacturing", 3.0),
                    ("production", 2.5),
                    ("assembly", 2.0),
                    ("quality", 2.0),
                    ("engineering", 2.5),
                    ("mechanical", 2.5),
                    ("electrical", 2.5),
                    ("industrial", 2.5),
                    ("process", 2.0),
                    ("automation", 2.5),
                    ("machinery", 2.0),
                    ("factory", 2.0),
                    ("plant", 2.0),
                    ("operations", 2.0),
                    ("supply chain", 2.0),
                    ("lean", 2.0),
                    ("six sigma", 2.5),
                    ("cad", 2.0),
                    ("plc", 2.0),
                ],
            ),
            (
                "consulting/professional",
                vec![
                    ("consulting", 3.0),
                    ("advisory", 2.5),
                    ("strategy", 2.5),
                    ("management", 2.0),
                    ("business", 2.0),
                    ("project", 1.5),
                    ("client", 2.0),
                    ("stakeholder", 2.0),
                    ("analysis", 2.0),
                    ("recommendations", 2.0),
                    ("implementation", 2.0),
                    ("change management", 2.5),
                    ("transformation", 2.5),
                    ("optimization", 2.0),
                ],
            ),
        ];

        let mut industry_scores: std::collections::HashMap<String, f64> =
            std::collections::HashMap::new();

        for (industry, keywords) in industry_keyword_weights {
            let mut score = 0.0;
            for (keyword, weight) in keywords {
                if content_lower.contains(keyword) {
                    score += weight;
                }
            }

            if score > 0.0 {
                industry_scores.insert(industry.to_string(), score);
            }
        }

        let detected_industry = industry_scores
            .iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(industry, score)| {
                info!(
                    "Fallback industry detection: {} with score: {:.1}",
                    industry, score
                );
                industry.clone()
            })
            .unwrap_or_else(|| "general".to_string());

        Ok(detected_industry)
    }

    fn calculate_industry_confidence(&self, detected: &str, target: &str) -> f64 {
        let detected_lower = detected.to_lowercase();
        let target_lower = target.to_lowercase();

        // Exact match gets highest confidence
        if detected_lower == target_lower {
            return 0.95;
        }

        // Check for semantic similarity and related industries
        let confidence = if self.industries_are_related(&detected_lower, &target_lower) {
            0.75 // High confidence for related industries
        } else if detected != "unknown" && detected != "general" {
            // We detected something specific, but it doesn't match target
            let similarity_score =
                self.calculate_industry_similarity(&detected_lower, &target_lower);
            0.40 + (similarity_score * 0.30) // Base confidence + similarity bonus
        } else {
            0.30 // Low confidence for unknown/general classification
        };

        info!(
            "Industry confidence: detected='{}', target='{}', confidence={:.2}",
            detected, target, confidence
        );
        confidence
    }

    fn industries_are_related(&self, industry1: &str, industry2: &str) -> bool {
        let related_groups = vec![
            vec![
                "technology/software",
                "manufacturing/engineering",
                "consulting/professional",
            ],
            vec!["healthcare/medical", "education/academic"],
            vec!["finance/banking", "consulting/professional", "real estate"],
            vec!["government/public", "non-profit/ngo", "education/academic"],
            vec![
                "media/entertainment",
                "consulting/professional",
                "technology/software",
            ],
            vec!["transportation/logistics", "manufacturing/engineering"],
            vec![
                "energy/utilities",
                "manufacturing/engineering",
                "consulting/professional",
            ],
            vec!["legal/law", "consulting/professional", "government/public"],
            vec!["construction", "manufacturing/engineering", "real estate"],
        ];

        related_groups
            .iter()
            .any(|group| group.contains(&industry1) && group.contains(&industry2))
    }

    fn calculate_industry_similarity(&self, industry1: &str, industry2: &str) -> f64 {
        // Simple keyword overlap-based similarity
        let words1: std::collections::HashSet<&str> =
            industry1.split(&['/', '-', ' '][..]).collect();
        let words2: std::collections::HashSet<&str> =
            industry2.split(&['/', '-', ' '][..]).collect();

        let intersection: std::collections::HashSet<_> = words1.intersection(&words2).collect();
        let union: std::collections::HashSet<_> = words1.union(&words2).collect();

        if union.is_empty() {
            0.0
        } else {
            intersection.len() as f64 / union.len() as f64
        }
    }

    fn assess_role_level(
        &self,
        _resume_content: &str,
        _job_description: &str,
    ) -> RoleLevelAssessment {
        RoleLevelAssessment {
            detected_level: "mid".to_string(),
            confidence: 0.8,
            experience_indicators: vec!["3+ years experience".to_string()],
            leadership_indicators: vec![],
            years_of_experience_estimate: Some(5),
        }
    }

    fn calculate_industry_keywords_score(
        &self,
        _resume_content: &str,
        _keywords: &[crate::models::IndustryKeyword],
    ) -> f64 {
        0.75 // Simplified implementation
    }

    fn check_industry_certifications(
        &self,
        _resume_content: &str,
        _industry: &str,
    ) -> Vec<CertificationCheck> {
        vec![] // Simplified implementation
    }

    async fn calculate_trends_alignment(
        &self,
        _resume_content: &str,
        _industry: &str,
    ) -> Result<f64> {
        Ok(0.8) // Simplified implementation
    }

    fn check_ats_rule_violation(
        &self,
        _resume_content: &str,
        _rule: &crate::models::ATSCompatibilityRule,
    ) -> bool {
        false // Simplified implementation
    }

    fn detect_parsing_issues(&self, _resume_content: &str) -> Vec<String> {
        vec![] // Simplified implementation
    }

    fn generate_ats_optimization_suggestions(&self, _issues: &[FormatIssue]) -> Vec<String> {
        vec![] // Simplified implementation
    }

    fn calculate_industry_adjustments(
        &self,
        _industry_result: &IndustryAnalysisResult,
    ) -> HashMap<String, f64> {
        HashMap::new() // Simplified implementation
    }

    fn calculate_role_level_multipliers(
        &self,
        _role_assessment: &RoleLevelAssessment,
    ) -> HashMap<String, f64> {
        HashMap::new() // Simplified implementation
    }

    fn generate_optimization_suggestions(
        &self,
        _semantic_result: &SemanticAnalysisResult,
        _industry_result: &IndustryAnalysisResult,
        _ats_result: &ATSCompatibilityResult,
        _scoring_breakdown: &ScoringBreakdown,
    ) -> Vec<OptimizationSuggestion> {
        vec![] // Simplified implementation
    }

    async fn compare_with_benchmarks(
        &self,
        _industry: &str,
        _role_level: &str,
        _scoring_breakdown: &ScoringBreakdown,
    ) -> Result<BenchmarkComparison> {
        Ok(BenchmarkComparison {
            industry_benchmark: 75.0,
            role_level_benchmark: 80.0,
            percentile_ranking: 65.0,
            peer_comparison: PeerComparison {
                above_average_areas: vec!["technical skills".to_string()],
                below_average_areas: vec!["leadership".to_string()],
                standout_strengths: vec!["programming skills".to_string()],
                critical_gaps: vec!["project management".to_string()],
            },
            improvement_potential: 20.0,
        })
    }
}
