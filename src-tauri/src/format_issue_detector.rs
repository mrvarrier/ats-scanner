use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::warn;

use crate::format_checker::{FormatIssue, FormatCompatibilityReport};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatIssueReport {
    pub critical_issues: Vec<FormatIssue>,
    pub high_priority_issues: Vec<FormatIssue>,
    pub medium_priority_issues: Vec<FormatIssue>,
    pub low_priority_issues: Vec<FormatIssue>,
    pub overall_format_score: f64,
    pub improvement_recommendations: Vec<ImprovementRecommendation>,
    pub before_after_examples: Vec<BeforeAfterExample>,
    pub ats_specific_impacts: HashMap<String, ATSImpact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementRecommendation {
    pub category: String,
    pub title: String,
    pub description: String,
    pub priority: String,
    pub implementation_difficulty: String, // "easy", "medium", "hard"
    pub time_estimate: String, // "5 minutes", "30 minutes", "2 hours"
    pub step_by_step_guide: Vec<String>,
    pub tools_needed: Vec<String>,
    pub expected_improvement: f64,
    pub related_issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeforeAfterExample {
    pub issue_type: String,
    pub before_example: String,
    pub after_example: String,
    pub explanation: String,
    pub improvement_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ATSImpact {
    pub ats_name: String,
    pub compatibility_score: f64,
    pub parsing_issues: Vec<String>,
    pub keyword_detection_impact: f64,
    pub overall_impact: String, // "severe", "moderate", "minor", "none"
    pub specific_recommendations: Vec<String>,
}

pub struct FormatIssueDetector {
    issue_patterns: HashMap<String, IssuePattern>,
    ats_sensitivities: HashMap<String, ATSSensitivity>,
    recommendation_templates: HashMap<String, RecommendationTemplate>,
}

#[derive(Debug, Clone)]
struct IssuePattern {
    pattern_type: String,
    regex_patterns: Vec<Regex>,
    severity_calculator: fn(&str) -> String,
    impact_calculator: fn(&str) -> f64,
    description: String,
}

#[derive(Debug, Clone)]
struct ATSSensitivity {
    ats_name: String,
    sensitive_issues: Vec<String>,
    impact_multipliers: HashMap<String, f64>,
    parsing_limitations: Vec<String>,
}

#[derive(Debug, Clone)]
struct RecommendationTemplate {
    category: String,
    title_template: String,
    description_template: String,
    steps: Vec<String>,
    tools: Vec<String>,
    difficulty: String,
    time_estimate: String,
}

impl FormatIssueDetector {
    pub fn new() -> Self {
        let mut detector = Self {
            issue_patterns: HashMap::new(),
            ats_sensitivities: HashMap::new(),
            recommendation_templates: HashMap::new(),
        };
        
        detector.initialize_issue_patterns();
        detector.initialize_ats_sensitivities();
        detector.initialize_recommendation_templates();
        detector
    }

    pub fn analyze_format_issues(&self, content: &str, compatibility_report: &FormatCompatibilityReport) -> Result<FormatIssueReport> {
        // Categorize issues by severity
        let mut critical_issues = Vec::new();
        let mut high_priority_issues = Vec::new();
        let mut medium_priority_issues = Vec::new();
        let mut low_priority_issues = Vec::new();

        for issue in &compatibility_report.format_issues {
            match issue.severity.as_str() {
                "critical" => critical_issues.push(issue.clone()),
                "high" => high_priority_issues.push(issue.clone()),
                "medium" => medium_priority_issues.push(issue.clone()),
                _ => low_priority_issues.push(issue.clone()),
            }
        }

        // Calculate overall format score
        let overall_format_score = self.calculate_overall_format_score(&compatibility_report.format_issues);

        // Generate improvement recommendations
        let improvement_recommendations = self.generate_improvement_recommendations(&compatibility_report.format_issues);

        // Create before/after examples
        let before_after_examples = self.generate_before_after_examples(&compatibility_report.format_issues, content);

        // Analyze ATS-specific impacts
        let ats_specific_impacts = self.analyze_ats_specific_impacts(&compatibility_report.format_issues, &compatibility_report.ats_specific_scores);

        Ok(FormatIssueReport {
            critical_issues,
            high_priority_issues,
            medium_priority_issues,
            low_priority_issues,
            overall_format_score,
            improvement_recommendations,
            before_after_examples,
            ats_specific_impacts,
        })
    }

    pub fn detect_advanced_issues(&self, content: &str) -> Result<Vec<FormatIssue>> {
        let mut issues = Vec::new();

        // Detect advanced formatting issues
        issues.extend(self.detect_hidden_text_issues(content));
        issues.extend(self.detect_accessibility_issues(content));
        issues.extend(self.detect_encoding_issues(content));
        issues.extend(self.detect_metadata_issues(content));
        issues.extend(self.detect_style_inconsistencies(content));
        issues.extend(self.detect_section_structure_issues(content));

        Ok(issues)
    }

    fn calculate_overall_format_score(&self, issues: &[FormatIssue]) -> f64 {
        let mut score = 100.0;
        
        for issue in issues {
            score -= issue.impact_score;
        }
        
        score.max(0.0)
    }

    fn generate_improvement_recommendations(&self, issues: &[FormatIssue]) -> Vec<ImprovementRecommendation> {
        let mut recommendations = Vec::new();
        
        // Group issues by category
        let mut issue_groups: HashMap<String, Vec<&FormatIssue>> = HashMap::new();
        for issue in issues {
            issue_groups.entry(issue.issue_type.clone()).or_insert_with(Vec::new).push(issue);
        }

        for (category, category_issues) in issue_groups {
            if let Some(template) = self.recommendation_templates.get(&category) {
                let total_impact: f64 = category_issues.iter().map(|i| i.impact_score).sum();
                let priority = self.determine_priority(total_impact);
                
                recommendations.push(ImprovementRecommendation {
                    category: category.clone(),
                    title: template.title_template.replace("{category}", &category.replace('_', " ")),
                    description: template.description_template.clone(),
                    priority,
                    implementation_difficulty: template.difficulty.clone(),
                    time_estimate: template.time_estimate.clone(),
                    step_by_step_guide: template.steps.clone(),
                    tools_needed: template.tools.clone(),
                    expected_improvement: total_impact,
                    related_issues: category_issues.iter().map(|i| i.description.clone()).collect(),
                });
            }
        }

        // Sort by priority and expected improvement
        recommendations.sort_by(|a, b| {
            let priority_order = ["critical", "high", "medium", "low"];
            let a_priority = priority_order.iter().position(|&p| p == a.priority).unwrap_or(3);
            let b_priority = priority_order.iter().position(|&p| p == b.priority).unwrap_or(3);
            
            match a_priority.cmp(&b_priority) {
                std::cmp::Ordering::Equal => b.expected_improvement.partial_cmp(&a.expected_improvement).unwrap_or(std::cmp::Ordering::Equal),
                other => other,
            }
        });

        recommendations
    }

    fn generate_before_after_examples(&self, issues: &[FormatIssue], _content: &str) -> Vec<BeforeAfterExample> {
        let mut examples = Vec::new();

        for issue in issues.iter().take(5) { // Show top 5 examples
            let example = match issue.issue_type.as_str() {
                "tables" => BeforeAfterExample {
                    issue_type: "tables".to_string(),
                    before_example: "| Skills | Level |\n|--------|-------|\n| Python | Expert |\n| Java   | Intermediate |".to_string(),
                    after_example: "Technical Skills:\n• Python (Expert level)\n• Java (Intermediate level)".to_string(),
                    explanation: "Tables are converted to bullet points for better ATS parsing".to_string(),
                    improvement_score: 25.0,
                },
                "text_in_images" => BeforeAfterExample {
                    issue_type: "text_in_images".to_string(),
                    before_example: "[Image containing contact information]".to_string(),
                    after_example: "Contact Information:\nJohn Doe\nEmail: john.doe@email.com\nPhone: (555) 123-4567".to_string(),
                    explanation: "Text within images is converted to readable text format".to_string(),
                    improvement_score: 30.0,
                },
                "text_boxes" => BeforeAfterExample {
                    issue_type: "text_boxes".to_string(),
                    before_example: "[Text box: Key achievements and awards]".to_string(),
                    after_example: "Key Achievements:\n• Award for Excellence in Software Development\n• Led team of 5 developers".to_string(),
                    explanation: "Text box content is moved to the main document body".to_string(),
                    improvement_score: 20.0,
                },
                "non_standard_font" => BeforeAfterExample {
                    issue_type: "non_standard_font".to_string(),
                    before_example: "Resume content in Comic Sans MS font".to_string(),
                    after_example: "Resume content in Arial font".to_string(),
                    explanation: "Non-standard fonts are replaced with ATS-friendly alternatives".to_string(),
                    improvement_score: 10.0,
                },
                _ => BeforeAfterExample {
                    issue_type: issue.issue_type.clone(),
                    before_example: "Problematic formatting".to_string(),
                    after_example: "Improved formatting".to_string(),
                    explanation: issue.recommendation.clone(),
                    improvement_score: issue.impact_score,
                },
            };
            examples.push(example);
        }

        examples
    }

    fn analyze_ats_specific_impacts(&self, issues: &[FormatIssue], ats_scores: &HashMap<String, f64>) -> HashMap<String, ATSImpact> {
        let mut impacts = HashMap::new();

        for (ats_name, score) in ats_scores {
            let sensitivity = self.ats_sensitivities.get(ats_name);
            
            let mut parsing_issues = Vec::new();
            let mut keyword_impact = 0.0;
            let mut total_impact = 0.0;

            for issue in issues {
                if let Some(sens) = sensitivity {
                    if sens.sensitive_issues.contains(&issue.issue_type) {
                        parsing_issues.push(issue.description.clone());
                        
                        let multiplier = sens.impact_multipliers.get(&issue.issue_type).unwrap_or(&1.0);
                        total_impact += issue.impact_score * multiplier;
                        
                        if issue.issue_type == "tables" || issue.issue_type == "text_in_images" {
                            keyword_impact += issue.impact_score * 0.5;
                        }
                    }
                }
            }

            let overall_impact = match total_impact {
                i if i >= 30.0 => "severe",
                i if i >= 20.0 => "moderate", 
                i if i >= 10.0 => "minor",
                _ => "none",
            };

            let specific_recommendations = self.generate_ats_specific_recommendations(ats_name, issues);

            impacts.insert(ats_name.clone(), ATSImpact {
                ats_name: ats_name.clone(),
                compatibility_score: *score,
                parsing_issues,
                keyword_detection_impact: keyword_impact,
                overall_impact: overall_impact.to_string(),
                specific_recommendations,
            });
        }

        impacts
    }

    fn determine_priority(&self, impact_score: f64) -> String {
        match impact_score {
            i if i >= 25.0 => "critical".to_string(),
            i if i >= 15.0 => "high".to_string(),
            i if i >= 8.0 => "medium".to_string(),
            _ => "low".to_string(),
        }
    }

    fn generate_ats_specific_recommendations(&self, ats_name: &str, issues: &[FormatIssue]) -> Vec<String> {
        let mut recommendations = Vec::new();

        match ats_name {
            "greenhouse" => {
                if issues.iter().any(|i| i.issue_type == "tables") {
                    recommendations.push("Greenhouse has significant difficulty parsing tables. Convert all tabular data to bullet points or plain text lists.".to_string());
                }
                if issues.iter().any(|i| i.issue_type == "special_characters") {
                    recommendations.push("Use standard bullet points (•) instead of special symbols for better Greenhouse compatibility.".to_string());
                }
            },
            "lever" => {
                if issues.iter().any(|i| i.issue_type == "text_boxes") {
                    recommendations.push("Lever often ignores content in text boxes. Move all important information to the main document body.".to_string());
                }
                if issues.iter().any(|i| i.issue_type == "complex_formatting") {
                    recommendations.push("Lever works best with simple, single-column layouts. Avoid complex multi-column designs.".to_string());
                }
            },
            "workday" => {
                if issues.iter().any(|i| i.issue_type == "text_in_images") {
                    recommendations.push("Workday cannot extract text from images. Ensure all text is in readable format, not embedded in graphics.".to_string());
                }
                if issues.iter().any(|i| i.issue_type == "non_standard_font") {
                    recommendations.push("Workday is sensitive to font choices. Use professional fonts like Arial, Calibri, or Times New Roman.".to_string());
                }
            },
            _ => {
                recommendations.push("Follow general ATS best practices for optimal compatibility.".to_string());
            }
        }

        recommendations
    }

    // Advanced issue detection methods
    fn detect_hidden_text_issues(&self, content: &str) -> Vec<FormatIssue> {
        let mut issues = Vec::new();

        // Check for white text on white background
        if content.contains("color: white") || content.contains("color: #ffffff") {
            issues.push(FormatIssue {
                issue_type: "hidden_text".to_string(),
                severity: "high".to_string(),
                description: "Hidden text detected (white text on white background)".to_string(),
                recommendation: "Remove hidden text as it may be flagged as keyword stuffing".to_string(),
                section_affected: "formatting".to_string(),
                impact_score: 15.0,
            });
        }

        // Check for very small font sizes
        if content.contains("font-size: 1px") || content.contains("font-size: 0px") {
            issues.push(FormatIssue {
                issue_type: "micro_text".to_string(),
                severity: "medium".to_string(),
                description: "Extremely small text detected".to_string(),
                recommendation: "Use readable font sizes (minimum 10pt)".to_string(),
                section_affected: "formatting".to_string(),
                impact_score: 10.0,
            });
        }

        issues
    }

    fn detect_accessibility_issues(&self, content: &str) -> Vec<FormatIssue> {
        let mut issues = Vec::new();

        // Check for missing alt text on images
        if content.contains("<img") && !content.contains("alt=") {
            issues.push(FormatIssue {
                issue_type: "missing_alt_text".to_string(),
                severity: "low".to_string(),
                description: "Images without alt text detected".to_string(),
                recommendation: "Add descriptive alt text to all images".to_string(),
                section_affected: "accessibility".to_string(),
                impact_score: 3.0,
            });
        }

        // Check for poor color contrast
        if content.contains("color: gray") || content.contains("color: #cccccc") {
            issues.push(FormatIssue {
                issue_type: "poor_contrast".to_string(),
                severity: "medium".to_string(),
                description: "Poor color contrast detected".to_string(),
                recommendation: "Use high contrast colors for better readability".to_string(),
                section_affected: "formatting".to_string(),
                impact_score: 8.0,
            });
        }

        issues
    }

    fn detect_encoding_issues(&self, content: &str) -> Vec<FormatIssue> {
        let mut issues = Vec::new();

        // Check for unusual characters that might indicate encoding issues
        let problematic_chars = ["\u{FFFD}", "�", "â€™", "â€œ", "â€�"];
        
        for char in &problematic_chars {
            if content.contains(char) {
                issues.push(FormatIssue {
                    issue_type: "encoding_issue".to_string(),
                    severity: "medium".to_string(),
                    description: "Text encoding issues detected".to_string(),
                    recommendation: "Save the document with UTF-8 encoding".to_string(),
                    section_affected: "content".to_string(),
                    impact_score: 12.0,
                });
                break;
            }
        }

        issues
    }

    fn detect_metadata_issues(&self, content: &str) -> Vec<FormatIssue> {
        let mut issues = Vec::new();

        // Check for tracked changes or comments
        if content.contains("track-changes") || content.contains("comment") {
            issues.push(FormatIssue {
                issue_type: "tracked_changes".to_string(),
                severity: "medium".to_string(),
                description: "Tracked changes or comments detected".to_string(),
                recommendation: "Accept all changes and remove comments before submitting".to_string(),
                section_affected: "document_metadata".to_string(),
                impact_score: 8.0,
            });
        }

        issues
    }

    fn detect_style_inconsistencies(&self, content: &str) -> Vec<FormatIssue> {
        let mut issues = Vec::new();

        // Check for multiple font families
        let font_regex = Regex::new(r"font-family:\s*([^;]+)").unwrap();
        let fonts: Vec<_> = font_regex.captures_iter(content).collect();
        
        if fonts.len() > 3 {
            issues.push(FormatIssue {
                issue_type: "font_inconsistency".to_string(),
                severity: "low".to_string(),
                description: "Multiple font families detected".to_string(),
                recommendation: "Use consistent fonts throughout the document".to_string(),
                section_affected: "formatting".to_string(),
                impact_score: 5.0,
            });
        }

        issues
    }

    fn detect_section_structure_issues(&self, content: &str) -> Vec<FormatIssue> {
        let mut issues = Vec::new();

        // Check for nested sections (sections within sections)
        let nested_section_regex = Regex::new(r"(?i)(experience|education|skills).*\n.*\n.*(experience|education|skills)").unwrap();
        
        if nested_section_regex.is_match(content) {
            issues.push(FormatIssue {
                issue_type: "nested_sections".to_string(),
                severity: "medium".to_string(),
                description: "Nested or overlapping sections detected".to_string(),
                recommendation: "Ensure clear separation between resume sections".to_string(),
                section_affected: "structure".to_string(),
                impact_score: 12.0,
            });
        }

        issues
    }

    fn initialize_issue_patterns(&mut self) {
        // Initialize regex patterns for different issue types
        // This would be expanded with more sophisticated patterns
        self.issue_patterns.insert("tables".to_string(), IssuePattern {
            pattern_type: "structure".to_string(),
            regex_patterns: vec![
                Regex::new(r"<table").unwrap(),
                Regex::new(r"\|.*\|").unwrap(),
            ],
            severity_calculator: |_| "critical".to_string(),
            impact_calculator: |_| 25.0,
            description: "Tables detected in resume".to_string(),
        });
    }

    fn initialize_ats_sensitivities(&mut self) {
        // Initialize ATS-specific sensitivities
        self.ats_sensitivities.insert("greenhouse".to_string(), ATSSensitivity {
            ats_name: "greenhouse".to_string(),
            sensitive_issues: vec!["tables".to_string(), "special_characters".to_string()],
            impact_multipliers: [("tables".to_string(), 2.0), ("special_characters".to_string(), 1.5)].iter().cloned().collect(),
            parsing_limitations: vec!["Cannot parse tables effectively".to_string()],
        });

        self.ats_sensitivities.insert("lever".to_string(), ATSSensitivity {
            ats_name: "lever".to_string(),
            sensitive_issues: vec!["text_boxes".to_string(), "complex_formatting".to_string()],
            impact_multipliers: [("text_boxes".to_string(), 1.8), ("complex_formatting".to_string(), 1.3)].iter().cloned().collect(),
            parsing_limitations: vec!["Ignores text boxes".to_string(), "Issues with multi-column layouts".to_string()],
        });

        self.ats_sensitivities.insert("workday".to_string(), ATSSensitivity {
            ats_name: "workday".to_string(),
            sensitive_issues: vec!["text_in_images".to_string(), "non_standard_font".to_string()],
            impact_multipliers: [("text_in_images".to_string(), 3.0), ("non_standard_font".to_string(), 1.2)].iter().cloned().collect(),
            parsing_limitations: vec!["Cannot extract text from images".to_string()],
        });
    }

    fn initialize_recommendation_templates(&mut self) {
        self.recommendation_templates.insert("tables".to_string(), RecommendationTemplate {
            category: "tables".to_string(),
            title_template: "Convert Tables to Text Format".to_string(),
            description_template: "Tables can cause significant parsing issues in ATS systems. Convert tabular data to bullet points or plain text.".to_string(),
            steps: vec![
                "1. Identify all tables in your resume".to_string(),
                "2. Convert table headers to section headings".to_string(),
                "3. Convert table rows to bullet points".to_string(),
                "4. Ensure proper spacing between items".to_string(),
                "5. Review for readability and flow".to_string(),
            ],
            tools: vec!["Word processor".to_string(), "Text editor".to_string()],
            difficulty: "easy".to_string(),
            time_estimate: "15 minutes".to_string(),
        });

        self.recommendation_templates.insert("text_in_images".to_string(), RecommendationTemplate {
            category: "text_in_images".to_string(),
            title_template: "Extract Text from Images".to_string(),
            description_template: "ATS systems cannot read text within images. All important information must be in text format.".to_string(),
            steps: vec![
                "1. Identify all images containing text".to_string(),
                "2. Extract the text content from each image".to_string(),
                "3. Add the text to the appropriate resume section".to_string(),
                "4. Remove or replace images with text equivalents".to_string(),
                "5. Verify all information is preserved".to_string(),
            ],
            tools: vec!["OCR software".to_string(), "Manual transcription".to_string()],
            difficulty: "medium".to_string(),
            time_estimate: "30 minutes".to_string(),
        });
    }
}

impl Default for FormatIssueDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format_checker::FormatCompatibilityChecker;

    #[test]
    fn test_issue_detection() {
        let detector = FormatIssueDetector::new();
        let content = "<table><tr><td>Test</td></tr></table>";
        
        let advanced_issues = detector.detect_advanced_issues(content).unwrap();
        assert!(!advanced_issues.is_empty());
    }

    #[test]
    fn test_recommendation_generation() {
        let detector = FormatIssueDetector::new();
        let issues = vec![
            FormatIssue {
                issue_type: "tables".to_string(),
                severity: "critical".to_string(),
                description: "Tables detected".to_string(),
                recommendation: "Convert to text".to_string(),
                section_affected: "structure".to_string(),
                impact_score: 25.0,
            }
        ];
        
        let recommendations = detector.generate_improvement_recommendations(&issues);
        assert!(!recommendations.is_empty());
        assert_eq!(recommendations[0].category, "tables");
    }

    #[test]
    fn test_format_score_calculation() {
        let detector = FormatIssueDetector::new();
        let issues = vec![
            FormatIssue {
                issue_type: "tables".to_string(),
                severity: "critical".to_string(),
                description: "Tables detected".to_string(),
                recommendation: "Convert to text".to_string(),
                section_affected: "structure".to_string(),
                impact_score: 25.0,
            }
        ];
        
        let score = detector.calculate_overall_format_score(&issues);
        assert_eq!(score, 75.0);
    }

    #[test]
    fn test_comprehensive_analysis() {
        let detector = FormatIssueDetector::new();
        let format_checker = FormatCompatibilityChecker::new();
        
        let content = "<table><tr><td>Test table</td></tr></table>";
        let compatibility_report = format_checker.check_comprehensive_compatibility(content).unwrap();
        
        let issue_report = detector.analyze_format_issues(content, &compatibility_report).unwrap();
        
        assert!(issue_report.overall_format_score <= 100.0);
        assert!(!issue_report.improvement_recommendations.is_empty());
    }
}