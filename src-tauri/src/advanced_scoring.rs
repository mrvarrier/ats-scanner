use anyhow::{anyhow, Result};
use log::{debug, info};
use regex::Regex;
use rust_stemmers::{Algorithm, Stemmer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

use crate::database::Database;
use crate::models::AnalysisResult;

/// Advanced scoring engine for Jobscan-level accuracy
#[allow(dead_code)]
pub struct AdvancedScoringEngine {
    keyword_analyzer: KeywordAnalyzer,
    ats_simulator: ATSSimulator,
    industry_weights: Arc<Mutex<IndustryWeights>>,
    format_analyzer: FormatAnalyzer,
    #[allow(dead_code)]
    db: Arc<Mutex<Database>>,
}

/// Multi-layered keyword analysis system
#[derive(Debug)]
pub struct KeywordAnalyzer {
    exact_matcher: ExactMatcher,
    stemmed_matcher: StemmedMatcher,
    contextual_matcher: ContextualMatcher,
    synonym_matcher: SynonymMatcher,
}

/// ATS system simulation for parsing behavior
pub struct ATSSimulator {
    parsers: Vec<Box<dyn ATSParser + Send + Sync>>,
    #[allow(dead_code)]
    format_rules: Vec<FormatRule>,
    #[allow(dead_code)]
    section_detectors: Vec<SectionDetector>,
}

/// Industry-specific scoring weights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryWeights {
    pub tech: ScoringWeights,
    pub finance: ScoringWeights,
    pub healthcare: ScoringWeights,
    pub marketing: ScoringWeights,
    pub general: ScoringWeights,
}

/// Scoring weights for different components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringWeights {
    pub keyword_match: f64,        // 40% weight
    pub format_compatibility: f64, // 20% weight
    pub section_completeness: f64, // 15% weight
    pub achievement_quality: f64,  // 15% weight
    pub industry_alignment: f64,   // 10% weight
}

/// Comprehensive keyword match analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordMatch {
    pub exact_matches: Vec<MatchResult>,
    pub stemmed_matches: Vec<MatchResult>,
    pub contextual_matches: Vec<MatchResult>,
    pub synonym_matches: Vec<MatchResult>,
    pub overall_score: f64,
    pub match_density: f64,
    pub section_distribution: HashMap<String, f64>,
}

/// Individual match result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub keyword: String,
    pub matched_text: String,
    pub section: String,
    pub position: usize,
    pub context: String,
    pub confidence: f64,
    pub weight: f64,
}

/// Format analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatAnalysis {
    pub ats_compatibility_score: f64,
    pub parsing_issues: Vec<FormatIssue>,
    pub section_detection_score: f64,
    pub font_compatibility: f64,
    pub layout_score: f64,
    pub encoding_issues: Vec<String>,
}

/// Format issue detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatIssue {
    pub issue_type: FormatIssueType,
    pub severity: IssueSeverity,
    pub location: String,
    pub description: String,
    pub fix_suggestion: String,
    pub ats_impact: f64,
}

/// Types of format issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormatIssueType {
    FontIncompatibility,
    LayoutProblem,
    EncodingIssue,
    ParsingError,
    SectionDetectionFail,
    TableFormatting,
    ImageText,
    SpecialCharacters,
}

/// Severity levels for issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Critical,
    High,
    Medium,
    Low,
}

/// ATS system types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ATSSystem {
    Workday,
    Taleo,
    Greenhouse,
    Lever,
    SmartRecruiters,
    BambooHR,
    Icims,
    Generic,
}

/// Format rules for ATS compatibility
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FormatRule {
    pub rule_type: String,
    pub severity: IssueSeverity,
    pub validator: fn(&str) -> bool,
    pub description: String,
}

/// Resume section detector
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SectionDetector {
    pub section_name: String,
    pub patterns: Vec<Regex>,
    pub importance: f64,
}

/// ATS parser trait
pub trait ATSParser {
    fn parse_resume(&self, content: &str) -> Result<ParsedResume>;
    fn get_system_type(&self) -> ATSSystem;
    fn get_compatibility_score(&self, resume: &ParsedResume) -> f64;
}

/// Parsed resume structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedResume {
    pub sections: HashMap<String, String>,
    pub contact_info: ContactInfo,
    pub experience: Vec<ExperienceEntry>,
    pub education: Vec<EducationEntry>,
    pub skills: Vec<String>,
    pub parsing_confidence: f64,
}

/// Contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub location: Option<String>,
}

/// Experience entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceEntry {
    pub title: String,
    pub company: String,
    pub duration: String,
    pub description: String,
    pub achievements: Vec<String>,
}

/// Education entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducationEntry {
    pub degree: String,
    pub institution: String,
    pub year: Option<String>,
    pub gpa: Option<f64>,
}

/// Experience pattern for industry matching
#[derive(Debug, Clone)]
pub struct ExperiencePattern {
    pub industry_keywords: Vec<String>,
}

/// Industry benchmark data
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IndustryBenchmark {
    pub average_score: f64,
    pub median_score: f64,
    pub top_10_percent_score: f64,
    pub bottom_10_percent_score: f64,
    pub score_distribution: Vec<(f64, f64)>, // (score_threshold, percentile)
    pub keyword_match_average: f64,
    pub format_score_average: f64,
    pub sections_average: f64,
}

impl Default for IndustryBenchmark {
    fn default() -> Self {
        Self {
            average_score: 70.0,
            median_score: 68.0,
            top_10_percent_score: 85.0,
            bottom_10_percent_score: 45.0,
            score_distribution: vec![
                (50.0, 10.0),
                (60.0, 25.0),
                (70.0, 50.0),
                (80.0, 75.0),
                (90.0, 90.0),
                (95.0, 95.0),
                (100.0, 100.0),
            ],
            keyword_match_average: 62.0,
            format_score_average: 78.0,
            sections_average: 5.0,
        }
    }
}

/// Experience level benchmark data
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExperienceLevelBenchmark {
    pub average_score: f64,
    pub median_score: f64,
    pub top_10_percent_score: f64,
    pub bottom_10_percent_score: f64,
    pub score_distribution: Vec<(f64, f64)>,
    pub expected_sections: f64,
    pub expected_keyword_density: f64,
}

impl Default for ExperienceLevelBenchmark {
    fn default() -> Self {
        Self {
            average_score: 70.0,
            median_score: 68.0,
            top_10_percent_score: 85.0,
            bottom_10_percent_score: 45.0,
            score_distribution: vec![
                (50.0, 10.0),
                (60.0, 25.0),
                (70.0, 50.0),
                (80.0, 75.0),
                (90.0, 90.0),
                (95.0, 95.0),
                (100.0, 100.0),
            ],
            expected_sections: 5.0,
            expected_keyword_density: 0.2,
        }
    }
}

/// Enhanced analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedAnalysisResult {
    pub base_analysis: AnalysisResult,
    pub keyword_analysis: KeywordMatch,
    pub format_analysis: FormatAnalysis,
    pub ats_compatibility: HashMap<ATSSystem, f64>,
    pub industry_alignment: f64,
    pub benchmark_comparison: BenchmarkComparison,
    pub improvement_suggestions: Vec<OptimizationSuggestion>,
}

/// Benchmark comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkComparison {
    pub industry_percentile: f64,
    pub experience_level_percentile: f64,
    pub overall_percentile: f64,
    pub top_performers_gap: f64,
}

/// Optimization suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    pub category: String,
    pub title: String,
    pub description: String,
    pub impact_score: f64,
    pub difficulty: String,
    pub specific_actions: Vec<SuggestionAction>,
    pub before_example: String,
    pub after_example: String,
}

/// Difficulty levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Easy,
    Medium,
    Hard,
    Expert,
}

/// Actionable change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionableChange {
    pub section: String,
    pub current_text: String,
    pub suggested_text: String,
    pub reasoning: String,
    pub expected_improvement: f64,
}

/// Suggestion action for detailed recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionAction {
    pub action: String,
    pub section: String,
    pub reasoning: String,
}

// Matcher implementations
#[derive(Debug)]
pub struct ExactMatcher;

#[derive(Debug)]
pub struct StemmedMatcher;

#[derive(Debug)]
pub struct ContextualMatcher;

#[derive(Debug)]
pub struct SynonymMatcher;

#[derive(Debug)]
pub struct FormatAnalyzer;

impl AdvancedScoringEngine {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        let keyword_analyzer = KeywordAnalyzer::new();
        let ats_simulator = ATSSimulator::new();
        let industry_weights = Arc::new(Mutex::new(IndustryWeights::default()));
        let format_analyzer = FormatAnalyzer::new();

