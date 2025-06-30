use anyhow::{anyhow, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatCompatibilityReport {
    pub overall_score: f64,
    pub ats_specific_scores: HashMap<String, f64>,
    pub format_issues: Vec<FormatIssue>,
    pub recommendations: Vec<String>,
    pub parsing_simulation: ParsingSimulation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatIssue {
    pub issue_type: String,
    pub severity: String, // "critical", "high", "medium", "low"
    pub description: String,
    pub recommendation: String,
    pub section_affected: String,
    pub impact_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsingSimulation {
    pub successful_sections: Vec<String>,
    pub failed_sections: Vec<String>,
    pub extraction_accuracy: f64,
    pub predicted_parsing_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatRule {
    pub rule_type: String,
    pub pattern: String,
    pub penalty_weight: f64,
    pub description: String,
    pub recommendation: String,
}

pub struct FormatCompatibilityChecker {
    ats_rules: HashMap<String, Vec<FormatRule>>,
    font_patterns: Vec<Regex>,
    structure_patterns: Vec<Regex>,
    problematic_elements: Vec<Regex>,
}

impl FormatCompatibilityChecker {
    pub fn new() -> Self {
        let mut checker = Self {
            ats_rules: HashMap::new(),
            font_patterns: Vec::new(),
            structure_patterns: Vec::new(),
            problematic_elements: Vec::new(),
        };
        
        checker.initialize_ats_rules();
        checker.initialize_patterns();
        checker
    }

    pub fn check_comprehensive_compatibility(&self, content: &str) -> Result<FormatCompatibilityReport> {
        info!("Starting comprehensive format compatibility check");
        
        let mut issues = Vec::new();
        let mut scores = HashMap::new();

        // Check against each ATS system's format requirements
        for (ats_name, rules) in &self.ats_rules {
            let (score, ats_issues) = self.check_ats_specific_format(content, rules);
            scores.insert(ats_name.clone(), score);
            issues.extend(ats_issues);
        }

        // Check for general problematic elements
        issues.extend(self.check_problematic_elements(content));

        // Simulate parsing process
        let parsing_simulation = self.simulate_parsing_process(content);

        // Calculate overall score
        let overall_score = if scores.is_empty() {
            0.0
        } else {
            scores.values().sum::<f64>() / scores.len() as f64
        };

        // Generate recommendations
        let recommendations = self.generate_format_recommendations(&issues);

        Ok(FormatCompatibilityReport {
            overall_score,
            ats_specific_scores: scores,
            format_issues: issues,
            recommendations,
            parsing_simulation,
        })
    }

    fn check_ats_specific_format(&self, content: &str, rules: &[FormatRule]) -> (f64, Vec<FormatIssue>) {
        let mut score = 100.0;
        let mut issues = Vec::new();

        for rule in rules {
            if let Ok(regex) = Regex::new(&rule.pattern) {
                if regex.is_match(content) {
                    score -= rule.penalty_weight;
                    issues.push(FormatIssue {
                        issue_type: rule.rule_type.clone(),
                        severity: self.determine_severity(rule.penalty_weight),
                        description: rule.description.clone(),
                        recommendation: rule.recommendation.clone(),
                        section_affected: self.identify_affected_section(content, &regex),
                        impact_score: rule.penalty_weight,
                    });
                }
            }
        }

        (score.max(0.0), issues)
    }

    fn check_problematic_elements(&self, content: &str) -> Vec<FormatIssue> {
        let mut issues = Vec::new();

        // Check for tables (ATS enemy #1)
        if self.contains_tables(content) {
            issues.push(FormatIssue {
                issue_type: "tables".to_string(),
                severity: "critical".to_string(),
                description: "Tables can cause parsing errors in ATS systems".to_string(),
                recommendation: "Convert table content to plain text with clear headings".to_string(),
                section_affected: "document_structure".to_string(),
                impact_score: 25.0,
            });
        }

        // Check for text boxes
        if self.contains_text_boxes(content) {
            issues.push(FormatIssue {
                issue_type: "text_boxes".to_string(),
                severity: "high".to_string(),
                description: "Text boxes are often ignored by ATS parsers".to_string(),
                recommendation: "Move content from text boxes to main document body".to_string(),
                section_affected: "document_structure".to_string(),
                impact_score: 20.0,
            });
        }

        // Check for images with text
        if self.contains_text_in_images(content) {
            issues.push(FormatIssue {
                issue_type: "text_in_images".to_string(),
                severity: "critical".to_string(),
                description: "Text within images cannot be read by ATS systems".to_string(),
                recommendation: "Convert all text in images to regular text in the document".to_string(),
                section_affected: "document_structure".to_string(),
                impact_score: 30.0,
            });
        }

        // Check for complex formatting
        if self.has_complex_formatting(content) {
            issues.push(FormatIssue {
                issue_type: "complex_formatting".to_string(),
                severity: "medium".to_string(),
                description: "Complex formatting may not be preserved in ATS parsing".to_string(),
                recommendation: "Use simple, clean formatting with clear section headers".to_string(),
                section_affected: "formatting".to_string(),
                impact_score: 10.0,
            });
        }

        // Check for non-standard fonts
        let problematic_fonts = self.check_fonts(content);
        for font in problematic_fonts {
            issues.push(FormatIssue {
                issue_type: "non_standard_font".to_string(),
                severity: "medium".to_string(),
                description: format!("Font '{}' may not be ATS-friendly", font),
                recommendation: "Use standard fonts like Arial, Calibri, or Times New Roman".to_string(),
                section_affected: "formatting".to_string(),
                impact_score: 5.0,
            });
        }

        // Check for special characters
        if self.has_problematic_characters(content) {
            issues.push(FormatIssue {
                issue_type: "special_characters".to_string(),
                severity: "low".to_string(),
                description: "Special characters may not parse correctly in all ATS systems".to_string(),
                recommendation: "Replace special characters with standard equivalents".to_string(),
                section_affected: "content".to_string(),
                impact_score: 3.0,
            });
        }

        // Check section structure
        if !self.has_clear_section_headers(content) {
            issues.push(FormatIssue {
                issue_type: "unclear_sections".to_string(),
                severity: "medium".to_string(),
                description: "Document lacks clear section headers for ATS parsing".to_string(),
                recommendation: "Add clear, standard section headers (Experience, Education, Skills, etc.)".to_string(),
                section_affected: "structure".to_string(),
                impact_score: 15.0,
            });
        }

        issues
    }

    fn simulate_parsing_process(&self, content: &str) -> ParsingSimulation {
        let standard_sections = vec![
            "contact", "summary", "experience", "education", "skills", "certifications"
        ];
        
        let mut successful_sections = Vec::new();
        let mut failed_sections = Vec::new();
        
        for section in &standard_sections {
            if self.can_identify_section(content, section) {
                successful_sections.push(section.to_string());
            } else {
                failed_sections.push(section.to_string());
            }
        }
        
        let extraction_accuracy = successful_sections.len() as f64 / standard_sections.len() as f64;
        
        // Calculate predicted parsing score based on format issues
        let format_penalty = self.calculate_format_penalty(content);
        let predicted_parsing_score = (extraction_accuracy * 100.0 - format_penalty).max(0.0);
        
        ParsingSimulation {
            successful_sections,
            failed_sections,
            extraction_accuracy,
            predicted_parsing_score,
        }
    }

    fn generate_format_recommendations(&self, issues: &[FormatIssue]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Group issues by type and generate consolidated recommendations
        let mut issue_groups: HashMap<String, Vec<&FormatIssue>> = HashMap::new();
        for issue in issues {
            issue_groups.entry(issue.issue_type.clone()).or_insert_with(Vec::new).push(issue);
        }
        
        // Priority order for recommendations
        let priority_order = vec![
            "text_in_images", "tables", "text_boxes", "unclear_sections", 
            "complex_formatting", "non_standard_font", "special_characters"
        ];
        
        for issue_type in priority_order {
            if let Some(type_issues) = issue_groups.get(issue_type) {
                if !type_issues.is_empty() {
                    recommendations.push(type_issues[0].recommendation.clone());
                }
            }
        }
        
        // Add general recommendations if any critical issues exist
        let has_critical_issues = issues.iter().any(|i| i.severity == "critical");
        if has_critical_issues {
            recommendations.push("Consider recreating the resume using a simple template to avoid parsing issues".to_string());
        }
        
        // Add positive recommendations
        if issues.is_empty() {
            recommendations.push("Your resume format is well-optimized for ATS systems".to_string());
        } else if issues.iter().all(|i| i.severity == "low") {
            recommendations.push("Your resume has good ATS compatibility with minor improvements needed".to_string());
        }
        
        recommendations
    }

    // Helper methods for specific checks
    fn contains_tables(&self, content: &str) -> bool {
        // Check for common table indicators
        let table_patterns = vec![
            r"<table",
            r"<tr>",
            r"<td>",
            r"\|\s*\|", // ASCII table borders
            r"┌.*┐", // Unicode table borders
            r"│.*│",
        ];
        
        table_patterns.iter().any(|pattern| {
            Regex::new(pattern).map(|re| re.is_match(content)).unwrap_or(false)
        })
    }

    fn contains_text_boxes(&self, content: &str) -> bool {
        // Check for text box indicators
        let textbox_patterns = vec![
            r"<textbox",
            r"text-box",
            r"textBox",
            r"floating.*text",
        ];
        
        textbox_patterns.iter().any(|pattern| {
            Regex::new(pattern).map(|re| re.is_match(content)).unwrap_or(false)
        })
    }

    fn contains_text_in_images(&self, content: &str) -> bool {
        // This is harder to detect from text content alone
        // We can check for image tags and warn about potential text in images
        let image_patterns = vec![
            r"<img",
            r"\[image\]",
            r"\.jpg",
            r"\.png",
            r"\.gif",
            r"\.jpeg",
        ];
        
        image_patterns.iter().any(|pattern| {
            Regex::new(pattern).map(|re| re.is_match(content)).unwrap_or(false)
        })
    }

    fn has_complex_formatting(&self, content: &str) -> bool {
        let complex_patterns = vec![
            r"<style",
            r"text-align:\s*justify",
            r"columns:\s*\d+",
            r"float:\s*(left|right)",
            r"position:\s*absolute",
        ];
        
        complex_patterns.iter().any(|pattern| {
            Regex::new(pattern).map(|re| re.is_match(content)).unwrap_or(false)
        })
    }

    fn check_fonts(&self, content: &str) -> Vec<String> {
        let mut problematic_fonts = Vec::new();
        
        // Common problematic fonts for ATS
        let problematic_font_names = vec![
            "Comic Sans", "Papyrus", "Brush Script", "Chalkduster", 
            "Fantasy", "Decorative", "Script", "Handwriting"
        ];
        
        for font in &problematic_font_names {
            if content.to_lowercase().contains(&font.to_lowercase()) {
                problematic_fonts.push(font.to_string());
            }
        }
        
        problematic_fonts
    }

    fn has_problematic_characters(&self, content: &str) -> bool {
        // Check for characters that might cause parsing issues
        let problematic_chars = vec![
            "•", "→", "←", "↑", "↓", "★", "♦", "♣", "♠", "♥",
            "✓", "✗", "⚫", "⚪", "◆", "◇", "■", "□", "▲", "▼"
        ];
        
        problematic_chars.iter().any(|char| content.contains(char))
    }

    fn has_clear_section_headers(&self, content: &str) -> bool {
        let standard_headers = vec![
            "experience", "education", "skills", "summary", "contact",
            "work experience", "employment", "qualifications", "background"
        ];
        
        let found_headers = standard_headers.iter()
            .filter(|header| {
                let pattern = format!(r"(?i)\b{}\b", regex::escape(header));
                Regex::new(&pattern).map(|re| re.is_match(content)).unwrap_or(false)
            })
            .count();
        
        found_headers >= 3 // At least 3 standard sections should be identifiable
    }

    fn can_identify_section(&self, content: &str, section: &str) -> bool {
        let section_patterns = match section {
            "contact" => vec![r"@\w+\.\w+", r"\(\d{3}\)", r"\d{3}-\d{3}-\d{4}"],
            "summary" => vec![r"(?i)\bsummary\b", r"(?i)\bobjective\b", r"(?i)\bprofile\b"],
            "experience" => vec![r"(?i)\bexperience\b", r"(?i)\bemployment\b", r"(?i)\bwork history\b"],
            "education" => vec![r"(?i)\beducation\b", r"(?i)\bdegree\b", r"(?i)\buniversity\b"],
            "skills" => vec![r"(?i)\bskills\b", r"(?i)\btechnical\b", r"(?i)\bcompetencies\b"],
            "certifications" => vec![r"(?i)\bcertification\b", r"(?i)\bcertified\b", r"(?i)\blicense\b"],
            _ => vec![""],
        };
        
        section_patterns.iter().any(|pattern| {
            Regex::new(pattern).map(|re| re.is_match(content)).unwrap_or(false)
        })
    }

    fn calculate_format_penalty(&self, content: &str) -> f64 {
        let mut penalty = 0.0;
        
        if self.contains_tables(content) { penalty += 25.0; }
        if self.contains_text_boxes(content) { penalty += 20.0; }
        if self.contains_text_in_images(content) { penalty += 30.0; }
        if self.has_complex_formatting(content) { penalty += 10.0; }
        if !self.has_clear_section_headers(content) { penalty += 15.0; }
        
        penalty
    }

    fn determine_severity(&self, penalty_weight: f64) -> String {
        match penalty_weight {
            p if p >= 20.0 => "critical".to_string(),
            p if p >= 10.0 => "high".to_string(),
            p if p >= 5.0 => "medium".to_string(),
            _ => "low".to_string(),
        }
    }

    fn identify_affected_section(&self, content: &str, regex: &Regex) -> String {
        // Try to identify which section of the resume is affected
        if let Some(matched) = regex.find(content) {
            let context = &content[matched.start().saturating_sub(100)..
                                  (matched.end() + 100).min(content.len())];
            
            if context.to_lowercase().contains("experience") {
                "experience".to_string()
            } else if context.to_lowercase().contains("education") {
                "education".to_string()
            } else if context.to_lowercase().contains("skills") {
                "skills".to_string()
            } else if context.to_lowercase().contains("summary") {
                "summary".to_string()
            } else {
                "general".to_string()
            }
        } else {
            "unknown".to_string()
        }
    }

    fn initialize_ats_rules(&mut self) {
        // Greenhouse rules
        let greenhouse_rules = vec![
            FormatRule {
                rule_type: "table_usage".to_string(),
                pattern: r"<table|<tr>|<td>".to_string(),
                penalty_weight: 25.0,
                description: "Greenhouse has difficulty parsing tables correctly".to_string(),
                recommendation: "Convert tables to plain text with clear formatting".to_string(),
            },
            FormatRule {
                rule_type: "special_characters".to_string(),
                pattern: r"[•★♦♣♠♥✓✗]".to_string(),
                penalty_weight: 5.0,
                description: "Special characters may not display correctly".to_string(),
                recommendation: "Use standard bullet points and symbols".to_string(),
            },
        ];
        self.ats_rules.insert("greenhouse".to_string(), greenhouse_rules);

        // Lever rules
        let lever_rules = vec![
            FormatRule {
                rule_type: "text_boxes".to_string(),
                pattern: r"<textbox|text-box|textBox".to_string(),
                penalty_weight: 20.0,
                description: "Lever often ignores content in text boxes".to_string(),
                recommendation: "Move all text to the main document body".to_string(),
            },
            FormatRule {
                rule_type: "complex_formatting".to_string(),
                pattern: r"columns:\s*\d+|float:\s*(left|right)".to_string(),
                penalty_weight: 15.0,
                description: "Complex layouts can confuse Lever's parser".to_string(),
                recommendation: "Use a single-column layout with simple formatting".to_string(),
            },
        ];
        self.ats_rules.insert("lever".to_string(), lever_rules);

        // Workday rules
        let workday_rules = vec![
            FormatRule {
                rule_type: "image_text".to_string(),
                pattern: r"<img|\.jpg|\.png|\.gif".to_string(),
                penalty_weight: 30.0,
                description: "Workday cannot extract text from images".to_string(),
                recommendation: "Ensure all text is in readable format, not embedded in images".to_string(),
            },
            FormatRule {
                rule_type: "font_issues".to_string(),
                pattern: r"(?i)(comic sans|papyrus|brush script)".to_string(),
                penalty_weight: 10.0,
                description: "Non-standard fonts may not render correctly".to_string(),
                recommendation: "Use professional fonts like Arial, Calibri, or Times New Roman".to_string(),
            },
        ];
        self.ats_rules.insert("workday".to_string(), workday_rules);

        // iCIMS rules
        let icims_rules = vec![
            FormatRule {
                rule_type: "section_headers".to_string(),
                pattern: r"^(?!.*(?i)(experience|education|skills|summary))".to_string(),
                penalty_weight: 15.0,
                description: "iCIMS relies heavily on clear section headers".to_string(),
                recommendation: "Use standard section headers that are clearly identifiable".to_string(),
            },
        ];
        self.ats_rules.insert("icims".to_string(), icims_rules);
    }

    fn initialize_patterns(&mut self) {
        // Initialize regex patterns for various checks
        self.font_patterns = vec![
            Regex::new(r"(?i)font-family:\s*([a-zA-Z\s]+)").unwrap(),
            Regex::new(r"(?i)font:\s*[^;]*?([a-zA-Z\s]+);").unwrap(),
        ];

        self.structure_patterns = vec![
            Regex::new(r"(?i)^(experience|education|skills|summary|contact)").unwrap(),
            Regex::new(r"\n\s*([A-Z\s]{3,})\s*\n").unwrap(), // All caps headers
        ];

        self.problematic_elements = vec![
            Regex::new(r"<table").unwrap(),
            Regex::new(r"<img").unwrap(),
            Regex::new(r"text-box").unwrap(),
            Regex::new(r"position:\s*absolute").unwrap(),
        ];
    }
}

impl Default for FormatCompatibilityChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_detection() {
        let checker = FormatCompatibilityChecker::new();
        
        let content_with_table = "<table><tr><td>Name</td><td>Value</td></tr></table>";
        assert!(checker.contains_tables(content_with_table));
        
        let content_without_table = "This is regular text content without tables.";
        assert!(!checker.contains_tables(content_without_table));
    }

    #[test]
    fn test_section_identification() {
        let checker = FormatCompatibilityChecker::new();
        
        let content = "EXPERIENCE\nSoftware Engineer at Company\nEDUCATION\nBachelor's Degree";
        assert!(checker.can_identify_section(content, "experience"));
        assert!(checker.can_identify_section(content, "education"));
        assert!(!checker.can_identify_section(content, "skills"));
    }

    #[test]
    fn test_format_compatibility_check() {
        let checker = FormatCompatibilityChecker::new();
        
        let good_content = "John Doe\njohn@email.com\n\nEXPERIENCE\nSoftware Engineer\n\nEDUCATION\nBS Computer Science";
        let result = checker.check_comprehensive_compatibility(good_content).unwrap();
        
        assert!(result.overall_score > 80.0);
        assert!(result.format_issues.len() < 3);
    }

    #[test]
    fn test_problematic_content() {
        let checker = FormatCompatibilityChecker::new();
        
        let bad_content = "<table><tr><td>Experience</td></tr></table><img src='test.jpg'>";
        let result = checker.check_comprehensive_compatibility(bad_content).unwrap();
        
        assert!(result.overall_score < 70.0);
        assert!(result.format_issues.len() > 0);
        
        let critical_issues: Vec<_> = result.format_issues.iter()
            .filter(|issue| issue.severity == "critical")
            .collect();
        assert!(critical_issues.len() > 0);
    }

    #[test]
    fn test_parsing_simulation() {
        let checker = FormatCompatibilityChecker::new();
        
        let content = "John Doe\nemail@test.com\n\nSUMMARY\nExperienced developer\n\nEXPERIENCE\nSoftware Engineer\n\nSKILLS\nPython, JavaScript";
        let simulation = checker.simulate_parsing_process(content);
        
        assert!(simulation.extraction_accuracy > 0.5);
        assert!(simulation.successful_sections.contains(&"contact".to_string()));
        assert!(simulation.successful_sections.contains(&"summary".to_string()));
        assert!(simulation.successful_sections.contains(&"experience".to_string()));
        assert!(simulation.successful_sections.contains(&"skills".to_string()));
    }
}