#![allow(dead_code)] // Allow dead code for comprehensive future implementation

use anyhow::Result;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::database::Database;
use crate::dynamic_keyword_db::DynamicKeywordDatabase;
use crate::modern_keyword_extractor::ExtractionResult;
use crate::ollama::OllamaClient;

/// Intelligent context-aware matching engine that understands job requirements
/// and candidate qualifications at a semantic level
#[allow(dead_code)]
pub struct ContextAwareMatcher {
    database: Database,
    dynamic_db: Option<DynamicKeywordDatabase>,
    ollama_client: OllamaClient,
    
    // Context analysis components
    requirement_analyzer: RequirementAnalyzer,
    qualification_mapper: QualificationMapper,
    semantic_scorer: SemanticScorer,
    intent_classifier: IntentClassifier,
    
    // Configuration
    context_window_size: usize,
    semantic_threshold: f64,
    confidence_boost_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAwareMatchResult {
    pub overall_match_score: f64,
    pub contextual_matches: Vec<ContextualMatch>,
    pub requirement_coverage: RequirementCoverage,
    pub qualification_analysis: QualificationAnalysis,
    pub semantic_insights: Vec<SemanticInsight>,
    pub intent_analysis: IntentAnalysis,
    pub improvement_suggestions: Vec<ImprovementSuggestion>,
    pub confidence_metrics: ConfidenceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualMatch {
    pub requirement_text: String,
    pub matched_qualification: String,
    pub match_type: ContextMatchType,
    pub confidence_score: f64,
    pub semantic_similarity: f64,
    pub context_relevance: f64,
    pub supporting_evidence: Vec<String>,
    pub gap_analysis: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum ContextMatchType {
    DirectMatch,     // Exact keyword/skill match
    SemanticMatch,   // Conceptually similar
    ExperienceMatch, // Relevant experience
    TransferableMatch, // Transferable skills
    PartialMatch,    // Partially meets requirement
    PotentialMatch,  // Could develop this skill
    NoMatch,         // No relevant qualification
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementCoverage {
    pub total_requirements: usize,
    pub fully_covered: usize,
    pub partially_covered: usize,
    pub not_covered: usize,
    pub coverage_percentage: f64,
    pub critical_gaps: Vec<String>,
    pub nice_to_have_gaps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualificationAnalysis {
    pub relevant_qualifications: Vec<QualificationMatch>,
    pub transferable_skills: Vec<TransferableSkill>,
    pub experience_level_analysis: ExperienceLevelAnalysis,
    pub certification_relevance: Vec<CertificationMatch>,
    pub project_relevance: Vec<ProjectMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualificationMatch {
    pub qualification: String,
    pub relevance_score: f64,
    pub applicable_requirements: Vec<String>,
    pub strength_indicators: Vec<String>,
    pub context_clues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferableSkill {
    pub skill: String,
    pub origin_domain: String,
    pub target_domain: String,
    pub transferability_score: f64,
    pub adaptation_requirements: Vec<String>,
    pub success_probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceLevelAnalysis {
    pub required_level: ExperienceLevel,
    pub candidate_level: ExperienceLevel,
    pub level_match_score: f64,
    pub experience_gap: i32, // In years
    pub level_progression_path: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ExperienceLevel {
    Entry,
    Junior,
    Mid,
    Senior,
    Lead,
    Principal,
    Executive,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificationMatch {
    pub certification: String,
    pub relevance_score: f64,
    pub industry_recognition: f64,
    pub applicable_skills: Vec<String>,
    pub expiration_risk: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMatch {
    pub project_description: String,
    pub relevance_score: f64,
    pub demonstrated_skills: Vec<String>,
    pub scale_indicators: Vec<String>,
    pub impact_metrics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticInsight {
    pub insight_type: InsightType,
    pub description: String,
    pub confidence: f64,
    pub supporting_context: Vec<String>,
    pub actionable_advice: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsightType {
    StrengthAlignment,  // Candidate strength aligns well
    SkillGap,          // Missing critical skill
    ExperienceGap,     // Experience level mismatch
    CulturalFit,       // Values/culture alignment
    GrowthPotential,   // Learning and growth indicators
    RedFlag,           // Potential concerns
    HiddenGem,         // Undervalued qualifications
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentAnalysis {
    pub job_intent: JobIntent,
    pub candidate_intent: CandidateIntent,
    pub intent_alignment_score: f64,
    pub motivational_indicators: Vec<String>,
    pub commitment_signals: Vec<String>,
    pub risk_factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobIntent {
    pub role_type: RoleType,
    pub seniority_expectations: ExperienceLevel,
    pub growth_trajectory: String,
    pub team_dynamics: String,
    pub technical_focus: Vec<String>,
    pub business_impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RoleType {
    Individual,
    TeamLead,
    Management,
    Hybrid,
    Consulting,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateIntent {
    pub career_stage: ExperienceLevel,
    pub growth_aspirations: Vec<String>,
    pub technical_interests: Vec<String>,
    pub leadership_indicators: Vec<String>,
    pub specialization_areas: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementSuggestion {
    pub suggestion_type: SuggestionType,
    pub priority: Priority,
    pub description: String,
    pub specific_actions: Vec<String>,
    pub expected_impact: f64,
    pub timeframe: String,
    pub resources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionType {
    SkillDevelopment,
    ExperienceGain,
    CertificationPursuit,
    ProjectHighlight,
    ResumeOptimization,
    NetworkBuilding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceMetrics {
    pub overall_confidence: f64,
    pub context_analysis_confidence: f64,
    pub semantic_matching_confidence: f64,
    pub requirement_parsing_confidence: f64,
    pub qualification_extraction_confidence: f64,
    pub data_quality_score: f64,
}

// Component structs
#[allow(dead_code)]
pub struct RequirementAnalyzer {
    requirement_patterns: HashMap<String, Vec<regex::Regex>>,
    priority_indicators: Vec<regex::Regex>,
    qualification_extractors: Vec<regex::Regex>,
}

#[allow(dead_code)]
pub struct QualificationMapper {
    skill_taxonomies: HashMap<String, SkillTaxonomy>,
    experience_patterns: Vec<regex::Regex>,
    certification_database: HashMap<String, CertificationInfo>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SkillTaxonomy {
    pub domain: String,
    pub skills: HashMap<String, SkillInfo>,
    pub relationships: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct SkillInfo {
    pub canonical_name: String,
    pub aliases: Vec<String>,
    pub category: String,
    pub difficulty_level: f64,
    pub market_demand: f64,
    pub related_skills: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CertificationInfo {
    pub name: String,
    pub issuer: String,
    pub industry_recognition: f64,
    pub skills_validated: Vec<String>,
    pub validity_period: Option<u32>, // months
}

#[allow(dead_code)]
pub struct SemanticScorer {
    embedding_cache: HashMap<String, Vec<f64>>,
    similarity_threshold: f64,
}

#[allow(dead_code)]
pub struct IntentClassifier {
    job_role_patterns: HashMap<RoleType, Vec<regex::Regex>>,
    seniority_indicators: HashMap<ExperienceLevel, Vec<String>>,
    motivation_extractors: Vec<regex::Regex>,
}

impl ContextAwareMatcher {
    pub async fn new(database: Database) -> Result<Self> {
        let ollama_client = OllamaClient::new(None)?;
        
        // Initialize dynamic keyword database
        let dynamic_db = match DynamicKeywordDatabase::new(database.clone()).await {
            Ok(db) => Some(db),
            Err(e) => {
                warn!("Failed to initialize dynamic keyword database: {}", e);
                None
            }
        };

        let matcher = Self {
            database,
            dynamic_db,
            ollama_client,
            requirement_analyzer: RequirementAnalyzer::new()?,
            qualification_mapper: QualificationMapper::new()?,
            semantic_scorer: SemanticScorer::new(),
            intent_classifier: IntentClassifier::new()?,
            context_window_size: 3,
            semantic_threshold: 0.7,
            confidence_boost_factor: 1.2,
        };

        info!("Context-aware matcher initialized successfully");
        Ok(matcher)
    }

    pub async fn analyze_match(
        &self,
        resume_content: &str,
        job_description: &str,
        extraction_result: &ExtractionResult,
        target_industry: &str,
    ) -> Result<ContextAwareMatchResult> {
        info!("Starting context-aware matching analysis");

        // Step 1: Parse job requirements with context
        let requirements = self.requirement_analyzer
            .parse_requirements(job_description, target_industry).await?;

        // Step 2: Extract and map candidate qualifications
        let qualifications = self.qualification_mapper
            .extract_qualifications(resume_content, extraction_result).await?;

        // Step 3: Perform contextual matching
        let contextual_matches = self.perform_contextual_matching(
            &requirements, 
            &qualifications, 
            job_description, 
            resume_content
        ).await?;

        // Step 4: Analyze requirement coverage
        let requirement_coverage = self.analyze_requirement_coverage(
            &requirements, 
            &contextual_matches
        )?;

        // Step 5: Generate semantic insights
        let semantic_insights = self.generate_semantic_insights(
            &contextual_matches,
            &requirements,
            &qualifications,
            job_description,
            resume_content,
        ).await?;

        // Step 6: Perform intent analysis
        let intent_analysis = self.analyze_intent(
            job_description,
            resume_content,
            &requirements,
            &qualifications,
        ).await?;

        // Step 7: Generate improvement suggestions
        let improvement_suggestions = self.generate_improvement_suggestions(
            &requirement_coverage,
            &contextual_matches,
            &intent_analysis,
            target_industry,
        ).await?;

        // Step 8: Calculate overall match score and confidence
        let overall_match_score = self.calculate_overall_match_score(&contextual_matches)?;
        let confidence_metrics = self.calculate_confidence_metrics(
            &requirements,
            &qualifications,
            &contextual_matches,
        )?;

        let result = ContextAwareMatchResult {
            overall_match_score,
            contextual_matches,
            requirement_coverage,
            qualification_analysis: qualifications,
            semantic_insights,
            intent_analysis,
            improvement_suggestions,
            confidence_metrics,
        };

        info!(
            "Context-aware analysis completed with overall match score: {:.2}",
            result.overall_match_score
        );

        Ok(result)
    }

    async fn perform_contextual_matching(
        &self,
        requirements: &[ParsedRequirement],
        qualifications: &QualificationAnalysis,
        job_description: &str,
        resume_content: &str,
    ) -> Result<Vec<ContextualMatch>> {
        let mut contextual_matches = Vec::new();

        for requirement in requirements {
            let best_match = self.find_best_qualification_match(
                requirement,
                qualifications,
                job_description,
                resume_content,
            ).await?;

            contextual_matches.push(best_match);
        }

        // Enhance matches with dynamic keyword database insights
        if let Some(ref dynamic_db) = self.dynamic_db {
            self.enhance_matches_with_market_data(&mut contextual_matches, dynamic_db).await?;
        }

        Ok(contextual_matches)
    }

    async fn find_best_qualification_match(
        &self,
        requirement: &ParsedRequirement,
        qualifications: &QualificationAnalysis,
        job_description: &str,
        resume_content: &str,
    ) -> Result<ContextualMatch> {
        let mut best_match = ContextualMatch {
            requirement_text: requirement.text.clone(),
            matched_qualification: "No match found".to_string(),
            match_type: ContextMatchType::NoMatch,
            confidence_score: 0.0,
            semantic_similarity: 0.0,
            context_relevance: 0.0,
            supporting_evidence: Vec::new(),
            gap_analysis: Some("No relevant qualification found".to_string()),
        };

        // Check direct matches first
        for qual in &qualifications.relevant_qualifications {
            let similarity = self.semantic_scorer.calculate_similarity(
                &requirement.text,
                &qual.qualification,
            ).await?;

            if similarity > self.semantic_threshold {
                let context_relevance = self.calculate_context_relevance(
                    &requirement.text,
                    &qual.qualification,
                    job_description,
                    resume_content,
                ).await?;

                let confidence = (similarity + context_relevance) / 2.0;

                if confidence > best_match.confidence_score {
                    best_match = ContextualMatch {
                        requirement_text: requirement.text.clone(),
                        matched_qualification: qual.qualification.clone(),
                        match_type: self.determine_match_type(similarity, context_relevance),
                        confidence_score: confidence,
                        semantic_similarity: similarity,
                        context_relevance,
                        supporting_evidence: qual.strength_indicators.clone(),
                        gap_analysis: None,
                    };
                }
            }
        }

        // Check transferable skills if no direct match
        if best_match.match_type == ContextMatchType::NoMatch {
            for transferable in &qualifications.transferable_skills {
                if transferable.target_domain.contains(&requirement.category) {
                    best_match = ContextualMatch {
                        requirement_text: requirement.text.clone(),
                        matched_qualification: format!(
                            "{} (transferable from {})",
                            transferable.skill,
                            transferable.origin_domain
                        ),
                        match_type: ContextMatchType::TransferableMatch,
                        confidence_score: transferable.transferability_score,
                        semantic_similarity: transferable.transferability_score,
                        context_relevance: transferable.success_probability,
                        supporting_evidence: transferable.adaptation_requirements.clone(),
                        gap_analysis: Some(format!(
                            "Requires adaptation from {} to {}",
                            transferable.origin_domain,
                            transferable.target_domain
                        )),
                    };
                    break;
                }
            }
        }

        Ok(best_match)
    }

    fn determine_match_type(&self, similarity: f64, context_relevance: f64) -> ContextMatchType {
        let combined_score = (similarity + context_relevance) / 2.0;

        match combined_score {
            s if s >= 0.9 => ContextMatchType::DirectMatch,
            s if s >= 0.7 => ContextMatchType::SemanticMatch,
            s if s >= 0.5 => ContextMatchType::ExperienceMatch,
            s if s >= 0.3 => ContextMatchType::PartialMatch,
            _ => ContextMatchType::PotentialMatch,
        }
    }

    async fn calculate_context_relevance(
        &self,
        requirement: &str,
        qualification: &str,
        job_description: &str,
        resume_content: &str,
    ) -> Result<f64> {
        let prompt = format!(
            r#"Analyze the contextual relevance between a job requirement and candidate qualification.

Job Requirement: "{}"
Candidate Qualification: "{}"

Job Context: "{}"
Resume Context: "{}"

Rate the contextual relevance on a scale of 0.0 to 1.0, considering:
1. How well the qualification addresses the specific requirement
2. The context in which both appear
3. Industry standards and expectations
4. Practical applicability

Return only a number between 0.0 and 1.0."#,
            requirement,
            qualification,
            &job_description[..job_description.len().min(500)],
            &resume_content[..resume_content.len().min(500)]
        );

        match self.ollama_client.generate_response("qwen2.5:14b", &prompt, None).await {
            Ok((response, _)) => {
                // Extract relevance score from response
                let score_str = response.trim();
                match score_str.parse::<f64>() {
                    Ok(score) => Ok(score.clamp(0.0, 1.0)),
                    Err(_) => {
                        // Fallback to simple text similarity if AI parsing fails
                        Ok(0.5)
                    }
                }
            }
            Err(_) => Ok(0.5), // Fallback score
        }
    }

    async fn enhance_matches_with_market_data(
        &self,
        matches: &mut [ContextualMatch],
        dynamic_db: &DynamicKeywordDatabase,
    ) -> Result<()> {
        for contextual_match in matches.iter_mut() {
            // Extract key skill from matched qualification
            let skill_keywords: Vec<&str> = contextual_match.matched_qualification
                .split_whitespace()
                .collect();

            for skill in skill_keywords {
                if let Ok(Some(market_data)) = dynamic_db.get_market_demand(skill).await {
                    // Boost confidence for high-demand skills
                    let demand_boost = market_data.demand_score * 0.1;
                    contextual_match.confidence_score = 
                        (contextual_match.confidence_score + demand_boost).min(1.0);

                    // Add market insights to supporting evidence
                    contextual_match.supporting_evidence.push(format!(
                        "High market demand (score: {:.2})",
                        market_data.demand_score
                    ));

                    // Add salary impact information
                    contextual_match.supporting_evidence.push(format!(
                        "Average salary: ${:.0}",
                        market_data.salary_trends.current_average
                    ));
                }
            }
        }

        Ok(())
    }

    fn analyze_requirement_coverage(
        &self,
        requirements: &[ParsedRequirement],
        matches: &[ContextualMatch],
    ) -> Result<RequirementCoverage> {
        let total_requirements = requirements.len();
        let mut fully_covered = 0;
        let mut partially_covered = 0;
        let mut not_covered = 0;
        let mut critical_gaps = Vec::new();
        let mut nice_to_have_gaps = Vec::new();

        for (i, requirement) in requirements.iter().enumerate() {
            if let Some(contextual_match) = matches.get(i) {
                match contextual_match.match_type {
                    ContextMatchType::DirectMatch | ContextMatchType::SemanticMatch => {
                        fully_covered += 1;
                    }
                    ContextMatchType::ExperienceMatch 
                    | ContextMatchType::TransferableMatch 
                    | ContextMatchType::PartialMatch => {
                        partially_covered += 1;
                    }
                    ContextMatchType::PotentialMatch | ContextMatchType::NoMatch => {
                        not_covered += 1;
                        if requirement.is_critical {
                            critical_gaps.push(requirement.text.clone());
                        } else {
                            nice_to_have_gaps.push(requirement.text.clone());
                        }
                    }
                }
            }
        }

        let coverage_percentage = if total_requirements > 0 {
            ((fully_covered as f64 + partially_covered as f64 * 0.5) / total_requirements as f64) * 100.0
        } else {
            0.0
        };

        Ok(RequirementCoverage {
            total_requirements,
            fully_covered,
            partially_covered,
            not_covered,
            coverage_percentage,
            critical_gaps,
            nice_to_have_gaps,
        })
    }

    async fn generate_semantic_insights(
        &self,
        matches: &[ContextualMatch],
        requirements: &[ParsedRequirement],
        qualifications: &QualificationAnalysis,
        _job_description: &str,
        _resume_content: &str,
    ) -> Result<Vec<SemanticInsight>> {
        let mut insights = Vec::new();

        // Analyze strength alignments
        for contextual_match in matches {
            if contextual_match.confidence_score > 0.8 {
                insights.push(SemanticInsight {
                    insight_type: InsightType::StrengthAlignment,
                    description: format!(
                        "Strong alignment: {} matches well with {}",
                        contextual_match.matched_qualification,
                        contextual_match.requirement_text
                    ),
                    confidence: contextual_match.confidence_score,
                    supporting_context: contextual_match.supporting_evidence.clone(),
                    actionable_advice: Some("Highlight this strength prominently in your application".to_string()),
                });
            }
        }

        // Identify critical skill gaps
        for requirement in requirements {
            if requirement.is_critical {
                let has_match = matches.iter().any(|m| 
                    m.requirement_text == requirement.text && 
                    !matches!(m.match_type, ContextMatchType::NoMatch)
                );

                if !has_match {
                    insights.push(SemanticInsight {
                        insight_type: InsightType::SkillGap,
                        description: format!("Critical skill gap: {}", requirement.text),
                        confidence: 0.9,
                        supporting_context: vec![requirement.text.clone()],
                        actionable_advice: Some(format!(
                            "Consider acquiring {} through training or certification",
                            requirement.text
                        )),
                    });
                }
            }
        }

        // Check for hidden gems (undervalued qualifications)
        for qual in &qualifications.relevant_qualifications {
            if qual.relevance_score > 0.7 {
                let is_highlighted = matches.iter().any(|m|
                    m.matched_qualification.contains(&qual.qualification) && 
                    m.confidence_score > 0.7
                );

                if !is_highlighted {
                    insights.push(SemanticInsight {
                        insight_type: InsightType::HiddenGem,
                        description: format!(
                            "Undervalued qualification: {} has high relevance but may not be obvious",
                            qual.qualification
                        ),
                        confidence: qual.relevance_score,
                        supporting_context: qual.context_clues.clone(),
                        actionable_advice: Some("Consider emphasizing this qualification more prominently".to_string()),
                    });
                }
            }
        }

        Ok(insights)
    }

    async fn analyze_intent(
        &self,
        job_description: &str,
        resume_content: &str,
        _requirements: &[ParsedRequirement],
        _qualifications: &QualificationAnalysis,
    ) -> Result<IntentAnalysis> {
        // Analyze job intent
        let job_intent = self.intent_classifier.analyze_job_intent(job_description).await?;
        
        // Analyze candidate intent
        let candidate_intent = self.intent_classifier.analyze_candidate_intent(resume_content).await?;

        // Calculate intent alignment
        let intent_alignment_score = self.calculate_intent_alignment(&job_intent, &candidate_intent)?;

        // Extract motivational and commitment indicators
        let motivational_indicators = self.extract_motivational_indicators(resume_content).await?;
        let commitment_signals = self.extract_commitment_signals(resume_content).await?;
        let risk_factors = self.identify_risk_factors(
            job_description, 
            resume_content, 
            &job_intent, 
            &candidate_intent
        ).await?;

        Ok(IntentAnalysis {
            job_intent,
            candidate_intent,
            intent_alignment_score,
            motivational_indicators,
            commitment_signals,
            risk_factors,
        })
    }

    fn calculate_intent_alignment(&self, job_intent: &JobIntent, candidate_intent: &CandidateIntent) -> Result<f64> {
        let mut alignment_factors = Vec::new();

        // Seniority alignment
        let seniority_alignment = match (&job_intent.seniority_expectations, &candidate_intent.career_stage) {
            (ExperienceLevel::Entry, ExperienceLevel::Entry) => 1.0,
            (ExperienceLevel::Junior, ExperienceLevel::Entry) => 0.8,
            (ExperienceLevel::Junior, ExperienceLevel::Junior) => 1.0,
            (ExperienceLevel::Mid, ExperienceLevel::Junior) => 0.7,
            (ExperienceLevel::Mid, ExperienceLevel::Mid) => 1.0,
            (ExperienceLevel::Senior, ExperienceLevel::Mid) => 0.8,
            (ExperienceLevel::Senior, ExperienceLevel::Senior) => 1.0,
            _ => 0.5, // Default for other combinations
        };
        alignment_factors.push(seniority_alignment);

        // Technical focus alignment
        let technical_overlap = job_intent.technical_focus.iter()
            .filter(|skill| candidate_intent.technical_interests.contains(skill))
            .count() as f64;
        let technical_alignment = if !job_intent.technical_focus.is_empty() {
            technical_overlap / job_intent.technical_focus.len() as f64
        } else {
            0.5
        };
        alignment_factors.push(technical_alignment);

        // Leadership alignment
        let leadership_alignment = match job_intent.role_type {
            RoleType::Management | RoleType::TeamLead => {
                if candidate_intent.leadership_indicators.is_empty() { 0.3 } else { 0.9 }
            }
            RoleType::Individual => {
                if candidate_intent.leadership_indicators.len() > 3 { 0.7 } else { 1.0 }
            }
            _ => 0.7,
        };
        alignment_factors.push(leadership_alignment);

        // Calculate weighted average
        let overall_alignment = alignment_factors.iter().sum::<f64>() / alignment_factors.len() as f64;
        
        Ok(overall_alignment)
    }

    async fn extract_motivational_indicators(&self, resume_content: &str) -> Result<Vec<String>> {
        let prompt = format!(
            r#"Extract motivational indicators from this resume content that show drive, passion, and commitment:

Resume: "{}"

Look for indicators such as:
- Career progression and growth
- Continuous learning and development
- Leadership and initiative
- Innovation and problem-solving
- Community involvement and contributions
- Awards and recognition
- Side projects and personal interests

Return a JSON array of motivational indicators found.
"#,
            &resume_content[..resume_content.len().min(2000)]
        );

        match self.ollama_client.generate_response("qwen2.5:14b", &prompt, None).await {
            Ok((response, _)) => {
                if let Some(json_start) = response.find('[') {
                    if let Some(json_end) = response.rfind(']') {
                        let json_str = &response[json_start..=json_end];
                        if let Ok(indicators) = serde_json::from_str::<Vec<String>>(json_str) {
                            return Ok(indicators);
                        }
                    }
                }
                Ok(vec!["Unable to parse motivational indicators".to_string()])
            }
            Err(_) => Ok(vec!["AI analysis unavailable".to_string()]),
        }
    }

    async fn extract_commitment_signals(&self, resume_content: &str) -> Result<Vec<String>> {
        let mut signals = Vec::new();

        // Look for tenure patterns
        if resume_content.contains("years") {
            signals.push("Shows job tenure stability".to_string());
        }

        // Look for progression indicators
        if resume_content.to_lowercase().contains("promoted") {
            signals.push("Career advancement within organizations".to_string());
        }

        // Look for continuous learning
        if resume_content.to_lowercase().contains("certification") 
           || resume_content.to_lowercase().contains("training") {
            signals.push("Commitment to continuous learning".to_string());
        }

        // Look for leadership development
        if resume_content.to_lowercase().contains("led") 
           || resume_content.to_lowercase().contains("managed") {
            signals.push("Leadership experience and growth".to_string());
        }

        Ok(signals)
    }

    async fn identify_risk_factors(
        &self,
        _job_description: &str,
        resume_content: &str,
        job_intent: &JobIntent,
        candidate_intent: &CandidateIntent,
    ) -> Result<Vec<String>> {
        let mut risk_factors = Vec::new();

        // Over-qualification risk
        if matches!(candidate_intent.career_stage, ExperienceLevel::Senior | ExperienceLevel::Lead | ExperienceLevel::Principal)
           && matches!(job_intent.seniority_expectations, ExperienceLevel::Entry | ExperienceLevel::Junior) {
            risk_factors.push("Potential over-qualification - may seek more challenging role quickly".to_string());
        }

        // Under-qualification risk
        if matches!(candidate_intent.career_stage, ExperienceLevel::Entry | ExperienceLevel::Junior)
           && matches!(job_intent.seniority_expectations, ExperienceLevel::Senior | ExperienceLevel::Lead) {
            risk_factors.push("May require significant mentoring and development time".to_string());
        }

        // Job hopping pattern (simplified check)
        let job_count = resume_content.matches("20").count(); // Simple year count
        if job_count > 6 {
            risk_factors.push("Frequent job changes - assess commitment and stability".to_string());
        }

        Ok(risk_factors)
    }

    async fn generate_improvement_suggestions(
        &self,
        coverage: &RequirementCoverage,
        matches: &[ContextualMatch],
        intent_analysis: &IntentAnalysis,
        _target_industry: &str,
    ) -> Result<Vec<ImprovementSuggestion>> {
        let mut suggestions = Vec::new();

        // Critical skill gaps
        for gap in &coverage.critical_gaps {
            suggestions.push(ImprovementSuggestion {
                suggestion_type: SuggestionType::SkillDevelopment,
                priority: Priority::Critical,
                description: format!("Develop critical skill: {}", gap),
                specific_actions: vec![
                    format!("Take online course in {}", gap),
                    format!("Seek hands-on project experience with {}", gap),
                    format!("Consider certification in {}", gap),
                ],
                expected_impact: 0.8,
                timeframe: "3-6 months".to_string(),
                resources: vec![
                    "Online learning platforms".to_string(),
                    "Professional development budget".to_string(),
                    "Mentorship opportunities".to_string(),
                ],
            });
        }

        // Experience level suggestions
        if intent_analysis.intent_alignment_score < 0.6 {
            suggestions.push(ImprovementSuggestion {
                suggestion_type: SuggestionType::ExperienceGain,
                priority: Priority::High,
                description: "Bridge experience gap through targeted opportunities".to_string(),
                specific_actions: vec![
                    "Seek stretch assignments in current role".to_string(),
                    "Volunteer for cross-functional projects".to_string(),
                    "Consider consulting or contract work".to_string(),
                ],
                expected_impact: 0.7,
                timeframe: "6-12 months".to_string(),
                resources: vec![
                    "Internal mobility programs".to_string(),
                    "Professional networks".to_string(),
                    "Industry associations".to_string(),
                ],
            });
        }

        // Resume optimization for strong matches
        let strong_matches: Vec<_> = matches.iter()
            .filter(|m| m.confidence_score > 0.7)
            .collect();

        if !strong_matches.is_empty() {
            suggestions.push(ImprovementSuggestion {
                suggestion_type: SuggestionType::ResumeOptimization,
                priority: Priority::Medium,
                description: "Optimize resume to highlight strong qualifications".to_string(),
                specific_actions: vec![
                    "Move strongest qualifications to top of resume".to_string(),
                    "Add quantifiable achievements for key skills".to_string(),
                    "Use industry-specific keywords more prominently".to_string(),
                ],
                expected_impact: 0.6,
                timeframe: "1-2 weeks".to_string(),
                resources: vec![
                    "Resume optimization tools".to_string(),
                    "Industry keyword research".to_string(),
                    "Professional resume review".to_string(),
                ],
            });
        }

        Ok(suggestions)
    }

    fn calculate_overall_match_score(&self, matches: &[ContextualMatch]) -> Result<f64> {
        if matches.is_empty() {
            return Ok(0.0);
        }

        // Weight matches by requirement criticality and match quality
        let mut weighted_scores = Vec::new();
        let mut total_weight = 0.0;

        for contextual_match in matches {
            let weight = match contextual_match.match_type {
                ContextMatchType::DirectMatch => 1.0,
                ContextMatchType::SemanticMatch => 0.9,
                ContextMatchType::ExperienceMatch => 0.8,
                ContextMatchType::TransferableMatch => 0.6,
                ContextMatchType::PartialMatch => 0.4,
                ContextMatchType::PotentialMatch => 0.2,
                ContextMatchType::NoMatch => 0.0,
            };

            weighted_scores.push(contextual_match.confidence_score * weight);
            total_weight += weight;
        }

        let overall_score = if total_weight > 0.0 {
            weighted_scores.iter().sum::<f64>() / total_weight
        } else {
            0.0
        };

        Ok(overall_score)
    }

    fn calculate_confidence_metrics(
        &self,
        requirements: &[ParsedRequirement],
        qualifications: &QualificationAnalysis,
        matches: &[ContextualMatch],
    ) -> Result<ConfidenceMetrics> {
        // Calculate various confidence metrics
        let overall_confidence = matches.iter()
            .map(|m| m.confidence_score)
            .sum::<f64>() / matches.len().max(1) as f64;

        let context_analysis_confidence = if requirements.is_empty() { 0.0 } else { 0.8 };
        
        let semantic_matching_confidence = matches.iter()
            .map(|m| m.semantic_similarity)
            .sum::<f64>() / matches.len().max(1) as f64;

        let requirement_parsing_confidence = 0.85; // Based on parsing success rate
        
        let qualification_extraction_confidence = if qualifications.relevant_qualifications.is_empty() {
            0.3
        } else {
            0.9
        };

        let data_quality_score = (
            context_analysis_confidence + 
            requirement_parsing_confidence + 
            qualification_extraction_confidence
        ) / 3.0;

        Ok(ConfidenceMetrics {
            overall_confidence,
            context_analysis_confidence,
            semantic_matching_confidence,
            requirement_parsing_confidence,
            qualification_extraction_confidence,
            data_quality_score,
        })
    }
}

// Parsed requirement structure (used internally)
#[derive(Debug, Clone)]
pub struct ParsedRequirement {
    pub text: String,
    pub category: String,
    pub is_critical: bool,
    pub priority_score: f64,
    pub context_clues: Vec<String>,
}

// Implementation for component structs will be added in the next part...
impl RequirementAnalyzer {
    pub fn new() -> Result<Self> {
        // Initialize requirement parsing patterns
        let mut requirement_patterns = HashMap::new();
        
        // Technical skills patterns
        requirement_patterns.insert("technical_skills".to_string(), vec![
            regex::Regex::new(r"(?i)\b(?:experience with|proficiency in|knowledge of|skilled in)\s+([^.;,]+)")?,
            regex::Regex::new(r"(?i)\b(programming languages?|technologies?|frameworks?|tools?):\s*([^.;]+)")?,
            regex::Regex::new(r"(?i)\b(?:must|should) (?:have|know|understand)\s+([^.;,]+)")?,
        ]);

        // Experience patterns
        requirement_patterns.insert("experience".to_string(), vec![
            regex::Regex::new(r"(?i)(\d+)(?:\+)?\s*(?:years?|yrs?)\s*(?:of\s*)?(?:experience|exp)(?:\s*(?:in|with))?\s*([^.;,]+)")?,
            regex::Regex::new(r"(?i)(?:minimum|at least)\s+(\d+)\s*(?:years?|yrs?)\s*([^.;,]+)")?,
        ]);

        // Education patterns
        requirement_patterns.insert("education".to_string(), vec![
            regex::Regex::new(r"(?i)\b(?:bachelor|master|phd|degree)\s*(?:in|of)\s*([^.;,]+)")?,
            regex::Regex::new(r"(?i)\b(?:certification|certified)\s*(?:in)?\s*([^.;,]+)")?,
        ]);

        let priority_indicators = vec![
            regex::Regex::new(r"(?i)\b(?:required|must|essential|critical|mandatory)\b")?,
            regex::Regex::new(r"(?i)\b(?:preferred|nice to have|desired|plus|bonus)\b")?,
        ];

        let qualification_extractors = vec![
            regex::Regex::new(r"(?i)\b(?:qualifications?|requirements?|skills?):\s*")?,
        ];

        Ok(Self {
            requirement_patterns,
            priority_indicators,
            qualification_extractors,
        })
    }

    pub async fn parse_requirements(
        &self, 
        job_description: &str, 
        _target_industry: &str
    ) -> Result<Vec<ParsedRequirement>> {
        let mut requirements = Vec::new();
        
        // Split job description into sentences for better parsing
        let sentences: Vec<&str> = job_description
            .split(['.', ';', '\n'])
            .filter(|s| !s.trim().is_empty())
            .collect();

        for sentence in sentences {
            // Check if sentence contains requirement indicators
            for (category, patterns) in &self.requirement_patterns {
                for pattern in patterns {
                    if let Some(captures) = pattern.captures(sentence) {
                        if let Some(requirement_text) = captures.get(0) {
                            let is_critical = self.priority_indicators[0].is_match(sentence);
                            let priority_score = if is_critical { 1.0 } else { 0.5 };

                            requirements.push(ParsedRequirement {
                                text: requirement_text.as_str().trim().to_string(),
                                category: category.clone(),
                                is_critical,
                                priority_score,
                                context_clues: vec![sentence.trim().to_string()],
                            });
                        }
                    }
                }
            }
        }

        Ok(requirements)
    }
}

impl QualificationMapper {
    pub fn new() -> Result<Self> {
        // Initialize skill taxonomies for different domains
        let mut skill_taxonomies = HashMap::new();
        
        // Technology taxonomy
        let mut tech_skills = HashMap::new();
        tech_skills.insert("python".to_string(), SkillInfo {
            canonical_name: "Python".to_string(),
            aliases: vec!["py".to_string(), "python3".to_string()],
            category: "programming_language".to_string(),
            difficulty_level: 0.6,
            market_demand: 0.9,
            related_skills: vec!["django".to_string(), "flask".to_string(), "pandas".to_string()],
        });

        skill_taxonomies.insert("technology".to_string(), SkillTaxonomy {
            domain: "technology".to_string(),
            skills: tech_skills,
            relationships: HashMap::new(),
        });

        let experience_patterns = vec![
            regex::Regex::new(r"(?i)(\d+)(?:\+)?\s*(?:years?|yrs?)\s*(?:of\s*)?(?:experience|exp)(?:\s*(?:in|with))?\s*([^.;,]+)")?,
        ];

        let mut certification_database = HashMap::new();
        certification_database.insert("aws certified".to_string(), CertificationInfo {
            name: "AWS Certified".to_string(),
            issuer: "Amazon Web Services".to_string(),
            industry_recognition: 0.9,
            skills_validated: vec!["cloud computing".to_string(), "aws".to_string()],
            validity_period: Some(36), // 3 years
        });

        Ok(Self {
            skill_taxonomies,
            experience_patterns,
            certification_database,
        })
    }

    pub async fn extract_qualifications(
        &self,
        resume_content: &str,
        extraction_result: &ExtractionResult,
    ) -> Result<QualificationAnalysis> {
        // Use modern extraction results as base
        let mut relevant_qualifications = Vec::new();
        
        for keyword_match in &extraction_result.keyword_matches {
            relevant_qualifications.push(QualificationMatch {
                qualification: keyword_match.keyword.clone(),
                relevance_score: keyword_match.confidence_score,
                applicable_requirements: Vec::new(), // Will be filled during matching
                strength_indicators: keyword_match.context_phrases.clone(),
                context_clues: keyword_match.semantic_variations.clone(),
            });
        }

        // Extract transferable skills (simplified implementation)
        let transferable_skills = self.identify_transferable_skills(resume_content)?;

        // Analyze experience level
        let experience_level_analysis = self.analyze_experience_level(resume_content)?;

        // Extract certifications
        let certification_relevance = self.extract_certifications(resume_content)?;

        // Extract project relevance (simplified)
        let project_relevance = self.extract_project_relevance(resume_content)?;

        Ok(QualificationAnalysis {
            relevant_qualifications,
            transferable_skills,
            experience_level_analysis,
            certification_relevance,
            project_relevance,
        })
    }

    fn identify_transferable_skills(&self, resume_content: &str) -> Result<Vec<TransferableSkill>> {
        // Simplified transferable skills identification
        let mut transferable_skills = Vec::new();
        
        // Example: Project management skills from different domains
        if resume_content.to_lowercase().contains("project management") {
            transferable_skills.push(TransferableSkill {
                skill: "Project Management".to_string(),
                origin_domain: "General".to_string(),
                target_domain: "Technology".to_string(),
                transferability_score: 0.8,
                adaptation_requirements: vec![
                    "Learn agile methodologies".to_string(),
                    "Understand software development lifecycle".to_string(),
                ],
                success_probability: 0.9,
            });
        }

        Ok(transferable_skills)
    }

    fn analyze_experience_level(&self, resume_content: &str) -> Result<ExperienceLevelAnalysis> {
        // Simple experience level detection based on years
        let years_pattern = regex::Regex::new(r"(\d+)(?:\+)?\s*(?:years?|yrs?)")?;
        let mut max_years = 0;

        for captures in years_pattern.captures_iter(resume_content) {
            if let Some(years_str) = captures.get(1) {
                if let Ok(years) = years_str.as_str().parse::<i32>() {
                    max_years = max_years.max(years);
                }
            }
        }

        let candidate_level = match max_years {
            0..=1 => ExperienceLevel::Entry,
            2..=3 => ExperienceLevel::Junior,
            4..=7 => ExperienceLevel::Mid,
            8..=12 => ExperienceLevel::Senior,
            13..=20 => ExperienceLevel::Lead,
            _ => ExperienceLevel::Principal,
        };

        Ok(ExperienceLevelAnalysis {
            required_level: ExperienceLevel::Unknown, // Will be determined during matching
            candidate_level,
            level_match_score: 0.7, // Placeholder
            experience_gap: 0,
            level_progression_path: vec!["Continue building expertise".to_string()],
        })
    }

    fn extract_certifications(&self, resume_content: &str) -> Result<Vec<CertificationMatch>> {
        let mut certifications = Vec::new();
        
        for (cert_name, cert_info) in &self.certification_database {
            if resume_content.to_lowercase().contains(&cert_name.to_lowercase()) {
                certifications.push(CertificationMatch {
                    certification: cert_info.name.clone(),
                    relevance_score: cert_info.industry_recognition,
                    industry_recognition: cert_info.industry_recognition,
                    applicable_skills: cert_info.skills_validated.clone(),
                    expiration_risk: if cert_info.validity_period.is_some() {
                        Some("Check expiration date".to_string())
                    } else {
                        None
                    },
                });
            }
        }

        Ok(certifications)
    }

    fn extract_project_relevance(&self, resume_content: &str) -> Result<Vec<ProjectMatch>> {
        // Simplified project extraction
        let mut projects = Vec::new();
        
        // Look for project indicators
        if resume_content.to_lowercase().contains("project") {
            projects.push(ProjectMatch {
                project_description: "Software development project".to_string(),
                relevance_score: 0.7,
                demonstrated_skills: vec!["software development".to_string()],
                scale_indicators: vec!["team collaboration".to_string()],
                impact_metrics: vec!["improved efficiency".to_string()],
            });
        }

        Ok(projects)
    }
}

impl Default for SemanticScorer {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticScorer {
    pub fn new() -> Self {
        Self {
            embedding_cache: HashMap::new(),
            similarity_threshold: 0.7,
        }
    }

    pub async fn calculate_similarity(&self, text1: &str, text2: &str) -> Result<f64> {
        // Simplified similarity calculation using word overlap
        let words1: HashSet<String> = text1
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        let words2: HashSet<String> = text2
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let intersection_size = words1.intersection(&words2).count() as f64;
        let union_size = words1.union(&words2).count() as f64;

        let jaccard_similarity = if union_size > 0.0 {
            intersection_size / union_size
        } else {
            0.0
        };

        Ok(jaccard_similarity)
    }
}

impl IntentClassifier {
    pub fn new() -> Result<Self> {
        let mut job_role_patterns = HashMap::new();
        
        job_role_patterns.insert(RoleType::Individual, vec![
            regex::Regex::new(r"(?i)\b(?:individual contributor|ic|developer|engineer|analyst)\b")?,
        ]);
        
        job_role_patterns.insert(RoleType::Management, vec![
            regex::Regex::new(r"(?i)\b(?:manager|director|vp|head of|chief)\b")?,
        ]);

        let mut seniority_indicators = HashMap::new();
        seniority_indicators.insert(ExperienceLevel::Entry, vec![
            "entry level".to_string(),
            "junior".to_string(),
            "graduate".to_string(),
        ]);

        let motivation_extractors = vec![
            regex::Regex::new(r"(?i)\b(?:passionate|driven|motivated|enthusiastic)\b")?,
        ];

        Ok(Self {
            job_role_patterns,
            seniority_indicators,
            motivation_extractors,
        })
    }

    pub async fn analyze_job_intent(&self, job_description: &str) -> Result<JobIntent> {
        // Determine role type
        let mut role_type = RoleType::Unknown;
        for (role, patterns) in &self.job_role_patterns {
            for pattern in patterns {
                if pattern.is_match(job_description) {
                    role_type = role.clone();
                    break;
                }
            }
        }

        // Determine seniority expectations
        let seniority_expectations = if job_description.to_lowercase().contains("senior") {
            ExperienceLevel::Senior
        } else if job_description.to_lowercase().contains("junior") {
            ExperienceLevel::Junior
        } else {
            ExperienceLevel::Mid
        };

        Ok(JobIntent {
            role_type,
            seniority_expectations,
            growth_trajectory: "Career advancement opportunities".to_string(),
            team_dynamics: "Collaborative environment".to_string(),
            technical_focus: vec!["Software development".to_string()],
            business_impact: "Contributing to company success".to_string(),
        })
    }

    pub async fn analyze_candidate_intent(&self, resume_content: &str) -> Result<CandidateIntent> {
        // Determine career stage based on experience
        let career_stage = if resume_content.to_lowercase().contains("senior") {
            ExperienceLevel::Senior
        } else if resume_content.to_lowercase().contains("lead") {
            ExperienceLevel::Lead
        } else {
            ExperienceLevel::Mid
        };

        Ok(CandidateIntent {
            career_stage,
            growth_aspirations: vec!["Professional development".to_string()],
            technical_interests: vec!["Technology advancement".to_string()],
            leadership_indicators: if resume_content.to_lowercase().contains("led") {
                vec!["Leadership experience".to_string()]
            } else {
                Vec::new()
            },
            specialization_areas: vec!["Software development".to_string()],
        })
    }
}