        Self {
            keyword_analyzer,
            ats_simulator,
            industry_weights,
            format_analyzer,
            db,
        }
    }

    /// Perform comprehensive analysis with enhanced scoring
    pub async fn analyze_comprehensive(
        &self,
        resume_content: &str,
        job_description: &str,
        industry: &str,
        experience_level: &str,
    ) -> Result<EnhancedAnalysisResult> {
        info!("Starting comprehensive analysis for {} industry", industry);

        // Parse resume with ATS simulation
        let parsed_resume = self
            .ats_simulator
            .parse_with_multiple_systems(resume_content)?;

        // Perform keyword analysis
        let keyword_analysis = self
            .keyword_analyzer
            .analyze_comprehensive(resume_content, job_description, industry)
            .await?;

        // Analyze format compatibility
        let format_analysis = self
            .format_analyzer
            .analyze_comprehensive(resume_content, &parsed_resume)?;

        // Get industry-specific weights
        let weights = self.get_industry_weights(industry).await?;

        // Calculate ATS compatibility scores
        let ats_compatibility = self
            .ats_simulator
            .calculate_compatibility_scores(&parsed_resume)?;

        // Calculate industry alignment
        let industry_alignment = self
            .calculate_industry_alignment(&parsed_resume, industry, experience_level)
            .await?;

        // Get benchmark comparison
        let benchmark_comparison = self
            .get_benchmark_comparison(
                &keyword_analysis,
                &format_analysis,
                industry,
                experience_level,
            )
            .await?;

        // Generate optimization suggestions
        let improvement_suggestions = self
            .generate_optimization_suggestions(
                &parsed_resume,
                &keyword_analysis,
                &format_analysis,
                job_description,
                industry,
            )
            .await?;

        // Calculate overall enhanced score
        let overall_score = self.calculate_weighted_score(
            &keyword_analysis,
            &format_analysis,
            industry_alignment,
            &weights,
        )?;

        // Create base analysis result for compatibility
        let base_analysis = AnalysisResult {
            overall_score,
            category_scores: self.create_category_scores(
                &keyword_analysis,
                &format_analysis,
                industry_alignment,
            ),
            detailed_feedback: self.generate_detailed_feedback(
                &keyword_analysis,
                &format_analysis,
                &improvement_suggestions,
            ),
            missing_keywords: self.extract_missing_keywords(&keyword_analysis),
            recommendations: self.extract_recommendations(&improvement_suggestions),
            processing_time_ms: 0, // Will be set by caller
        };

        Ok(EnhancedAnalysisResult {
            base_analysis,
            keyword_analysis,
            format_analysis,
            ats_compatibility,
            industry_alignment,
            benchmark_comparison,
            improvement_suggestions,
        })
    }

    async fn get_industry_weights(&self, industry: &str) -> Result<ScoringWeights> {
        let weights = self.industry_weights.lock().await;
        let industry_weights = match industry.to_lowercase().as_str() {
            "technology" | "tech" | "software" => &weights.tech,
            "finance" | "financial" | "banking" => &weights.finance,
            "healthcare" | "medical" | "pharma" => &weights.healthcare,
            "marketing" | "advertising" | "digital" => &weights.marketing,
            _ => &weights.general,
        };
        Ok(industry_weights.clone())
    }

    fn calculate_weighted_score(
        &self,
        keyword_analysis: &KeywordMatch,
        format_analysis: &FormatAnalysis,
        industry_alignment: f64,
        weights: &ScoringWeights,
    ) -> Result<f64> {
        let keyword_score = keyword_analysis.overall_score * weights.keyword_match;
        let format_score = format_analysis.ats_compatibility_score * weights.format_compatibility;
        let section_score = format_analysis.section_detection_score * weights.section_completeness;
        let achievement_score =
            self.calculate_achievement_score(keyword_analysis) * weights.achievement_quality;
        let industry_score = industry_alignment * weights.industry_alignment;

        let total_score =
            keyword_score + format_score + section_score + achievement_score + industry_score;
        Ok(total_score.clamp(0.0, 100.0))
    }

    fn calculate_achievement_score(&self, keyword_analysis: &KeywordMatch) -> f64 {
        // Calculate achievement quality based on contextual matches and positioning
        let achievement_matches = keyword_analysis
            .contextual_matches
            .iter()
            .filter(|m| m.section.contains("experience") || m.section.contains("work"))
            .count();

        let total_matches = keyword_analysis.exact_matches.len()
            + keyword_analysis.stemmed_matches.len()
            + keyword_analysis.contextual_matches.len();

        if total_matches == 0 {
            return 0.0;
        }

        ((achievement_matches as f64 / total_matches as f64) * 100.0).min(100.0)
    }

    fn create_category_scores(
        &self,
        keyword_analysis: &KeywordMatch,
        format_analysis: &FormatAnalysis,
        _industry_alignment: f64,
    ) -> crate::models::CategoryScores {
        crate::models::CategoryScores {
            skills: keyword_analysis.overall_score,
            experience: self.calculate_achievement_score(keyword_analysis),
            education: self.calculate_education_score(keyword_analysis),
            keywords: keyword_analysis.overall_score,
            format: format_analysis.ats_compatibility_score,
        }
    }

    fn calculate_education_score(&self, keyword_analysis: &KeywordMatch) -> f64 {
        // Calculate education relevance based on education section matches
        let education_matches = keyword_analysis
            .exact_matches
            .iter()
            .filter(|m| m.section.contains("education") || m.section.contains("degree"))
            .count();

        if education_matches == 0 {
            return 50.0; // Neutral score if no education matches
        }

        ((education_matches as f64 / 5.0) * 100.0).min(100.0) // Assume 5 max relevant education keywords
    }

    fn generate_detailed_feedback(
        &self,
        keyword_analysis: &KeywordMatch,
        format_analysis: &FormatAnalysis,
        suggestions: &[OptimizationSuggestion],
    ) -> String {
        let mut feedback = String::new();

        feedback.push_str(&format!(
            "Keyword Analysis: Your resume matches {:.1}% of relevant keywords. ",
            keyword_analysis.overall_score
        ));

        if keyword_analysis.overall_score < 70.0 {
            feedback.push_str("Consider incorporating more industry-specific keywords to improve ATS compatibility. ");
        }

        feedback.push_str(&format!(
            "Format Compatibility: Your resume scores {:.1}% for ATS readability. ",
            format_analysis.ats_compatibility_score
        ));

        if format_analysis.ats_compatibility_score < 80.0 {
            feedback.push_str("Some formatting issues may affect ATS parsing. ");
        }

        if !suggestions.is_empty() {
            feedback.push_str(&format!(
                "We've identified {} key areas for improvement that could boost your score significantly.",
                suggestions.len()
            ));
        }

        feedback
    }

    fn extract_missing_keywords(&self, _keyword_analysis: &KeywordMatch) -> Vec<String> {
        // Extract keywords that had no matches
        // This would be populated based on job description analysis
        // For now, return empty vec as this requires job description parsing
        Vec::new()
    }

    fn extract_recommendations(&self, suggestions: &[OptimizationSuggestion]) -> Vec<String> {
        suggestions
            .iter()
            .take(5) // Top 5 recommendations
            .map(|s| {
                format!(
                    "{}: {}",
                    s.category,
                    s.specific_actions
                        .first()
                        .map(|a| a.reasoning.as_str())
                        .unwrap_or("Improve this section")
                )
            })
            .collect()
    }

    async fn calculate_industry_alignment(
        &self,
        parsed_resume: &ParsedResume,
        industry: &str,
        experience_level: &str,
    ) -> Result<f64> {
        // Build comprehensive industry keyword database
        let industry_db = self.build_industry_keyword_database();

        // Get industry-specific keywords and weights
        let empty_map = HashMap::new();
        let industry_keywords = industry_db.get(industry).unwrap_or(&empty_map);

        // Calculate alignment score based on multiple factors
        let keyword_alignment =
            self.calculate_keyword_alignment(parsed_resume, industry_keywords)?;
        let skill_alignment = self.calculate_skill_alignment(parsed_resume, industry)?;
        let experience_alignment =
            self.calculate_experience_alignment(parsed_resume, industry, experience_level)?;
        let education_alignment = self.calculate_education_alignment(parsed_resume, industry)?;

        // Weighted combination of alignment factors
        let total_alignment = keyword_alignment * 0.4
            + skill_alignment * 0.3
            + experience_alignment * 0.2
            + education_alignment * 0.1;

        Ok(total_alignment.clamp(0.0, 100.0))
    }

    /// Build comprehensive industry keyword database with weights
    fn build_industry_keyword_database(&self) -> HashMap<String, HashMap<String, f64>> {
        let mut db = HashMap::new();

        // Technology Industry Keywords
        let mut tech_keywords = HashMap::new();

        // Programming Languages (High weight)
        let programming_languages = [
            ("python", 3.0),
            ("java", 3.0),
            ("javascript", 3.0),
            ("typescript", 2.8),
            ("c++", 2.8),
            ("c#", 2.8),
            ("go", 2.5),
            ("rust", 2.5),
            ("swift", 2.5),
            ("kotlin", 2.3),
            ("scala", 2.3),
            ("ruby", 2.3),
            ("php", 2.0),
            ("perl", 1.8),
            ("r", 2.5),
            ("matlab", 2.3),
            ("sql", 2.8),
            ("html", 2.0),
            ("css", 2.0),
        ];
        for (keyword, weight) in &programming_languages {
            tech_keywords.insert(keyword.to_string(), *weight);
        }

        // Frameworks & Libraries (High weight)
        let frameworks = [
            ("react", 2.8),
            ("angular", 2.8),
            ("vue", 2.5),
            ("node.js", 2.8),
            ("express", 2.3),
            ("django", 2.5),
            ("flask", 2.3),
            ("spring", 2.8),
            ("hibernate", 2.3),
            ("tensorflow", 3.0),
            ("pytorch", 3.0),
            ("scikit-learn", 2.8),
            ("pandas", 2.5),
            ("numpy", 2.3),
            ("matplotlib", 2.0),
            ("bootstrap", 2.0),
            ("jquery", 1.8),
            ("d3.js", 2.3),
            ("three.js", 2.3),
            ("webpack", 2.3),
            ("babel", 2.0),
            ("redux", 2.5),
        ];
        for (keyword, weight) in &frameworks {
            tech_keywords.insert(keyword.to_string(), *weight);
        }

        // Cloud & DevOps (Very High weight)
        let cloud_devops = [
            ("aws", 3.0),
            ("azure", 3.0),
            ("gcp", 2.8),
            ("google cloud", 2.8),
            ("docker", 2.8),
            ("kubernetes", 3.0),
            ("jenkins", 2.5),
            ("ci/cd", 2.8),
            ("devops", 2.8),
            ("terraform", 2.8),
            ("ansible", 2.5),
            ("puppet", 2.3),
            ("chef", 2.3),
            ("microservices", 2.8),
            ("serverless", 2.5),
            ("lambda", 2.5),
        ];
        for (keyword, weight) in &cloud_devops {
            tech_keywords.insert(keyword.to_string(), *weight);
        }

        // Databases (High weight)
        let databases = [
            ("mysql", 2.5),
            ("postgresql", 2.8),
            ("mongodb", 2.5),
            ("redis", 2.3),
            ("elasticsearch", 2.5),
            ("cassandra", 2.3),
            ("dynamodb", 2.5),
            ("sqlite", 2.0),
            ("oracle", 2.3),
            ("sql server", 2.3),
            ("nosql", 2.3),
            ("database design", 2.5),
        ];
        for (keyword, weight) in &databases {
            tech_keywords.insert(keyword.to_string(), *weight);
        }

        // AI/ML (Very High weight)
        let ai_ml = [
            ("machine learning", 3.0),
            ("artificial intelligence", 3.0),
            ("deep learning", 3.0),
            ("neural networks", 2.8),
            ("data science", 2.8),
            ("nlp", 2.8),
            ("computer vision", 2.8),
            ("reinforcement learning", 2.8),
            ("mlops", 2.8),
            ("data mining", 2.5),
            ("statistics", 2.5),
        ];
        for (keyword, weight) in &ai_ml {
            tech_keywords.insert(keyword.to_string(), *weight);
        }

        // Tech Methodologies (Medium weight)
        let methodologies = [
            ("agile", 2.3),
            ("scrum", 2.3),
            ("kanban", 2.0),
            ("tdd", 2.5),
            ("bdd", 2.3),
            ("clean code", 2.3),
            ("solid principles", 2.5),
            ("design patterns", 2.5),
            ("api design", 2.5),
            ("rest", 2.3),
            ("graphql", 2.5),
            ("microservices architecture", 2.8),
        ];
        for (keyword, weight) in &methodologies {
            tech_keywords.insert(keyword.to_string(), *weight);
        }

        db.insert("technology".to_string(), tech_keywords);

        // Finance Industry Keywords
        let mut finance_keywords = HashMap::new();

        // Financial Analysis (Very High weight)
        let financial_analysis = [
            ("financial modeling", 3.0),
            ("valuation", 3.0),
            ("dcf", 2.8),
            ("financial analysis", 3.0),
            ("risk management", 3.0),
            ("portfolio management", 2.8),
            ("investment analysis", 2.8),
            ("equity research", 2.8),
            ("fixed income", 2.5),
            ("derivatives", 2.8),
            ("options trading", 2.5),
            ("algorithmic trading", 2.8),
            ("quantitative analysis", 2.8),
        ];
        for (keyword, weight) in &financial_analysis {
            finance_keywords.insert(keyword.to_string(), *weight);
        }

        // Financial Software (High weight)
        let financial_software = [
            ("bloomberg", 2.8),
            ("excel", 2.5),
            ("vba", 2.3),
            ("matlab", 2.5),
            ("r", 2.5),
            ("python", 2.5),
            ("sql", 2.3),
            ("tableau", 2.3),
            ("power bi", 2.3),
            ("factset", 2.5),
            ("refinitiv", 2.3),
            ("quickbooks", 2.0),
            ("sap", 2.3),
        ];
        for (keyword, weight) in &financial_software {
            finance_keywords.insert(keyword.to_string(), *weight);
        }

        // Banking & Trading (High weight)
        let banking_trading = [
            ("investment banking", 2.8),
            ("commercial banking", 2.5),
            ("retail banking", 2.3),
            ("trading", 2.8),
            ("market making", 2.8),
            ("sales trading", 2.5),
            ("prime brokerage", 2.5),
            ("custody", 2.3),
            ("clearing", 2.3),
            ("settlement", 2.3),
            ("regulatory reporting", 2.5),
            ("compliance", 2.5),
        ];
        for (keyword, weight) in &banking_trading {
            finance_keywords.insert(keyword.to_string(), *weight);
        }

        // Fintech (Very High weight)
        let fintech = [
            ("fintech", 2.8),
            ("blockchain", 3.0),
            ("cryptocurrency", 2.8),
            ("defi", 2.8),
            ("payments", 2.5),
            ("digital banking", 2.5),
            ("robo advisor", 2.5),
            ("insurtech", 2.3),
            ("regtech", 2.3),
            ("wealthtech", 2.3),
        ];
        for (keyword, weight) in &fintech {
            finance_keywords.insert(keyword.to_string(), *weight);
        }

        // Accounting (Medium weight)
        let accounting = [
            ("gaap", 2.5),
            ("ifrs", 2.5),
            ("financial statements", 2.3),
            ("audit", 2.3),
            ("tax preparation", 2.0),
            ("budgeting", 2.0),
            ("forecasting", 2.3),
            ("variance analysis", 2.3),
            ("cost accounting", 2.3),
            ("management accounting", 2.3),
        ];
        for (keyword, weight) in &accounting {
            finance_keywords.insert(keyword.to_string(), *weight);
        }

        db.insert("finance".to_string(), finance_keywords);

        // Healthcare Industry Keywords
        let mut healthcare_keywords = HashMap::new();

        // Clinical & Medical (Very High weight)
        let clinical_medical = [
            ("clinical research", 3.0),
            ("clinical trials", 3.0),
            ("medical device", 2.8),
            ("pharmaceutical", 2.8),
            ("biotechnology", 2.8),
            ("drug development", 2.8),
            ("fda", 2.8),
            ("gcp", 2.5),
            ("gmp", 2.5),
            ("regulatory affairs", 2.8),
            ("pharmacovigilance", 2.5),
            ("biostatistics", 2.8),
            ("epidemiology", 2.5),
        ];
        for (keyword, weight) in &clinical_medical {
            healthcare_keywords.insert(keyword.to_string(), *weight);
        }

        // Healthcare IT (High weight)
        let healthcare_it = [
            ("ehr", 2.8),
            ("emr", 2.8),
            ("epic", 2.5),
            ("cerner", 2.5),
            ("allscripts", 2.3),
            ("hl7", 2.5),
            ("fhir", 2.5),
            ("dicom", 2.3),
            ("hipaa", 2.8),
            ("hitech", 2.3),
            ("healthcare analytics", 2.5),
            ("population health", 2.3),
            ("telemedicine", 2.5),
        ];
        for (keyword, weight) in &healthcare_it {
            healthcare_keywords.insert(keyword.to_string(), *weight);
        }

        // Healthcare Operations (Medium weight)
        let healthcare_ops = [
            ("patient care", 2.3),
            ("quality improvement", 2.3),
            ("healthcare administration", 2.0),
            ("medical coding", 2.3),
            ("icd-10", 2.3),
            ("cpt", 2.3),
            ("revenue cycle", 2.3),
            ("case management", 2.0),
            ("utilization review", 2.0),
            ("discharge planning", 2.0),
        ];
        for (keyword, weight) in &healthcare_ops {
            healthcare_keywords.insert(keyword.to_string(), *weight);
        }

        // Medical Research (High weight)
        let medical_research = [
            ("medical research", 2.8),
            ("clinical data management", 2.5),
            ("biomarkers", 2.5),
            ("genomics", 2.8),
            ("proteomics", 2.5),
            ("bioinformatics", 2.8),
            ("precision medicine", 2.5),
            ("translational research", 2.5),
            ("oncology", 2.3),
        ];
        for (keyword, weight) in &medical_research {
            healthcare_keywords.insert(keyword.to_string(), *weight);
        }

        db.insert("healthcare".to_string(), healthcare_keywords);

        // Marketing Industry Keywords
        let mut marketing_keywords = HashMap::new();

        // Digital Marketing (Very High weight)
        let digital_marketing = [
            ("digital marketing", 3.0),
            ("seo", 2.8),
            ("sem", 2.8),
            ("ppc", 2.8),
            ("google ads", 2.8),
            ("facebook ads", 2.5),
            ("social media marketing", 2.8),
            ("content marketing", 2.8),
            ("email marketing", 2.5),
            ("marketing automation", 2.8),
            ("lead generation", 2.5),
            ("conversion optimization", 2.8),
            ("a/b testing", 2.5),
        ];
        for (keyword, weight) in &digital_marketing {
            marketing_keywords.insert(keyword.to_string(), *weight);
        }

        // Marketing Analytics (High weight)
        let marketing_analytics = [
            ("google analytics", 2.8),
            ("marketing analytics", 2.8),
            ("customer analytics", 2.5),
            ("marketing attribution", 2.5),
            ("cohort analysis", 2.3),
            ("funnel analysis", 2.5),
            ("customer lifetime value", 2.5),
            ("churn analysis", 2.3),
            ("segment analysis", 2.3),
        ];
        for (keyword, weight) in &marketing_analytics {
            marketing_keywords.insert(keyword.to_string(), *weight);
        }

        // Marketing Technology (High weight)
        let marketing_tech = [
            ("martech", 2.8),
            ("crm", 2.5),
            ("salesforce", 2.5),
            ("hubspot", 2.5),
            ("marketo", 2.3),
            ("pardot", 2.3),
            ("mailchimp", 2.0),
            ("hootsuite", 2.0),
            ("buffer", 1.8),
            ("sprout social", 2.0),
            ("adobe creative suite", 2.3),
        ];
        for (keyword, weight) in &marketing_tech {
            marketing_keywords.insert(keyword.to_string(), *weight);
        }

        // Brand & Creative (Medium weight)
        let brand_creative = [
            ("brand management", 2.3),
            ("brand strategy", 2.3),
            ("creative strategy", 2.3),
            ("copywriting", 2.0),
            ("graphic design", 2.0),
            ("video production", 2.0),
            ("influencer marketing", 2.3),
            ("public relations", 2.0),
            ("crisis communication", 2.0),
        ];
        for (keyword, weight) in &brand_creative {
            marketing_keywords.insert(keyword.to_string(), *weight);
        }

        // Growth Marketing (High weight)
        let growth_marketing = [
            ("growth hacking", 2.5),
            ("growth marketing", 2.8),
            ("product marketing", 2.5),
            ("customer acquisition", 2.5),
            ("retention marketing", 2.3),
            ("referral marketing", 2.3),
            ("viral marketing", 2.0),
            ("performance marketing", 2.8),
            ("programmatic advertising", 2.5),
        ];
        for (keyword, weight) in &growth_marketing {
            marketing_keywords.insert(keyword.to_string(), *weight);
        }

        db.insert("marketing".to_string(), marketing_keywords);

        // General Business Keywords (lower weights, applicable across industries)
        let mut general_keywords = HashMap::new();
        let general_business = [
            ("project management", 2.0),
            ("agile", 1.8),
            ("scrum", 1.8),
            ("kanban", 1.5),
            ("leadership", 1.8),
            ("team management", 1.8),
            ("strategic planning", 2.0),
            ("business analysis", 2.0),
            ("process improvement", 1.8),
            ("stakeholder management", 1.8),
            ("communication", 1.5),
            ("presentation", 1.5),
            ("negotiation", 1.8),
            ("problem solving", 1.5),
        ];
        for (keyword, weight) in &general_business {
            general_keywords.insert(keyword.to_string(), *weight);
        }

        db.insert("general".to_string(), general_keywords);

        db
    }

    /// Calculate keyword alignment with industry-specific weights
    fn calculate_keyword_alignment(
        &self,
        parsed_resume: &ParsedResume,
        industry_keywords: &HashMap<String, f64>,
    ) -> Result<f64> {
        if industry_keywords.is_empty() {
            return Ok(50.0); // Neutral score if no industry keywords
        }

        let mut total_weight = 0.0;
        let mut matched_weight = 0.0;

        // Check each industry keyword against resume content
        for (keyword, weight) in industry_keywords {
            total_weight += weight;

            // Check if keyword appears in resume (case insensitive)
            let keyword_lower = keyword.to_lowercase();
            let mut found = false;

            // Check in skills
            for skill in &parsed_resume.skills {
                if skill.to_lowercase().contains(&keyword_lower) {
                    matched_weight += weight;
                    found = true;
                    break;
                }
            }

            if !found {
                // Check in experience descriptions
                for exp in &parsed_resume.experience {
                    if exp.title.to_lowercase().contains(&keyword_lower)
                        || exp.description.to_lowercase().contains(&keyword_lower)
                        || exp
                            .achievements
                            .iter()
                            .any(|a| a.to_lowercase().contains(&keyword_lower))
                    {
                        matched_weight += weight * 0.8; // Slightly lower weight for experience mentions
                        break;
                    }
                }
            }

            if !found {
                // Check in sections
                for section_content in parsed_resume.sections.values() {
                    if section_content.to_lowercase().contains(&keyword_lower) {
                        matched_weight += weight * 0.6; // Lower weight for general section mentions
                        break;
                    }
                }
            }
        }

        let alignment_score = if total_weight > 0.0 {
            (matched_weight / total_weight) * 100.0
        } else {
            50.0
        };

        Ok(alignment_score.clamp(0.0, 100.0))
    }

    /// Calculate skill alignment based on industry-specific skill requirements
    fn calculate_skill_alignment(
        &self,
        parsed_resume: &ParsedResume,
        industry: &str,
    ) -> Result<f64> {
        let industry_skill_requirements = self.get_industry_skill_requirements(industry);
        let resume_skills: Vec<String> = parsed_resume
            .skills
            .iter()
            .map(|s| s.to_lowercase())
            .collect();

        if industry_skill_requirements.is_empty() {
            return Ok(50.0);
        }

        let mut total_importance = 0.0;
        let mut matched_importance = 0.0;

        for skills_and_importance in industry_skill_requirements.values() {
            for (skill, importance) in skills_and_importance {
                total_importance += importance;

                // Check if resume contains this skill (fuzzy matching)
                let skill_lower = skill.to_lowercase();
                if resume_skills
                    .iter()
                    .any(|rs| rs.contains(&skill_lower) || skill_lower.contains(rs))
                {
                    matched_importance += importance;
                }
            }
        }

        let skill_score = if total_importance > 0.0 {
            (matched_importance / total_importance) * 100.0
        } else {
            50.0
        };

        Ok(skill_score.clamp(0.0, 100.0))
    }

    /// Get industry-specific skill requirements with importance weights
    fn get_industry_skill_requirements(
        &self,
        industry: &str,
    ) -> HashMap<String, Vec<(String, f64)>> {
        let mut requirements = HashMap::new();

        match industry {
            "technology" => {
                requirements.insert(
                    "core_programming".to_string(),
                    vec![
                        ("python".to_string(), 3.0),
                        ("java".to_string(), 3.0),
                        ("javascript".to_string(), 3.0),
                        ("sql".to_string(), 2.8),
                        ("git".to_string(), 2.5),
                    ],
                );
                requirements.insert(
                    "cloud_devops".to_string(),
                    vec![
                        ("aws".to_string(), 2.8),
                        ("docker".to_string(), 2.5),
                        ("kubernetes".to_string(), 2.8),
                        ("ci/cd".to_string(), 2.5),
                    ],
                );
                requirements.insert(
                    "frameworks".to_string(),
                    vec![
                        ("react".to_string(), 2.5),
                        ("angular".to_string(), 2.5),
                        ("node.js".to_string(), 2.5),
                        ("spring".to_string(), 2.3),
                    ],
                );
            }
            "finance" => {
                requirements.insert(
                    "financial_analysis".to_string(),
                    vec![
                        ("financial modeling".to_string(), 3.0),
                        ("excel".to_string(), 2.8),
                        ("bloomberg".to_string(), 2.5),
                        ("risk management".to_string(), 2.8),
                    ],
                );
                requirements.insert(
                    "quantitative".to_string(),
                    vec![
                        ("python".to_string(), 2.5),
                        ("r".to_string(), 2.5),
                        ("sql".to_string(), 2.3),
                        ("statistics".to_string(), 2.3),
                    ],
                );
            }
            "healthcare" => {
                requirements.insert(
                    "clinical".to_string(),
                    vec![
                        ("clinical research".to_string(), 3.0),
                        ("gcp".to_string(), 2.5),
                        ("fda regulations".to_string(), 2.8),
                        ("medical writing".to_string(), 2.3),
                    ],
                );
                requirements.insert(
                    "healthcare_it".to_string(),
                    vec![
                        ("ehr".to_string(), 2.5),
                        ("hipaa".to_string(), 2.5),
                        ("hl7".to_string(), 2.3),
                    ],
                );
            }
            "marketing" => {
                requirements.insert(
                    "digital_marketing".to_string(),
                    vec![
                        ("google analytics".to_string(), 2.8),
                        ("seo".to_string(), 2.8),
                        ("ppc".to_string(), 2.5),
                        ("social media".to_string(), 2.3),
                    ],
                );
                requirements.insert(
                    "marketing_tools".to_string(),
                    vec![
                        ("hubspot".to_string(), 2.3),
                        ("salesforce".to_string(), 2.3),
                        ("adobe creative suite".to_string(), 2.0),
                    ],
                );
            }
            _ => {
                // General business skills
                requirements.insert(
                    "general".to_string(),
                    vec![
                        ("project management".to_string(), 2.0),
                        ("communication".to_string(), 1.8),
                        ("leadership".to_string(), 1.8),
                    ],
                );
            }
        }

        requirements
    }

    /// Calculate experience alignment based on industry and level
    fn calculate_experience_alignment(
        &self,
        parsed_resume: &ParsedResume,
        industry: &str,
        experience_level: &str,
    ) -> Result<f64> {
        let expected_experience = self.get_expected_experience_patterns(industry, experience_level);
        let mut alignment_score = 50.0; // Base score

        // Check experience count
        let experience_count = parsed_resume.experience.len();
        match experience_level {
            "entry" => {
                if experience_count >= 1 {
                    alignment_score += 20.0;
                }
            }
            "mid" => {
                if experience_count >= 2 {
                    alignment_score += 15.0;
                }
                if experience_count >= 3 {
                    alignment_score += 10.0;
                }
            }
            "senior" => {
                if experience_count >= 3 {
                    alignment_score += 10.0;
                }
                if experience_count >= 5 {
                    alignment_score += 15.0;
                }
            }
            _ => {}
        }

        // Check for industry-relevant experience
        let mut industry_relevant_count = 0;
        for exp in &parsed_resume.experience {
            let exp_text =
                format!("{} {} {}", exp.title, exp.company, exp.description).to_lowercase();

            for pattern in &expected_experience.industry_keywords {
                if exp_text.contains(&pattern.to_lowercase()) {
                    industry_relevant_count += 1;
                    break;
                }
            }
        }

        if industry_relevant_count > 0 {
            alignment_score += (industry_relevant_count as f64 * 10.0).min(30.0);
        }

        // Check for leadership/progression indicators
        if experience_level == "senior" {
            let leadership_indicators = [
                "lead",
                "manager",
                "director",
                "senior",
                "principal",
                "architect",
            ];
            for exp in &parsed_resume.experience {
                let title_lower = exp.title.to_lowercase();
                if leadership_indicators
                    .iter()
                    .any(|indicator| title_lower.contains(indicator))
                {
                    alignment_score += 15.0;
                    break;
                }
            }
        }

        Ok(alignment_score.clamp(0.0, 100.0))
    }

    /// Get expected experience patterns for industry and level
    fn get_expected_experience_patterns(
        &self,
        industry: &str,
        _experience_level: &str,
    ) -> ExperiencePattern {
        let industry_keywords = match industry {
            "technology" => vec![
                "software",
                "developer",
                "engineer",
                "programming",
                "coding",
                "technical",
                "system",
                "application",
                "web",
                "mobile",
                "database",
                "cloud",
                "devops",
            ],
            "finance" => vec![
                "financial",
                "banking",
                "investment",
                "trading",
                "analyst",
                "portfolio",
                "risk",
                "credit",
                "wealth",
                "capital",
                "asset",
                "fund",
                "insurance",
            ],
            "healthcare" => vec![
                "healthcare",
                "medical",
                "clinical",
                "hospital",
                "pharmaceutical",
                "biotech",
                "patient",
                "therapy",
                "diagnosis",
                "treatment",
                "research",
                "regulatory",
            ],
            "marketing" => vec![
                "marketing",
                "advertising",
                "brand",
                "campaign",
                "digital",
                "social",
                "content",
                "seo",
                "analytics",
                "growth",
                "customer",
                "lead",
                "conversion",
            ],
            _ => vec![
                "business",
                "management",
                "operations",
                "strategy",
                "analysis",
                "consulting",
            ],
        };

        ExperiencePattern {
            industry_keywords: industry_keywords.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Calculate education alignment with industry requirements
    fn calculate_education_alignment(
        &self,
        parsed_resume: &ParsedResume,
        industry: &str,
    ) -> Result<f64> {
        let preferred_degrees = self.get_preferred_degrees(industry);
        let mut alignment_score = 50.0; // Base score

        if parsed_resume.education.is_empty() {
            return Ok(30.0); // Lower score for no education listed
        }

        for education in &parsed_resume.education {
            let degree_lower = education.degree.to_lowercase();
            let institution_lower = education.institution.to_lowercase();

            // Check for preferred degree types
            for (degree_type, weight) in &preferred_degrees {
                if degree_lower.contains(&degree_type.to_lowercase()) {
                    alignment_score += weight;
                }
            }

            // Bonus for prestigious institutions (simplified list)
            let prestigious_indicators = [
                "harvard",
                "mit",
                "stanford",
                "berkeley",
                "carnegie mellon",
                "caltech",
                "princeton",
                "yale",
                "columbia",
                "cornell",
            ];

            if prestigious_indicators
                .iter()
                .any(|inst| institution_lower.contains(inst))
            {
                alignment_score += 10.0;
            }
        }

        Ok(alignment_score.clamp(0.0, 100.0))
    }

    /// Get preferred degrees for each industry with weights
    fn get_preferred_degrees(&self, industry: &str) -> Vec<(String, f64)> {
        match industry {
            "technology" => vec![
                ("computer science".to_string(), 20.0),
                ("software engineering".to_string(), 18.0),
                ("electrical engineering".to_string(), 15.0),
                ("mathematics".to_string(), 12.0),
                ("physics".to_string(), 10.0),
                ("data science".to_string(), 18.0),
                ("information systems".to_string(), 15.0),
            ],
            "finance" => vec![
                ("finance".to_string(), 20.0),
                ("economics".to_string(), 18.0),
                ("accounting".to_string(), 15.0),
                ("business administration".to_string(), 12.0),
                ("mathematics".to_string(), 15.0),
                ("statistics".to_string(), 12.0),
                ("mba".to_string(), 15.0),
            ],
            "healthcare" => vec![
                ("medicine".to_string(), 25.0),
                ("nursing".to_string(), 20.0),
                ("biology".to_string(), 15.0),
                ("chemistry".to_string(), 15.0),
                ("biomedical engineering".to_string(), 18.0),
                ("public health".to_string(), 15.0),
                ("pharmacy".to_string(), 20.0),
            ],
            "marketing" => vec![
                ("marketing".to_string(), 20.0),
                ("business administration".to_string(), 15.0),
                ("communications".to_string(), 12.0),
                ("psychology".to_string(), 10.0),
                ("advertising".to_string(), 18.0),
                ("digital marketing".to_string(), 18.0),
                ("mba".to_string(), 15.0),
            ],
            _ => vec![
                ("business administration".to_string(), 15.0),
                ("management".to_string(), 12.0),
                ("economics".to_string(), 10.0),
                ("mba".to_string(), 15.0),
            ],
        }
    }

    async fn get_benchmark_comparison(
        &self,
        keyword_analysis: &KeywordMatch,
        format_analysis: &FormatAnalysis,
        industry: &str,
        experience_level: &str,
    ) -> Result<BenchmarkComparison> {
        // Build industry and experience level benchmarks
        let industry_benchmarks = self.build_industry_benchmarks();
        let experience_benchmarks = self.build_experience_level_benchmarks();

        // Calculate current resume's overall score
        let current_score = self.calculate_composite_score(keyword_analysis, format_analysis);

        // Get industry-specific benchmark data
        let default_industry = IndustryBenchmark::default();
        let industry_data = industry_benchmarks
            .get(industry)
            .unwrap_or(&default_industry);

        // Get experience-level-specific benchmark data
        let default_experience = ExperienceLevelBenchmark::default();
        let experience_data = experience_benchmarks
            .get(experience_level)
            .unwrap_or(&default_experience);

        // Calculate percentiles
        let industry_percentile =
            self.calculate_percentile(current_score, &industry_data.score_distribution);
        let experience_level_percentile =
            self.calculate_percentile(current_score, &experience_data.score_distribution);

        // Calculate overall percentile (weighted average)
        let overall_percentile = (industry_percentile * 0.6) + (experience_level_percentile * 0.4);

        // Calculate gap to top performers
        let top_performers_score = industry_data.top_10_percent_score;
        let top_performers_gap = if current_score >= top_performers_score {
            0.0
        } else {
            top_performers_score - current_score
        };

        Ok(BenchmarkComparison {
            industry_percentile,
            experience_level_percentile,
            overall_percentile,
            top_performers_gap,
        })
    }

    /// Build industry-specific benchmarks
    fn build_industry_benchmarks(&self) -> HashMap<String, IndustryBenchmark> {
        let mut benchmarks = HashMap::new();

        // Technology Industry Benchmarks
        benchmarks.insert(
            "technology".to_string(),
            IndustryBenchmark {
                average_score: 78.5,
                median_score: 75.0,
                top_10_percent_score: 92.0,
                bottom_10_percent_score: 52.0,
                score_distribution: vec![
                    (50.0, 5.0),    // 5% score below 50
                    (60.0, 15.0),   // 15% score below 60
                    (70.0, 35.0),   // 35% score below 70
                    (80.0, 65.0),   // 65% score below 80
                    (90.0, 85.0),   // 85% score below 90
                    (95.0, 95.0),   // 95% score below 95
                    (100.0, 100.0), // 100% score below 100
                ],
                keyword_match_average: 72.0,
                format_score_average: 85.0,
                sections_average: 6.2,
            },
        );

        // Finance Industry Benchmarks
        benchmarks.insert(
            "finance".to_string(),
            IndustryBenchmark {
                average_score: 76.2,
                median_score: 73.0,
                top_10_percent_score: 91.5,
                bottom_10_percent_score: 48.0,
                score_distribution: vec![
                    (50.0, 8.0),
                    (60.0, 20.0),
                    (70.0, 40.0),
                    (80.0, 70.0),
                    (90.0, 88.0),
                    (95.0, 96.0),
                    (100.0, 100.0),
                ],
                keyword_match_average: 69.5,
                format_score_average: 82.0,
                sections_average: 5.8,
            },
        );

        // Healthcare Industry Benchmarks
        benchmarks.insert(
            "healthcare".to_string(),
            IndustryBenchmark {
                average_score: 74.8,
                median_score: 72.0,
                top_10_percent_score: 89.0,
                bottom_10_percent_score: 51.0,
                score_distribution: vec![
                    (50.0, 6.0),
                    (60.0, 18.0),
                    (70.0, 42.0),
                    (80.0, 72.0),
                    (90.0, 90.0),
                    (95.0, 97.0),
                    (100.0, 100.0),
                ],
                keyword_match_average: 68.0,
                format_score_average: 81.5,
                sections_average: 6.0,
            },
        );

        // Marketing Industry Benchmarks
        benchmarks.insert(
            "marketing".to_string(),
            IndustryBenchmark {
                average_score: 73.5,
                median_score: 71.0,
                top_10_percent_score: 88.5,
                bottom_10_percent_score: 49.0,
                score_distribution: vec![
                    (50.0, 7.0),
                    (60.0, 22.0),
                    (70.0, 45.0),
                    (80.0, 75.0),
                    (90.0, 92.0),
                    (95.0, 98.0),
                    (100.0, 100.0),
                ],
                keyword_match_average: 66.5,
                format_score_average: 80.0,
                sections_average: 5.5,
            },
        );

        // General/Other Industries
        benchmarks.insert(
            "general".to_string(),
            IndustryBenchmark {
                average_score: 71.0,
                median_score: 68.0,
                top_10_percent_score: 85.0,
                bottom_10_percent_score: 46.0,
                score_distribution: vec![
                    (50.0, 10.0),
                    (60.0, 25.0),
                    (70.0, 50.0),
                    (80.0, 75.0),
                    (90.0, 90.0),
                    (95.0, 95.0),
                    (100.0, 100.0),
                ],
                keyword_match_average: 63.0,
                format_score_average: 78.0,
                sections_average: 5.0,
            },
        );

        benchmarks
    }

    /// Build experience level benchmarks
    fn build_experience_level_benchmarks(&self) -> HashMap<String, ExperienceLevelBenchmark> {
        let mut benchmarks = HashMap::new();

        // Entry Level (0-2 years)
        benchmarks.insert(
            "entry".to_string(),
            ExperienceLevelBenchmark {
                average_score: 68.5,
                median_score: 66.0,
                top_10_percent_score: 82.0,
                bottom_10_percent_score: 45.0,
                score_distribution: vec![
                    (50.0, 12.0),
                    (60.0, 30.0),
                    (70.0, 55.0),
                    (80.0, 80.0),
                    (90.0, 95.0),
                    (95.0, 98.0),
                    (100.0, 100.0),
                ],
                expected_sections: 4.5,
                expected_keyword_density: 0.15,
            },
        );

        // Mid Level (3-7 years)
        benchmarks.insert(
            "mid".to_string(),
            ExperienceLevelBenchmark {
                average_score: 75.2,
                median_score: 73.0,
                top_10_percent_score: 89.0,
                bottom_10_percent_score: 52.0,
                score_distribution: vec![
                    (50.0, 5.0),
                    (60.0, 15.0),
                    (70.0, 35.0),
                    (80.0, 65.0),
                    (90.0, 85.0),
                    (95.0, 95.0),
                    (100.0, 100.0),
                ],
                expected_sections: 5.8,
                expected_keyword_density: 0.22,
            },
        );

        // Senior Level (8+ years)
        benchmarks.insert(
            "senior".to_string(),
            ExperienceLevelBenchmark {
                average_score: 81.0,
                median_score: 79.0,
                top_10_percent_score: 94.0,
                bottom_10_percent_score: 58.0,
                score_distribution: vec![
                    (50.0, 2.0),
                    (60.0, 8.0),
                    (70.0, 25.0),
                    (80.0, 50.0),
                    (90.0, 75.0),
                    (95.0, 90.0),
                    (100.0, 100.0),
                ],
                expected_sections: 6.5,
                expected_keyword_density: 0.28,
            },
        );

        benchmarks
    }

    /// Calculate composite score from keyword and format analysis
    fn calculate_composite_score(
        &self,
        keyword_analysis: &KeywordMatch,
        format_analysis: &FormatAnalysis,
    ) -> f64 {
        // Weighted combination of different score components
        let keyword_weight = 0.5;
        let format_weight = 0.3;
        let density_weight = 0.2;

        let keyword_score = keyword_analysis.overall_score;
        let format_score = format_analysis.ats_compatibility_score;
        let density_score = keyword_analysis.match_density * 100.0;

        let composite = (keyword_score * keyword_weight)
            + (format_score * format_weight)
            + (density_score * density_weight);

        composite.clamp(0.0, 100.0)
    }

    /// Calculate percentile based on score distribution
    fn calculate_percentile(&self, score: f64, distribution: &[(f64, f64)]) -> f64 {
        if distribution.is_empty() {
            return 50.0; // Default percentile
        }

        // Find the percentile using linear interpolation
        for (i, (threshold, percentile)) in distribution.iter().enumerate() {
            if score <= *threshold {
                if i == 0 {
                    return *percentile;
                }

                // Linear interpolation between two points
                let (prev_threshold, prev_percentile) = distribution[i - 1];
                let ratio = (score - prev_threshold) / (threshold - prev_threshold);
                return prev_percentile + ratio * (percentile - prev_percentile);
            }
        }

        // If score is above all thresholds, return the highest percentile
        distribution.last().map(|(_, p)| *p).unwrap_or(95.0)
    }

    async fn generate_optimization_suggestions(
        &self,
        parsed_resume: &ParsedResume,
        keyword_analysis: &KeywordMatch,
        format_analysis: &FormatAnalysis,
        job_description: &str,
        industry: &str,
    ) -> Result<Vec<OptimizationSuggestion>> {
        let mut suggestions = Vec::new();

        // Extract target keywords from job description
        let target_keywords = self
            .keyword_analyzer
            .extract_keywords_from_job_description(job_description)?;

        // Get industry-specific recommendations
        let industry_db = self.build_industry_keyword_database();
        let empty_map = HashMap::new();
        let industry_keywords = industry_db.get(industry).unwrap_or(&empty_map);

        // Generate keyword optimization suggestions
        suggestions.extend(self.generate_keyword_suggestions(
            parsed_resume,
            keyword_analysis,
            &target_keywords,
            industry_keywords,
        )?);

        // Generate format optimization suggestions
        suggestions.extend(self.generate_format_suggestions(parsed_resume, format_analysis)?);

        // Generate section optimization suggestions
        suggestions.extend(self.generate_section_suggestions(parsed_resume, industry)?);

        // Generate content optimization suggestions
        suggestions.extend(self.generate_content_suggestions(
            parsed_resume,
            &target_keywords,
            industry,
        )?);

        // Generate ATS-specific suggestions
        suggestions.extend(self.generate_ats_suggestions(parsed_resume, format_analysis)?);

        // Sort by impact score (highest first)
        suggestions.sort_by(|a, b| {
            b.impact_score
                .partial_cmp(&a.impact_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Take top 15 suggestions to avoid overwhelming the user
        suggestions.truncate(15);

        Ok(suggestions)
    }

    /// Generate keyword-related optimization suggestions
    fn generate_keyword_suggestions(
        &self,
        parsed_resume: &ParsedResume,
        keyword_analysis: &KeywordMatch,
        target_keywords: &[String],
        industry_keywords: &HashMap<String, f64>,
    ) -> Result<Vec<OptimizationSuggestion>> {
        let mut suggestions = Vec::new();

        // Find missing high-value keywords
        let resume_text = self.get_resume_text(parsed_resume);
        let missing_keywords =
            self.find_missing_keywords(&resume_text, target_keywords, industry_keywords);

        // Suggest adding missing keywords
        for (keyword, importance) in missing_keywords.iter().take(5) {
            let suggestion = OptimizationSuggestion {
                category: "Keywords".to_string(),
                title: format!("Add '{}' keyword", keyword),
                description: "This keyword appears in the job description and is highly valued in your industry. Consider adding it to your skills section or work experience descriptions.".to_string(),
                impact_score: importance * 20.0,
                difficulty: if parsed_resume.skills.is_empty() { "Medium".to_string() } else { "Easy".to_string() },
                specific_actions: vec![
                    SuggestionAction {
                        action: format!("Add '{}' to your skills section", keyword),
                        section: "Skills".to_string(),
                        reasoning: "Skills section is the most direct place for keyword inclusion".to_string(),
                    },
                    SuggestionAction {
                        action: format!("Incorporate '{}' into a work experience description", keyword),
                        section: "Experience".to_string(),
                        reasoning: "Contextual keyword usage in experience shows practical application".to_string(),
                    },
                ],
                before_example: "Skills: Java, Python, SQL".to_string(),
                after_example: format!("Skills: Java, Python, SQL, {}", keyword),
            };
            suggestions.push(suggestion);
        }

        // Suggest improving keyword density if too low
        if keyword_analysis.match_density < 0.15 {
            let suggestion = OptimizationSuggestion {
                category: "Keywords".to_string(),
                title: "Increase keyword density".to_string(),
                description: "Your resume has low keyword density. ATS systems favor resumes with appropriate keyword usage throughout.".to_string(),
                impact_score: 85.0,
                difficulty: "Medium".to_string(),
                specific_actions: vec![
                    SuggestionAction {
                        action: "Rewrite job descriptions to include more relevant keywords".to_string(),
                        section: "Experience".to_string(),
                        reasoning: "Natural keyword integration improves ATS parsing and relevance".to_string(),
                    },
                    SuggestionAction {
                        action: "Add a 'Core Competencies' section with key skills".to_string(),
                        section: "Skills".to_string(),
                        reasoning: "Dedicated skills section increases keyword density effectively".to_string(),
                    },
                ],
                before_example: "Worked on software projects".to_string(),
                after_example: "Developed Python applications using React frontend and PostgreSQL database".to_string(),
            };
            suggestions.push(suggestion);
        }

        // Suggest better keyword placement
        if keyword_analysis.exact_matches.len() < 3 {
            let suggestion = OptimizationSuggestion {
                category: "Keywords".to_string(),
                title: "Improve keyword placement".to_string(),
                description: "Place important keywords in multiple sections (skills, experience, summary) for better ATS recognition.".to_string(),
                impact_score: 75.0,
                difficulty: "Easy".to_string(),
                specific_actions: vec![
                    SuggestionAction {
                        action: "Add a professional summary with key keywords".to_string(),
                        section: "Summary".to_string(),
                        reasoning: "Summary section is often the first section ATS systems parse".to_string(),
                    },
                    SuggestionAction {
                        action: "Use keywords in job titles and descriptions".to_string(),
                        section: "Experience".to_string(),
                        reasoning: "Keywords in job titles and descriptions have high ATS weight".to_string(),
                    },
                ],
                before_example: "Summary: Experienced professional with strong background".to_string(),
                after_example: "Summary: Senior Software Engineer with 5+ years Python, React, and AWS experience".to_string(),
            };
            suggestions.push(suggestion);
        }

        Ok(suggestions)
    }

    /// Generate format-related optimization suggestions
    fn generate_format_suggestions(
        &self,
        _parsed_resume: &ParsedResume,
        format_analysis: &FormatAnalysis,
    ) -> Result<Vec<OptimizationSuggestion>> {
        let mut suggestions = Vec::new();

        // ATS compatibility suggestions
        if format_analysis.ats_compatibility_score < 80.0 {
            let suggestion = OptimizationSuggestion {
                category: "Format".to_string(),
                title: "Improve ATS compatibility".to_string(),
                description: "Your resume format may not be fully compatible with ATS systems. Use standard section headers and avoid complex formatting.".to_string(),
                impact_score: 90.0,
                difficulty: "Medium".to_string(),
                specific_actions: vec![
                    SuggestionAction {
                        action: "Use standard section headers (Experience, Education, Skills)".to_string(),
                        section: "Format".to_string(),
                        reasoning: "ATS systems are trained to recognize standard section headers".to_string(),
                    },
                    SuggestionAction {
                        action: "Remove tables, columns, and complex formatting".to_string(),
                        section: "Format".to_string(),
                        reasoning: "Complex formatting can confuse ATS parsing algorithms".to_string(),
                    },
                ],
                before_example: "║ PROFESSIONAL BACKGROUND ║".to_string(),
                after_example: "EXPERIENCE".to_string(),
            };
            suggestions.push(suggestion);
        }

        // Font compatibility suggestions
        if format_analysis.font_compatibility < 85.0 {
            let suggestion = OptimizationSuggestion {
                category: "Format".to_string(),
                title: "Use ATS-friendly fonts".to_string(),
                description: "Use standard fonts like Arial, Calibri, or Times New Roman for better ATS readability.".to_string(),
                impact_score: 70.0,
                difficulty: "Easy".to_string(),
                specific_actions: vec![
                    SuggestionAction {
                        action: "Change font to Arial, Calibri, or Times New Roman".to_string(),
                        section: "Format".to_string(),
                        reasoning: "These fonts are universally recognized by ATS systems".to_string(),
                    },
                    SuggestionAction {
                        action: "Use font sizes between 10-12 points".to_string(),
                        section: "Format".to_string(),
                        reasoning: "Standard font sizes ensure proper text recognition".to_string(),
                    },
                ],
                before_example: "Using decorative or script fonts".to_string(),
                after_example: "Using Arial 11pt for body text".to_string(),
            };
            suggestions.push(suggestion);
        }

        // Layout suggestions
        if format_analysis.layout_score < 80.0 {
            let suggestion = OptimizationSuggestion {
                category: "Format".to_string(),
                title: "Simplify layout structure".to_string(),
                description: "Use a simple, single-column layout with clear section breaks for optimal ATS parsing.".to_string(),
                impact_score: 80.0,
                difficulty: "Medium".to_string(),
                specific_actions: vec![
                    SuggestionAction {
                        action: "Convert to single-column layout".to_string(),
                        section: "Format".to_string(),
                        reasoning: "Single-column layouts are parsed most reliably by ATS systems".to_string(),
                    },
                    SuggestionAction {
                        action: "Use consistent formatting for similar elements".to_string(),
                        section: "Format".to_string(),
                        reasoning: "Consistency helps ATS systems identify patterns and structure".to_string(),
                    },
                ],
                before_example: "Two-column layout with sidebar".to_string(),
                after_example: "Single-column layout with clear sections".to_string(),
            };
            suggestions.push(suggestion);
        }

        Ok(suggestions)
    }

    /// Generate section-related optimization suggestions
    fn generate_section_suggestions(
        &self,
        parsed_resume: &ParsedResume,
        industry: &str,
    ) -> Result<Vec<OptimizationSuggestion>> {
        let mut suggestions = Vec::new();

        // Missing sections suggestions
        if !parsed_resume.sections.contains_key("Summary") {
            let suggestion = OptimizationSuggestion {
                category: "Sections".to_string(),
                title: "Add professional summary".to_string(),
                description: "A professional summary at the top of your resume helps ATS systems and recruiters quickly understand your value proposition.".to_string(),
                impact_score: 85.0,
                difficulty: "Medium".to_string(),
                specific_actions: vec![
                    SuggestionAction {
                        action: "Write a 2-3 sentence professional summary".to_string(),
                        section: "Summary".to_string(),
                        reasoning: "Summary section is often the first section ATS systems parse".to_string(),
                    },
                    SuggestionAction {
                        action: "Include your years of experience and key skills".to_string(),
                        section: "Summary".to_string(),
                        reasoning: "Key information in summary improves initial ATS scoring".to_string(),
                    },
                ],
                before_example: "Resume starts with contact information".to_string(),
                after_example: "Professional Summary: Senior Software Engineer with 5+ years developing scalable web applications using Python, React, and AWS".to_string(),
            };
            suggestions.push(suggestion);
        }

        // Industry-specific section suggestions
        match industry {
            "technology" => {
                if !parsed_resume.sections.contains_key("Projects") {
                    let suggestion = OptimizationSuggestion {
                        category: "Sections".to_string(),
                        title: "Add technical projects section".to_string(),
                        description: "For technology roles, a projects section showcases your technical skills and experience with specific technologies.".to_string(),
                        impact_score: 75.0,
                        difficulty: "Medium".to_string(),
                        specific_actions: vec![
                            SuggestionAction {
                                action: "Add a 'Projects' or 'Technical Projects' section".to_string(),
                                section: "Projects".to_string(),
                                reasoning: "Projects section is highly valued in technology industry".to_string(),
                            },
                            SuggestionAction {
                                action: "Include 2-3 relevant projects with technologies used".to_string(),
                                section: "Projects".to_string(),
                                reasoning: "Specific project details demonstrate practical skills".to_string(),
                            },
                        ],
                        before_example: "Only Experience and Education sections".to_string(),
                        after_example: "Added Projects section with E-commerce Platform (React, Node.js, MongoDB)".to_string(),
                    };
                    suggestions.push(suggestion);
                }
            }
            "finance" => {
                if !parsed_resume.sections.contains_key("Certifications") {
                    let suggestion = OptimizationSuggestion {
                        category: "Sections".to_string(),
                        title: "Add certifications section".to_string(),
                        description: "Financial industry values certifications. Add a section for CFA, FRM, or other relevant certifications.".to_string(),
                        impact_score: 70.0,
                        difficulty: "Easy".to_string(),
                        specific_actions: vec![
                            SuggestionAction {
                                action: "Add 'Certifications' section".to_string(),
                                section: "Certifications".to_string(),
                                reasoning: "Certifications are highly valued in finance industry".to_string(),
                            },
                        ],
                        before_example: "No certifications mentioned".to_string(),
                        after_example: "Certifications: CFA Level II Candidate, FRM Part I".to_string(),
                    };
                    suggestions.push(suggestion);
                }
            }
            _ => {}
        }

        // Skills section optimization
        if parsed_resume.skills.len() < 5 {
            let suggestion = OptimizationSuggestion {
                category: "Sections".to_string(),
                title: "Expand skills section".to_string(),
                description: "Add more relevant skills to improve keyword matching and demonstrate your capabilities.".to_string(),
                impact_score: 80.0,
                difficulty: "Easy".to_string(),
                specific_actions: vec![
                    SuggestionAction {
                        action: "Add 5-10 relevant technical and soft skills".to_string(),
                        section: "Skills".to_string(),
                        reasoning: "Comprehensive skills section improves ATS keyword matching".to_string(),
                    },
                    SuggestionAction {
                        action: "Organize skills into categories (Technical, Tools, Languages)".to_string(),
                        section: "Skills".to_string(),
                        reasoning: "Organized skills are easier for ATS systems to parse".to_string(),
                    },
                ],
                before_example: "Skills: Java, Python".to_string(),
                after_example: "Technical Skills: Java, Python, JavaScript, React, SQL, AWS, Git, Docker".to_string(),
            };
            suggestions.push(suggestion);
        }

        Ok(suggestions)
    }

    /// Generate content-related optimization suggestions
    fn generate_content_suggestions(
        &self,
        parsed_resume: &ParsedResume,
        _target_keywords: &[String],
        industry: &str,
    ) -> Result<Vec<OptimizationSuggestion>> {
        let mut suggestions = Vec::new();

        // Experience section improvements
        if parsed_resume.experience.is_empty() {
            let suggestion = OptimizationSuggestion {
                category: "Content".to_string(),
                title: "Add work experience".to_string(),
                description:
                    "Include your work experience with specific achievements and responsibilities."
                        .to_string(),
                impact_score: 95.0,
                difficulty: "Medium".to_string(),
                specific_actions: vec![SuggestionAction {
                    action: "Add work experience entries".to_string(),
                    section: "Experience".to_string(),
                    reasoning: "Experience section is crucial for ATS systems and recruiters"
                        .to_string(),
                }],
                before_example: "No experience section".to_string(),
                after_example: "Experience: Software Engineer at Tech Corp (2020-2023)".to_string(),
            };
            suggestions.push(suggestion);
        } else {
            // Check for achievements in experience
            let has_achievements = parsed_resume
                .experience
                .iter()
                .any(|exp| !exp.achievements.is_empty());
            if !has_achievements {
                let suggestion = OptimizationSuggestion {
                    category: "Content".to_string(),
                    title: "Add quantified achievements".to_string(),
                    description: "Include specific, measurable achievements in your work experience to demonstrate impact.".to_string(),
                    impact_score: 88.0,
                    difficulty: "Medium".to_string(),
                    specific_actions: vec![
                        SuggestionAction {
                            action: "Add 2-3 bullet points with quantified results for each role".to_string(),
                            section: "Experience".to_string(),
                            reasoning: "Quantified achievements demonstrate concrete value and impact".to_string(),
                        },
                        SuggestionAction {
                            action: "Use action verbs and include numbers, percentages, or metrics".to_string(),
                            section: "Experience".to_string(),
                            reasoning: "Action verbs and metrics make achievements more compelling".to_string(),
                        },
                    ],
                    before_example: "Worked on software development projects".to_string(),
                    after_example: "• Developed 5 web applications using React and Node.js, increasing user engagement by 25%".to_string(),
                };
                suggestions.push(suggestion);
            }
        }

        // Education section improvements
        if parsed_resume.education.is_empty() {
            let suggestion = OptimizationSuggestion {
                category: "Content".to_string(),
                title: "Add education information".to_string(),
                description: "Include your educational background, which is important for ATS systems and recruiters.".to_string(),
                impact_score: 75.0,
                difficulty: "Easy".to_string(),
                specific_actions: vec![
                    SuggestionAction {
                        action: "Add degree, institution, and graduation year".to_string(),
                        section: "Education".to_string(),
                        reasoning: "Education section is required by most ATS systems".to_string(),
                    },
                ],
                before_example: "No education section".to_string(),
                after_example: "Education: Bachelor of Science in Computer Science, University of Technology, 2020".to_string(),
            };
            suggestions.push(suggestion);
        }

        // Industry-specific content suggestions
        if industry == "technology" {
            let resume_text = self.get_resume_text(parsed_resume);
            if !resume_text.to_lowercase().contains("github")
                && !resume_text.to_lowercase().contains("portfolio")
            {
                let suggestion = OptimizationSuggestion {
                    category: "Content".to_string(),
                    title: "Add GitHub/portfolio link".to_string(),
                    description: "Include links to your GitHub profile or portfolio to showcase your technical work.".to_string(),
                    impact_score: 70.0,
                    difficulty: "Easy".to_string(),
                    specific_actions: vec![
                        SuggestionAction {
                            action: "Add GitHub profile link to contact information".to_string(),
                            section: "Contact".to_string(),
                            reasoning: "GitHub profile demonstrates coding skills and project experience".to_string(),
                        },
                    ],
                    before_example: "Contact: email@example.com, (555) 123-4567".to_string(),
                    after_example: "Contact: email@example.com, (555) 123-4567, github.com/username".to_string(),
                };
                suggestions.push(suggestion);
            }
        }

        Ok(suggestions)
    }

    /// Generate ATS-specific optimization suggestions
    fn generate_ats_suggestions(
        &self,
        _parsed_resume: &ParsedResume,
        format_analysis: &FormatAnalysis,
    ) -> Result<Vec<OptimizationSuggestion>> {
        let mut suggestions = Vec::new();

        // File format suggestion
        let suggestion = OptimizationSuggestion {
            category: "ATS".to_string(),
            title: "Use PDF or Word format".to_string(),
            description: "Save your resume as PDF or Word document for best ATS compatibility."
                .to_string(),
            impact_score: 85.0,
            difficulty: "Easy".to_string(),
            specific_actions: vec![SuggestionAction {
                action: "Save resume as PDF (preferred) or Word document".to_string(),
                section: "Format".to_string(),
                reasoning: "PDF preserves formatting while remaining ATS-readable".to_string(),
            }],
            before_example: "Resume saved as image or uncommon format".to_string(),
            after_example: "Resume saved as PDF with proper text encoding".to_string(),
        };
        suggestions.push(suggestion);

        // Parsing issues suggestions
        if !format_analysis.parsing_issues.is_empty() {
            let suggestion = OptimizationSuggestion {
                category: "ATS".to_string(),
                title: "Fix parsing issues".to_string(),
                description: "Address formatting issues that may prevent ATS systems from properly reading your resume.".to_string(),
                impact_score: 90.0,
                difficulty: "Medium".to_string(),
                specific_actions: vec![
                    SuggestionAction {
                        action: "Remove headers, footers, and complex formatting elements".to_string(),
                        section: "Format".to_string(),
                        reasoning: "Simple formatting ensures reliable ATS parsing".to_string(),
                    },
                    SuggestionAction {
                        action: "Use standard bullet points instead of custom symbols".to_string(),
                        section: "Format".to_string(),
                        reasoning: "Standard bullet points are universally recognized".to_string(),
                    },
                ],
                before_example: "Using complex formatting with headers/footers".to_string(),
                after_example: "Clean, simple formatting with standard elements".to_string(),
            };
            suggestions.push(suggestion);
        }

        Ok(suggestions)
    }

    /// Find missing keywords by comparing resume content with target keywords
    fn find_missing_keywords(
        &self,
        resume_text: &str,
        target_keywords: &[String],
        industry_keywords: &HashMap<String, f64>,
    ) -> Vec<(String, f64)> {
        let mut missing_keywords = Vec::new();
        let resume_lower = resume_text.to_lowercase();

        // Check target keywords from job description
        for keyword in target_keywords {
            let keyword_lower = keyword.to_lowercase();
            if !resume_lower.contains(&keyword_lower) {
                let importance = industry_keywords.get(keyword).unwrap_or(&1.0);
                missing_keywords.push((keyword.clone(), *importance));
            }
        }

        // Check high-value industry keywords
        for (keyword, importance) in industry_keywords {
            if *importance > 2.0 && !resume_lower.contains(&keyword.to_lowercase()) {
                // Check if it's already in missing keywords
                if !missing_keywords.iter().any(|(k, _)| k == keyword) {
                    missing_keywords.push((keyword.clone(), *importance));
                }
            }
        }

        // Sort by importance
        missing_keywords.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        missing_keywords
    }

    /// Get all resume text for analysis
    fn get_resume_text(&self, parsed_resume: &ParsedResume) -> String {
        let mut text = String::new();

        // Add sections
        for section_content in parsed_resume.sections.values() {
            text.push_str(section_content);
            text.push(' ');
        }

        // Add experience
        for exp in &parsed_resume.experience {
            text.push_str(&exp.title);
            text.push(' ');
            text.push_str(&exp.company);
            text.push(' ');
            text.push_str(&exp.description);
            text.push(' ');
            for achievement in &exp.achievements {
                text.push_str(achievement);
                text.push(' ');
            }
        }

        // Add education
        for edu in &parsed_resume.education {
            text.push_str(&edu.degree);
            text.push(' ');
            text.push_str(&edu.institution);
            text.push(' ');
        }

        // Add skills
        for skill in &parsed_resume.skills {
            text.push_str(skill);
            text.push(' ');
        }

        text
    }
}

impl Default for KeywordAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl KeywordAnalyzer {
    pub fn new() -> Self {
        Self {
            exact_matcher: ExactMatcher,
            stemmed_matcher: StemmedMatcher,
            contextual_matcher: ContextualMatcher,
            synonym_matcher: SynonymMatcher,
        }
    }

    pub async fn analyze_comprehensive(
        &self,
        resume_content: &str,
        job_description: &str,
        industry: &str,
    ) -> Result<KeywordMatch> {
        debug!(
            "Starting comprehensive keyword analysis for {} industry",
            industry
        );

        // Extract keywords from job description
        let target_keywords = self.extract_keywords_from_job_description(job_description)?;

        // Perform different types of matching
        let exact_matches = self
            .exact_matcher
            .find_matches(resume_content, &target_keywords)?;
        let stemmed_matches = self
            .stemmed_matcher
            .find_matches(resume_content, &target_keywords)?;
        let contextual_matches = self
            .contextual_matcher
            .find_matches(resume_content, &target_keywords)?;
        let synonym_matches = self
            .synonym_matcher
            .find_matches(resume_content, &target_keywords)?;

        // Calculate overall score
        let overall_score = self.calculate_overall_keyword_score(
            &exact_matches,
            &stemmed_matches,
            &contextual_matches,
            &synonym_matches,
        )?;

        // Calculate match density
        let match_density =
            self.calculate_match_density(resume_content, &exact_matches, &stemmed_matches)?;

        // Calculate section distribution
        let section_distribution =
            self.calculate_section_distribution(&exact_matches, &stemmed_matches)?;

        Ok(KeywordMatch {
            exact_matches,
            stemmed_matches,
            contextual_matches,
            synonym_matches,
            overall_score,
            match_density,
            section_distribution,
        })
    }

    pub fn extract_keywords_from_job_description(
        &self,
        job_description: &str,
    ) -> Result<Vec<String>> {
        let mut keywords = Vec::new();

        // Normalize the job description
        let normalized = job_description.nfc().collect::<String>();
        let text_lower = normalized.to_lowercase();

        // Extract different types of keywords
        keywords.extend(self.extract_technical_skills(&text_lower));
        keywords.extend(self.extract_soft_skills(&text_lower));
        keywords.extend(self.extract_tools_and_technologies(&text_lower));
        keywords.extend(self.extract_industry_terms(&text_lower));
        keywords.extend(self.extract_experience_requirements(&text_lower));
        keywords.extend(self.extract_education_requirements(&text_lower));
        keywords.extend(self.extract_certification_requirements(&text_lower));
        keywords.extend(self.extract_business_keywords(&text_lower));

        // Remove duplicates and sort
        keywords.sort();
        keywords.dedup();

        // Filter out noise words and very short/long terms
        let filtered_keywords: Vec<String> = keywords
            .into_iter()
            .filter(|word| {
                word.len() >= 2
                    && word.len() <= 50
                    && !self.is_noise_word(word)
                    && !self.is_common_word(word)
            })
            .collect();

        Ok(filtered_keywords)
    }

    /// Extract technical skills from job description
    fn extract_technical_skills(&self, text: &str) -> Vec<String> {
        let mut skills = Vec::new();

        // Programming languages
        let programming_languages = [
            "python",
            "java",
            "javascript",
            "typescript",
            "c++",
            "c#",
            "go",
            "rust",
            "swift",
            "kotlin",
            "scala",
            "ruby",
            "php",
            "perl",
            "r",
            "matlab",
            "sql",
            "html",
            "css",
            "react",
            "angular",
            "vue",
            "node.js",
            "django",
            "flask",
            "spring",
            "express",
        ];

        for lang in &programming_languages {
            if text.contains(lang) {
                skills.push(lang.to_string());
            }
        }

        // Frameworks and libraries
        let frameworks = [
            "tensorflow",
            "pytorch",
            "scikit-learn",
            "pandas",
            "numpy",
            "matplotlib",
            "bootstrap",
            "jquery",
            "d3.js",
            "three.js",
            "webpack",
            "babel",
            "redux",
            "graphql",
            "rest api",
            "microservices",
            "kubernetes",
            "docker",
            "jenkins",
        ];

        for framework in &frameworks {
            if text.contains(framework) {
                skills.push(framework.to_string());
            }
        }

        // Cloud and DevOps
        let cloud_devops = [
            "aws",
            "azure",
            "gcp",
            "google cloud",
            "amazon web services",
            "ci/cd",
            "devops",
            "infrastructure",
            "terraform",
            "ansible",
            "puppet",
            "chef",
        ];

        for tool in &cloud_devops {
            if text.contains(tool) {
                skills.push(tool.to_string());
            }
        }

        skills
    }

    /// Extract soft skills from job description
    fn extract_soft_skills(&self, text: &str) -> Vec<String> {
        let mut skills = Vec::new();

        let soft_skills = [
            "leadership",
            "communication",
            "teamwork",
            "problem solving",
            "analytical",
            "creative",
            "innovative",
            "adaptable",
            "flexible",
            "detail-oriented",
            "organized",
            "time management",
            "project management",
            "collaboration",
            "mentoring",
            "coaching",
            "presentation",
            "negotiation",
            "customer service",
        ];

        for skill in &soft_skills {
            if text.contains(skill) {
                skills.push(skill.to_string());
            }
        }

        skills
    }

    /// Extract tools and technologies
    fn extract_tools_and_technologies(&self, text: &str) -> Vec<String> {
        let mut tools = Vec::new();

        let technologies = [
            "git",
            "github",
            "gitlab",
            "bitbucket",
            "jira",
            "confluence",
            "slack",
            "microsoft office",
            "excel",
            "powerpoint",
            "word",
            "outlook",
            "teams",
            "zoom",
            "figma",
            "sketch",
            "adobe",
            "photoshop",
            "illustrator",
            "indesign",
            "salesforce",
            "hubspot",
            "tableau",
            "power bi",
            "google analytics",
            "mysql",
            "postgresql",
            "mongodb",
            "redis",
            "elasticsearch",
            "cassandra",
        ];

        for tool in &technologies {
            if text.contains(tool) {
                tools.push(tool.to_string());
            }
        }

        tools
    }

    /// Extract industry-specific terms
    fn extract_industry_terms(&self, text: &str) -> Vec<String> {
        let mut terms = Vec::new();

        // Tech industry terms
        let tech_terms = [
            "agile",
            "scrum",
            "kanban",
            "sprint",
            "api",
            "sdk",
            "ui/ux",
            "frontend",
            "backend",
            "full stack",
            "machine learning",
            "artificial intelligence",
            "data science",
            "big data",
            "analytics",
            "blockchain",
            "cybersecurity",
            "mobile development",
            "web development",
            "software engineering",
        ];

        // Finance industry terms
        let finance_terms = [
            "financial modeling",
            "risk management",
            "portfolio management",
            "trading",
            "investment",
            "banking",
            "fintech",
            "compliance",
            "audit",
            "accounting",
            "budgeting",
            "forecasting",
            "valuation",
            "derivatives",
            "equity",
            "bonds",
        ];

        // Healthcare industry terms
        let healthcare_terms = [
            "healthcare",
            "medical",
            "clinical",
            "patient care",
            "hipaa",
            "ehr",
            "emr",
            "telemedicine",
            "pharmaceutical",
            "biotechnology",
            "medical device",
            "regulatory",
            "fda",
            "clinical trials",
            "healthcare analytics",
        ];

        // Marketing industry terms
        let marketing_terms = [
            "digital marketing",
            "seo",
            "sem",
            "social media",
            "content marketing",
            "email marketing",
            "marketing automation",
            "crm",
            "lead generation",
            "conversion optimization",
            "a/b testing",
            "google ads",
            "facebook ads",
            "influencer marketing",
            "brand management",
            "public relations",
        ];

        let all_terms = [
            tech_terms.as_ref(),
            finance_terms.as_ref(),
            healthcare_terms.as_ref(),
            marketing_terms.as_ref(),
        ]
        .concat();

        for term in &all_terms {
            if text.contains(term) {
                terms.push(term.to_string());
            }
        }

        terms
    }

    /// Extract experience requirements
    fn extract_experience_requirements(&self, text: &str) -> Vec<String> {
        let mut requirements = Vec::new();

        // Look for experience patterns
        let experience_patterns = [
            r"\d+\+?\s*years?\s*(?:of\s*)?experience",
            r"senior\s+(?:level|position|role)",
            r"junior\s+(?:level|position|role)",
            r"mid\s*(?:level|position|role)",
            r"entry\s*(?:level|position|role)",
            r"lead\s+(?:developer|engineer|analyst)",
            r"principal\s+(?:developer|engineer|analyst)",
            r"staff\s+(?:developer|engineer|analyst)",
        ];

        for pattern in &experience_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                for mat in regex.find_iter(text) {
                    requirements.push(mat.as_str().to_string());
                }
            }
        }

        requirements
    }

    /// Extract education requirements
    fn extract_education_requirements(&self, text: &str) -> Vec<String> {
        let mut requirements = Vec::new();

        let education_terms = [
            "bachelor",
            "master",
            "phd",
            "doctorate",
            "degree",
            "computer science",
            "engineering",
            "mathematics",
            "statistics",
            "business",
            "mba",
            "information technology",
            "information systems",
            "data science",
        ];

        for term in &education_terms {
            if text.contains(term) {
                requirements.push(term.to_string());
            }
        }

        requirements
    }

    /// Extract certification requirements
    fn extract_certification_requirements(&self, text: &str) -> Vec<String> {
        let mut certifications = Vec::new();

        let cert_terms = [
            "certification",
            "certified",
            "aws certified",
            "azure certified",
            "google cloud certified",
            "pmp",
            "cissp",
            "cisa",
            "cism",
            "comptia",
            "ccna",
            "ccnp",
            "mcse",
            "oracle certified",
            "salesforce certified",
            "scrum master",
            "agile certified",
            "six sigma",
            "itil",
        ];

        for cert in &cert_terms {
            if text.contains(cert) {
                certifications.push(cert.to_string());
            }
        }

        certifications
    }

    /// Extract business-related keywords
    fn extract_business_keywords(&self, text: &str) -> Vec<String> {
        let mut keywords = Vec::new();

        let business_terms = [
            "revenue",
            "profit",
            "growth",
            "roi",
            "kpi",
            "metrics",
            "performance",
            "strategy",
            "planning",
            "execution",
            "operations",
            "process improvement",
            "efficiency",
            "optimization",
            "scalability",
            "innovation",
            "transformation",
            "stakeholder",
            "customer",
            "client",
            "vendor",
            "partnership",
            "negotiation",
        ];

        for term in &business_terms {
            if text.contains(term) {
                keywords.push(term.to_string());
            }
        }

        keywords
    }

    /// Check if a word is noise (should be filtered out)
    fn is_noise_word(&self, word: &str) -> bool {
        let noise_words = [
            "the", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with", "by", "from",
            "up", "about", "into", "through", "during", "before", "after", "above", "below",
            "between", "among", "under", "over", "is", "are", "was", "were", "be", "been", "being",
            "have", "has", "had", "do", "does", "did", "will", "would", "could", "should", "may",
            "might", "must", "shall", "can", "this", "that", "these", "those", "a", "an",
        ];

        noise_words.contains(&word)
    }

    /// Check if a word is too common to be valuable
    fn is_common_word(&self, word: &str) -> bool {
        let common_words = [
            "work", "job", "position", "role", "company", "team", "people", "time", "day", "year",
            "way", "use", "make", "get", "know", "think", "see", "come", "take", "want", "look",
            "good", "new", "first", "last", "long", "great", "little", "own", "other", "old",
            "right", "big", "high", "small",
        ];

        common_words.contains(&word)
    }

    fn calculate_overall_keyword_score(
        &self,
        exact_matches: &[MatchResult],
        stemmed_matches: &[MatchResult],
        contextual_matches: &[MatchResult],
        synonym_matches: &[MatchResult],
    ) -> Result<f64> {
        let exact_score = exact_matches.len() as f64 * 1.0;
        let stemmed_score = stemmed_matches.len() as f64 * 0.85;
        let contextual_score = contextual_matches.len() as f64 * 0.6;
        let synonym_score = synonym_matches.len() as f64 * 0.7;

        let total_score = exact_score + stemmed_score + contextual_score + synonym_score;
        let max_possible = 20.0; // Assume 20 keywords max

        Ok((total_score / max_possible * 100.0).min(100.0))
    }

    fn calculate_match_density(
        &self,
        resume_content: &str,
        exact_matches: &[MatchResult],
        stemmed_matches: &[MatchResult],
    ) -> Result<f64> {
        let word_count = resume_content.split_whitespace().count();
        let match_count = exact_matches.len() + stemmed_matches.len();

        if word_count == 0 {
            return Ok(0.0);
        }

        Ok((match_count as f64 / word_count as f64) * 100.0)
    }

    fn calculate_section_distribution(
        &self,
        exact_matches: &[MatchResult],
        stemmed_matches: &[MatchResult],
    ) -> Result<HashMap<String, f64>> {
        let mut distribution = HashMap::new();
        let total_matches = exact_matches.len() + stemmed_matches.len();

        if total_matches == 0 {
            return Ok(distribution);
        }

        for match_result in exact_matches.iter().chain(stemmed_matches.iter()) {
            let count = distribution
                .entry(match_result.section.clone())
                .or_insert(0.0);
            *count += 1.0;
        }

        // Convert to percentages
        for (_, count) in distribution.iter_mut() {
            *count = (*count / total_matches as f64) * 100.0;
        }

        Ok(distribution)
    }
}

impl Default for ATSSimulator {
    fn default() -> Self {
        Self::new()
    }
}

impl ATSSimulator {
    pub fn new() -> Self {
        let parsers: Vec<Box<dyn ATSParser + Send + Sync>> = vec![
            Box::new(WorkdayParser::new()),
            Box::new(TaleoParser::new()),
            Box::new(GenericParser::new()),
        ];

        let format_rules = vec![FormatRule {
            rule_type: "font_compatibility".to_string(),
            severity: IssueSeverity::Medium,
            validator: |content: &str| !content.contains("Wingdings"),
            description: "Avoid decorative fonts".to_string(),
        }];

        let section_detectors = vec![SectionDetector {
            section_name: "experience".to_string(),
            patterns: vec![Regex::new(
                r"(?i)(work\s+experience|experience|employment|professional)",
            )
            .unwrap()],
            importance: 1.0,
        }];

        Self {
            parsers,
            format_rules,
            section_detectors,
        }
    }

    pub fn parse_with_multiple_systems(&self, resume_content: &str) -> Result<ParsedResume> {
        // Use the first parser for now - in real implementation, would aggregate results
        if let Some(parser) = self.parsers.first() {
            parser.parse_resume(resume_content)
        } else {
            Err(anyhow!("No ATS parsers available"))
        }
    }

    pub fn calculate_compatibility_scores(
        &self,
        parsed_resume: &ParsedResume,
    ) -> Result<HashMap<ATSSystem, f64>> {
        let mut scores = HashMap::new();

        for parser in &self.parsers {
            let score = parser.get_compatibility_score(parsed_resume);
            scores.insert(parser.get_system_type(), score);
        }

        Ok(scores)
    }
}

impl Default for FormatAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze_comprehensive(
        &self,
        resume_content: &str,
        parsed_resume: &ParsedResume,
    ) -> Result<FormatAnalysis> {
        let ats_compatibility_score = self.calculate_ats_compatibility(resume_content)?;
        let parsing_issues = self.detect_parsing_issues(resume_content)?;
        let section_detection_score = parsed_resume.parsing_confidence;
        let font_compatibility = self.analyze_font_compatibility(resume_content)?;
        let layout_score = self.analyze_layout(resume_content)?;
        let encoding_issues = self.detect_encoding_issues(resume_content)?;

        Ok(FormatAnalysis {
            ats_compatibility_score,
            parsing_issues,
            section_detection_score,
            font_compatibility,
            layout_score,
            encoding_issues,
        })
    }

    fn calculate_ats_compatibility(&self, resume_content: &str) -> Result<f64> {
        let mut compatibility_score = 100.0;

        // Check for ATS-unfriendly formatting elements
        let problematic_patterns = [
            (
                r"[│║┌┐└┘├┤┬┴┼─━]",
                15.0,
                "Table borders and special characters",
            ),
            (r"[★☆●○▪▫■□▲△▼▽◆◇]", 10.0, "Special symbols and bullets"),
            (r"[①②③④⑤⑥⑦⑧⑨⑩]", 8.0, "Numbered circles"),
            (r"[➤➢➣➤➥➦➧➨➩]", 8.0, "Arrow symbols"),
            (r"[✓✔✗✘]", 5.0, "Checkmarks and crosses"),
            (
                r"@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}",
                0.0,
                "Email addresses (good)",
            ),
            (r"\(\d{3}\)\s?\d{3}-?\d{4}", 0.0, "Phone numbers (good)"),
        ];

        for (pattern, penalty, description) in &problematic_patterns {
            let regex = Regex::new(pattern)?;
            let match_count = regex.find_iter(resume_content).count();
            if match_count > 0 && *penalty > 0.0 {
                compatibility_score -= (match_count as f64 * penalty).min(penalty * 2.0);
                debug!(
                    "ATS compatibility penalty: {} for {} matches of {}",
                    penalty, match_count, description
                );
            }
        }

        // Check for proper section structure
        let section_headers = [
            "experience",
            "work experience",
            "professional experience",
            "employment",
            "education",
            "academic background",
            "qualifications",
            "skills",
            "technical skills",
            "core competencies",
            "expertise",
            "summary",
            "profile",
            "objective",
            "about",
        ];

        let mut found_sections = 0;
        for header in &section_headers {
            if resume_content.to_lowercase().contains(header) {
                found_sections += 1;
            }
        }

        if found_sections < 3 {
            compatibility_score -= 20.0;
        } else if found_sections >= 4 {
            compatibility_score += 5.0;
        }

        // Check for consistent formatting
        let bullet_patterns = [
            r"^[\s]*[•·▪▫■□▲△▼▽◆◇]", // Unicode bullets
            r"^[\s]*[-*+]",          // ASCII bullets
            r"^[\s]*\d+\.",          // Numbered lists
        ];

        let mut bullet_consistency = 0;
        for pattern in &bullet_patterns {
            let regex = Regex::new(pattern)?;
            let matches = regex.find_iter(resume_content).count();
            if matches > 0 {
                bullet_consistency += 1;
            }
        }

        if bullet_consistency > 2 {
            compatibility_score -= 10.0; // Inconsistent bullet usage
        }

        // Check for proper contact information placement
        let lines: Vec<&str> = resume_content.lines().collect();
        let first_section: String = lines
            .iter()
            .take(10)
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        let email_regex = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b")?;
        let phone_regex = Regex::new(r"(\+?1[-.\s]?)?(\(?\d{3}\)?[-.\s]?\d{3}[-.\s]?\d{4})")?;

        if !email_regex.is_match(&first_section) {
            compatibility_score -= 10.0;
        }
        if !phone_regex.is_match(&first_section) {
            compatibility_score -= 5.0;
        }

        // Check for excessive formatting
        let formatting_indicators = [
            r"<[^>]+>",    // HTML tags
            r"\{[^}]+\}",  // Curly braces
            r"\[[^\]]+\]", // Square brackets (except normal usage)
        ];

        for pattern in &formatting_indicators {
            let regex = Regex::new(pattern)?;
            let matches = regex.find_iter(resume_content).count();
            if matches > 3 {
                compatibility_score -= 5.0;
            }
        }

        // Check for reasonable line lengths
        let long_lines = lines.iter().filter(|line| line.len() > 150).count();
        if long_lines > lines.len() / 5 {
            compatibility_score -= 10.0;
        }

        // Check for proper date formats
        let date_patterns = [
            r"\b\d{1,2}/\d{1,2}/\d{2,4}\b", // MM/DD/YYYY
            r"\b\d{1,2}-\d{1,2}-\d{2,4}\b", // MM-DD-YYYY
            r"\b(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)\s+\d{4}\b", // Month YYYY
            r"\b\d{4}\s*-\s*\d{4}\b",       // YYYY - YYYY
        ];

        let mut date_consistency = 0;
        for pattern in &date_patterns {
            let regex = Regex::new(pattern)?;
            if regex.is_match(resume_content) {
                date_consistency += 1;
            }
        }

        if date_consistency > 2 {
            compatibility_score -= 5.0; // Inconsistent date formatting
        }

        Ok(compatibility_score.clamp(0.0, 100.0))
    }

    fn detect_parsing_issues(&self, resume_content: &str) -> Result<Vec<FormatIssue>> {
        let mut issues = Vec::new();

        // Check for multi-column layout issues
        let lines: Vec<&str> = resume_content.lines().collect();
        let mut potential_column_issues = 0;

        for line in &lines {
            // Look for excessive whitespace that might indicate columns
            let tab_count = line.matches('\t').count();
            let space_groups = line.split_whitespace().count();

            if tab_count > 5 || (line.len() > 50 && space_groups < 5) {
                potential_column_issues += 1;
            }
        }

        if potential_column_issues > lines.len() / 10 {
            issues.push(FormatIssue {
                issue_type: FormatIssueType::LayoutProblem,
                description:
                    "Resume appears to use a multi-column layout which may cause parsing issues"
                        .to_string(),
                severity: IssueSeverity::High,
                location: "Layout structure".to_string(),
                fix_suggestion: "Convert to single-column layout for better ATS compatibility"
                    .to_string(),
                ats_impact: 20.0,
            });
        }

        // Check for header/footer issues
        let header_footer_indicators = [
            r"page \d+ of \d+",
            r"confidential",
            r"resume of",
            r"curriculum vitae",
        ];

        for pattern in &header_footer_indicators {
            let regex = Regex::new(pattern)?;
            if regex.is_match(&resume_content.to_lowercase()) {
                issues.push(FormatIssue {
                    issue_type: FormatIssueType::ParsingError,
                    description:
                        "Resume contains header or footer content that may interfere with parsing"
                            .to_string(),
                    severity: IssueSeverity::Medium,
                    location: "Header/Footer sections".to_string(),
                    fix_suggestion: "Remove headers and footers, keep only main content"
                        .to_string(),
                    ats_impact: 15.0,
                });
                break;
            }
        }

        // Check for table structures
        let table_indicators = [r"[│║┌┐└┘├┤┬┴┼─━]", r"\|[^\|]*\|[^\|]*\|", r"_{3,}"];

        for pattern in &table_indicators {
            let regex = Regex::new(pattern)?;
            if regex.is_match(resume_content) {
                issues.push(FormatIssue {
                    issue_type: FormatIssueType::TableFormatting,
                    description: "Resume contains table structures that may not parse correctly"
                        .to_string(),
                    severity: IssueSeverity::High,
                    location: "Table structures".to_string(),
                    fix_suggestion: "Convert tables to simple lists with clear formatting"
                        .to_string(),
                    ats_impact: 18.0,
                });
                break;
            }
        }

        // Check for text boxes and graphics
        let graphics_indicators = [
            r"\[image\]",
            r"\[graphic\]",
            r"\[logo\]",
            r"█",
            r"▓",
            r"▒",
            r"░",
        ];

        for pattern in &graphics_indicators {
            let regex = Regex::new(pattern)?;
            if regex.is_match(resume_content) {
                issues.push(FormatIssue {
                    issue_type: FormatIssueType::ImageText,
                    description: "Resume contains graphics or images that cannot be parsed by ATS"
                        .to_string(),
                    severity: IssueSeverity::Critical,
                    location: "Graphics/Images".to_string(),
                    fix_suggestion: "Remove all graphics and images, use text-only format"
                        .to_string(),
                    ats_impact: 30.0,
                });
                break;
            }
        }

        // Check for unusual spacing patterns
        let mut excessive_spacing = 0;
        for line in &lines {
            let consecutive_spaces = line.matches("  ").count();
            if consecutive_spaces > 5 {
                excessive_spacing += 1;
            }
        }

        if excessive_spacing > lines.len() / 20 {
            issues.push(FormatIssue {
                issue_type: FormatIssueType::LayoutProblem,
                description: "Resume has excessive spacing that may indicate formatting issues"
                    .to_string(),
                severity: IssueSeverity::Medium,
                location: "Spacing throughout document".to_string(),
                fix_suggestion: "Use consistent, minimal spacing between elements".to_string(),
                ats_impact: 10.0,
            });
        }

        // Check for mixed bullet styles
        let bullet_styles = [
            r"^[\s]*[•·▪▫■□▲△▼▽◆◇]",
            r"^[\s]*[-*+]",
            r"^[\s]*\d+\.",
            r"^[\s]*[a-zA-Z]\)",
        ];

        let mut bullet_style_count = 0;
        for pattern in &bullet_styles {
            let regex = Regex::new(pattern)?;
            if regex.is_match(resume_content) {
                bullet_style_count += 1;
            }
        }

        if bullet_style_count > 2 {
            issues.push(FormatIssue {
                issue_type: FormatIssueType::SpecialCharacters,
                description: "Resume uses multiple bullet styles which may confuse ATS parsing"
                    .to_string(),
                severity: IssueSeverity::Medium,
                location: "Bullet points throughout document".to_string(),
                fix_suggestion:
                    "Use consistent bullet style throughout (preferably simple dashes or bullets)"
                        .to_string(),
                ats_impact: 8.0,
            });
        }

        // Check for special characters that might not render properly
        let problematic_chars = [
            r"[\u{201C}\u{201D}\u{2018}\u{2019}`´]", // Smart quotes
            r"[\u{2013}\u{2014}]",                   // Em/en dashes
            r"[\u{2026}]",                           // Ellipsis
            r"[\u{00A9}\u{00AE}\u{2122}]",           // Copyright symbols
        ];

        for pattern in &problematic_chars {
            let regex = Regex::new(pattern)?;
            if regex.is_match(resume_content) {
                issues.push(FormatIssue {
                    issue_type: FormatIssueType::SpecialCharacters,
                    description: "Resume contains special characters that may not display correctly in all ATS systems".to_string(),
                    severity: IssueSeverity::Low,
                    location: "Multiple locations".to_string(),
                    fix_suggestion: "Replace smart quotes with regular quotes, use standard punctuation".to_string(),
                    ats_impact: 5.0,
                });
                break;
            }
        }

        // Check for very long lines that might wrap poorly
        let long_lines = lines.iter().filter(|line| line.len() > 100).count();
        if long_lines > lines.len() / 5 {
            issues.push(FormatIssue {
                issue_type: FormatIssueType::LayoutProblem,
                description: "Resume has many long lines that may wrap poorly in ATS systems"
                    .to_string(),
                severity: IssueSeverity::Medium,
                location: "Multiple text sections".to_string(),
                fix_suggestion: "Break long lines into shorter, more readable segments".to_string(),
                ats_impact: 10.0,
            });
        }

        // Check for missing section breaks
        let section_breaks = resume_content.matches("\n\n").count();
        if section_breaks < 3 {
            issues.push(FormatIssue {
                issue_type: FormatIssueType::SectionDetectionFail,
                description:
                    "Resume lacks clear section breaks which may make it difficult to parse"
                        .to_string(),
                severity: IssueSeverity::Medium,
                location: "Section breaks".to_string(),
                fix_suggestion: "Add clear spacing between sections (double line breaks)"
                    .to_string(),
                ats_impact: 15.0,
            });
        }

        Ok(issues)
    }

    fn analyze_font_compatibility(&self, resume_content: &str) -> Result<f64> {
        let mut compatibility_score: f64 = 100.0;

        // Check for basic font compatibility indicators
        let content_lower = resume_content.to_lowercase();

        // Check for font-specific indicators in the content
        if content_lower.contains("wingdings")
            || content_lower.contains("symbol")
            || content_lower.contains("webdings")
        {
            compatibility_score -= 20.0;
        }

        if content_lower.contains("comic sans")
            || content_lower.contains("papyrus")
            || content_lower.contains("brush script")
        {
            compatibility_score -= 15.0;
        }

        if content_lower.contains("courier new") {
            compatibility_score -= 5.0; // Monospace can be problematic
        }

        if content_lower.contains("times new roman") {
            compatibility_score += 5.0; // Standard, good font
        }

        // Check for excessive ALL CAPS which might indicate font styling
        let words: Vec<&str> = resume_content.split_whitespace().collect();
        let caps_words = words
            .iter()
            .filter(|word| {
                word.len() > 2 && word.chars().all(|c| c.is_uppercase() || !c.is_alphabetic())
            })
            .count();

        if caps_words > words.len() / 20 {
            compatibility_score -= 5.0;
        }

        // Check for smart quotes and special characters
        if resume_content.contains('"') || resume_content.contains('"') {
            compatibility_score -= 8.0;
        }

        if resume_content.contains('\u{2018}') || resume_content.contains('\u{2019}') {
            compatibility_score -= 5.0;
        }

        if resume_content.contains('–') || resume_content.contains('—') {
            compatibility_score -= 5.0;
        }

        Ok(compatibility_score.clamp(0.0, 100.0))
    }

    fn analyze_layout(&self, resume_content: &str) -> Result<f64> {
        let mut layout_score: f64 = 100.0;
        let lines: Vec<&str> = resume_content.lines().collect();

        // Check for single-column layout (preferred for ATS)
        let mut potential_multi_column = 0;
        let mut excessive_tabs = 0;

        for line in &lines {
            // Count tabs and excessive spacing that might indicate columns
            let tab_count = line.matches('\t').count();
            let consecutive_spaces = line.matches("    ").count(); // 4+ spaces

            if tab_count > 3 || consecutive_spaces > 3 {
                potential_multi_column += 1;
            }

            if tab_count > 5 {
                excessive_tabs += 1;
            }
        }

        if potential_multi_column > lines.len() / 8 {
            layout_score -= 25.0; // Likely multi-column layout
        }

        if excessive_tabs > lines.len() / 10 {
            layout_score -= 15.0; // Excessive tab usage
        }

        // Check for consistent indentation
        let mut indent_patterns = HashMap::new();
        let _inconsistent_indents = 0;

        for line in &lines {
            if !line.trim().is_empty() {
                let leading_spaces = line.len() - line.trim_start().len();
                *indent_patterns.entry(leading_spaces).or_insert(0) += 1;
            }
        }

        // If there are too many different indentation levels, it may indicate poor structure
        if indent_patterns.len() > 6 {
            layout_score -= 10.0;
        }

        // Check for proper section spacing
        let mut section_breaks = 0;
        let mut previous_line_empty = false;

        for line in &lines {
            if line.trim().is_empty() {
                if !previous_line_empty {
                    section_breaks += 1;
                }
                previous_line_empty = true;
            } else {
                previous_line_empty = false;
            }
        }

        if section_breaks < 3 {
            layout_score -= 15.0; // Poor section separation
        } else if section_breaks > lines.len() / 3 {
            layout_score -= 10.0; // Too much whitespace
        }

        // Check for reasonable line lengths
        let mut line_length_distribution = [0; 5]; // 0-40, 41-80, 81-120, 121-160, 161+

        for line in &lines {
            if line.trim().is_empty() {
                continue;
            }

            let len = line.len();
            let bucket = match len {
                0..=40 => 0,
                41..=80 => 1,
                81..=120 => 2,
                121..=160 => 3,
                _ => 4,
            };
            line_length_distribution[bucket] += 1;
        }

        let total_content_lines = line_length_distribution.iter().sum::<i32>();
        if total_content_lines > 0 {
            // Too many very short lines (might indicate poor formatting)
            let short_line_ratio = line_length_distribution[0] as f64 / total_content_lines as f64;
            if short_line_ratio > 0.4 {
                layout_score -= 8.0;
            }

            // Too many very long lines (might wrap poorly)
            let long_line_ratio = line_length_distribution[4] as f64 / total_content_lines as f64;
            if long_line_ratio > 0.2 {
                layout_score -= 12.0;
            }
        }

        // Check for consistent bullet point alignment
        let mut bullet_count = 0;
        for line in &lines {
            if line.trim_start().starts_with('-')
                || line.trim_start().starts_with('*')
                || line.trim_start().starts_with('+')
            {
                bullet_count += 1;
            }
        }

        // If there are bullet points, that's good for ATS
        if bullet_count > 0 {
            layout_score += 5.0;
        }

        // Check for table-like structures (problematic for ATS)
        if resume_content.contains("___")
            || resume_content.contains("===")
            || resume_content.contains("|||")
        {
            layout_score -= 20.0;
        }

        // Check for centered text (might indicate poor ATS compatibility)
        let mut potentially_centered = 0;
        for line in &lines {
            if !line.trim().is_empty() {
                let leading_spaces = line.len() - line.trim_start().len();
                let _trailing_spaces = line.len() - line.trim_end().len();

                // If a line has significant leading spaces and the content is short, it might be centered
                if leading_spaces > 20 && line.trim().len() < 50 {
                    potentially_centered += 1;
                }
            }
        }

        if potentially_centered > lines.len() / 20 {
            layout_score -= 10.0;
        }

        // Check for proper header structure
        let mut header_lines = 0;
        let first_section = lines.iter().take(5).collect::<Vec<_>>();

        for line in &first_section {
            if !line.trim().is_empty() && line.trim().len() < 50 {
                // Likely header content (name, contact info, etc.)
                header_lines += 1;
            }
        }

        if header_lines < 2 {
            layout_score -= 8.0; // Poor header structure
        }

        // Check for footer content (problematic for ATS)
        let last_section = lines.iter().rev().take(3).collect::<Vec<_>>();
        let footer_indicators = ["page", "confidential", "references", "available"];

        for line in &last_section {
            let line_lower = line.to_lowercase();
            for indicator in &footer_indicators {
                if line_lower.contains(indicator) {
                    layout_score -= 10.0;
                    break;
                }
            }
        }

        // Check for consistent section headers
        let section_headers = [
            "experience",
            "education",
            "skills",
            "summary",
            "objective",
            "work",
            "professional",
            "technical",
            "qualifications",
            "achievements",
            "certifications",
            "projects",
        ];

        let mut header_formatting = HashMap::new();
        for line in &lines {
            let line_lower = line.to_lowercase();
            let line_lower_trimmed = line_lower.trim();
            for header in &section_headers {
                if line_lower_trimmed == *header || line_lower_trimmed == header.to_uppercase() {
                    // Analyze the formatting of this header
                    let formatting_key = (
                        line.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()),
                        line.len() - line.trim_start().len(), // Indentation
                        line.trim() != line_lower,            // Has mixed case
                    );
                    *header_formatting.entry(formatting_key).or_insert(0) += 1;
                }
            }
        }

        // If headers have inconsistent formatting, it may indicate poor structure
        if header_formatting.len() > 2 {
            layout_score -= 8.0;
        }

        // Check for proper spacing around sections
        let mut section_spacing_issues = 0;
        let mut in_section = false;
        let mut lines_since_header = 0;

        for line in &lines {
            let line_lower = line.to_lowercase();
            let line_lower_trimmed = line_lower.trim();
            let is_section_header = section_headers
                .iter()
                .any(|h| line_lower_trimmed == *h || line_lower_trimmed == h.to_uppercase());

            if is_section_header {
                if in_section && lines_since_header < 2 {
                    section_spacing_issues += 1; // Too little content under previous section
                }
                in_section = true;
                lines_since_header = 0;
            } else if !line.trim().is_empty() {
                lines_since_header += 1;
            }
        }

        if section_spacing_issues > 1 {
            layout_score -= 5.0;
        }

        Ok(layout_score.clamp(0.0, 100.0))
    }

    fn detect_encoding_issues(&self, resume_content: &str) -> Result<Vec<String>> {
        let mut issues = Vec::new();

        // Check for common encoding problems
        let problematic_sequences = [
            "\u{2019}", // Right single quotation mark (corrupted as â€™)
            "\u{201C}", // Left double quotation mark (corrupted as â€œ)
            "\u{201D}", // Right double quotation mark (corrupted as â€)
            "\u{2026}", // Horizontal ellipsis (corrupted as â€¦)
            "\u{2013}", // En dash (corrupted as â€")
            "\u{2014}", // Em dash (corrupted as â€")
            "\u{00A0}", // Non-breaking space (corrupted as Â )
            "\u{00C3}", // Latin capital letter A with tilde (corrupted as Ã)
            "\u{00A9}", // Copyright sign (corrupted as Â©)
            "\u{00AE}", // Registered sign (corrupted as Â®)
            "\u{2122}", // Trade mark sign (corrupted as Â™)
            "\u{20AC}", // Euro sign (corrupted as â‚¬)
            "\u{200B}", // Zero width space (corrupted as â€‹)
            "\u{FFFD}", // Replacement character (corrupted as ï¿½)
        ];

        for sequence in &problematic_sequences {
            if resume_content.contains(sequence) {
                issues.push(format!("Encoding issue detected: {}", sequence));
            }
        }

        // Check for mixed character encodings
        let mut has_latin1 = false;
        let mut has_utf8 = false;
        let mut has_windows1252 = false;

        for char in resume_content.chars() {
            match char as u32 {
                0x80..=0x9F => has_windows1252 = true, // Windows-1252 control characters
                0xA0..=0xFF => has_latin1 = true,      // Latin-1 supplement
                0x100..=0x17F => has_utf8 = true,      // Latin Extended-A
                0x2000..=0x206F => has_utf8 = true,    // General Punctuation
                0x20A0..=0x20CF => has_utf8 = true,    // Currency Symbols
                0x2100..=0x214F => has_utf8 = true,    // Letterlike Symbols
                _ => {}
            }
        }

        if has_latin1 && has_utf8 {
            issues.push("Mixed character encodings detected (Latin-1 and UTF-8)".to_string());
        }

        if has_windows1252 {
            issues.push(
                "Windows-1252 characters detected (may not display correctly on all systems)"
                    .to_string(),
            );
        }

        // Check for byte order marks (BOM)
        if resume_content.starts_with('\u{FEFF}') {
            issues.push("Byte Order Mark (BOM) detected at start of content".to_string());
        }

        // Check for null bytes (shouldn't be in text)
        if resume_content.contains('\0') {
            issues
                .push("Null bytes detected in text (possible binary data corruption)".to_string());
        }

        // Check for excessive non-ASCII characters
        let total_chars = resume_content.chars().count();
        let non_ascii_chars = resume_content.chars().filter(|c| !c.is_ascii()).count();

        if total_chars > 0 && non_ascii_chars as f64 / total_chars as f64 > 0.1 {
            issues.push(format!(
                "High percentage of non-ASCII characters ({}%)",
                (non_ascii_chars as f64 / total_chars as f64 * 100.0) as i32
            ));
        }

        // Check for problematic Unicode categories
        let mut control_chars = 0;
        let mut private_use_chars = 0;
        let mut surrogate_chars = 0;

        for char in resume_content.chars() {
            match char as u32 {
                0x00..=0x1F | 0x7F..=0x9F => control_chars += 1,
                0xE000..=0xF8FF | 0xF0000..=0xFFFFD | 0x100000..=0x10FFFD => private_use_chars += 1,
                0xD800..=0xDFFF => surrogate_chars += 1,
                _ => {}
            }
        }

        if control_chars > 0 {
            issues.push(format!(
                "Control characters detected ({} instances)",
                control_chars
            ));
        }

        if private_use_chars > 0 {
            issues.push(format!(
                "Private use Unicode characters detected ({} instances)",
                private_use_chars
            ));
        }

        if surrogate_chars > 0 {
            issues.push(format!(
                "Invalid Unicode surrogate characters detected ({} instances)",
                surrogate_chars
            ));
        }

        // Check for common smart quote issues
        if resume_content.contains('"') || resume_content.contains('"') {
            issues.push(
                "Smart double quotes detected (may not display correctly in all ATS systems)"
                    .to_string(),
            );
        }

        if resume_content.contains('\u{2018}') || resume_content.contains('\u{2019}') {
            issues.push(
                "Smart single quotes detected (may not display correctly in all ATS systems)"
                    .to_string(),
            );
        }

        if resume_content.contains('–') {
            issues.push(
                "En dash detected (may not display correctly in all ATS systems)".to_string(),
            );
        }

        if resume_content.contains('—') {
            issues.push(
                "Em dash detected (may not display correctly in all ATS systems)".to_string(),
            );
        }

        if resume_content.contains('…') {
            issues.push(
                "Horizontal ellipsis detected (may not display correctly in all ATS systems)"
                    .to_string(),
            );
        }

        // Check for invisible characters
        let invisible_chars = [
            ('\u{200B}', "Zero-width space"),
            ('\u{200C}', "Zero-width non-joiner"),
            ('\u{200D}', "Zero-width joiner"),
            ('\u{FEFF}', "Zero-width no-break space"),
            ('\u{2060}', "Word joiner"),
            ('\u{2061}', "Function application"),
            ('\u{2062}', "Invisible times"),
            ('\u{2063}', "Invisible separator"),
            ('\u{2064}', "Invisible plus"),
        ];

        for (char, description) in &invisible_chars {
            if resume_content.contains(*char) {
                issues.push(format!(
                    "Invisible character detected: {} (may cause parsing issues)",
                    description
                ));
            }
        }

        // Check for normalization issues
        let normalized_nfc = resume_content.nfc().collect::<String>();
        let normalized_nfd = resume_content.nfd().collect::<String>();

        if normalized_nfc != resume_content {
            issues.push("Text is not in NFC (Canonical Decomposition followed by Canonical Composition) form".to_string());
        }

        if normalized_nfc.len() != normalized_nfd.len() {
            issues.push(
                "Text contains composed characters that may not be handled consistently"
                    .to_string(),
            );
        }

        // Check for excessive whitespace variations
        let whitespace_chars = [
            ('\u{00A0}', "Non-breaking space"),
            ('\u{1680}', "Ogham space mark"),
            ('\u{2000}', "En quad"),
            ('\u{2001}', "Em quad"),
            ('\u{2002}', "En space"),
            ('\u{2003}', "Em space"),
            ('\u{2004}', "Three-per-em space"),
            ('\u{2005}', "Four-per-em space"),
            ('\u{2006}', "Six-per-em space"),
            ('\u{2007}', "Figure space"),
            ('\u{2008}', "Punctuation space"),
            ('\u{2009}', "Thin space"),
            ('\u{200A}', "Hair space"),
            ('\u{2028}', "Line separator"),
            ('\u{2029}', "Paragraph separator"),
            ('\u{202F}', "Narrow no-break space"),
            ('\u{205F}', "Medium mathematical space"),
            ('\u{3000}', "Ideographic space"),
        ];

        for (char, description) in &whitespace_chars {
            if resume_content.contains(*char) {
                issues.push(format!(
                    "Non-standard whitespace detected: {} (may cause parsing issues)",
                    description
                ));
            }
        }

        // Check for text direction issues
        let direction_chars = [
            ('\u{202A}', "Left-to-right embedding"),
            ('\u{202B}', "Right-to-left embedding"),
            ('\u{202C}', "Pop directional formatting"),
            ('\u{202D}', "Left-to-right override"),
            ('\u{202E}', "Right-to-left override"),
            ('\u{2066}', "Left-to-right isolate"),
            ('\u{2067}', "Right-to-left isolate"),
            ('\u{2068}', "First strong isolate"),
            ('\u{2069}', "Pop directional isolate"),
        ];

        for (char, description) in &direction_chars {
            if resume_content.contains(*char) {
                issues.push(format!(
                    "Text direction control character detected: {} (may cause display issues)",
                    description
                ));
            }
        }

        Ok(issues)
    }
}

