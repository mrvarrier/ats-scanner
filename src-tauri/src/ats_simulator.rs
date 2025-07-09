use anyhow::Result;
use log::info;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::database::Database;
use crate::format_checker::{FormatCompatibilityChecker, FormatCompatibilityReport};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ATSSimulationResult {
    pub overall_ats_score: f64,
    pub system_simulations: HashMap<String, ATSSystemResult>,
    pub parsing_analysis: ParsingAnalysis,
    pub keyword_extraction: KeywordExtractionResult,
    pub format_analysis: FormatAnalysis,
    pub optimization_recommendations: Vec<ATSOptimizationRecommendation>,
    pub compatibility_issues: Vec<CompatibilityIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ATSSystemResult {
    pub system_name: String,
    pub compatibility_score: f64,
    pub parsing_success_rate: f64,
    pub extracted_sections: HashMap<String, ExtractionQuality>,
    pub keyword_detection_rate: f64,
    pub format_compliance: FormatCompliance,
    pub specific_issues: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsingAnalysis {
    pub structure_clarity: f64,
    pub section_detection: HashMap<String, bool>,
    pub contact_info_extraction: ContactExtractionResult,
    pub work_experience_parsing: ExperienceParsingResult,
    pub education_parsing: EducationParsingResult,
    pub skills_parsing: SkillsParsingResult,
    pub formatting_issues: Vec<FormattingIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordExtractionResult {
    pub extraction_accuracy: f64,
    pub keywords_found: Vec<ExtractedKeyword>,
    pub missed_keywords: Vec<String>,
    pub context_preservation: f64,
    pub semantic_understanding: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatAnalysis {
    pub file_format_compatibility: HashMap<String, bool>,
    pub layout_complexity: f64,
    pub font_compatibility: FontCompatibility,
    pub graphics_elements: GraphicsAnalysis,
    pub table_usage: TableAnalysis,
    pub line_spacing: f64,
    pub margin_analysis: MarginAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ATSOptimizationRecommendation {
    pub category: String,
    pub priority: String, // critical, high, medium, low
    pub title: String,
    pub description: String,
    pub implementation_steps: Vec<String>,
    pub expected_improvement: f64,
    pub affected_systems: Vec<String>,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityIssue {
    pub severity: String, // critical, major, minor, warning
    pub issue_type: String,
    pub description: String,
    pub affected_systems: Vec<String>,
    pub impact_score: f64,
    pub resolution_difficulty: String, // easy, medium, hard
    pub fix_suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionQuality {
    pub accuracy: f64,
    pub completeness: f64,
    pub structure_preservation: f64,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatCompliance {
    pub meets_standards: bool,
    pub compliance_score: f64,
    pub violations: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactExtractionResult {
    pub email_detected: bool,
    pub phone_detected: bool,
    pub address_detected: bool,
    pub linkedin_detected: bool,
    pub extraction_confidence: f64,
    pub formatting_issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceParsingResult {
    pub jobs_detected: i32,
    pub date_parsing_accuracy: f64,
    pub title_extraction_accuracy: f64,
    pub company_extraction_accuracy: f64,
    pub description_parsing_quality: f64,
    pub chronological_order_detected: bool,
    pub parsing_issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducationParsingResult {
    pub institutions_detected: i32,
    pub degree_extraction_accuracy: f64,
    pub date_parsing_accuracy: f64,
    pub gpa_detection: bool,
    pub certification_detection: i32,
    pub parsing_issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillsParsingResult {
    pub skills_detected: i32,
    pub categorization_accuracy: f64,
    pub technical_skills_ratio: f64,
    pub soft_skills_ratio: f64,
    pub skill_context_preservation: f64,
    pub parsing_issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattingIssue {
    pub issue_type: String,
    pub description: String,
    pub severity: String,
    pub line_number: Option<i32>,
    pub suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedKeyword {
    pub keyword: String,
    pub confidence: f64,
    pub context: String,
    pub section: String,
    pub importance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontCompatibility {
    pub standard_fonts_used: bool,
    pub font_consistency: f64,
    pub readability_score: f64,
    pub problematic_fonts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsAnalysis {
    pub has_graphics: bool,
    pub graphics_compatibility: f64,
    pub alt_text_present: bool,
    pub graphics_impact: String, // positive, negative, neutral
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableAnalysis {
    pub tables_detected: i32,
    pub table_compatibility: f64,
    pub parsing_difficulty: f64,
    pub alternative_suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginAnalysis {
    pub margin_consistency: f64,
    pub standard_margins: bool,
    pub readability_impact: f64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ATSSystemConfig {
    pub system_name: String,
    pub parsing_capabilities: ParsingCapabilities,
    pub format_preferences: FormatPreferences,
    pub keyword_matching: KeywordMatchingConfig,
    pub scoring_algorithm: ScoringAlgorithm,
    pub known_limitations: Vec<String>,
    pub optimization_tips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsingCapabilities {
    pub pdf_support: bool,
    pub docx_support: bool,
    pub txt_support: bool,
    pub html_support: bool,
    pub image_text_extraction: bool,
    pub table_parsing: bool,
    pub multi_column_support: bool,
    pub header_footer_handling: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatPreferences {
    pub preferred_fonts: Vec<String>,
    pub max_pages: Option<i32>,
    pub preferred_margins: String,
    pub bullet_point_style: Vec<String>,
    pub date_format_preferences: Vec<String>,
    pub section_header_style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordMatchingConfig {
    pub exact_match_weight: f64,
    pub partial_match_weight: f64,
    pub synonym_support: bool,
    pub case_sensitive: bool,
    pub context_analysis: bool,
    pub frequency_consideration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringAlgorithm {
    pub algorithm_type: String,
    pub weights: HashMap<String, f64>,
    pub keyword_importance: f64,
    pub experience_importance: f64,
    pub education_importance: f64,
    pub skills_importance: f64,
    pub format_penalty_factor: f64,
}

// Trait for ATS-specific parsers
pub trait ATSParser: Send + Sync {
    fn parse_resume(&self, content: &str, format: &str) -> Result<ParsedResume>;
    fn extract_keywords(&self, content: &str) -> Result<Vec<String>>;
    fn check_format_compatibility(&self, content: &str) -> Result<FormatCompatibilityScore>;
    #[allow(dead_code)]
    fn get_system_name(&self) -> &str;
    #[allow(dead_code)]
    fn get_parsing_rules(&self) -> &[ParsingRule];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedResume {
    pub success_rate: f64,
    pub extracted_sections: HashMap<String, ParsedSection>,
    pub parsing_errors: Vec<String>,
    pub confidence_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedSection {
    pub content: String,
    pub confidence: f64,
    pub structure_preserved: bool,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatCompatibilityScore {
    pub score: f64,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsingRule {
    pub rule_type: String,
    pub pattern: String,
    pub weight: f64,
    pub description: String,
}

// Greenhouse ATS Parser
pub struct GreenhouseParser {
    parsing_rules: Vec<ParsingRule>,
}

impl Default for GreenhouseParser {
    fn default() -> Self {
        Self::new()
    }
}

impl GreenhouseParser {
    pub fn new() -> Self {
        Self {
            parsing_rules: vec![
                ParsingRule {
                    rule_type: "section_header".to_string(),
                    pattern: r"(?i)^(experience|education|skills|summary)".to_string(),
                    weight: 1.0,
                    description: "Standard section headers".to_string(),
                },
                ParsingRule {
                    rule_type: "contact_info".to_string(),
                    pattern: r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}".to_string(),
                    weight: 1.0,
                    description: "Email detection".to_string(),
                },
                ParsingRule {
                    rule_type: "phone".to_string(),
                    pattern: r"\(?\d{3}\)?[-.\s]?\d{3}[-.\s]?\d{4}".to_string(),
                    weight: 0.8,
                    description: "Phone number detection".to_string(),
                },
            ],
        }
    }
}

impl ATSParser for GreenhouseParser {
    fn parse_resume(&self, content: &str, _format: &str) -> Result<ParsedResume> {
        let mut extracted_sections = HashMap::new();
        let mut parsing_errors = Vec::new();
        let mut confidence_scores = Vec::new();

        // Parse each section
        for section in &["contact", "summary", "experience", "education", "skills"] {
            match self.parse_section(content, section) {
                Ok(parsed_section) => {
                    confidence_scores.push(parsed_section.confidence);
                    extracted_sections.insert(section.to_string(), parsed_section);
                }
                Err(e) => {
                    parsing_errors.push(format!("Failed to parse {}: {}", section, e));
                }
            }
        }

        let success_rate = extracted_sections.len() as f64 / 5.0; // 5 expected sections
        let confidence_score = if confidence_scores.is_empty() {
            0.0
        } else {
            confidence_scores.iter().sum::<f64>() / confidence_scores.len() as f64
        };

        Ok(ParsedResume {
            success_rate,
            extracted_sections,
            parsing_errors,
            confidence_score,
        })
    }

    fn extract_keywords(&self, content: &str) -> Result<Vec<String>> {
        let mut keywords = Vec::new();

        // Extract technical keywords
        let tech_patterns = vec![
            r"(?i)\b(python|java|javascript|react|angular|vue|node\.?js)\b",
            r"(?i)\b(aws|azure|gcp|docker|kubernetes|git)\b",
            r"(?i)\b(sql|mysql|postgresql|mongodb|redis)\b",
        ];

        for pattern in tech_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                for captures in regex.captures_iter(content) {
                    if let Some(keyword) = captures.get(0) {
                        keywords.push(keyword.as_str().to_lowercase());
                    }
                }
            }
        }

        keywords.sort();
        keywords.dedup();
        Ok(keywords)
    }

    fn check_format_compatibility(&self, content: &str) -> Result<FormatCompatibilityScore> {
        let mut score: f64 = 100.0;
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        // Greenhouse has issues with tables
        if content.contains("<table") || content.contains("|") {
            score -= 25.0;
            issues.push("Tables detected - may cause parsing issues".to_string());
            recommendations.push("Convert tables to plain text format".to_string());
        }

        // Check for complex formatting
        if content.contains("<style") || content.contains("text-align") {
            score -= 10.0;
            issues.push("Complex formatting detected".to_string());
            recommendations.push("Use simple formatting".to_string());
        }

        Ok(FormatCompatibilityScore {
            score: score.max(0.0),
            issues,
            recommendations,
        })
    }

    fn get_system_name(&self) -> &str {
        "Greenhouse"
    }

    fn get_parsing_rules(&self) -> &[ParsingRule] {
        &self.parsing_rules
    }
}

impl GreenhouseParser {
    fn parse_section(&self, content: &str, section: &str) -> Result<ParsedSection> {
        let pattern = match section {
            "contact" => {
                r"([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}|\(?\d{3}\)?[-.\s]?\d{3}[-.\s]?\d{4})"
            }
            "summary" => r"(?i)(summary|objective|profile)[\s\S]*?(?=\n\s*[A-Z])",
            "experience" => r"(?i)(experience|employment)[\s\S]*?(?=\n\s*[A-Z])",
            "education" => r"(?i)(education|academic)[\s\S]*?(?=\n\s*[A-Z])",
            "skills" => r"(?i)(skills|competencies)[\s\S]*?(?=\n\s*[A-Z])",
            _ => return Err(anyhow::anyhow!("Unknown section: {}", section)),
        };

        if let Ok(regex) = Regex::new(pattern) {
            if let Some(captures) = regex.find(content) {
                let extracted_content = captures.as_str().to_string();
                return Ok(ParsedSection {
                    content: extracted_content.clone(),
                    confidence: 0.8,
                    structure_preserved: true,
                    issues: vec![],
                });
            }
        }

        Ok(ParsedSection {
            content: String::new(),
            confidence: 0.0,
            structure_preserved: false,
            issues: vec![format!("Could not detect {} section", section)],
        })
    }
}

// Lever ATS Parser
pub struct LeverParser {
    parsing_rules: Vec<ParsingRule>,
}

impl Default for LeverParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LeverParser {
    pub fn new() -> Self {
        Self {
            parsing_rules: vec![
                ParsingRule {
                    rule_type: "bullet_points".to_string(),
                    pattern: r"[•\-\*]\s+".to_string(),
                    weight: 1.0,
                    description: "Bullet point detection".to_string(),
                },
                ParsingRule {
                    rule_type: "dates".to_string(),
                    pattern: r"\d{4}[\-/]\d{4}|\d{1,2}/\d{4}".to_string(),
                    weight: 0.9,
                    description: "Date range detection".to_string(),
                },
            ],
        }
    }
}

impl ATSParser for LeverParser {
    fn parse_resume(&self, content: &str, _format: &str) -> Result<ParsedResume> {
        // Lever is better at parsing structured content but has issues with graphics
        let mut extracted_sections = HashMap::new();
        let parsing_errors = Vec::new();

        // Lever specific parsing logic
        for section in &["contact", "summary", "experience", "education", "skills"] {
            let parsed_section = self.parse_section_lever(content, section);
            extracted_sections.insert(section.to_string(), parsed_section);
        }

        let success_rate = extracted_sections
            .values()
            .map(|s| if s.confidence > 0.5 { 1.0 } else { 0.0 })
            .sum::<f64>()
            / extracted_sections.len() as f64;

        let confidence_score = extracted_sections
            .values()
            .map(|s| s.confidence)
            .sum::<f64>()
            / extracted_sections.len() as f64;

        Ok(ParsedResume {
            success_rate,
            extracted_sections,
            parsing_errors,
            confidence_score,
        })
    }

    fn extract_keywords(&self, content: &str) -> Result<Vec<String>> {
        // Lever is good at extracting keywords from bullet points
        let mut keywords = Vec::new();

        let bullet_regex = Regex::new(r"[•\-\*]\s+(.+)").unwrap();
        for captures in bullet_regex.captures_iter(content) {
            if let Some(bullet_content) = captures.get(1) {
                let words: Vec<&str> = bullet_content.as_str().split_whitespace().collect();
                for word in words {
                    if word.len() > 3 && !word.chars().all(|c| c.is_numeric()) {
                        keywords.push(word.to_lowercase());
                    }
                }
            }
        }

        keywords.sort();
        keywords.dedup();
        Ok(keywords)
    }

    fn check_format_compatibility(&self, content: &str) -> Result<FormatCompatibilityScore> {
        let mut score: f64 = 100.0;
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        // Lever has issues with text boxes
        if content.to_lowercase().contains("textbox") || content.contains("text-box") {
            score -= 20.0;
            issues.push("Text boxes detected - content may be ignored".to_string());
            recommendations.push("Move text box content to main document body".to_string());
        }

        // Lever handles graphics poorly
        if content.contains("<img") || content.contains("image") {
            score -= 15.0;
            issues.push("Graphics detected - may impact parsing".to_string());
            recommendations.push("Ensure all important information is in text format".to_string());
        }

        Ok(FormatCompatibilityScore {
            score: score.max(0.0),
            issues,
            recommendations,
        })
    }

    fn get_system_name(&self) -> &str {
        "Lever"
    }

    fn get_parsing_rules(&self) -> &[ParsingRule] {
        &self.parsing_rules
    }
}

impl LeverParser {
    fn parse_section_lever(&self, _content: &str, section: &str) -> ParsedSection {
        // Lever-specific parsing logic
        let confidence = match section {
            "experience" | "education" => 0.85, // Lever is good at these
            "skills" => 0.90,                   // Excellent at skills
            "summary" => 0.75,
            "contact" => 0.80,
            _ => 0.5,
        };

        ParsedSection {
            content: format!("Lever parsed {} section", section),
            confidence,
            structure_preserved: true,
            issues: vec![],
        }
    }
}

// Workday ATS Parser
pub struct WorkdayParser {
    parsing_rules: Vec<ParsingRule>,
}

impl Default for WorkdayParser {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkdayParser {
    pub fn new() -> Self {
        Self {
            parsing_rules: vec![
                ParsingRule {
                    rule_type: "education".to_string(),
                    pattern: r"(?i)(bachelor|master|phd|degree)".to_string(),
                    weight: 1.0,
                    description: "Education keywords".to_string(),
                },
                ParsingRule {
                    rule_type: "certification".to_string(),
                    pattern: r"(?i)(certified|certification|license)".to_string(),
                    weight: 0.9,
                    description: "Certification detection".to_string(),
                },
            ],
        }
    }
}

impl ATSParser for WorkdayParser {
    fn parse_resume(&self, content: &str, _format: &str) -> Result<ParsedResume> {
        // Workday has strict format requirements
        let mut extracted_sections = HashMap::new();
        let parsing_errors = Vec::new();

        // Workday specific parsing with stricter requirements
        for section in &["contact", "summary", "experience", "education", "skills"] {
            let parsed_section = self.parse_section_workday(content, section);
            extracted_sections.insert(section.to_string(), parsed_section);
        }

        let success_rate = extracted_sections
            .values()
            .map(|s| if s.confidence > 0.6 { 1.0 } else { 0.0 })
            .sum::<f64>()
            / extracted_sections.len() as f64;

        let confidence_score = extracted_sections
            .values()
            .map(|s| s.confidence)
            .sum::<f64>()
            / extracted_sections.len() as f64;

        Ok(ParsedResume {
            success_rate,
            extracted_sections,
            parsing_errors,
            confidence_score,
        })
    }

    fn extract_keywords(&self, content: &str) -> Result<Vec<String>> {
        // Workday focuses heavily on education and certifications
        let mut keywords = Vec::new();

        let patterns = vec![
            r"(?i)\b(bachelor|master|phd|doctorate|degree)\b",
            r"(?i)\b(certified|certification|license|accredited)\b",
            r"(?i)\b(university|college|institute|school)\b",
        ];

        for pattern in patterns {
            if let Ok(regex) = Regex::new(pattern) {
                for captures in regex.captures_iter(content) {
                    if let Some(keyword) = captures.get(0) {
                        keywords.push(keyword.as_str().to_lowercase());
                    }
                }
            }
        }

        keywords.sort();
        keywords.dedup();
        Ok(keywords)
    }

    fn check_format_compatibility(&self, content: &str) -> Result<FormatCompatibilityScore> {
        let mut score: f64 = 100.0;
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        // Workday is very strict about images
        if content.contains("<img") || content.contains("image") {
            score -= 30.0;
            issues.push("Images detected - Workday cannot extract text from images".to_string());
            recommendations.push("Convert all image content to text".to_string());
        }

        // Check for non-standard fonts
        let problematic_fonts = vec!["comic sans", "papyrus", "brush script"];
        for font in problematic_fonts {
            if content.to_lowercase().contains(font) {
                score -= 10.0;
                issues.push(format!("Non-standard font '{}' detected", font));
                recommendations
                    .push("Use professional fonts like Arial or Times New Roman".to_string());
            }
        }

        Ok(FormatCompatibilityScore {
            score: score.max(0.0),
            issues,
            recommendations,
        })
    }

    fn get_system_name(&self) -> &str {
        "Workday"
    }

    fn get_parsing_rules(&self) -> &[ParsingRule] {
        &self.parsing_rules
    }
}

impl WorkdayParser {
    fn parse_section_workday(&self, _content: &str, section: &str) -> ParsedSection {
        // Workday has varying success rates by section
        let confidence = match section {
            "education" => 0.95, // Excellent at education
            "contact" => 0.85,
            "experience" => 0.75,
            "skills" => 0.70,
            "summary" => 0.65,
            _ => 0.5,
        };

        ParsedSection {
            content: format!("Workday parsed {} section", section),
            confidence,
            structure_preserved: confidence > 0.7,
            issues: if confidence < 0.7 {
                vec![format!("Lower confidence parsing for {}", section)]
            } else {
                vec![]
            },
        }
    }
}

pub struct ATSSimulator {
    _database: Database,
    ats_systems: HashMap<String, ATSSystemConfig>,
    _parsing_patterns: HashMap<String, Vec<Regex>>,
    _format_checkers: HashMap<String, Regex>,
    format_checker: FormatCompatibilityChecker,
    parsers: HashMap<String, Box<dyn ATSParser>>,
}

impl ATSSimulator {
    pub fn new(database: Database) -> Self {
        let ats_systems = Self::build_ats_systems_config();
        let parsing_patterns = Self::build_parsing_patterns();
        let format_checkers = Self::build_format_checkers();
        let format_checker = FormatCompatibilityChecker::new();

        // Initialize ATS parsers
        let mut parsers: HashMap<String, Box<dyn ATSParser>> = HashMap::new();
        parsers.insert("greenhouse".to_string(), Box::new(GreenhouseParser::new()));
        parsers.insert("lever".to_string(), Box::new(LeverParser::new()));
        parsers.insert("workday".to_string(), Box::new(WorkdayParser::new()));

        ATSSimulator {
            _database: database,
            ats_systems,
            _parsing_patterns: parsing_patterns,
            _format_checkers: format_checkers,
            format_checker,
            parsers,
        }
    }

    pub async fn simulate_ats_processing(
        &self,
        resume_content: &str,
        target_job_keywords: &[String],
    ) -> Result<ATSSimulationResult> {
        info!("Starting ATS simulation for resume processing");

        // 1. Analyze parsing capability across different ATS systems
        let parsing_analysis = self.analyze_parsing_capability(resume_content).await?;

        // 2. Simulate keyword extraction
        let keyword_extraction =
            self.simulate_keyword_extraction(resume_content, target_job_keywords);

        // 3. Analyze format compatibility
        let format_analysis = self.analyze_format_compatibility(resume_content);

        // 4. Run system-specific simulations
        let mut system_simulations = HashMap::new();
        for (system_name, system_config) in &self.ats_systems {
            let system_result = self
                .simulate_system_processing(
                    resume_content,
                    system_config,
                    target_job_keywords,
                    &parsing_analysis,
                    &format_analysis,
                )
                .await?;
            system_simulations.insert(system_name.clone(), system_result);
        }

        // 5. Calculate overall ATS score
        let overall_ats_score = self.calculate_overall_ats_score(&system_simulations);

        // 6. Generate optimization recommendations
        let optimization_recommendations = self.generate_optimization_recommendations(
            &parsing_analysis,
            &format_analysis,
            &keyword_extraction,
            &system_simulations,
        );

        // 7. Identify compatibility issues
        let compatibility_issues = self.identify_compatibility_issues(
            &parsing_analysis,
            &format_analysis,
            &system_simulations,
        );

        Ok(ATSSimulationResult {
            overall_ats_score,
            system_simulations,
            parsing_analysis,
            keyword_extraction,
            format_analysis,
            optimization_recommendations,
            compatibility_issues,
        })
    }

    /// Enhanced simulation using specific ATS parsers
    pub async fn simulate_multiple_ats_systems(
        &self,
        resume_content: &str,
        target_job_keywords: &[String],
    ) -> Result<ATSSimulationResult> {
        info!("Starting enhanced ATS simulation with multiple parsers");

        // 1. Run comprehensive format compatibility check
        let format_report = self
            .format_checker
            .check_comprehensive_compatibility(resume_content)?;

        // 2. Simulate parsing with each ATS system
        let mut system_simulations = HashMap::new();
        for (system_name, parser) in &self.parsers {
            info!("Simulating parsing with {}", system_name);

            let parsed_resume = parser.parse_resume(resume_content, "docx")?;
            let extracted_keywords = parser.extract_keywords(resume_content)?;
            let format_compatibility = parser.check_format_compatibility(resume_content)?;

            let keyword_detection_rate =
                self.calculate_keyword_detection_rate(&extracted_keywords, target_job_keywords);

            let system_result = ATSSystemResult {
                system_name: system_name.clone(),
                compatibility_score: format_compatibility.score,
                parsing_success_rate: parsed_resume.success_rate,
                extracted_sections: self
                    .convert_to_extraction_quality(&parsed_resume.extracted_sections),
                keyword_detection_rate,
                format_compliance: FormatCompliance {
                    meets_standards: format_compatibility.score > 70.0,
                    compliance_score: format_compatibility.score,
                    violations: format_compatibility.issues.clone(),
                    recommendations: format_compatibility.recommendations.clone(),
                },
                specific_issues: parsed_resume.parsing_errors,
                recommendations: format_compatibility.recommendations,
            };

            system_simulations.insert(system_name.clone(), system_result);
        }

        // 3. Calculate overall metrics
        let overall_ats_score = self.calculate_overall_ats_score(&system_simulations);

        // 4. Create enhanced parsing analysis from format report
        let parsing_analysis =
            self.create_parsing_analysis_from_format_report(&format_report, resume_content)?;

        // 5. Enhanced keyword extraction using all parsers
        let keyword_extraction =
            self.enhanced_keyword_extraction(resume_content, target_job_keywords);

        // 6. Create format analysis from format report
        let format_analysis = self.create_format_analysis_from_report(&format_report);

        // 7. Generate comprehensive optimization recommendations
        let optimization_recommendations = self.generate_enhanced_optimization_recommendations(
            &format_report,
            &system_simulations,
            &keyword_extraction,
        );

        // 8. Identify critical compatibility issues
        let compatibility_issues =
            self.identify_enhanced_compatibility_issues(&format_report, &system_simulations);

        Ok(ATSSimulationResult {
            overall_ats_score,
            system_simulations,
            parsing_analysis,
            keyword_extraction,
            format_analysis,
            optimization_recommendations,
            compatibility_issues,
        })
    }

    fn calculate_keyword_detection_rate(
        &self,
        extracted_keywords: &[String],
        target_keywords: &[String],
    ) -> f64 {
        if target_keywords.is_empty() {
            return 1.0;
        }

        let mut found_count = 0;
        for target in target_keywords {
            if extracted_keywords
                .iter()
                .any(|k| k.to_lowercase().contains(&target.to_lowercase()))
            {
                found_count += 1;
            }
        }

        found_count as f64 / target_keywords.len() as f64
    }

    fn convert_to_extraction_quality(
        &self,
        sections: &HashMap<String, ParsedSection>,
    ) -> HashMap<String, ExtractionQuality> {
        sections
            .iter()
            .map(|(key, section)| {
                (
                    key.clone(),
                    ExtractionQuality {
                        accuracy: section.confidence,
                        completeness: if section.content.is_empty() { 0.0 } else { 0.8 },
                        structure_preservation: if section.structure_preserved {
                            1.0
                        } else {
                            0.5
                        },
                        issues: section.issues.clone(),
                    },
                )
            })
            .collect()
    }

    fn create_parsing_analysis_from_format_report(
        &self,
        format_report: &FormatCompatibilityReport,
        content: &str,
    ) -> Result<ParsingAnalysis> {
        // Convert format report data to parsing analysis
        let section_detection = format_report
            .parsing_simulation
            .successful_sections
            .iter()
            .map(|s| (s.clone(), true))
            .chain(
                format_report
                    .parsing_simulation
                    .failed_sections
                    .iter()
                    .map(|s| (s.clone(), false)),
            )
            .collect();

        Ok(ParsingAnalysis {
            structure_clarity: format_report.parsing_simulation.extraction_accuracy,
            section_detection,
            contact_info_extraction: self.analyze_contact_extraction(content),
            work_experience_parsing: self.analyze_experience_parsing(content),
            education_parsing: self.analyze_education_parsing(content),
            skills_parsing: self.analyze_skills_parsing(content),
            formatting_issues: format_report
                .format_issues
                .iter()
                .map(|issue| FormattingIssue {
                    issue_type: issue.issue_type.clone(),
                    description: issue.description.clone(),
                    severity: issue.severity.clone(),
                    line_number: None,
                    suggestion: issue.recommendation.clone(),
                })
                .collect(),
        })
    }

    fn enhanced_keyword_extraction(
        &self,
        resume_content: &str,
        target_keywords: &[String],
    ) -> KeywordExtractionResult {
        let mut all_keywords = Vec::new();
        let mut extraction_scores = Vec::new();

        // Use all parsers to extract keywords
        for parser in self.parsers.values() {
            if let Ok(keywords) = parser.extract_keywords(resume_content) {
                all_keywords.extend(keywords);
                extraction_scores.push(0.8); // Base confidence for parser extraction
            }
        }

        // Remove duplicates
        all_keywords.sort();
        all_keywords.dedup();

        // Convert to ExtractedKeyword format
        let keywords_found: Vec<ExtractedKeyword> = all_keywords
            .iter()
            .map(|keyword| ExtractedKeyword {
                keyword: keyword.clone(),
                confidence: 0.8,
                context: "Extracted by ATS parser".to_string(),
                section: "general".to_string(),
                importance: if target_keywords
                    .iter()
                    .any(|t| t.to_lowercase() == keyword.to_lowercase())
                {
                    1.0
                } else {
                    0.5
                },
            })
            .collect();

        let missed_keywords: Vec<String> = target_keywords
            .iter()
            .filter(|target| {
                !all_keywords
                    .iter()
                    .any(|k| k.to_lowercase().contains(&target.to_lowercase()))
            })
            .cloned()
            .collect();

        let extraction_accuracy = if target_keywords.is_empty() {
            1.0
        } else {
            (target_keywords.len() - missed_keywords.len()) as f64 / target_keywords.len() as f64
        };

        KeywordExtractionResult {
            extraction_accuracy,
            keywords_found,
            missed_keywords,
            context_preservation: 0.75,
            semantic_understanding: 0.7,
        }
    }

    fn create_format_analysis_from_report(
        &self,
        format_report: &FormatCompatibilityReport,
    ) -> FormatAnalysis {
        // Convert format compatibility report to format analysis
        let file_format_compatibility = format_report
            .ats_specific_scores
            .iter()
            .map(|(ats, score)| (ats.clone(), *score > 70.0))
            .collect();

        FormatAnalysis {
            file_format_compatibility,
            layout_complexity: 100.0 - format_report.overall_score,
            font_compatibility: FontCompatibility {
                standard_fonts_used: !format_report
                    .format_issues
                    .iter()
                    .any(|i| i.issue_type == "non_standard_font"),
                font_consistency: 0.9,
                readability_score: 0.85,
                problematic_fonts: format_report
                    .format_issues
                    .iter()
                    .filter(|i| i.issue_type == "non_standard_font")
                    .map(|i| i.description.clone())
                    .collect(),
            },
            graphics_elements: GraphicsAnalysis {
                has_graphics: format_report
                    .format_issues
                    .iter()
                    .any(|i| i.issue_type == "text_in_images"),
                graphics_compatibility: if format_report
                    .format_issues
                    .iter()
                    .any(|i| i.issue_type == "text_in_images")
                {
                    0.0
                } else {
                    1.0
                },
                alt_text_present: false,
                graphics_impact: if format_report
                    .format_issues
                    .iter()
                    .any(|i| i.issue_type == "text_in_images")
                {
                    "negative".to_string()
                } else {
                    "neutral".to_string()
                },
                recommendations: vec!["Ensure all text is in readable format".to_string()],
            },
            table_usage: TableAnalysis {
                tables_detected: if format_report
                    .format_issues
                    .iter()
                    .any(|i| i.issue_type == "tables")
                {
                    1
                } else {
                    0
                },
                table_compatibility: if format_report
                    .format_issues
                    .iter()
                    .any(|i| i.issue_type == "tables")
                {
                    0.3
                } else {
                    1.0
                },
                parsing_difficulty: if format_report
                    .format_issues
                    .iter()
                    .any(|i| i.issue_type == "tables")
                {
                    0.8
                } else {
                    0.1
                },
                alternative_suggestions: vec!["Convert tables to plain text format".to_string()],
            },
            line_spacing: 1.0,
            margin_analysis: MarginAnalysis {
                margin_consistency: 0.9,
                standard_margins: true,
                readability_impact: 0.85,
                recommendations: vec![],
            },
        }
    }

    fn generate_enhanced_optimization_recommendations(
        &self,
        format_report: &FormatCompatibilityReport,
        system_simulations: &HashMap<String, ATSSystemResult>,
        _keyword_extraction: &KeywordExtractionResult,
    ) -> Vec<ATSOptimizationRecommendation> {
        let mut recommendations = Vec::new();

        // Generate recommendations based on format issues
        for issue in &format_report.format_issues {
            let priority = match issue.severity.as_str() {
                "critical" => "critical",
                "high" => "high",
                "medium" => "medium",
                _ => "low",
            };

            let affected_systems: Vec<String> = system_simulations
                .iter()
                .filter(|(_, result)| result.compatibility_score < 80.0)
                .map(|(name, _)| name.clone())
                .collect();

            recommendations.push(ATSOptimizationRecommendation {
                category: issue.issue_type.clone(),
                priority: priority.to_string(),
                title: format!("Fix {}", issue.issue_type.replace('_', " ")),
                description: issue.description.clone(),
                implementation_steps: vec![issue.recommendation.clone()],
                expected_improvement: issue.impact_score,
                affected_systems,
                examples: vec![],
            });
        }

        // Sort by priority
        recommendations.sort_by(|a, b| {
            let priority_order = ["critical", "high", "medium", "low"];
            let a_index = priority_order
                .iter()
                .position(|&p| p == a.priority)
                .unwrap_or(3);
            let b_index = priority_order
                .iter()
                .position(|&p| p == b.priority)
                .unwrap_or(3);
            a_index.cmp(&b_index)
        });

        recommendations
    }

    fn identify_enhanced_compatibility_issues(
        &self,
        format_report: &FormatCompatibilityReport,
        system_simulations: &HashMap<String, ATSSystemResult>,
    ) -> Vec<CompatibilityIssue> {
        let mut issues = Vec::new();

        // Convert format issues to compatibility issues
        for format_issue in &format_report.format_issues {
            let affected_systems: Vec<String> = system_simulations
                .iter()
                .filter(|(_, result)| {
                    result
                        .specific_issues
                        .iter()
                        .any(|issue| issue.contains(&format_issue.issue_type))
                })
                .map(|(name, _)| name.clone())
                .collect();

            issues.push(CompatibilityIssue {
                severity: format_issue.severity.clone(),
                issue_type: format_issue.issue_type.clone(),
                description: format_issue.description.clone(),
                affected_systems,
                impact_score: format_issue.impact_score,
                resolution_difficulty: if format_issue.impact_score > 20.0 {
                    "hard".to_string()
                } else {
                    "medium".to_string()
                },
                fix_suggestions: vec![format_issue.recommendation.clone()],
            });
        }

        issues
    }

    async fn analyze_parsing_capability(&self, resume_content: &str) -> Result<ParsingAnalysis> {
        let sections = self.detect_resume_sections(resume_content);
        let contact_info = self.analyze_contact_extraction(resume_content);
        let work_experience = self.analyze_experience_parsing(resume_content);
        let education = self.analyze_education_parsing(resume_content);
        let skills = self.analyze_skills_parsing(resume_content);
        let formatting_issues = self.detect_formatting_issues(resume_content);

        let structure_clarity = self.calculate_structure_clarity(resume_content, &sections);

        Ok(ParsingAnalysis {
            structure_clarity,
            section_detection: sections,
            contact_info_extraction: contact_info,
            work_experience_parsing: work_experience,
            education_parsing: education,
            skills_parsing: skills,
            formatting_issues,
        })
    }

    fn simulate_keyword_extraction(
        &self,
        resume_content: &str,
        target_keywords: &[String],
    ) -> KeywordExtractionResult {
        let mut keywords_found = Vec::new();
        let mut missed_keywords = Vec::new();
        let content_lower = resume_content.to_lowercase();

        for target_keyword in target_keywords {
            let keyword_lower = target_keyword.to_lowercase();
            if content_lower.contains(&keyword_lower) {
                // Find context around the keyword
                if let Some(pos) = content_lower.find(&keyword_lower) {
                    let start = (pos as i32 - 50).max(0) as usize;
                    let end = (pos + keyword_lower.len() + 50).min(resume_content.len());
                    let context = resume_content[start..end].trim().to_string();

                    keywords_found.push(ExtractedKeyword {
                        keyword: target_keyword.clone(),
                        confidence: 0.95,
                        context,
                        section: self.identify_keyword_section(resume_content, pos),
                        importance: 1.0,
                    });
                }
            } else {
                missed_keywords.push(target_keyword.clone());
            }
        }

        let extraction_accuracy = if target_keywords.is_empty() {
            1.0
        } else {
            keywords_found.len() as f64 / target_keywords.len() as f64
        };

        KeywordExtractionResult {
            extraction_accuracy,
            keywords_found,
            missed_keywords,
            context_preservation: 0.85,
            semantic_understanding: 0.75,
        }
    }

    fn analyze_format_compatibility(&self, resume_content: &str) -> FormatAnalysis {
        let file_format_compatibility = self.check_file_format_compatibility();
        let layout_complexity = self.assess_layout_complexity(resume_content);
        let font_compatibility = self.analyze_font_compatibility(resume_content);
        let graphics_elements = self.analyze_graphics_elements(resume_content);
        let table_usage = self.analyze_table_usage(resume_content);
        let line_spacing = self.calculate_line_spacing(resume_content);
        let margin_analysis = self.analyze_margins(resume_content);

        FormatAnalysis {
            file_format_compatibility,
            layout_complexity,
            font_compatibility,
            graphics_elements,
            table_usage,
            line_spacing,
            margin_analysis,
        }
    }

    async fn simulate_system_processing(
        &self,
        resume_content: &str,
        system_config: &ATSSystemConfig,
        target_keywords: &[String],
        parsing_analysis: &ParsingAnalysis,
        format_analysis: &FormatAnalysis,
    ) -> Result<ATSSystemResult> {
        let compatibility_score = self.calculate_system_compatibility_score(
            system_config,
            parsing_analysis,
            format_analysis,
        );

        let parsing_success_rate =
            self.calculate_parsing_success_rate(system_config, parsing_analysis);

        let extracted_sections = self.simulate_section_extraction(system_config, parsing_analysis);

        // Extract keywords from content first
        let extracted_keywords = self.extract_keywords(resume_content);
        let keyword_detection_rate =
            self.calculate_keyword_detection_rate(&extracted_keywords, target_keywords);

        let format_compliance = self.assess_format_compliance(system_config, format_analysis);

        let specific_issues =
            self.identify_system_specific_issues(system_config, parsing_analysis, format_analysis);

        let recommendations = self.generate_system_recommendations(system_config, &specific_issues);

        Ok(ATSSystemResult {
            system_name: system_config.system_name.clone(),
            compatibility_score,
            parsing_success_rate,
            extracted_sections,
            keyword_detection_rate,
            format_compliance,
            specific_issues,
            recommendations,
        })
    }

    // Helper methods for ATS simulation
    fn detect_resume_sections(&self, content: &str) -> HashMap<String, bool> {
        let mut sections = HashMap::new();
        let content_lower = content.to_lowercase();

        // Define section patterns
        let section_patterns = vec![
            ("contact_info", vec!["email", "phone", "@", "linkedin"]),
            ("summary", vec!["summary", "objective", "profile"]),
            (
                "experience",
                vec!["experience", "employment", "work history", "professional"],
            ),
            (
                "education",
                vec!["education", "academic", "degree", "university", "college"],
            ),
            (
                "skills",
                vec!["skills", "technical", "proficiencies", "technologies"],
            ),
            (
                "certifications",
                vec!["certification", "certified", "license"],
            ),
            ("projects", vec!["projects", "portfolio", "accomplishments"]),
            (
                "awards",
                vec!["awards", "honors", "recognition", "achievements"],
            ),
        ];

        for (section_name, keywords) in section_patterns {
            let found = keywords
                .iter()
                .any(|&keyword| content_lower.contains(keyword));
            sections.insert(section_name.to_string(), found);
        }

        sections
    }

    fn analyze_contact_extraction(&self, content: &str) -> ContactExtractionResult {
        let email_pattern = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
        let phone_pattern =
            Regex::new(r"(\+?1[-.\s]?)?\(?([0-9]{3})\)?[-.\s]?([0-9]{3})[-.\s]?([0-9]{4})")
                .unwrap();
        let linkedin_pattern = Regex::new(r"linkedin\.com/in/[a-zA-Z0-9-]+").unwrap();

        let email_detected = email_pattern.is_match(content);
        let phone_detected = phone_pattern.is_match(content);
        let linkedin_detected =
            linkedin_pattern.is_match(content) || content.to_lowercase().contains("linkedin");

        // Simple address detection
        let address_detected = content.to_lowercase().contains("address")
            || content.contains(", ")
                && (content.contains("Street")
                    || content.contains("Ave")
                    || content.contains("Rd"));

        let detected_count = [
            email_detected,
            phone_detected,
            address_detected,
            linkedin_detected,
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        let extraction_confidence = detected_count as f64 / 4.0;

        let mut formatting_issues = Vec::new();
        if !email_detected {
            formatting_issues.push("Email address not clearly formatted".to_string());
        }
        if !phone_detected {
            formatting_issues.push("Phone number not in standard format".to_string());
        }

        ContactExtractionResult {
            email_detected,
            phone_detected,
            address_detected,
            linkedin_detected,
            extraction_confidence,
            formatting_issues,
        }
    }

    fn analyze_experience_parsing(&self, content: &str) -> ExperienceParsingResult {
        let content_lower = content.to_lowercase();

        // Count potential job entries
        let job_indicators = ["company", "employer", "position", "role", "title"];
        let jobs_detected = job_indicators
            .iter()
            .map(|&indicator| content_lower.matches(indicator).count())
            .max()
            .unwrap_or(0) as i32;

        // Analyze date patterns
        let date_patterns = [
            Regex::new(r"\d{4}\s*[-–]\s*\d{4}").unwrap(),
            Regex::new(r"\d{4}\s*[-–]\s*present").unwrap(),
            Regex::new(r"(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)\s+\d{4}").unwrap(),
        ];

        let date_matches: usize = date_patterns
            .iter()
            .map(|pattern| pattern.find_iter(content).count())
            .sum();

        let date_parsing_accuracy = if jobs_detected > 0 {
            (date_matches as f64 / jobs_detected as f64).min(1.0)
        } else {
            0.0
        };

        let title_extraction_accuracy =
            if content_lower.contains("title") || content_lower.contains("position") {
                0.9
            } else {
                0.6
            };

        let company_extraction_accuracy =
            if content_lower.contains("company") || content_lower.contains("employer") {
                0.9
            } else {
                0.7
            };

        let description_parsing_quality = 0.8; // Simplified assessment

        let chronological_order_detected = date_matches > 1;

        let mut parsing_issues = Vec::new();
        if date_parsing_accuracy < 0.5 {
            parsing_issues.push("Date formats are inconsistent or unclear".to_string());
        }
        if jobs_detected == 0 {
            parsing_issues.push("No clear work experience section detected".to_string());
        }

        ExperienceParsingResult {
            jobs_detected,
            date_parsing_accuracy,
            title_extraction_accuracy,
            company_extraction_accuracy,
            description_parsing_quality,
            chronological_order_detected,
            parsing_issues,
        }
    }

    fn analyze_education_parsing(&self, content: &str) -> EducationParsingResult {
        let content_lower = content.to_lowercase();

        let education_keywords = [
            "university",
            "college",
            "degree",
            "bachelor",
            "master",
            "phd",
            "doctorate",
        ];
        let institutions_detected = education_keywords
            .iter()
            .map(|&keyword| content_lower.matches(keyword).count())
            .max()
            .unwrap_or(0) as i32;

        let degree_keywords = [
            "bachelor",
            "master",
            "phd",
            "doctorate",
            "b.s.",
            "m.s.",
            "b.a.",
            "m.a.",
        ];
        let degree_mentions = degree_keywords
            .iter()
            .map(|&keyword| content_lower.matches(keyword).count())
            .sum::<usize>();

        let degree_extraction_accuracy = if institutions_detected > 0 {
            (degree_mentions as f64 / institutions_detected as f64).min(1.0)
        } else {
            0.0
        };

        let date_parsing_accuracy = 0.8; // Simplified
        let gpa_detection = content_lower.contains("gpa") || content_lower.contains("grade point");

        let certification_keywords = ["certification", "certified", "license", "credential"];
        let certification_detection = certification_keywords
            .iter()
            .map(|&keyword| content_lower.matches(keyword).count())
            .sum::<usize>() as i32;

        let mut parsing_issues = Vec::new();
        if institutions_detected == 0 {
            parsing_issues.push("No clear education section detected".to_string());
        }

        EducationParsingResult {
            institutions_detected,
            degree_extraction_accuracy,
            date_parsing_accuracy,
            gpa_detection,
            certification_detection,
            parsing_issues,
        }
    }

    fn analyze_skills_parsing(&self, content: &str) -> SkillsParsingResult {
        let content_lower = content.to_lowercase();

        // Technical skills patterns
        let technical_patterns = [
            Regex::new(r"(?i)\b(python|java|javascript|c\+\+|sql|html|css|react|angular|vue)\b")
                .unwrap(),
            Regex::new(r"(?i)\b(aws|azure|docker|kubernetes|git|linux|windows)\b").unwrap(),
        ];

        let technical_skills: usize = technical_patterns
            .iter()
            .map(|pattern| pattern.find_iter(content).count())
            .sum();

        // Soft skills patterns
        let soft_skills_keywords = [
            "leadership",
            "communication",
            "teamwork",
            "problem solving",
            "analytical",
        ];
        let soft_skills = soft_skills_keywords
            .iter()
            .map(|&keyword| content_lower.matches(keyword).count())
            .sum::<usize>();

        let total_skills = technical_skills + soft_skills;
        let skills_detected = total_skills as i32;

        let technical_skills_ratio = if total_skills > 0 {
            technical_skills as f64 / total_skills as f64
        } else {
            0.0
        };

        let soft_skills_ratio = if total_skills > 0 {
            soft_skills as f64 / total_skills as f64
        } else {
            0.0
        };

        let categorization_accuracy = 0.85;
        let skill_context_preservation = 0.8;

        let mut parsing_issues = Vec::new();
        if skills_detected == 0 {
            parsing_issues.push("No clear skills section detected".to_string());
        }

        SkillsParsingResult {
            skills_detected,
            categorization_accuracy,
            technical_skills_ratio,
            soft_skills_ratio,
            skill_context_preservation,
            parsing_issues,
        }
    }

    fn detect_formatting_issues(&self, content: &str) -> Vec<FormattingIssue> {
        let mut issues = Vec::new();

        // Check for excessive line breaks
        if content.matches("\n\n\n").count() > 0 {
            issues.push(FormattingIssue {
                issue_type: "excessive_whitespace".to_string(),
                description: "Multiple consecutive line breaks detected".to_string(),
                severity: "minor".to_string(),
                line_number: None,
                suggestion: "Use single line breaks between sections".to_string(),
            });
        }

        // Check for inconsistent bullet points
        let bullet_patterns = ["•", "◦", "*", "-", "▪"];
        let bullet_counts: Vec<usize> = bullet_patterns
            .iter()
            .map(|&pattern| content.matches(pattern).count())
            .collect();

        let different_bullets = bullet_counts.iter().filter(|&&count| count > 0).count();
        if different_bullets > 2 {
            issues.push(FormattingIssue {
                issue_type: "inconsistent_bullets".to_string(),
                description: "Multiple bullet point styles detected".to_string(),
                severity: "medium".to_string(),
                line_number: None,
                suggestion: "Use consistent bullet point style throughout".to_string(),
            });
        }

        // Check for potential encoding issues
        if content.contains("�") {
            issues.push(FormattingIssue {
                issue_type: "encoding_issue".to_string(),
                description: "Character encoding issues detected".to_string(),
                severity: "high".to_string(),
                line_number: None,
                suggestion: "Save document with UTF-8 encoding".to_string(),
            });
        }

        issues
    }

    // Additional helper methods would continue here...
    // For brevity, I'm including key methods and placeholders for others

    fn calculate_structure_clarity(&self, _content: &str, sections: &HashMap<String, bool>) -> f64 {
        let total_sections = sections.len() as f64;
        let detected_sections = sections.values().filter(|&&detected| detected).count() as f64;

        if total_sections > 0.0 {
            detected_sections / total_sections
        } else {
            0.0
        }
    }

    fn identify_keyword_section(&self, content: &str, position: usize) -> String {
        let before_keyword = &content[..position];
        let lines_before: Vec<&str> = before_keyword.lines().collect();

        // Look for section headers in the preceding lines
        for line in lines_before.iter().rev().take(10) {
            let line_lower = line.to_lowercase();
            if line_lower.contains("experience") || line_lower.contains("work") {
                return "experience".to_string();
            } else if line_lower.contains("skill") {
                return "skills".to_string();
            } else if line_lower.contains("education") {
                return "education".to_string();
            } else if line_lower.contains("summary") || line_lower.contains("objective") {
                return "summary".to_string();
            }
        }

        "unknown".to_string()
    }

    fn check_file_format_compatibility(&self) -> HashMap<String, bool> {
        let mut compatibility = HashMap::new();

        // Simulate format compatibility checks
        compatibility.insert("pdf".to_string(), true);
        compatibility.insert("docx".to_string(), true);
        compatibility.insert("txt".to_string(), true);
        compatibility.insert("html".to_string(), false); // Usually not supported
        compatibility.insert("rtf".to_string(), true);

        compatibility
    }

    fn assess_layout_complexity(&self, content: &str) -> f64 {
        let line_count = content.lines().count();
        let avg_line_length = if line_count > 0 {
            content.len() as f64 / line_count as f64
        } else {
            0.0
        };

        // Simple complexity assessment based on length and structure
        if avg_line_length > 100.0 || line_count > 200 {
            0.7 // High complexity
        } else if avg_line_length > 60.0 || line_count > 100 {
            0.5 // Medium complexity
        } else {
            0.3 // Low complexity
        }
    }

    fn analyze_font_compatibility(&self, _content: &str) -> FontCompatibility {
        // Simplified font analysis
        FontCompatibility {
            standard_fonts_used: true,
            font_consistency: 0.95,
            readability_score: 0.9,
            problematic_fonts: vec![],
        }
    }

    fn analyze_graphics_elements(&self, content: &str) -> GraphicsAnalysis {
        let has_graphics = content.to_lowercase().contains("image")
            || content.contains("[image]")
            || content.contains("graphic");

        GraphicsAnalysis {
            has_graphics,
            graphics_compatibility: if has_graphics { 0.3 } else { 1.0 },
            alt_text_present: false,
            graphics_impact: if has_graphics {
                "negative".to_string()
            } else {
                "neutral".to_string()
            },
            recommendations: if has_graphics {
                vec!["Remove images and graphics for better ATS compatibility".to_string()]
            } else {
                vec![]
            },
        }
    }

    fn analyze_table_usage(&self, content: &str) -> TableAnalysis {
        let table_indicators = content.matches("|").count() + content.matches("\t").count();
        let tables_detected = (table_indicators / 5).max(0) as i32; // Rough estimate

        TableAnalysis {
            tables_detected,
            table_compatibility: if tables_detected > 0 { 0.4 } else { 1.0 },
            parsing_difficulty: if tables_detected > 0 { 0.8 } else { 0.0 },
            alternative_suggestions: if tables_detected > 0 {
                vec![
                    "Convert tables to bulleted lists".to_string(),
                    "Use simple formatting instead of tables".to_string(),
                ]
            } else {
                vec![]
            },
        }
    }

    fn calculate_line_spacing(&self, content: &str) -> f64 {
        let lines = content.lines().collect::<Vec<_>>();
        let empty_lines = lines.iter().filter(|line| line.trim().is_empty()).count();
        let total_lines = lines.len();

        if total_lines > 0 {
            1.0 + (empty_lines as f64 / total_lines as f64)
        } else {
            1.0
        }
    }

    fn analyze_margins(&self, _content: &str) -> MarginAnalysis {
        // Simplified margin analysis
        MarginAnalysis {
            margin_consistency: 0.9,
            standard_margins: true,
            readability_impact: 0.95,
            recommendations: vec![],
        }
    }

    fn calculate_system_compatibility_score(
        &self,
        _system_config: &ATSSystemConfig,
        parsing_analysis: &ParsingAnalysis,
        format_analysis: &FormatAnalysis,
    ) -> f64 {
        let parsing_score = parsing_analysis.structure_clarity * 0.4;
        let format_score = (1.0 - format_analysis.layout_complexity) * 0.3;
        let extraction_score = parsing_analysis
            .contact_info_extraction
            .extraction_confidence
            * 0.3;

        parsing_score + format_score + extraction_score
    }

    fn calculate_parsing_success_rate(
        &self,
        _system_config: &ATSSystemConfig,
        parsing_analysis: &ParsingAnalysis,
    ) -> f64 {
        let sections_detected = parsing_analysis
            .section_detection
            .values()
            .filter(|&&detected| detected)
            .count() as f64;
        let total_sections = parsing_analysis.section_detection.len() as f64;

        if total_sections > 0.0 {
            sections_detected / total_sections
        } else {
            0.0
        }
    }

    fn simulate_section_extraction(
        &self,
        _system_config: &ATSSystemConfig,
        parsing_analysis: &ParsingAnalysis,
    ) -> HashMap<String, ExtractionQuality> {
        let mut extracted_sections = HashMap::new();

        for (section_name, detected) in &parsing_analysis.section_detection {
            let quality = if *detected {
                ExtractionQuality {
                    accuracy: 0.85,
                    completeness: 0.8,
                    structure_preservation: 0.9,
                    issues: vec![],
                }
            } else {
                ExtractionQuality {
                    accuracy: 0.0,
                    completeness: 0.0,
                    structure_preservation: 0.0,
                    issues: vec![format!("Section '{}' not detected", section_name)],
                }
            };
            extracted_sections.insert(section_name.clone(), quality);
        }

        extracted_sections
    }

    fn extract_keywords(&self, content: &str) -> Vec<String> {
        // Simple keyword extraction - this could be enhanced
        content
            .split_whitespace()
            .map(|word| {
                word.to_lowercase()
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .collect()
            })
            .filter(|word: &String| word.len() > 2)
            .collect()
    }

    fn assess_format_compliance(
        &self,
        _system_config: &ATSSystemConfig,
        format_analysis: &FormatAnalysis,
    ) -> FormatCompliance {
        let mut violations = Vec::new();
        let mut compliance_score: f64 = 1.0;

        if format_analysis.graphics_elements.has_graphics {
            violations.push("Graphics detected - may cause parsing issues".to_string());
            compliance_score -= 0.2;
        }

        if format_analysis.table_usage.tables_detected > 0 {
            violations.push("Tables detected - may not parse correctly".to_string());
            compliance_score -= 0.15;
        }

        if format_analysis.layout_complexity > 0.6 {
            violations.push("Complex layout detected".to_string());
            compliance_score -= 0.1;
        }

        let meets_standards = violations.is_empty();
        compliance_score = compliance_score.max(0.0);

        let recommendations = if !meets_standards {
            vec![
                "Simplify document layout".to_string(),
                "Remove graphics and images".to_string(),
                "Convert tables to bulleted lists".to_string(),
            ]
        } else {
            vec![]
        };

        FormatCompliance {
            meets_standards,
            compliance_score,
            violations,
            recommendations,
        }
    }

    fn identify_system_specific_issues(
        &self,
        system_config: &ATSSystemConfig,
        parsing_analysis: &ParsingAnalysis,
        format_analysis: &FormatAnalysis,
    ) -> Vec<String> {
        let mut issues = Vec::new();

        // Check against system capabilities
        if !system_config.parsing_capabilities.table_parsing
            && format_analysis.table_usage.tables_detected > 0
        {
            issues.push(format!(
                "{} has limited table parsing capabilities",
                system_config.system_name
            ));
        }

        if !system_config.parsing_capabilities.multi_column_support
            && format_analysis.layout_complexity > 0.5
        {
            issues.push(format!(
                "{} may struggle with multi-column layouts",
                system_config.system_name
            ));
        }

        if parsing_analysis
            .contact_info_extraction
            .extraction_confidence
            < 0.7
        {
            issues.push(format!(
                "Contact information may not be extracted correctly by {}",
                system_config.system_name
            ));
        }

        issues
    }

    fn generate_system_recommendations(
        &self,
        system_config: &ATSSystemConfig,
        issues: &[String],
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        if !issues.is_empty() {
            recommendations.extend(system_config.optimization_tips.clone());
        }

        // Add general recommendations
        recommendations.extend(vec![
            "Use standard fonts like Arial or Times New Roman".to_string(),
            "Keep formatting simple and consistent".to_string(),
            "Use clear section headers".to_string(),
            "Include relevant keywords naturally in context".to_string(),
        ]);

        recommendations
    }

    fn calculate_overall_ats_score(
        &self,
        system_simulations: &HashMap<String, ATSSystemResult>,
    ) -> f64 {
        if system_simulations.is_empty() {
            return 0.0;
        }

        let total_score: f64 = system_simulations
            .values()
            .map(|result| result.compatibility_score)
            .sum();

        total_score / system_simulations.len() as f64
    }

    fn generate_optimization_recommendations(
        &self,
        parsing_analysis: &ParsingAnalysis,
        format_analysis: &FormatAnalysis,
        keyword_extraction: &KeywordExtractionResult,
        _system_simulations: &HashMap<String, ATSSystemResult>,
    ) -> Vec<ATSOptimizationRecommendation> {
        let mut recommendations = Vec::new();

        // Keywords optimization
        if keyword_extraction.extraction_accuracy < 0.8 {
            recommendations.push(ATSOptimizationRecommendation {
                category: "Keywords".to_string(),
                priority: "high".to_string(),
                title: "Improve Keyword Integration".to_string(),
                description: "Include more relevant keywords naturally throughout your resume"
                    .to_string(),
                implementation_steps: vec![
                    "Review job description for key terms".to_string(),
                    "Integrate keywords in context, not as lists".to_string(),
                    "Use industry-standard terminology".to_string(),
                ],
                expected_improvement: 15.0,
                affected_systems: vec!["All ATS systems".to_string()],
                examples: vec![
                    "Instead of 'coded', use 'developed software applications'".to_string()
                ],
            });
        }

        // Format optimization
        if format_analysis.graphics_elements.has_graphics {
            recommendations.push(ATSOptimizationRecommendation {
                category: "Format".to_string(),
                priority: "critical".to_string(),
                title: "Remove Graphics and Images".to_string(),
                description: "Graphics can prevent proper parsing by ATS systems".to_string(),
                implementation_steps: vec![
                    "Remove all images, logos, and graphics".to_string(),
                    "Replace visual elements with text descriptions".to_string(),
                    "Use text-based formatting only".to_string(),
                ],
                expected_improvement: 25.0,
                affected_systems: vec!["Most ATS systems".to_string()],
                examples: vec!["Remove header logos and profile pictures".to_string()],
            });
        }

        // Structure optimization
        if parsing_analysis.structure_clarity < 0.7 {
            recommendations.push(ATSOptimizationRecommendation {
                category: "Structure".to_string(),
                priority: "high".to_string(),
                title: "Improve Document Structure".to_string(),
                description: "Clear section headers help ATS systems parse your resume correctly"
                    .to_string(),
                implementation_steps: vec![
                    "Use standard section headers (Experience, Education, Skills)".to_string(),
                    "Maintain consistent formatting throughout".to_string(),
                    "Use clear hierarchy with headers and bullet points".to_string(),
                ],
                expected_improvement: 20.0,
                affected_systems: vec!["All ATS systems".to_string()],
                examples: vec!["Use 'Work Experience' instead of 'My Journey'".to_string()],
            });
        }

        recommendations
    }

    fn identify_compatibility_issues(
        &self,
        parsing_analysis: &ParsingAnalysis,
        format_analysis: &FormatAnalysis,
        system_simulations: &HashMap<String, ATSSystemResult>,
    ) -> Vec<CompatibilityIssue> {
        let mut issues = Vec::new();

        // Critical issues
        if format_analysis.graphics_elements.has_graphics {
            issues.push(CompatibilityIssue {
                severity: "critical".to_string(),
                issue_type: "graphics".to_string(),
                description: "Graphics and images detected - will likely cause parsing failures"
                    .to_string(),
                affected_systems: vec![
                    "Greenhouse".to_string(),
                    "Lever".to_string(),
                    "Workday".to_string(),
                ],
                impact_score: 0.8,
                resolution_difficulty: "easy".to_string(),
                fix_suggestions: vec![
                    "Remove all images and graphics".to_string(),
                    "Save as plain text format if necessary".to_string(),
                ],
            });
        }

        // Major issues
        if parsing_analysis
            .contact_info_extraction
            .extraction_confidence
            < 0.5
        {
            issues.push(CompatibilityIssue {
                severity: "major".to_string(),
                issue_type: "contact_info".to_string(),
                description: "Contact information may not be properly extracted".to_string(),
                affected_systems: vec!["All systems".to_string()],
                impact_score: 0.6,
                resolution_difficulty: "medium".to_string(),
                fix_suggestions: vec![
                    "Use standard email and phone formats".to_string(),
                    "Place contact info at the top of resume".to_string(),
                    "Use clear labels for contact information".to_string(),
                ],
            });
        }

        // System-specific issues
        for (system_name, result) in system_simulations {
            if result.compatibility_score < 0.6 {
                issues.push(CompatibilityIssue {
                    severity: "major".to_string(),
                    issue_type: "system_compatibility".to_string(),
                    description: format!("Low compatibility with {} ATS system", system_name),
                    affected_systems: vec![system_name.clone()],
                    impact_score: 0.7,
                    resolution_difficulty: "medium".to_string(),
                    fix_suggestions: result.recommendations.clone(),
                });
            }
        }

        issues
    }

    // Static configuration builders
    fn build_ats_systems_config() -> HashMap<String, ATSSystemConfig> {
        let mut systems = HashMap::new();

        // Greenhouse ATS
        systems.insert(
            "greenhouse".to_string(),
            ATSSystemConfig {
                system_name: "Greenhouse".to_string(),
                parsing_capabilities: ParsingCapabilities {
                    pdf_support: true,
                    docx_support: true,
                    txt_support: true,
                    html_support: false,
                    image_text_extraction: false,
                    table_parsing: false,
                    multi_column_support: false,
                    header_footer_handling: true,
                },
                format_preferences: FormatPreferences {
                    preferred_fonts: vec!["Arial".to_string(), "Times New Roman".to_string()],
                    max_pages: Some(2),
                    preferred_margins: "1 inch".to_string(),
                    bullet_point_style: vec!["•".to_string(), "-".to_string()],
                    date_format_preferences: vec!["MM/YYYY".to_string(), "Month YYYY".to_string()],
                    section_header_style: "Bold".to_string(),
                },
                keyword_matching: KeywordMatchingConfig {
                    exact_match_weight: 1.0,
                    partial_match_weight: 0.7,
                    synonym_support: true,
                    case_sensitive: false,
                    context_analysis: true,
                    frequency_consideration: true,
                },
                scoring_algorithm: ScoringAlgorithm {
                    algorithm_type: "weighted".to_string(),
                    weights: [
                        ("keywords".to_string(), 0.3),
                        ("experience".to_string(), 0.25),
                        ("education".to_string(), 0.2),
                        ("skills".to_string(), 0.15),
                        ("format".to_string(), 0.1),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                    keyword_importance: 0.3,
                    experience_importance: 0.25,
                    education_importance: 0.2,
                    skills_importance: 0.15,
                    format_penalty_factor: 0.1,
                },
                known_limitations: vec![
                    "Limited table parsing".to_string(),
                    "No image text extraction".to_string(),
                    "Struggles with complex layouts".to_string(),
                ],
                optimization_tips: vec![
                    "Use standard section headers".to_string(),
                    "Avoid tables and graphics".to_string(),
                    "Keep formatting simple".to_string(),
                ],
            },
        );

        // Lever ATS
        systems.insert(
            "lever".to_string(),
            ATSSystemConfig {
                system_name: "Lever".to_string(),
                parsing_capabilities: ParsingCapabilities {
                    pdf_support: true,
                    docx_support: true,
                    txt_support: true,
                    html_support: false,
                    image_text_extraction: false,
                    table_parsing: true,
                    multi_column_support: false,
                    header_footer_handling: true,
                },
                format_preferences: FormatPreferences {
                    preferred_fonts: vec!["Arial".to_string(), "Helvetica".to_string()],
                    max_pages: Some(3),
                    preferred_margins: "0.75 inch".to_string(),
                    bullet_point_style: vec!["•".to_string()],
                    date_format_preferences: vec!["YYYY-MM".to_string(), "Month YYYY".to_string()],
                    section_header_style: "Bold and Underlined".to_string(),
                },
                keyword_matching: KeywordMatchingConfig {
                    exact_match_weight: 0.9,
                    partial_match_weight: 0.6,
                    synonym_support: true,
                    case_sensitive: false,
                    context_analysis: false,
                    frequency_consideration: true,
                },
                scoring_algorithm: ScoringAlgorithm {
                    algorithm_type: "machine_learning".to_string(),
                    weights: [
                        ("keywords".to_string(), 0.35),
                        ("experience".to_string(), 0.3),
                        ("skills".to_string(), 0.2),
                        ("education".to_string(), 0.1),
                        ("format".to_string(), 0.05),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                    keyword_importance: 0.35,
                    experience_importance: 0.3,
                    education_importance: 0.1,
                    skills_importance: 0.2,
                    format_penalty_factor: 0.05,
                },
                known_limitations: vec![
                    "Limited context analysis".to_string(),
                    "No multi-column support".to_string(),
                ],
                optimization_tips: vec![
                    "Focus on keyword density".to_string(),
                    "Use action verbs consistently".to_string(),
                ],
            },
        );

        // Add more ATS systems as needed...
        systems.insert(
            "workday".to_string(),
            ATSSystemConfig {
                system_name: "Workday".to_string(),
                parsing_capabilities: ParsingCapabilities {
                    pdf_support: true,
                    docx_support: true,
                    txt_support: true,
                    html_support: true,
                    image_text_extraction: true,
                    table_parsing: true,
                    multi_column_support: true,
                    header_footer_handling: true,
                },
                format_preferences: FormatPreferences {
                    preferred_fonts: vec!["Arial".to_string(), "Calibri".to_string()],
                    max_pages: Some(4),
                    preferred_margins: "1 inch".to_string(),
                    bullet_point_style: vec!["•".to_string(), "◦".to_string()],
                    date_format_preferences: vec![
                        "MM/DD/YYYY".to_string(),
                        "Month DD, YYYY".to_string(),
                    ],
                    section_header_style: "Bold".to_string(),
                },
                keyword_matching: KeywordMatchingConfig {
                    exact_match_weight: 1.0,
                    partial_match_weight: 0.8,
                    synonym_support: true,
                    case_sensitive: false,
                    context_analysis: true,
                    frequency_consideration: false,
                },
                scoring_algorithm: ScoringAlgorithm {
                    algorithm_type: "advanced_ml".to_string(),
                    weights: [
                        ("experience".to_string(), 0.4),
                        ("skills".to_string(), 0.25),
                        ("keywords".to_string(), 0.2),
                        ("education".to_string(), 0.1),
                        ("format".to_string(), 0.05),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                    keyword_importance: 0.2,
                    experience_importance: 0.4,
                    education_importance: 0.1,
                    skills_importance: 0.25,
                    format_penalty_factor: 0.05,
                },
                known_limitations: vec!["Complex parsing may miss nuances".to_string()],
                optimization_tips: vec![
                    "Focus on quantified achievements".to_string(),
                    "Use industry-standard terminology".to_string(),
                ],
            },
        );

        systems
    }

    fn build_parsing_patterns() -> HashMap<String, Vec<Regex>> {
        let mut patterns = HashMap::new();

        patterns.insert(
            "email".to_string(),
            vec![Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap()],
        );

        patterns.insert(
            "phone".to_string(),
            vec![
                Regex::new(r"(\+?1[-.\s]?)?\(?([0-9]{3})\)?[-.\s]?([0-9]{3})[-.\s]?([0-9]{4})")
                    .unwrap(),
            ],
        );

        patterns.insert(
            "dates".to_string(),
            vec![
                Regex::new(r"\d{4}\s*[-–]\s*\d{4}").unwrap(),
                Regex::new(r"\d{4}\s*[-–]\s*present").unwrap(),
                Regex::new(r"(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)\s+\d{4}").unwrap(),
            ],
        );

        patterns
    }

    fn build_format_checkers() -> HashMap<String, Regex> {
        let mut checkers = HashMap::new();

        checkers.insert(
            "bullet_consistency".to_string(),
            Regex::new(r"[•◦▪▫‣⁃*-]").unwrap(),
        );

        checkers.insert(
            "section_headers".to_string(),
            Regex::new(
                r"(?i)^(experience|education|skills|summary|objective|certifications|projects)",
            )
            .unwrap(),
        );

        checkers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;

    async fn setup_test_simulator() -> ATSSimulator {
        let db = Database::new().await.unwrap();
        ATSSimulator::new(db)
    }

    #[tokio::test]
    async fn test_ats_simulation_basic() {
        let simulator = setup_test_simulator().await;
        let resume_content = "John Doe\njohn@email.com\n(555) 123-4567\n\nExperience:\nSoftware Engineer at TechCorp\n2020-2023\n\nEducation:\nB.S. Computer Science\nTech University, 2020";
        let keywords = vec![
            "software engineer".to_string(),
            "computer science".to_string(),
        ];

        let result = simulator
            .simulate_ats_processing(resume_content, &keywords)
            .await;
        assert!(result.is_ok());

        let simulation = result.unwrap();
        assert!(simulation.overall_ats_score >= 0.0);
        assert!(!simulation.system_simulations.is_empty());
        assert!(
            simulation
                .parsing_analysis
                .contact_info_extraction
                .email_detected
        );
    }

    #[tokio::test]
    async fn test_keyword_extraction_simulation() {
        let simulator = setup_test_simulator().await;
        let resume_content = "Python developer with React experience";
        let keywords = vec![
            "Python".to_string(),
            "React".to_string(),
            "Node.js".to_string(),
        ];

        let result = simulator.simulate_keyword_extraction(resume_content, &keywords);

        assert_eq!(result.keywords_found.len(), 2); // Python and React found
        assert_eq!(result.missed_keywords.len(), 1); // Node.js missed
        assert!(result.extraction_accuracy > 0.5);
    }

    #[tokio::test]
    async fn test_format_analysis() {
        let simulator = setup_test_simulator().await;
        let resume_content = "Simple text resume without graphics or tables";

        let format_analysis = simulator.analyze_format_compatibility(resume_content);

        assert!(!format_analysis.graphics_elements.has_graphics);
        assert_eq!(format_analysis.table_usage.tables_detected, 0);
        assert!(format_analysis.layout_complexity < 0.5);
    }
}