// Default implementations for matchers
impl ExactMatcher {
    pub fn find_matches(
        &self,
        resume_content: &str,
        keywords: &[String],
    ) -> Result<Vec<MatchResult>> {
        let mut matches = Vec::new();

        for keyword in keywords {
            if let Some(pos) = resume_content.to_lowercase().find(&keyword.to_lowercase()) {
                matches.push(MatchResult {
                    keyword: keyword.clone(),
                    matched_text: keyword.clone(),
                    section: "general".to_string(),
                    position: pos,
                    context: "".to_string(),
                    confidence: 1.0,
                    weight: 1.0,
                });
            }
        }

        Ok(matches)
    }
}

impl StemmedMatcher {
    pub fn find_matches(
        &self,
        resume_content: &str,
        keywords: &[String],
    ) -> Result<Vec<MatchResult>> {
        let mut matches = Vec::new();

        // Initialize Porter stemmer
        let stemmer = Stemmer::create(Algorithm::English);

        // Normalize resume content
        let normalized_content = resume_content.nfc().collect::<String>();

        // Split resume into words and stem them
        let resume_words: Vec<(String, String, usize)> = normalized_content
            .unicode_words()
            .enumerate()
            .map(|(index, word)| {
                let lower_word = word.to_lowercase();
                let stemmed = stemmer.stem(&lower_word).to_string();
                (word.to_string(), stemmed, index)
            })
            .collect();

        // Process each keyword
        for keyword in keywords {
            let keyword_lower = keyword.to_lowercase();
            let keyword_stemmed = stemmer.stem(&keyword_lower).to_string();

            // Find matches by stemmed form
            for (original_word, stemmed_word, position) in &resume_words {
                if *stemmed_word == keyword_stemmed {
                    // Extract context around the match
                    let context =
                        self.extract_context(&normalized_content, *position, original_word);

                    // Determine section
                    let section = self.determine_section(&context);

                    // Calculate confidence based on stem similarity
                    let confidence = self.calculate_stem_confidence(
                        keyword,
                        original_word,
                        &keyword_stemmed,
                        stemmed_word,
                    );

                    // Calculate weight based on keyword importance
                    let weight = self.calculate_keyword_weight(keyword, &section);

                    matches.push(MatchResult {
                        keyword: keyword.clone(),
                        matched_text: original_word.clone(),
                        section: section.clone(),
                        position: *position,
                        context: context.clone(),
                        confidence,
                        weight,
                    });
                }
            }
        }

        // Sort by confidence and position
        matches.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.position.cmp(&b.position))
        });

        Ok(matches)
    }

    /// Extract context around a matched word
    fn extract_context(&self, content: &str, position: usize, _word: &str) -> String {
        let words: Vec<&str> = content.unicode_words().collect();
        let context_size = 5; // 5 words before and after

        let start = position.saturating_sub(context_size);
        let end = std::cmp::min(position + context_size + 1, words.len());

        words[start..end].join(" ")
    }

    /// Determine section based on context
    fn determine_section(&self, context: &str) -> String {
        let context_lower = context.to_lowercase();

        if context_lower.contains("experience")
            || context_lower.contains("work")
            || context_lower.contains("employment")
        {
            "Experience".to_string()
        } else if context_lower.contains("skill")
            || context_lower.contains("technical")
            || context_lower.contains("proficient")
        {
            "Skills".to_string()
        } else if context_lower.contains("education")
            || context_lower.contains("degree")
            || context_lower.contains("university")
        {
            "Education".to_string()
        } else if context_lower.contains("project") || context_lower.contains("portfolio") {
            "Projects".to_string()
        } else if context_lower.contains("achievement")
            || context_lower.contains("award")
            || context_lower.contains("honor")
        {
            "Achievements".to_string()
        } else {
            "General".to_string()
        }
    }

    /// Calculate confidence based on stem similarity
    fn calculate_stem_confidence(
        &self,
        keyword: &str,
        matched_word: &str,
        keyword_stem: &str,
        matched_stem: &str,
    ) -> f64 {
        // Base confidence for stem match
        let mut confidence = 0.7;

        // Boost confidence if it's an exact match
        if keyword.to_lowercase() == matched_word.to_lowercase() {
            confidence = 1.0;
        } else if keyword_stem == matched_stem {
            // Calculate similarity based on string similarity
            let similarity = self.string_similarity(keyword, matched_word);
            confidence = 0.7 + (similarity * 0.3);
        }

        confidence.clamp(0.0, 1.0)
    }

    /// Calculate string similarity between two words
    fn string_similarity(&self, word1: &str, word2: &str) -> f64 {
        let len1 = word1.len();
        let len2 = word2.len();

        if len1 == 0 || len2 == 0 {
            return 0.0;
        }

        let max_len = std::cmp::max(len1, len2);
        let common_chars = word1
            .chars()
            .zip(word2.chars())
            .take_while(|(a, b)| a == b)
            .count();

        common_chars as f64 / max_len as f64
    }

    /// Calculate keyword weight based on importance and section
    fn calculate_keyword_weight(&self, keyword: &str, section: &str) -> f64 {
        let mut weight = 1.0;

        // Increase weight for technical terms
        if keyword.len() > 3
            && (keyword.contains("script")
                || keyword.contains("java")
                || keyword.contains("python")
                || keyword.contains("react"))
        {
            weight *= 1.5;
        }

        // Increase weight for skills section
        if section == "Skills" {
            weight *= 1.3;
        } else if section == "Experience" {
            weight *= 1.2;
        }

        // Decrease weight for common words
        if keyword.len() <= 3 {
            weight *= 0.8;
        }

        weight
    }
}

impl ContextualMatcher {
    pub fn find_matches(
        &self,
        resume_content: &str,
        keywords: &[String],
    ) -> Result<Vec<MatchResult>> {
        let mut matches = Vec::new();

        // Normalize resume content
        let normalized_content = resume_content.nfc().collect::<String>();

        // Split into sentences for context analysis
        let sentences: Vec<&str> = normalized_content
            .split(['.', '!', '?'])
            .filter(|s| !s.trim().is_empty())
            .collect();

        // Process each keyword
        for keyword in keywords {
            let keyword_lower = keyword.to_lowercase();

            // Find contextual matches
            for (sentence_idx, sentence) in sentences.iter().enumerate() {
                let sentence_lower = sentence.to_lowercase();

                // Check for keyword variations and contextual clues
                if let Some(contextual_match) = self.find_contextual_match(
                    &sentence_lower,
                    &keyword_lower,
                    sentence,
                    sentence_idx,
                ) {
                    matches.push(contextual_match);
                }
            }
        }

        // Sort by confidence and context relevance
        matches.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| {
                    b.weight
                        .partial_cmp(&a.weight)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        });

        Ok(matches)
    }

    /// Find contextual matches considering surrounding words and phrases
    fn find_contextual_match(
        &self,
        sentence_lower: &str,
        keyword_lower: &str,
        original_sentence: &str,
        sentence_idx: usize,
    ) -> Option<MatchResult> {
        // Context patterns for different keyword types
        let tech_indicators = [
            "developed",
            "implemented",
            "built",
            "created",
            "designed",
            "managed",
            "led",
            "architected",
            "optimized",
        ];
        let skill_indicators = [
            "experienced",
            "proficient",
            "skilled",
            "expert",
            "knowledge",
            "familiar",
            "versed",
        ];
        let achievement_indicators = [
            "achieved",
            "improved",
            "increased",
            "reduced",
            "delivered",
            "completed",
            "successful",
        ];

        // Look for keyword in various forms
        let keyword_variations = self.generate_keyword_variations(keyword_lower);

        for variation in &keyword_variations {
            if sentence_lower.contains(variation) {
                // Found keyword variation, analyze context
                let context_score = self.analyze_context_relevance(
                    sentence_lower,
                    variation,
                    &tech_indicators,
                    &skill_indicators,
                    &achievement_indicators,
                );

                if context_score > 0.3 {
                    // Extract the specific matched text
                    let matched_text = self.extract_matched_text(original_sentence, variation);
                    let section = self.determine_section_from_context(sentence_lower);

                    return Some(MatchResult {
                        keyword: keyword_lower.to_string(),
                        matched_text,
                        section: section.clone(),
                        position: sentence_idx,
                        context: original_sentence.to_string(),
                        confidence: context_score,
                        weight: self.calculate_contextual_weight(
                            sentence_lower,
                            variation,
                            &section,
                        ),
                    });
                }
            }
        }

        None
    }

    /// Generate variations of a keyword for contextual matching
    fn generate_keyword_variations(&self, keyword: &str) -> Vec<String> {
        let mut variations = vec![keyword.to_string()];

        // Add plural forms
        if !keyword.ends_with('s') {
            variations.push(format!("{}s", keyword));
        }

        // Add -ing forms for verbs
        if keyword.len() > 3 {
            variations.push(format!("{}ing", keyword));
            if let Some(stripped) = keyword.strip_suffix('e') {
                variations.push(format!("{}ing", stripped));
            }
        }

        // Add -ed forms for verbs
        if keyword.len() > 3 {
            variations.push(format!("{}ed", keyword));
            if keyword.ends_with('e') {
                variations.push(format!("{}d", keyword));
            }
        }

        // Add common technical abbreviations
        match keyword {
            "javascript" => variations.push("js".to_string()),
            "typescript" => variations.push("ts".to_string()),
            "python" => variations.push("py".to_string()),
            "application programming interface" => variations.push("api".to_string()),
            "user interface" => variations.push("ui".to_string()),
            "user experience" => variations.push("ux".to_string()),
            _ => {}
        }

        variations
    }

    /// Analyze context relevance based on surrounding words
    fn analyze_context_relevance(
        &self,
        sentence: &str,
        keyword: &str,
        tech_indicators: &[&str],
        skill_indicators: &[&str],
        achievement_indicators: &[&str],
    ) -> f64 {
        let mut score: f64 = 0.5; // Base score for finding the keyword

        // Look for action verbs around the keyword
        for indicator in tech_indicators {
            if sentence.contains(indicator) {
                score += 0.3;
                break;
            }
        }

        // Look for skill-related context
        for indicator in skill_indicators {
            if sentence.contains(indicator) {
                score += 0.2;
                break;
            }
        }

        // Look for achievement context
        for indicator in achievement_indicators {
            if sentence.contains(indicator) {
                score += 0.2;
                break;
            }
        }

        // Boost score for technical terms in proper context
        if self.is_technical_term(keyword)
            && (sentence.contains("develop")
                || sentence.contains("implement")
                || sentence.contains("use"))
        {
            score += 0.3;
        }

        // Reduce score for very common words without strong context
        if keyword.len() <= 3 && score < 0.8 {
            score *= 0.7;
        }

        score.clamp(0.0, 1.0)
    }

    /// Check if a term is technical
    fn is_technical_term(&self, term: &str) -> bool {
        let technical_terms = [
            "python",
            "java",
            "javascript",
            "react",
            "angular",
            "vue",
            "node",
            "sql",
            "mongodb",
            "postgresql",
            "redis",
            "docker",
            "kubernetes",
            "aws",
            "azure",
            "gcp",
            "git",
            "github",
            "jenkins",
            "ci/cd",
            "machine learning",
            "artificial intelligence",
            "data science",
            "api",
            "rest",
            "graphql",
            "microservices",
            "devops",
        ];

        technical_terms.contains(&term) || term.contains("script") || term.contains("ql")
    }

    /// Extract the actual matched text from the original sentence
    fn extract_matched_text(&self, sentence: &str, keyword: &str) -> String {
        let sentence_lower = sentence.to_lowercase();
        if let Some(start) = sentence_lower.find(keyword) {
            let end = start + keyword.len();
            sentence[start..end].to_string()
        } else {
            keyword.to_string()
        }
    }

    /// Determine section from context clues
    fn determine_section_from_context(&self, sentence: &str) -> String {
        if sentence.contains("work")
            || sentence.contains("employ")
            || sentence.contains("position")
            || sentence.contains("role")
        {
            "Experience".to_string()
        } else if sentence.contains("skill")
            || sentence.contains("proficient")
            || sentence.contains("experience with")
        {
            "Skills".to_string()
        } else if sentence.contains("education")
            || sentence.contains("degree")
            || sentence.contains("university")
            || sentence.contains("college")
        {
            "Education".to_string()
        } else if sentence.contains("project")
            || sentence.contains("built")
            || sentence.contains("developed")
        {
            "Projects".to_string()
        } else if sentence.contains("achieve")
            || sentence.contains("award")
            || sentence.contains("recognition")
        {
            "Achievements".to_string()
        } else {
            "General".to_string()
        }
    }

    /// Calculate weight based on contextual relevance
    fn calculate_contextual_weight(&self, sentence: &str, keyword: &str, section: &str) -> f64 {
        let mut weight = 1.0;

        // Increase weight for strong action verbs
        if sentence.contains("led")
            || sentence.contains("managed")
            || sentence.contains("architected")
        {
            weight *= 1.8;
        } else if sentence.contains("developed")
            || sentence.contains("implemented")
            || sentence.contains("built")
        {
            weight *= 1.5;
        } else if sentence.contains("used") || sentence.contains("worked with") {
            weight *= 1.2;
        }

        // Increase weight for quantified achievements
        if sentence.contains('%')
            || sentence.contains("increased")
            || sentence.contains("reduced")
            || sentence.contains("improved")
        {
            weight *= 1.4;
        }

        // Adjust weight based on section
        match section {
            "Experience" => weight *= 1.3,
            "Skills" => weight *= 1.2,
            "Projects" => weight *= 1.1,
            _ => {}
        }

        // Increase weight for technical terms
        if self.is_technical_term(keyword) {
            weight *= 1.3;
        }

        weight
    }
}

impl SynonymMatcher {
    pub fn find_matches(
        &self,
        resume_content: &str,
        keywords: &[String],
    ) -> Result<Vec<MatchResult>> {
        let mut matches = Vec::new();

        // Normalize resume content
        let normalized_content = resume_content.nfc().collect::<String>();
        let content_lower = normalized_content.to_lowercase();

        // Initialize synonym database
        let synonym_db = self.build_synonym_database();

        // Process each keyword
        for keyword in keywords {
            let keyword_lower = keyword.to_lowercase();

            // Get synonyms for the keyword
            let synonyms = self.get_synonyms(&keyword_lower, &synonym_db);

            // Search for the keyword and its synonyms
            for synonym in &synonyms {
                if let Some(synonym_matches) =
                    self.find_synonym_matches(&content_lower, &normalized_content, keyword, synonym)
                {
                    matches.extend(synonym_matches);
                }
            }
        }

        // Remove duplicates and sort by confidence
        self.deduplicate_and_sort_matches(&mut matches);

        Ok(matches)
    }

    /// Build comprehensive synonym database
    fn build_synonym_database(&self) -> HashMap<String, Vec<String>> {
        let mut db = HashMap::new();

        // Technical skills synonyms
        db.insert(
            "javascript".to_string(),
            vec![
                "js".to_string(),
                "ecmascript".to_string(),
                "node.js".to_string(),
            ],
        );
        db.insert("typescript".to_string(), vec!["ts".to_string()]);
        db.insert(
            "python".to_string(),
            vec!["py".to_string(), "django".to_string(), "flask".to_string()],
        );
        db.insert(
            "java".to_string(),
            vec![
                "jvm".to_string(),
                "spring".to_string(),
                "hibernate".to_string(),
            ],
        );
        db.insert(
            "c++".to_string(),
            vec!["cpp".to_string(), "c plus plus".to_string()],
        );
        db.insert(
            "c#".to_string(),
            vec![
                "csharp".to_string(),
                "c sharp".to_string(),
                ".net".to_string(),
            ],
        );

        // Database synonyms
        db.insert(
            "sql".to_string(),
            vec![
                "database".to_string(),
                "rdbms".to_string(),
                "structured query language".to_string(),
            ],
        );
        db.insert(
            "mysql".to_string(),
            vec!["sql".to_string(), "database".to_string()],
        );
        db.insert(
            "postgresql".to_string(),
            vec!["postgres".to_string(), "sql".to_string()],
        );
        db.insert(
            "mongodb".to_string(),
            vec![
                "mongo".to_string(),
                "nosql".to_string(),
                "document database".to_string(),
            ],
        );
        db.insert(
            "redis".to_string(),
            vec!["cache".to_string(), "in-memory database".to_string()],
        );

        // Cloud services synonyms
        db.insert(
            "aws".to_string(),
            vec![
                "amazon web services".to_string(),
                "cloud".to_string(),
                "ec2".to_string(),
                "s3".to_string(),
            ],
        );
        db.insert(
            "azure".to_string(),
            vec!["microsoft azure".to_string(), "cloud".to_string()],
        );
        db.insert(
            "gcp".to_string(),
            vec![
                "google cloud platform".to_string(),
                "google cloud".to_string(),
            ],
        );

        // DevOps synonyms
        db.insert(
            "docker".to_string(),
            vec!["containerization".to_string(), "containers".to_string()],
        );
        db.insert(
            "kubernetes".to_string(),
            vec!["k8s".to_string(), "container orchestration".to_string()],
        );
        db.insert(
            "jenkins".to_string(),
            vec!["ci/cd".to_string(), "continuous integration".to_string()],
        );
        db.insert(
            "git".to_string(),
            vec![
                "version control".to_string(),
                "github".to_string(),
                "gitlab".to_string(),
            ],
        );

        // Frontend synonyms
        db.insert(
            "react".to_string(),
            vec![
                "reactjs".to_string(),
                "jsx".to_string(),
                "frontend".to_string(),
            ],
        );
        db.insert(
            "angular".to_string(),
            vec!["angularjs".to_string(), "frontend".to_string()],
        );
        db.insert(
            "vue".to_string(),
            vec!["vue.js".to_string(), "vuejs".to_string()],
        );
        db.insert(
            "html".to_string(),
            vec!["markup".to_string(), "web development".to_string()],
        );
        db.insert(
            "css".to_string(),
            vec![
                "styling".to_string(),
                "sass".to_string(),
                "less".to_string(),
            ],
        );

        // Soft skills synonyms
        db.insert(
            "leadership".to_string(),
            vec![
                "management".to_string(),
                "team lead".to_string(),
                "supervisor".to_string(),
            ],
        );
        db.insert(
            "communication".to_string(),
            vec!["interpersonal".to_string(), "collaboration".to_string()],
        );
        db.insert(
            "problem-solving".to_string(),
            vec![
                "analytical".to_string(),
                "troubleshooting".to_string(),
                "debugging".to_string(),
            ],
        );
        db.insert(
            "project management".to_string(),
            vec![
                "agile".to_string(),
                "scrum".to_string(),
                "kanban".to_string(),
            ],
        );

        // Industry-specific synonyms
        db.insert(
            "machine learning".to_string(),
            vec![
                "ml".to_string(),
                "ai".to_string(),
                "artificial intelligence".to_string(),
                "deep learning".to_string(),
            ],
        );
        db.insert(
            "data science".to_string(),
            vec![
                "analytics".to_string(),
                "big data".to_string(),
                "statistics".to_string(),
            ],
        );
        db.insert(
            "cybersecurity".to_string(),
            vec![
                "security".to_string(),
                "infosec".to_string(),
                "information security".to_string(),
            ],
        );
        db.insert(
            "ui/ux".to_string(),
            vec![
                "user interface".to_string(),
                "user experience".to_string(),
                "design".to_string(),
            ],
        );

        // Business synonyms
        db.insert(
            "sales".to_string(),
            vec![
                "business development".to_string(),
                "revenue".to_string(),
                "account management".to_string(),
            ],
        );
        db.insert(
            "marketing".to_string(),
            vec![
                "digital marketing".to_string(),
                "advertising".to_string(),
                "promotion".to_string(),
            ],
        );
        db.insert(
            "finance".to_string(),
            vec![
                "accounting".to_string(),
                "financial analysis".to_string(),
                "budgeting".to_string(),
            ],
        );

        db
    }

    /// Get synonyms for a keyword
    fn get_synonyms(
        &self,
        keyword: &str,
        synonym_db: &HashMap<String, Vec<String>>,
    ) -> Vec<String> {
        let mut synonyms = vec![keyword.to_string()];

        // Direct lookup
        if let Some(direct_synonyms) = synonym_db.get(keyword) {
            synonyms.extend(direct_synonyms.clone());
        }

        // Reverse lookup (find keywords that have this as a synonym)
        for (key, values) in synonym_db {
            if values.contains(&keyword.to_string()) {
                synonyms.push(key.clone());
            }
        }

        // Add common variations
        synonyms.extend(self.generate_common_variations(keyword));

        // Remove duplicates
        synonyms.sort();
        synonyms.dedup();

        synonyms
    }

    /// Generate common variations of a keyword
    fn generate_common_variations(&self, keyword: &str) -> Vec<String> {
        let mut variations = Vec::new();

        // Handle acronyms
        if keyword.contains('.') {
            variations.push(keyword.replace('.', ""));
        }

        // Handle spaces and hyphens
        variations.push(keyword.replace(' ', "-"));
        variations.push(keyword.replace('-', " "));
        variations.push(keyword.replace(' ', ""));

        // Handle common abbreviations
        if keyword.contains("application") {
            variations.push(keyword.replace("application", "app"));
        }
        if keyword.contains("development") {
            variations.push(keyword.replace("development", "dev"));
        }
        if keyword.contains("management") {
            variations.push(keyword.replace("management", "mgmt"));
        }

        variations
    }

    /// Find synonym matches in the content
    fn find_synonym_matches(
        &self,
        content_lower: &str,
        original_content: &str,
        original_keyword: &str,
        synonym: &str,
    ) -> Option<Vec<MatchResult>> {
        let mut matches = Vec::new();

        // Find all occurrences of the synonym
        let mut start = 0;
        while let Some(pos) = content_lower[start..].find(synonym) {
            let actual_pos = start + pos;

            // Check if it's a whole word match
            if self.is_whole_word_match(content_lower, actual_pos, synonym) {
                let context = self.extract_context_around_position(
                    original_content,
                    actual_pos,
                    synonym.len(),
                );
                let section = self.determine_section_from_context(&context);

                // Calculate confidence based on synonym relationship
                let confidence = self.calculate_synonym_confidence(original_keyword, synonym);
                let weight = self.calculate_synonym_weight(original_keyword, synonym, &section);

                matches.push(MatchResult {
                    keyword: original_keyword.to_string(),
                    matched_text: self.extract_original_text(
                        original_content,
                        actual_pos,
                        synonym.len(),
                    ),
                    section,
                    position: actual_pos,
                    context,
                    confidence,
                    weight,
                });
            }

            start = actual_pos + 1;
        }

        if matches.is_empty() {
            None
        } else {
            Some(matches)
        }
    }

    /// Check if the match is a whole word
    fn is_whole_word_match(&self, content: &str, position: usize, word: &str) -> bool {
        let word_end = position + word.len();

        // Check character before
        let before_ok = position == 0 || {
            let before_char = content.chars().nth(position - 1).unwrap_or(' ');
            !before_char.is_alphanumeric() && before_char != '_'
        };

        // Check character after
        let after_ok = word_end >= content.len() || {
            let after_char = content.chars().nth(word_end).unwrap_or(' ');
            !after_char.is_alphanumeric() && after_char != '_'
        };

        before_ok && after_ok
    }

    /// Extract context around a position
    fn extract_context_around_position(
        &self,
        content: &str,
        position: usize,
        _word_len: usize,
    ) -> String {
        let words: Vec<&str> = content.unicode_words().collect();
        let target_word_idx = content[..position].unicode_words().count();

        let context_size = 5;
        let start = target_word_idx.saturating_sub(context_size);
        let end = std::cmp::min(target_word_idx + context_size + 1, words.len());

        words[start..end].join(" ")
    }

    /// Extract original text from content
    fn extract_original_text(&self, content: &str, position: usize, length: usize) -> String {
        let end = std::cmp::min(position + length, content.len());
        content[position..end].to_string()
    }

    /// Calculate confidence for synonym matches
    fn calculate_synonym_confidence(&self, original_keyword: &str, synonym: &str) -> f64 {
        if original_keyword == synonym {
            1.0
        } else {
            // Base confidence for synonym match
            let mut confidence: f64 = 0.8;

            // Increase confidence for common abbreviations
            if (original_keyword == "javascript" && synonym == "js")
                || (original_keyword == "typescript" && synonym == "ts")
                || (original_keyword == "python" && synonym == "py")
            {
                confidence = 0.95;
            }

            // Slightly lower confidence for broader synonyms
            if synonym.contains("development") || synonym.contains("management") {
                confidence *= 0.9;
            }

            confidence.clamp(0.0, 1.0)
        }
    }

    /// Calculate weight for synonym matches
    fn calculate_synonym_weight(
        &self,
        original_keyword: &str,
        synonym: &str,
        section: &str,
    ) -> f64 {
        // Exact matches get full weight
        let mut weight = if original_keyword == synonym {
            1.0
        } else {
            // Synonym matches get reduced weight
            let mut base_weight = 0.8;

            // But technical abbreviations get higher weight
            if (original_keyword == "javascript" && synonym == "js")
                || (original_keyword == "typescript" && synonym == "ts")
                || (original_keyword == "python" && synonym == "py")
            {
                base_weight = 0.95;
            }

            base_weight
        };

        // Adjust based on section
        match section {
            "Skills" => weight *= 1.2,
            "Experience" => weight *= 1.1,
            _ => {}
        }

        weight
    }

    /// Determine section from context
    fn determine_section_from_context(&self, context: &str) -> String {
        let context_lower = context.to_lowercase();

        if context_lower.contains("skill")
            || context_lower.contains("technical")
            || context_lower.contains("proficient")
        {
            "Skills".to_string()
        } else if context_lower.contains("experience")
            || context_lower.contains("work")
            || context_lower.contains("position")
        {
            "Experience".to_string()
        } else if context_lower.contains("project")
            || context_lower.contains("built")
            || context_lower.contains("developed")
        {
            "Projects".to_string()
        } else if context_lower.contains("education")
            || context_lower.contains("degree")
            || context_lower.contains("university")
        {
            "Education".to_string()
        } else {
            "General".to_string()
        }
    }

    /// Remove duplicates and sort matches
    fn deduplicate_and_sort_matches(&self, matches: &mut Vec<MatchResult>) {
        // Sort by position first to identify duplicates
        matches.sort_by(|a, b| a.position.cmp(&b.position));

        // Remove duplicates based on position and keyword
        let mut unique_matches = Vec::new();
        for match_result in matches.iter() {
            if !unique_matches.iter().any(|m: &MatchResult| {
                m.position == match_result.position
                    && m.keyword == match_result.keyword
                    && (m.position as i32 - match_result.position as i32).abs() < 10
            }) {
                unique_matches.push(match_result.clone());
            }
        }

        // Sort by confidence and weight
        unique_matches.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| {
                    b.weight
                        .partial_cmp(&a.weight)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        });

        *matches = unique_matches;
    }
}

impl Default for IndustryWeights {
    fn default() -> Self {
        let default_weights = ScoringWeights {
            keyword_match: 0.4,
            format_compatibility: 0.2,
            section_completeness: 0.15,
            achievement_quality: 0.15,
            industry_alignment: 0.1,
        };

        Self {
            tech: ScoringWeights {
                keyword_match: 0.45,
                format_compatibility: 0.25,
                section_completeness: 0.1,
                achievement_quality: 0.15,
                industry_alignment: 0.05,
            },
            finance: ScoringWeights {
                keyword_match: 0.35,
                format_compatibility: 0.2,
                section_completeness: 0.2,
                achievement_quality: 0.2,
                industry_alignment: 0.05,
            },
            healthcare: default_weights.clone(),
            marketing: default_weights.clone(),
            general: default_weights,
        }
    }
}

// Sample ATS parser implementations
pub struct WorkdayParser;
pub struct TaleoParser;
pub struct GenericParser;

impl Default for WorkdayParser {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkdayParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TaleoParser {
    fn default() -> Self {
        Self::new()
    }
}

impl TaleoParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GenericParser {
    fn default() -> Self {
        Self::new()
    }
}

impl GenericParser {
    pub fn new() -> Self {
        Self
    }
}

impl ATSParser for WorkdayParser {
    fn parse_resume(&self, content: &str) -> Result<ParsedResume> {
        // Workday has sophisticated parsing but is sensitive to formatting
        let normalized_content = content.nfc().collect::<String>();

        // Parse different sections
        let sections = self.parse_sections(&normalized_content)?;
        let contact_info = self.parse_contact_info(&normalized_content)?;
        let experience = self.parse_experience(&normalized_content)?;
        let education = self.parse_education(&normalized_content)?;
        let skills = self.parse_skills(&normalized_content)?;

        // Calculate parsing confidence based on how well we could extract information
        let parsing_confidence = self.calculate_parsing_confidence(
            &sections,
            &contact_info,
            &experience,
            &education,
            &skills,
        );

        Ok(ParsedResume {
            sections,
            contact_info,
            experience,
            education,
            skills,
            parsing_confidence,
        })
    }

    fn get_system_type(&self) -> ATSSystem {
        ATSSystem::Workday
    }

    fn get_compatibility_score(&self, resume: &ParsedResume) -> f64 {
        let mut score: f64 = 85.0; // Workday's base score

        // Workday prefers well-structured resumes with clear sections
        if resume.sections.len() >= 4 {
            score += 5.0;
        }

        // Strong preference for complete contact information
        if resume.contact_info.name.is_some() && resume.contact_info.email.is_some() {
            score += 10.0;
        }

        // Penalize if parsing confidence is low
        if resume.parsing_confidence < 0.7 {
            score -= 15.0;
        }

        // Workday handles complex formatting well but prefers standard structure
        if !resume.experience.is_empty() && !resume.education.is_empty() {
            score += 5.0;
        }

        score.clamp(0.0, 100.0)
    }
}

impl WorkdayParser {
    /// Parse resume sections (Workday expects clear section headers)
    fn parse_sections(&self, content: &str) -> Result<HashMap<String, String>> {
        let mut sections = HashMap::new();

        // Common section headers that Workday recognizes
        let section_patterns = [
            (
                r"(?i)(?:^|\n)\s*(?:summary|professional\s+summary|profile|objective)[\s:\-]*\n",
                "Summary",
            ),
            (
                r"(?i)(?:^|\n)\s*(?:experience|professional\s+experience|work\s+experience|employment)[\s:\-]*\n",
                "Experience",
            ),
            (
                r"(?i)(?:^|\n)\s*(?:education|academic\s+background|educational\s+background)[\s:\-]*\n",
                "Education",
            ),
            (
                r"(?i)(?:^|\n)\s*(?:skills|technical\s+skills|core\s+competencies|proficiencies)[\s:\-]*\n",
                "Skills",
            ),
            (
                r"(?i)(?:^|\n)\s*(?:projects|key\s+projects|notable\s+projects)[\s:\-]*\n",
                "Projects",
            ),
            (
                r"(?i)(?:^|\n)\s*(?:certifications|certificates|professional\s+certifications)[\s:\-]*\n",
                "Certifications",
            ),
            (
                r"(?i)(?:^|\n)\s*(?:achievements|accomplishments|awards)[\s:\-]*\n",
                "Achievements",
            ),
        ];

        for (pattern, section_name) in &section_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(mat) = regex.find(content) {
                    let section_content =
                        self.extract_section_content(content, mat.end(), section_name);
                    if !section_content.trim().is_empty() {
                        sections.insert(section_name.to_string(), section_content);
                    }
                }
            }
        }

        Ok(sections)
    }

    /// Extract content for a specific section
    fn extract_section_content(
        &self,
        content: &str,
        start: usize,
        _current_section: &str,
    ) -> String {
        let remaining = &content[start..];

        // Look for the next section header or end of content
        let section_end_pattern = r"(?i)(?:^|\n)\s*(?:summary|experience|education|skills|projects|certifications|achievements|professional\s+summary|work\s+experience|technical\s+skills|core\s+competencies|key\s+projects|notable\s+projects|professional\s+certifications|academic\s+background|educational\s+background)[\s:\-]*\n";

        if let Ok(regex) = Regex::new(section_end_pattern) {
            if let Some(mat) = regex.find(remaining) {
                remaining[..mat.start()].trim().to_string()
            } else {
                remaining.trim().to_string()
            }
        } else {
            remaining.trim().to_string()
        }
    }

    /// Parse contact information (Workday is good at extracting this)
    fn parse_contact_info(&self, content: &str) -> Result<ContactInfo> {
        let mut contact = ContactInfo {
            name: None,
            email: None,
            phone: None,
            location: None,
        };

        // Extract name (usually at the top)
        let name_patterns = [
            r"(?i)^([A-Z][a-z]+(?:\s+[A-Z][a-z]+)+)", // First line with proper capitalization
            r"(?i)(?:^|\n)\s*([A-Z][a-z]+(?:\s+[A-Z][a-z]+)+)(?:\s*\n)", // Name on its own line
        ];

        for pattern in &name_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(cap) = regex.captures(content) {
                    contact.name = Some(cap[1].to_string());
                    break;
                }
            }
        }

        // Extract email
        let email_pattern = r"(?i)([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})";
        if let Ok(regex) = Regex::new(email_pattern) {
            if let Some(cap) = regex.captures(content) {
                contact.email = Some(cap[1].to_string());
            }
        }

        // Extract phone
        let phone_patterns = [
            r"(?:\+?1[-.\s]?)?\(?([0-9]{3})\)?[-.\s]?([0-9]{3})[-.\s]?([0-9]{4})", // US format
            r"(?:\+?1[-.\s]?)?([0-9]{3})[-.\s]?([0-9]{3})[-.\s]?([0-9]{4})", // Alternative format
        ];

        for pattern in &phone_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(cap) = regex.captures(content) {
                    contact.phone = Some(format!("({}) {}-{}", &cap[1], &cap[2], &cap[3]));
                    break;
                }
            }
        }

        // Extract location (city, state or city, country)
        let location_patterns = [
            r"(?i)([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*),\s*([A-Z]{2}(?:\s+[0-9]{5})?)", // City, ST 12345
            r"(?i)([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*),\s*([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*)", // City, Country
        ];

        for pattern in &location_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(cap) = regex.captures(content) {
                    contact.location = Some(format!("{}, {}", &cap[1], &cap[2]));
                    break;
                }
            }
        }

        Ok(contact)
    }

    /// Parse work experience (Workday expects chronological order)
    fn parse_experience(&self, content: &str) -> Result<Vec<ExperienceEntry>> {
        let mut experience = Vec::new();

        // Look for experience section
        let experience_pattern = r"(?i)(?:experience|professional\s+experience|work\s+experience|employment)[\s:\-]*\n(.*?)(?=\n\s*(?:education|skills|projects|certifications|achievements|$))";

        if let Ok(regex) = Regex::new(experience_pattern) {
            if let Some(cap) = regex.captures(content) {
                let experience_section = &cap[1];

                // Parse individual experience entries
                let job_pattern = r"(?i)([^(\n]+?)(?:\s*\|\s*|\s*,\s*|\s*-\s*|\s+)([^(\n]+?)(?:\s*\|\s*|\s*,\s*|\s*-\s*|\s+)([^(\n]+?)(?:\n|\s*$)";

                if let Ok(job_regex) = Regex::new(job_pattern) {
                    for cap in job_regex.captures_iter(experience_section) {
                        let title = cap[1].trim().to_string();
                        let company = cap[2].trim().to_string();
                        let duration = cap[3].trim().to_string();

                        // Extract description and achievements
                        let (description, achievements) =
                            self.parse_job_description(experience_section, &title, &company);

                        experience.push(ExperienceEntry {
                            title,
                            company,
                            duration,
                            description,
                            achievements,
                        });
                    }
                }
            }
        }

        Ok(experience)
    }

    /// Parse job description and extract achievements
    fn parse_job_description(
        &self,
        section: &str,
        title: &str,
        company: &str,
    ) -> (String, Vec<String>) {
        let mut description = String::new();
        let mut achievements = Vec::new();

        // Look for bullet points or achievements after the job title/company
        let lines: Vec<&str> = section.lines().collect();
        let mut in_current_job = false;
        let mut collecting_description = false;

        for line in lines {
            let line_trimmed = line.trim();

            if line_trimmed.contains(title) && line_trimmed.contains(company) {
                in_current_job = true;
                collecting_description = true;
                continue;
            }

            if in_current_job && collecting_description {
                // Stop if we hit another job title
                if !line_trimmed.is_empty()
                    && !line_trimmed.starts_with('•')
                    && !line_trimmed.starts_with('-')
                    && !line_trimmed.starts_with('*')
                {
                    // Check if this might be another job
                    if line_trimmed.contains("20") || line_trimmed.len() > 50 {
                        break;
                    }
                }

                if line_trimmed.starts_with('•')
                    || line_trimmed.starts_with('-')
                    || line_trimmed.starts_with('*')
                {
                    let achievement = line_trimmed
                        .trim_start_matches('•')
                        .trim_start_matches('-')
                        .trim_start_matches('*')
                        .trim();
                    if !achievement.is_empty() {
                        achievements.push(achievement.to_string());
                    }
                } else if !line_trimmed.is_empty() {
                    if !description.is_empty() {
                        description.push(' ');
                    }
                    description.push_str(line_trimmed);
                }
            }
        }

        (description, achievements)
    }

    /// Parse education information
    fn parse_education(&self, content: &str) -> Result<Vec<EducationEntry>> {
        let mut education = Vec::new();

        let education_pattern = r"(?i)(?:education|academic\s+background|educational\s+background)[\s:\-]*\n(.*?)(?=\n\s*(?:experience|skills|projects|certifications|achievements|$))";

        if let Ok(regex) = Regex::new(education_pattern) {
            if let Some(cap) = regex.captures(content) {
                let education_section = &cap[1];

                // Parse degree entries
                let degree_pattern = r"(?i)([^(\n]+?)(?:\s*\|\s*|\s*,\s*|\s*-\s*|\s+)([^(\n]+?)(?:\s*\|\s*|\s*,\s*|\s*-\s*|\s+)?([0-9]{4})?";

                if let Ok(degree_regex) = Regex::new(degree_pattern) {
                    for cap in degree_regex.captures_iter(education_section) {
                        let degree = cap[1].trim().to_string();
                        let institution = cap[2].trim().to_string();
                        let year = cap.get(3).map(|m| m.as_str().to_string());

                        education.push(EducationEntry {
                            degree,
                            institution,
                            year,
                            gpa: None, // Could be enhanced to parse GPA
                        });
                    }
                }
            }
        }

        Ok(education)
    }

    /// Parse skills section
    fn parse_skills(&self, content: &str) -> Result<Vec<String>> {
        let mut skills = Vec::new();

        let skills_pattern = r"(?i)(?:skills|technical\s+skills|core\s+competencies|proficiencies)[\s:\-]*\n(.*?)(?=\n\s*(?:experience|education|projects|certifications|achievements|$))";

        if let Ok(regex) = Regex::new(skills_pattern) {
            if let Some(cap) = regex.captures(content) {
                let skills_section = &cap[1];

                // Parse skills - they can be comma-separated, bullet points, or line-separated
                let skill_patterns = [
                    r"(?i)([^,\n•\-\*]+)(?:,|\n|•|\-|\*|$)", // Comma or line separated
                ];

                for pattern in &skill_patterns {
                    if let Ok(skill_regex) = Regex::new(pattern) {
                        for cap in skill_regex.captures_iter(skills_section) {
                            let skill = cap[1].trim().to_string();
                            if !skill.is_empty() && skill.len() > 1 {
                                skills.push(skill);
                            }
                        }
                    }
                }
            }
        }

        Ok(skills)
    }

    /// Calculate parsing confidence based on extracted information
    fn calculate_parsing_confidence(
        &self,
        sections: &HashMap<String, String>,
        contact: &ContactInfo,
        experience: &[ExperienceEntry],
        education: &[EducationEntry],
        skills: &[String],
    ) -> f64 {
        let mut confidence = 0.0;

        // Base confidence for finding sections
        confidence += sections.len() as f64 * 0.1;

        // Contact information confidence
        if contact.name.is_some() {
            confidence += 0.2;
        }
        if contact.email.is_some() {
            confidence += 0.2;
        }
        if contact.phone.is_some() {
            confidence += 0.1;
        }
        if contact.location.is_some() {
            confidence += 0.1;
        }

        // Experience confidence
        if !experience.is_empty() {
            confidence += 0.3;
            if experience.len() > 1 {
                confidence += 0.1;
            }
        }

        // Education confidence
        if !education.is_empty() {
            confidence += 0.2;
        }

        // Skills confidence
        if !skills.is_empty() {
            confidence += 0.2;
            if skills.len() > 5 {
                confidence += 0.1;
            }
        }

        confidence.clamp(0.0, 1.0)
    }
}

impl ATSParser for TaleoParser {
    fn parse_resume(&self, content: &str) -> Result<ParsedResume> {
        // Taleo is more rigid and has issues with complex formatting
        let normalized_content = content.nfc().collect::<String>();

        // Taleo struggles with complex layouts - simplify the content first
        let simplified_content = self.simplify_content(&normalized_content);

        // Parse with Taleo's more basic parsing approach
        let sections = self.parse_sections_basic(&simplified_content)?;
        let contact_info = self.parse_contact_info_basic(&simplified_content)?;
        let experience = self.parse_experience_basic(&simplified_content)?;
        let education = self.parse_education_basic(&simplified_content)?;
        let skills = self.parse_skills_basic(&simplified_content)?;

        // Taleo typically has lower parsing confidence due to its limitations
        let parsing_confidence = self.calculate_parsing_confidence(
            &sections,
            &contact_info,
            &experience,
            &education,
            &skills,
        ) * 0.8;

        Ok(ParsedResume {
            sections,
            contact_info,
            experience,
            education,
            skills,
            parsing_confidence,
        })
    }

    fn get_system_type(&self) -> ATSSystem {
        ATSSystem::Taleo
    }

    fn get_compatibility_score(&self, resume: &ParsedResume) -> f64 {
        let mut score: f64 = 80.0; // Taleo's base score

        // Taleo penalizes complex formatting heavily
        if resume.sections.len() > 6 {
            score -= 10.0; // Too many sections confuse Taleo
        }

        // Taleo requires very clear, simple structure
        if resume.contact_info.name.is_some()
            && resume.contact_info.email.is_some()
            && resume.contact_info.phone.is_some()
        {
            score += 10.0;
        }

        // Taleo struggles with parsing, so low confidence is heavily penalized
        if resume.parsing_confidence < 0.5 {
            score -= 25.0;
        } else if resume.parsing_confidence < 0.7 {
            score -= 10.0;
        }

        // Taleo prefers standard formats
        if !resume.experience.is_empty()
            && !resume.education.is_empty()
            && !resume.skills.is_empty()
        {
            score += 5.0;
        }

        // Penalize if too many or too few sections
        if resume.sections.len() < 3 {
            score -= 5.0;
        }

        score.clamp(0.0, 100.0)
    }
}

impl TaleoParser {
    /// Simplify content for Taleo's basic parsing
    fn simplify_content(&self, content: &str) -> String {
        // Remove complex formatting that Taleo can't handle
        let mut simplified = content.to_string();

        // Remove multiple spaces and normalize whitespace
        simplified = simplified.replace("  ", " ");
        simplified = simplified.replace("\t", " ");

        // Remove special characters that might confuse Taleo
        simplified = simplified.replace("•", "-");
        simplified = simplified.replace("▪", "-");
        simplified = simplified.replace("◦", "-");

        simplified
    }

    /// Basic section parsing (Taleo doesn't handle complex section detection well)
    fn parse_sections_basic(&self, content: &str) -> Result<HashMap<String, String>> {
        let mut sections = HashMap::new();

        // Very basic section headers - Taleo only recognizes simple patterns
        let section_patterns = [
            (r"(?i)(?:^|\n)\s*(?:summary|objective)[\s:\-]*\n", "Summary"),
            (
                r"(?i)(?:^|\n)\s*(?:experience|work experience)[\s:\-]*\n",
                "Experience",
            ),
            (r"(?i)(?:^|\n)\s*(?:education)[\s:\-]*\n", "Education"),
            (r"(?i)(?:^|\n)\s*(?:skills)[\s:\-]*\n", "Skills"),
        ];

        for (pattern, section_name) in &section_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(mat) = regex.find(content) {
                    let section_content = self.extract_section_content_basic(content, mat.end());
                    if !section_content.trim().is_empty() {
                        sections.insert(section_name.to_string(), section_content);
                    }
                }
            }
        }

        Ok(sections)
    }

    /// Basic section content extraction
    fn extract_section_content_basic(&self, content: &str, start: usize) -> String {
        let remaining = &content[start..];

        // Look for next section (very basic patterns only)
        let section_end_pattern = r"(?i)(?:^|\n)\s*(?:summary|objective|experience|work experience|education|skills)[\s:\-]*\n";

        if let Ok(regex) = Regex::new(section_end_pattern) {
            if let Some(mat) = regex.find(remaining) {
                remaining[..mat.start()].trim().to_string()
            } else {
                remaining.trim().to_string()
            }
        } else {
            remaining.trim().to_string()
        }
    }

    /// Basic contact info parsing (Taleo struggles with complex formats)
    fn parse_contact_info_basic(&self, content: &str) -> Result<ContactInfo> {
        let mut contact = ContactInfo {
            name: None,
            email: None,
            phone: None,
            location: None,
        };

        // Very basic name extraction - first line approach
        let lines: Vec<&str> = content.lines().collect();
        if !lines.is_empty() {
            let first_line = lines[0].trim();
            if first_line.len() > 2 && first_line.len() < 50 && !first_line.contains("@") {
                contact.name = Some(first_line.to_string());
            }
        }

        // Basic email extraction
        let email_pattern = r"([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})";
        if let Ok(regex) = Regex::new(email_pattern) {
            if let Some(cap) = regex.captures(content) {
                contact.email = Some(cap[1].to_string());
            }
        }

        // Basic phone extraction - simpler pattern
        let phone_pattern = r"([0-9]{3}[-.\s]?[0-9]{3}[-.\s]?[0-9]{4})";
        if let Ok(regex) = Regex::new(phone_pattern) {
            if let Some(cap) = regex.captures(content) {
                contact.phone = Some(cap[1].to_string());
            }
        }

        Ok(contact)
    }

    /// Basic experience parsing (Taleo misses complex job descriptions)
    fn parse_experience_basic(&self, content: &str) -> Result<Vec<ExperienceEntry>> {
        let mut experience = Vec::new();

        // Look for experience section with basic pattern
        let experience_pattern =
            r"(?i)(?:experience|work experience)[\s:\-]*\n(.*?)(?=\n\s*(?:education|skills|$))";

        if let Ok(regex) = Regex::new(experience_pattern) {
            if let Some(cap) = regex.captures(content) {
                let experience_section = &cap[1];

                // Very basic job parsing - Taleo often misses details
                let lines: Vec<&str> = experience_section.lines().collect();
                let mut current_job: Option<ExperienceEntry> = None;

                for line in lines {
                    let line_trimmed = line.trim();
                    if line_trimmed.is_empty() {
                        continue;
                    }

                    // Look for job titles (very basic heuristic)
                    if line_trimmed.len() > 10
                        && line_trimmed.len() < 60
                        && !line_trimmed.starts_with('-')
                    {
                        // Save previous job if exists
                        if let Some(job) = current_job.take() {
                            experience.push(job);
                        }

                        // Try to parse job title - company - duration
                        let parts: Vec<&str> = line_trimmed.split(" - ").collect();
                        if parts.len() >= 2 {
                            current_job = Some(ExperienceEntry {
                                title: parts[0].to_string(),
                                company: parts[1].to_string(),
                                duration: parts.get(2).unwrap_or(&"").to_string(),
                                description: String::new(),
                                achievements: Vec::new(),
                            });
                        }
                    }
                }

                // Add the last job
                if let Some(job) = current_job {
                    experience.push(job);
                }
            }
        }

        Ok(experience)
    }

    /// Basic education parsing
    fn parse_education_basic(&self, content: &str) -> Result<Vec<EducationEntry>> {
        let mut education = Vec::new();

        let education_pattern = r"(?i)(?:education)[\s:\-]*\n(.*?)(?=\n\s*(?:experience|skills|$))";

        if let Ok(regex) = Regex::new(education_pattern) {
            if let Some(cap) = regex.captures(content) {
                let education_section = &cap[1];

                let lines: Vec<&str> = education_section.lines().collect();
                for line in lines {
                    let line_trimmed = line.trim();
                    if line_trimmed.is_empty() {
                        continue;
                    }

                    // Basic degree parsing - assume format: "Degree - Institution"
                    let parts: Vec<&str> = line_trimmed.split(" - ").collect();
                    if parts.len() >= 2 {
                        education.push(EducationEntry {
                            degree: parts[0].to_string(),
                            institution: parts[1].to_string(),
                            year: None,
                            gpa: None,
                        });
                    }
                }
            }
        }

        Ok(education)
    }

    /// Basic skills parsing
    fn parse_skills_basic(&self, content: &str) -> Result<Vec<String>> {
        let mut skills = Vec::new();

        let skills_pattern = r"(?i)(?:skills)[\s:\-]*\n(.*?)(?=\n\s*(?:experience|education|$))";

        if let Ok(regex) = Regex::new(skills_pattern) {
            if let Some(cap) = regex.captures(content) {
                let skills_section = &cap[1];

                // Very basic skill parsing - just split by commas and newlines
                let skill_text = skills_section.replace('\n', ",");
                for skill in skill_text.split(',') {
                    let skill_trimmed = skill.trim();
                    if !skill_trimmed.is_empty() && skill_trimmed.len() > 1 {
                        skills.push(skill_trimmed.to_string());
                    }
                }
            }
        }

        Ok(skills)
    }

    /// Calculate parsing confidence (Taleo typically lower)
    fn calculate_parsing_confidence(
        &self,
        sections: &HashMap<String, String>,
        contact: &ContactInfo,
        experience: &[ExperienceEntry],
        education: &[EducationEntry],
        skills: &[String],
    ) -> f64 {
        let mut confidence = 0.0;

        // Taleo gets less confident with more sections
        confidence += (sections.len() as f64 * 0.1).min(0.4);

        // Contact information confidence
        if contact.name.is_some() {
            confidence += 0.15;
        }
        if contact.email.is_some() {
            confidence += 0.15;
        }
        if contact.phone.is_some() {
            confidence += 0.1;
        }

        // Experience confidence (Taleo often misses experience details)
        if !experience.is_empty() {
            confidence += 0.25;
        }

        // Education confidence
        if !education.is_empty() {
            confidence += 0.15;
        }

        // Skills confidence
        if !skills.is_empty() {
            confidence += 0.15;
            if skills.len() > 3 {
                confidence += 0.05;
            }
        }

        confidence.clamp(0.0, 1.0)
    }
}

impl ATSParser for GenericParser {
    fn parse_resume(&self, content: &str) -> Result<ParsedResume> {
        // Generic parser represents smaller/simpler ATS systems with basic parsing
        let normalized_content = content.nfc().collect::<String>();

        // Generic ATS systems typically have very basic parsing capabilities
        let sections = self.parse_sections_generic(&normalized_content)?;
        let contact_info = self.parse_contact_info_generic(&normalized_content)?;
        let experience = self.parse_experience_generic(&normalized_content)?;
        let education = self.parse_education_generic(&normalized_content)?;
        let skills = self.parse_skills_generic(&normalized_content)?;

        // Generic systems typically have moderate parsing confidence
        let parsing_confidence = self.calculate_parsing_confidence(
            &sections,
            &contact_info,
            &experience,
            &education,
            &skills,
        );

        Ok(ParsedResume {
            sections,
            contact_info,
            experience,
            education,
            skills,
            parsing_confidence,
        })
    }

    fn get_system_type(&self) -> ATSSystem {
        ATSSystem::Generic
    }

    fn get_compatibility_score(&self, resume: &ParsedResume) -> f64 {
        let mut score: f64 = 75.0; // Generic ATS base score

        // Generic systems are usually more forgiving than Taleo but less sophisticated than Workday
        if resume.sections.len() >= 3 && resume.sections.len() <= 8 {
            score += 10.0;
        }

        // Complete contact info is important but not as critical as in Taleo
        if resume.contact_info.name.is_some() && resume.contact_info.email.is_some() {
            score += 8.0;
        }

        // Moderate penalty for low parsing confidence
        if resume.parsing_confidence < 0.6 {
            score -= 15.0;
        } else if resume.parsing_confidence > 0.8 {
            score += 5.0;
        }

        // Reward well-structured resumes
        if !resume.experience.is_empty() && !resume.education.is_empty() {
            score += 7.0;
        }

        // Small penalty for very sparse or very dense resumes
        if resume.sections.len() < 2 {
            score -= 8.0;
        } else if resume.sections.len() > 10 {
            score -= 5.0;
        }

        score.clamp(0.0, 100.0)
    }
}

impl GenericParser {
    /// Generic section parsing (moderate capabilities)
    fn parse_sections_generic(&self, content: &str) -> Result<HashMap<String, String>> {
        let mut sections = HashMap::new();

        // Generic ATS systems recognize common section patterns
        let section_patterns = [
            (
                r"(?i)(?:^|\n)\s*(?:summary|professional summary|profile|objective|career objective)[\s:\-]*\n",
                "Summary",
            ),
            (
                r"(?i)(?:^|\n)\s*(?:experience|professional experience|work experience|employment history|career history)[\s:\-]*\n",
                "Experience",
            ),
            (
                r"(?i)(?:^|\n)\s*(?:education|educational background|academic background|qualifications)[\s:\-]*\n",
                "Education",
            ),
            (
                r"(?i)(?:^|\n)\s*(?:skills|technical skills|core competencies|key skills|expertise)[\s:\-]*\n",
                "Skills",
            ),
            (
                r"(?i)(?:^|\n)\s*(?:projects|key projects|notable projects|project experience)[\s:\-]*\n",
                "Projects",
            ),
            (
                r"(?i)(?:^|\n)\s*(?:certifications|certificates|professional certifications|licenses)[\s:\-]*\n",
                "Certifications",
            ),
        ];

        for (pattern, section_name) in &section_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(mat) = regex.find(content) {
                    let section_content = self.extract_section_content_generic(content, mat.end());
                    if !section_content.trim().is_empty() {
                        sections.insert(section_name.to_string(), section_content);
                    }
                }
            }
        }

        Ok(sections)
    }

    /// Generic section content extraction
    fn extract_section_content_generic(&self, content: &str, start: usize) -> String {
        let remaining = &content[start..];

        // Look for next section header
        let section_end_pattern = r"(?i)(?:^|\n)\s*(?:summary|professional summary|profile|objective|career objective|experience|professional experience|work experience|employment history|career history|education|educational background|academic background|qualifications|skills|technical skills|core competencies|key skills|expertise|projects|key projects|notable projects|project experience|certifications|certificates|professional certifications|licenses)[\s:\-]*\n";

        if let Ok(regex) = Regex::new(section_end_pattern) {
            if let Some(mat) = regex.find(remaining) {
                remaining[..mat.start()].trim().to_string()
            } else {
                remaining.trim().to_string()
            }
        } else {
            remaining.trim().to_string()
        }
    }

    /// Generic contact info parsing
    fn parse_contact_info_generic(&self, content: &str) -> Result<ContactInfo> {
        let mut contact = ContactInfo {
            name: None,
            email: None,
            phone: None,
            location: None,
        };

        // Name extraction - try multiple approaches
        let name_patterns = [
            r"(?i)^([A-Z][a-z]+\s+[A-Z][a-z]+(?:\s+[A-Z][a-z]+)?)", // First line approach
            r"(?i)(?:^|\n)\s*([A-Z][a-z]+\s+[A-Z][a-z]+(?:\s+[A-Z][a-z]+)?)(?:\s*\n)", // Name on its own line
            r"(?i)name[\s:]*([A-Z][a-z]+\s+[A-Z][a-z]+(?:\s+[A-Z][a-z]+)?)", // After "Name:" label
        ];

        for pattern in &name_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(cap) = regex.captures(content) {
                    contact.name = Some(cap[1].to_string());
                    break;
                }
            }
        }

        // Email extraction
        let email_pattern = r"(?i)([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})";
        if let Ok(regex) = Regex::new(email_pattern) {
            if let Some(cap) = regex.captures(content) {
                contact.email = Some(cap[1].to_string());
            }
        }

        // Phone extraction - multiple formats
        let phone_patterns = [
            r"(?:\+?1[-.\s]?)?\(?([0-9]{3})\)?[-.\s]?([0-9]{3})[-.\s]?([0-9]{4})", // Standard US format
            r"(?:\+?1[-.\s]?)?([0-9]{3})[-.\s]?([0-9]{3})[-.\s]?([0-9]{4})", // Alternative format
            r"(?i)(?:phone|tel|telephone)[\s:]*([0-9]{3}[-.\s]?[0-9]{3}[-.\s]?[0-9]{4})", // After label
        ];

        for pattern in &phone_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(cap) = regex.captures(content) {
                    if cap.len() == 4 {
                        contact.phone = Some(format!("({}) {}-{}", &cap[1], &cap[2], &cap[3]));
                    } else {
                        contact.phone = Some(cap[1].to_string());
                    }
                    break;
                }
            }
        }

        // Location extraction
        let location_patterns = [
            r"(?i)([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*),\s*([A-Z]{2}(?:\s+[0-9]{5})?)", // City, ST ZIP
            r"(?i)([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*),\s*([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*)", // City, Country
            r"(?i)(?:address|location)[\s:]*([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*(?:,\s*[A-Z]{2})?)", // After label
        ];

        for pattern in &location_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(cap) = regex.captures(content) {
                    if cap.len() == 3 {
                        contact.location = Some(format!("{}, {}", &cap[1], &cap[2]));
                    } else {
                        contact.location = Some(cap[1].to_string());
                    }
                    break;
                }
            }
        }

        Ok(contact)
    }

    /// Generic experience parsing
    fn parse_experience_generic(&self, content: &str) -> Result<Vec<ExperienceEntry>> {
        let mut experience = Vec::new();

        // Look for experience section
        let experience_pattern = r"(?i)(?:experience|professional experience|work experience|employment history|career history)[\s:\-]*\n(.*?)(?=\n\s*(?:education|skills|projects|certifications|$))";

        if let Ok(regex) = Regex::new(experience_pattern) {
            if let Some(cap) = regex.captures(content) {
                let experience_section = &cap[1];

                // Parse job entries - generic systems can handle moderate complexity
                let job_entries = self.parse_job_entries(experience_section);
                experience.extend(job_entries);
            }
        }

        Ok(experience)
    }

    /// Parse individual job entries
    fn parse_job_entries(&self, section: &str) -> Vec<ExperienceEntry> {
        let mut jobs = Vec::new();

        // Split by double newlines or obvious job separators
        let job_blocks: Vec<&str> = section.split("\n\n").collect();

        for block in job_blocks {
            if block.trim().is_empty() {
                continue;
            }

            let lines: Vec<&str> = block.lines().collect();
            if lines.is_empty() {
                continue;
            }

            // First line usually contains job title, company, and dates
            let first_line = lines[0].trim();
            let (title, company, duration) = self.parse_job_header(first_line);

            // Remaining lines are description and achievements
            let mut description = String::new();
            let mut achievements = Vec::new();

            for line in lines.iter().skip(1) {
                let line_trimmed = line.trim();
                if line_trimmed.is_empty() {
                    continue;
                }

                if line_trimmed.starts_with('•')
                    || line_trimmed.starts_with('-')
                    || line_trimmed.starts_with('*')
                {
                    let achievement = line_trimmed
                        .trim_start_matches('•')
                        .trim_start_matches('-')
                        .trim_start_matches('*')
                        .trim();
                    if !achievement.is_empty() {
                        achievements.push(achievement.to_string());
                    }
                } else {
                    if !description.is_empty() {
                        description.push(' ');
                    }
                    description.push_str(line_trimmed);
                }
            }

            jobs.push(ExperienceEntry {
                title,
                company,
                duration,
                description,
                achievements,
            });
        }

        jobs
    }

    /// Parse job header line
    fn parse_job_header(&self, header: &str) -> (String, String, String) {
        // Try different patterns for job header
        let patterns = [
            r"([^|]+)\s*\|\s*([^|]+)\s*\|\s*([^|]+)", // Title | Company | Duration
            r"([^,]+),\s*([^,]+),\s*([^,]+)",         // Title, Company, Duration
            r"([^-]+)\s*-\s*([^-]+)\s*-\s*([^-]+)",   // Title - Company - Duration
            r"([^•]+)\s*•\s*([^•]+)\s*•\s*([^•]+)",   // Title • Company • Duration
        ];

        for pattern in &patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(cap) = regex.captures(header) {
                    return (
                        cap[1].trim().to_string(),
                        cap[2].trim().to_string(),
                        cap[3].trim().to_string(),
                    );
                }
            }
        }

        // Fallback: assume the whole line is the title
        (
            header.to_string(),
            "Unknown Company".to_string(),
            "Unknown Duration".to_string(),
        )
    }

    /// Generic education parsing
    fn parse_education_generic(&self, content: &str) -> Result<Vec<EducationEntry>> {
        let mut education = Vec::new();

        let education_pattern = r"(?i)(?:education|educational background|academic background|qualifications)[\s:\-]*\n(.*?)(?=\n\s*(?:experience|skills|projects|certifications|$))";

        if let Ok(regex) = Regex::new(education_pattern) {
            if let Some(cap) = regex.captures(content) {
                let education_section = &cap[1];

                let lines: Vec<&str> = education_section.lines().collect();
                for line in lines {
                    let line_trimmed = line.trim();
                    if line_trimmed.is_empty() {
                        continue;
                    }

                    // Parse degree line - try multiple patterns
                    let (degree, institution, year) = self.parse_education_line(line_trimmed);

                    education.push(EducationEntry {
                        degree,
                        institution,
                        year,
                        gpa: None,
                    });
                }
            }
        }

        Ok(education)
    }

    /// Parse individual education line
    fn parse_education_line(&self, line: &str) -> (String, String, Option<String>) {
        // Try different patterns for education
        let patterns = [
            r"([^|]+)\s*\|\s*([^|]+)\s*\|\s*([0-9]{4})", // Degree | Institution | Year
            r"([^,]+),\s*([^,]+),\s*([0-9]{4})",         // Degree, Institution, Year
            r"([^-]+)\s*-\s*([^-]+)\s*-\s*([0-9]{4})",   // Degree - Institution - Year
            r"([^•]+)\s*•\s*([^•]+)\s*•\s*([0-9]{4})",   // Degree • Institution • Year
            r"([^|]+)\s*\|\s*([^|]+)",                   // Degree | Institution
            r"([^,]+),\s*([^,]+)",                       // Degree, Institution
            r"([^-]+)\s*-\s*([^-]+)",                    // Degree - Institution
        ];

        for pattern in &patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(cap) = regex.captures(line) {
                    let degree = cap[1].trim().to_string();
                    let institution = cap[2].trim().to_string();
                    let year = cap.get(3).map(|m| m.as_str().to_string());
                    return (degree, institution, year);
                }
            }
        }

        // Fallback: assume the whole line is the degree
        (line.to_string(), "Unknown Institution".to_string(), None)
    }

    /// Generic skills parsing
    fn parse_skills_generic(&self, content: &str) -> Result<Vec<String>> {
        let mut skills = Vec::new();

        let skills_pattern = r"(?i)(?:skills|technical skills|core competencies|key skills|expertise)[\s:\-]*\n(.*?)(?=\n\s*(?:experience|education|projects|certifications|$))";

        if let Ok(regex) = Regex::new(skills_pattern) {
            if let Some(cap) = regex.captures(content) {
                let skills_section = &cap[1];

                // Parse skills - multiple formats supported
                let skill_text = skills_section.replace('\n', " ");
                let separators = [",", "•", "-", "*", "|"];

                for separator in &separators {
                    if skill_text.contains(separator) {
                        for skill in skill_text.split(separator) {
                            let skill_trimmed = skill.trim();
                            if !skill_trimmed.is_empty() && skill_trimmed.len() > 1 {
                                skills.push(skill_trimmed.to_string());
                            }
                        }
                        break;
                    }
                }

                // If no separators found, treat each line as a skill
                if skills.is_empty() {
                    for line in skills_section.lines() {
                        let skill_trimmed = line.trim();
                        if !skill_trimmed.is_empty() && skill_trimmed.len() > 1 {
                            skills.push(skill_trimmed.to_string());
                        }
                    }
                }
            }
        }

        Ok(skills)
    }

    /// Calculate parsing confidence for generic systems
    fn calculate_parsing_confidence(
        &self,
        sections: &HashMap<String, String>,
        contact: &ContactInfo,
        experience: &[ExperienceEntry],
        education: &[EducationEntry],
        skills: &[String],
    ) -> f64 {
        let mut confidence = 0.0;

        // Base confidence for finding sections
        confidence += (sections.len() as f64 * 0.12).min(0.6);

        // Contact information confidence
        if contact.name.is_some() {
            confidence += 0.15;
        }
        if contact.email.is_some() {
            confidence += 0.15;
        }
        if contact.phone.is_some() {
            confidence += 0.1;
        }
        if contact.location.is_some() {
            confidence += 0.05;
        }

        // Experience confidence
        if !experience.is_empty() {
            confidence += 0.25;
            if experience.len() > 1 {
                confidence += 0.1;
            }
        }

        // Education confidence
        if !education.is_empty() {
            confidence += 0.15;
        }

        // Skills confidence
        if !skills.is_empty() {
            confidence += 0.15;
            if skills.len() > 3 {
                confidence += 0.1;
            }
        }

        confidence.clamp(0.0, 1.0)
    }
}
