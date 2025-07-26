use anyhow::Result;
use log::info;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementAnalysis {
    pub strong_achievements: Vec<BulletAnalysis>,
    pub improvement_opportunities: Vec<XYZSuggestion>,
    pub overall_achievement_score: f64,
    pub section_scores: HashMap<String, f64>,
    pub achievement_distribution: AchievementDistribution,
    pub xyz_formula_compliance: f64,
    pub action_verb_strength: f64,
    pub quantification_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulletAnalysis {
    pub original_text: String,
    pub section: String,
    pub has_action_verb: bool,
    pub action_verb: Option<String>,
    pub action_verb_strength: String, // "weak", "medium", "strong"
    pub has_quantification: bool,
    pub quantifications: Vec<String>,
    pub has_outcome: bool,
    pub outcome_description: Option<String>,
    pub has_xyz_formula: bool,
    pub xyz_components: XYZComponents,
    pub strength_score: f64,
    pub improvement_suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XYZComponents {
    pub x_accomplishment: Option<String>,
    pub y_measurement: Option<String>,
    pub z_method: Option<String>,
    pub completeness_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XYZSuggestion {
    pub original: String,
    pub section: String,
    pub weakness_type: String, // "missing_quantification", "weak_action_verb", "no_outcome", "incomplete_xyz"
    pub suggested_x: Option<String>,
    pub suggested_y: Option<String>,
    pub suggested_z: Option<String>,
    pub improved_version: String,
    pub improvement_impact: f64,
    pub implementation_difficulty: String, // "easy", "medium", "hard"
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementDistribution {
    pub total_bullets: usize,
    pub strong_achievements: usize,
    pub medium_achievements: usize,
    pub weak_achievements: usize,
    pub xyz_compliant: usize,
    pub quantified_bullets: usize,
    pub action_verb_bullets: usize,
}

pub struct AchievementAnalyzer {
    strong_action_verbs: HashSet<String>,
    medium_action_verbs: HashSet<String>,
    weak_action_verbs: HashSet<String>,
    quantification_patterns: Vec<Regex>,
    achievement_patterns: Vec<Regex>,
    outcome_patterns: Vec<Regex>,
    stop_words: HashSet<String>,
}

impl AchievementAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            strong_action_verbs: HashSet::new(),
            medium_action_verbs: HashSet::new(),
            weak_action_verbs: HashSet::new(),
            quantification_patterns: Vec::new(),
            achievement_patterns: Vec::new(),
            outcome_patterns: Vec::new(),
            stop_words: HashSet::new(),
        };

        analyzer.initialize_action_verbs();
        analyzer.initialize_patterns();
        analyzer.initialize_stop_words();
        analyzer
    }

    pub fn analyze_achievements(&self, resume_content: &str) -> Result<AchievementAnalysis> {
        info!("Starting comprehensive achievement analysis");

        let sections = self.extract_sections(resume_content);
        let mut all_analyses = Vec::new();
        let mut improvement_opportunities = Vec::new();
        let mut section_scores = HashMap::new();

        for (section_name, section_content) in sections {
            let bullet_points = self.extract_bullet_points(&section_content);
            let mut section_analyses = Vec::new();
            let mut section_improvements = Vec::new();

            for bullet in bullet_points {
                let analysis = self.analyze_single_bullet(&bullet, &section_name);

                if analysis.strength_score >= 70.0 {
                    section_analyses.push(analysis);
                } else {
                    let suggestion =
                        self.generate_xyz_improvement(&bullet, &section_name, &analysis);
                    section_improvements.push(suggestion);
                }
            }

            // Calculate section score
            let section_score = if section_analyses.is_empty() && section_improvements.is_empty() {
                0.0
            } else {
                let total_bullets = section_analyses.len() + section_improvements.len();
                let strong_count = section_analyses.len();
                (strong_count as f64 / total_bullets as f64) * 100.0
            };

            section_scores.insert(section_name, section_score);
            all_analyses.extend(section_analyses);
            improvement_opportunities.extend(section_improvements);
        }

        // Calculate overall metrics
        let overall_achievement_score =
            self.calculate_overall_score(&all_analyses, &improvement_opportunities);
        let achievement_distribution =
            self.calculate_distribution(&all_analyses, &improvement_opportunities);
        let xyz_formula_compliance =
            self.calculate_xyz_compliance(&all_analyses, &improvement_opportunities);
        let action_verb_strength =
            self.calculate_action_verb_strength(&all_analyses, &improvement_opportunities);
        let quantification_rate =
            self.calculate_quantification_rate(&all_analyses, &improvement_opportunities);

        Ok(AchievementAnalysis {
            strong_achievements: all_analyses,
            improvement_opportunities,
            overall_achievement_score,
            section_scores,
            achievement_distribution,
            xyz_formula_compliance,
            action_verb_strength,
            quantification_rate,
        })
    }

    fn analyze_single_bullet(&self, bullet: &str, section: &str) -> BulletAnalysis {
        let cleaned_bullet = self.clean_bullet_text(bullet);

        // Analyze action verbs
        let (has_action_verb, action_verb, action_verb_strength) =
            self.analyze_action_verbs(&cleaned_bullet);

        // Analyze quantification
        let (has_quantification, quantifications) = self.analyze_quantification(&cleaned_bullet);

        // Analyze outcomes
        let (has_outcome, outcome_description) = self.analyze_outcomes(&cleaned_bullet);

        // Analyze X-Y-Z formula compliance
        let xyz_components = self.analyze_xyz_components(&cleaned_bullet);
        let has_xyz_formula = xyz_components.completeness_score >= 0.8;

        // Calculate strength score
        let strength_score = self.calculate_bullet_strength(
            &action_verb_strength,
            has_quantification,
            has_outcome,
            xyz_components.completeness_score,
        );

        // Generate improvement suggestions
        let improvement_suggestions = self.generate_bullet_improvements(
            &cleaned_bullet,
            &action_verb_strength,
            has_quantification,
            has_outcome,
            &xyz_components,
        );

        BulletAnalysis {
            original_text: bullet.to_string(),
            section: section.to_string(),
            has_action_verb,
            action_verb,
            action_verb_strength,
            has_quantification,
            quantifications,
            has_outcome,
            outcome_description,
            has_xyz_formula,
            xyz_components,
            strength_score,
            improvement_suggestions,
        }
    }

    fn analyze_action_verbs(&self, text: &str) -> (bool, Option<String>, String) {
        let words: Vec<&str> = text.split_whitespace().collect();

        // Look for action verbs in the first few words (typically at the beginning)
        for word in words.iter().take(3) {
            let word_lower = word.to_lowercase();
            let clean_word = word_lower.trim_matches(|c: char| !c.is_alphabetic());

            if self.strong_action_verbs.contains(clean_word) {
                return (true, Some(clean_word.to_string()), "strong".to_string());
            } else if self.medium_action_verbs.contains(clean_word) {
                return (true, Some(clean_word.to_string()), "medium".to_string());
            } else if self.weak_action_verbs.contains(clean_word) {
                return (true, Some(clean_word.to_string()), "weak".to_string());
            }
        }

        (false, None, "none".to_string())
    }

    fn analyze_quantification(&self, text: &str) -> (bool, Vec<String>) {
        let mut quantifications = Vec::new();

        for pattern in &self.quantification_patterns {
            for mat in pattern.find_iter(text) {
                quantifications.push(mat.as_str().to_string());
            }
        }

        // Remove duplicates
        quantifications.sort();
        quantifications.dedup();

        (!quantifications.is_empty(), quantifications)
    }

    fn analyze_outcomes(&self, text: &str) -> (bool, Option<String>) {
        for pattern in &self.outcome_patterns {
            if let Some(mat) = pattern.find(text) {
                return (true, Some(mat.as_str().to_string()));
            }
        }

        // Check for implicit outcomes (improvement language)
        let improvement_indicators = [
            "increased",
            "decreased",
            "improved",
            "enhanced",
            "optimized",
            "reduced",
            "streamlined",
            "accelerated",
            "achieved",
            "exceeded",
            "delivered",
            "generated",
            "saved",
            "earned",
            "won",
            "gained",
        ];

        for indicator in &improvement_indicators {
            if text.to_lowercase().contains(indicator) {
                return (
                    true,
                    Some(format!("Improvement indicated by '{}'", indicator)),
                );
            }
        }

        (false, None)
    }

    fn analyze_xyz_components(&self, text: &str) -> XYZComponents {
        let mut x_accomplishment = None;
        let mut y_measurement = None;
        let mut z_method = None;

        // X: Accomplishment (what was achieved)
        let accomplishment_indicators = [
            "accomplished",
            "achieved",
            "delivered",
            "completed",
            "implemented",
            "developed",
            "created",
            "built",
            "designed",
            "led",
            "managed",
        ];
        for indicator in &accomplishment_indicators {
            if text.to_lowercase().contains(indicator) {
                // Extract the accomplishment description
                if let Some(pos) = text.to_lowercase().find(indicator) {
                    let after_indicator = &text[pos..];
                    if let Some(end_pos) = after_indicator.find(" by ") {
                        x_accomplishment = Some(after_indicator[..end_pos].to_string());
                    } else {
                        // Take a reasonable chunk after the indicator
                        let words: Vec<&str> = after_indicator.split_whitespace().take(8).collect();
                        x_accomplishment = Some(words.join(" "));
                    }
                }
                break;
            }
        }

        // Y: Measurement (quantifiable results)
        let (has_quantification, quantifications) = self.analyze_quantification(text);
        if has_quantification && !quantifications.is_empty() {
            y_measurement = Some(quantifications.join(", "));
        }

        // Z: Method (how it was done)
        if let Some(by_pos) = text.to_lowercase().find(" by ") {
            let method_part = &text[by_pos + 4..];
            // Take the method description
            let words: Vec<&str> = method_part.split_whitespace().take(10).collect();
            z_method = Some(words.join(" "));
        } else if let Some(through_pos) = text.to_lowercase().find(" through ") {
            let method_part = &text[through_pos + 9..];
            let words: Vec<&str> = method_part.split_whitespace().take(10).collect();
            z_method = Some(words.join(" "));
        } else if let Some(using_pos) = text.to_lowercase().find(" using ") {
            let method_part = &text[using_pos + 7..];
            let words: Vec<&str> = method_part.split_whitespace().take(8).collect();
            z_method = Some(words.join(" "));
        }

        // Calculate completeness score
        let mut score = 0.0;
        if x_accomplishment.is_some() {
            score += 0.4;
        }
        if y_measurement.is_some() {
            score += 0.4;
        }
        if z_method.is_some() {
            score += 0.2;
        }

        XYZComponents {
            x_accomplishment,
            y_measurement,
            z_method,
            completeness_score: score,
        }
    }

    fn calculate_bullet_strength(
        &self,
        action_verb_strength: &str,
        has_quantification: bool,
        has_outcome: bool,
        xyz_completeness: f64,
    ) -> f64 {
        let mut score = 0.0;

        // Action verb contribution (30%)
        score += match action_verb_strength {
            "strong" => 30.0,
            "medium" => 20.0,
            "weak" => 10.0,
            _ => 0.0,
        };

        // Quantification contribution (25%)
        if has_quantification {
            score += 25.0;
        }

        // Outcome contribution (20%)
        if has_outcome {
            score += 20.0;
        }

        // X-Y-Z formula contribution (25%)
        score += xyz_completeness * 25.0;

        score
    }

    fn generate_bullet_improvements(
        &self,
        _text: &str,
        action_verb_strength: &str,
        has_quantification: bool,
        has_outcome: bool,
        xyz_components: &XYZComponents,
    ) -> Vec<String> {
        let mut suggestions = Vec::new();

        if action_verb_strength == "weak" || action_verb_strength == "none" {
            suggestions.push("Use a stronger action verb to start your bullet point".to_string());
        }

        if !has_quantification {
            suggestions.push(
                "Add specific numbers, percentages, or metrics to quantify your impact".to_string(),
            );
        }

        if !has_outcome {
            suggestions.push("Clearly state the result or outcome of your actions".to_string());
        }

        if xyz_components.completeness_score < 0.6 {
            suggestions.push(
                "Structure using X-Y-Z formula: Accomplished [X] as measured by [Y], by doing [Z]"
                    .to_string(),
            );
        }

        if xyz_components.x_accomplishment.is_none() {
            suggestions.push("Clearly state what you accomplished or achieved".to_string());
        }

        if xyz_components.y_measurement.is_none() {
            suggestions.push("Add measurable results or metrics to demonstrate impact".to_string());
        }

        if xyz_components.z_method.is_none() {
            suggestions.push(
                "Explain how you achieved the results (methods, tools, strategies used)"
                    .to_string(),
            );
        }

        suggestions
    }

    fn generate_xyz_improvement(
        &self,
        bullet: &str,
        section: &str,
        analysis: &BulletAnalysis,
    ) -> XYZSuggestion {
        let weakness_type = self.identify_primary_weakness(analysis);
        let improved_version = self.create_improved_bullet(bullet, analysis);
        let improvement_impact = 100.0 - analysis.strength_score;
        let implementation_difficulty = self.assess_implementation_difficulty(analysis);
        let explanation = self.generate_improvement_explanation(analysis, &weakness_type);

        // Generate specific X-Y-Z suggestions based on current content
        let suggested_x = self.suggest_accomplishment(bullet, section);
        let suggested_y = self.suggest_measurement(bullet, section);
        let suggested_z = self.suggest_method(bullet, section);

        XYZSuggestion {
            original: bullet.to_string(),
            section: section.to_string(),
            weakness_type,
            suggested_x,
            suggested_y,
            suggested_z,
            improved_version,
            improvement_impact,
            implementation_difficulty,
            explanation,
        }
    }

    fn identify_primary_weakness(&self, analysis: &BulletAnalysis) -> String {
        if analysis.action_verb_strength == "none" || analysis.action_verb_strength == "weak" {
            "weak_action_verb".to_string()
        } else if !analysis.has_quantification {
            "missing_quantification".to_string()
        } else if !analysis.has_outcome {
            "no_outcome".to_string()
        } else if analysis.xyz_components.completeness_score < 0.6 {
            "incomplete_xyz".to_string()
        } else {
            "general_improvement".to_string()
        }
    }

    fn create_improved_bullet(&self, bullet: &str, analysis: &BulletAnalysis) -> String {
        let mut improved = bullet.to_string();

        // If no strong action verb, suggest replacement
        if analysis.action_verb_strength != "strong" {
            if let Some(strong_verb) = self.get_replacement_action_verb(&improved) {
                // Replace the first word or add at the beginning
                let words: Vec<&str> = improved.split_whitespace().collect();
                if !words.is_empty() {
                    improved = format!("{} {}", strong_verb, words[1..].join(" "));
                } else {
                    improved = format!("{} {}", strong_verb, improved);
                }
            }
        }

        // Add quantification if missing
        if !analysis.has_quantification {
            improved = format!("{} (increased by X% / reduced by Y minutes)", improved);
        }

        // Add method if missing
        if analysis.xyz_components.z_method.is_none() {
            improved = format!(
                "{} by implementing [specific method/tool/strategy]",
                improved
            );
        }

        improved
    }

    fn get_replacement_action_verb(&self, text: &str) -> Option<String> {
        // Map weak verbs to strong alternatives
        let verb_replacements = [
            ("helped", "collaborated"),
            ("worked", "executed"),
            ("did", "accomplished"),
            ("made", "developed"),
            ("responsible", "led"),
            ("involved", "spearheaded"),
            ("handled", "managed"),
            ("dealt", "resolved"),
        ];

        for (weak, strong) in &verb_replacements {
            if text.to_lowercase().contains(weak) {
                return Some(strong.to_string());
            }
        }

        // Default strong verbs by context
        if text.to_lowercase().contains("team") {
            Some("led".to_string())
        } else if text.to_lowercase().contains("project") {
            Some("delivered".to_string())
        } else if text.to_lowercase().contains("system") {
            Some("implemented".to_string())
        } else {
            Some("achieved".to_string())
        }
    }

    fn suggest_accomplishment(&self, bullet: &str, section: &str) -> Option<String> {
        match section.to_lowercase().as_str() {
            "experience" | "work experience" => {
                if bullet.to_lowercase().contains("project") {
                    Some("delivered project milestone".to_string())
                } else if bullet.to_lowercase().contains("team") {
                    Some("led team initiative".to_string())
                } else {
                    Some("achieved operational improvement".to_string())
                }
            }
            "education" => Some("completed academic achievement".to_string()),
            "projects" => Some("developed technical solution".to_string()),
            _ => Some("accomplished key objective".to_string()),
        }
    }

    fn suggest_measurement(&self, bullet: &str, _section: &str) -> Option<String> {
        if bullet.to_lowercase().contains("performance") {
            Some("measured by X% performance improvement".to_string())
        } else if bullet.to_lowercase().contains("time") {
            Some("reducing processing time by X minutes".to_string())
        } else if bullet.to_lowercase().contains("cost") {
            Some("saving $X in costs".to_string())
        } else if bullet.to_lowercase().contains("revenue") {
            Some("generating $X in revenue".to_string())
        } else {
            Some("measured by X% improvement in key metrics".to_string())
        }
    }

    fn suggest_method(&self, bullet: &str, _section: &str) -> Option<String> {
        if bullet.to_lowercase().contains("software") || bullet.to_lowercase().contains("system") {
            Some("by implementing automated solutions".to_string())
        } else if bullet.to_lowercase().contains("team") {
            Some("by coordinating cross-functional collaboration".to_string())
        } else if bullet.to_lowercase().contains("process") {
            Some("by optimizing workflows and procedures".to_string())
        } else {
            Some("by applying strategic methodologies".to_string())
        }
    }

    fn assess_implementation_difficulty(&self, analysis: &BulletAnalysis) -> String {
        let issues_count = analysis.improvement_suggestions.len();

        match issues_count {
            0..=1 => "easy".to_string(),
            2..=3 => "medium".to_string(),
            _ => "hard".to_string(),
        }
    }

    fn generate_improvement_explanation(
        &self,
        _analysis: &BulletAnalysis,
        weakness_type: &str,
    ) -> String {
        match weakness_type {
            "weak_action_verb" => "Start with a strong action verb that demonstrates leadership and impact. Avoid passive language and weak verbs like 'helped' or 'worked on'.".to_string(),
            "missing_quantification" => "Add specific numbers, percentages, dollar amounts, or other metrics to quantify your impact and make your achievements more compelling.".to_string(),
            "no_outcome" => "Clearly state the result or positive outcome of your actions. What changed as a result of your work?".to_string(),
            "incomplete_xyz" => "Structure your bullet point using Google's X-Y-Z formula: 'Accomplished [X] as measured by [Y], by doing [Z]' to create more impactful statements.".to_string(),
            _ => "Enhance this bullet point by adding stronger action verbs, quantifiable results, and clear outcomes using the X-Y-Z formula.".to_string(),
        }
    }

    // Helper methods for extracting sections and bullet points
    fn extract_sections(&self, content: &str) -> Vec<(String, String)> {
        let mut sections = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut current_section = "General".to_string();
        let mut current_content = Vec::new();

        let section_headers = [
            "experience",
            "work experience",
            "professional experience",
            "employment",
            "education",
            "academic background",
            "qualifications",
            "projects",
            "key projects",
            "notable projects",
            "achievements",
            "accomplishments",
            "awards",
            "skills",
            "technical skills",
            "core competencies",
        ];

        for line in lines {
            let line_lower = line.trim().to_lowercase();

            // Check if this line is a section header
            let is_section_header = section_headers.iter().any(|&header| {
                line_lower == header || line_lower.starts_with(&format!("{} ", header))
            }) && line.trim().len() > 2
                && line.trim().len() < 50;

            if is_section_header {
                // Save the previous section
                if !current_content.is_empty() {
                    sections.push((current_section.clone(), current_content.join("\n")));
                }

                // Start new section
                current_section = line.trim().to_string();
                current_content.clear();
            } else if !line.trim().is_empty() {
                current_content.push(line);
            }
        }

        // Add the last section
        if !current_content.is_empty() {
            sections.push((current_section, current_content.join("\n")));
        }

        sections
    }

    fn extract_bullet_points(&self, section_content: &str) -> Vec<String> {
        let lines: Vec<&str> = section_content.lines().collect();
        let mut bullets = Vec::new();

        for line in lines {
            let trimmed = line.trim();

            // Check if line starts with bullet point indicators
            if trimmed.starts_with("•")
                || trimmed.starts_with("-")
                || trimmed.starts_with("*")
                || trimmed.starts_with("◦")
                || trimmed.starts_with("▪")
                || trimmed.starts_with("‣")
            {
                // Remove bullet point character and clean up
                let bullet_text = trimmed
                    .chars()
                    .skip(1)
                    .collect::<String>()
                    .trim()
                    .to_string();
                if bullet_text.len() > 10 {
                    // Only include substantial bullet points
                    bullets.push(bullet_text);
                }
            }
        }

        bullets
    }

    fn clean_bullet_text(&self, bullet: &str) -> String {
        // Remove common prefixes and clean up text
        bullet
            .trim()
            .trim_start_matches("•")
            .trim_start_matches("-")
            .trim_start_matches("*")
            .trim()
            .to_string()
    }

    // Calculation helper methods
    fn calculate_overall_score(&self, strong: &[BulletAnalysis], weak: &[XYZSuggestion]) -> f64 {
        let total_bullets = strong.len() + weak.len();
        if total_bullets == 0 {
            return 0.0;
        }

        let strong_count = strong.len();
        (strong_count as f64 / total_bullets as f64) * 100.0
    }

    fn calculate_distribution(
        &self,
        strong: &[BulletAnalysis],
        weak: &[XYZSuggestion],
    ) -> AchievementDistribution {
        let total_bullets = strong.len() + weak.len();
        let strong_count = strong.iter().filter(|b| b.strength_score >= 80.0).count();
        let medium_count = strong
            .iter()
            .filter(|b| b.strength_score >= 60.0 && b.strength_score < 80.0)
            .count()
            + weak.iter().filter(|w| w.improvement_impact <= 40.0).count();
        let weak_count = total_bullets - strong_count - medium_count;

        let xyz_compliant = strong.iter().filter(|b| b.has_xyz_formula).count();
        let quantified_bullets = strong.iter().filter(|b| b.has_quantification).count();
        let action_verb_bullets = strong.iter().filter(|b| b.has_action_verb).count();

        AchievementDistribution {
            total_bullets,
            strong_achievements: strong_count,
            medium_achievements: medium_count,
            weak_achievements: weak_count,
            xyz_compliant,
            quantified_bullets,
            action_verb_bullets,
        }
    }

    fn calculate_xyz_compliance(&self, strong: &[BulletAnalysis], weak: &[XYZSuggestion]) -> f64 {
        let total_bullets = strong.len() + weak.len();
        if total_bullets == 0 {
            return 0.0;
        }

        let xyz_compliant = strong.iter().filter(|b| b.has_xyz_formula).count();
        (xyz_compliant as f64 / total_bullets as f64) * 100.0
    }

    fn calculate_action_verb_strength(
        &self,
        strong: &[BulletAnalysis],
        _weak: &[XYZSuggestion],
    ) -> f64 {
        if strong.is_empty() {
            return 0.0;
        }

        let total_score: f64 = strong
            .iter()
            .map(|b| match b.action_verb_strength.as_str() {
                "strong" => 100.0,
                "medium" => 70.0,
                "weak" => 40.0,
                _ => 0.0,
            })
            .sum();

        total_score / strong.len() as f64
    }

    fn calculate_quantification_rate(
        &self,
        strong: &[BulletAnalysis],
        _weak: &[XYZSuggestion],
    ) -> f64 {
        let total_bullets = strong.len();
        if total_bullets == 0 {
            return 0.0;
        }

        let quantified_count = strong.iter().filter(|b| b.has_quantification).count();
        (quantified_count as f64 / total_bullets as f64) * 100.0
    }

    // Initialization methods
    fn initialize_action_verbs(&mut self) {
        // Strong action verbs (leadership, impact, achievement)
        self.strong_action_verbs.extend(
            [
                "accelerated",
                "achieved",
                "acquired",
                "administered",
                "advanced",
                "analyzed",
                "architected",
                "automated",
                "built",
                "championed",
                "collaborated",
                "completed",
                "conceived",
                "conducted",
                "constructed",
                "created",
                "delivered",
                "demonstrated",
                "designed",
                "developed",
                "directed",
                "drove",
                "engineered",
                "enhanced",
                "established",
                "evaluated",
                "exceeded",
                "executed",
                "expanded",
                "generated",
                "implemented",
                "improved",
                "increased",
                "initiated",
                "innovated",
                "launched",
                "led",
                "managed",
                "maximized",
                "optimized",
                "orchestrated",
                "organized",
                "pioneered",
                "produced",
                "reduced",
                "resolved",
                "spearheaded",
                "streamlined",
                "strengthened",
                "transformed",
            ]
            .iter()
            .map(|s| s.to_string()),
        );

        // Medium action verbs (competent but less impactful)
        self.medium_action_verbs.extend(
            [
                "assisted",
                "conducted",
                "coordinated",
                "facilitated",
                "maintained",
                "monitored",
                "operated",
                "participated",
                "performed",
                "prepared",
                "presented",
                "processed",
                "provided",
                "reviewed",
                "supported",
                "trained",
                "updated",
                "utilized",
            ]
            .iter()
            .map(|s| s.to_string()),
        );

        // Weak action verbs (passive or vague)
        self.weak_action_verbs.extend(
            [
                "dealt",
                "did",
                "handled",
                "helped",
                "involved",
                "made",
                "responsible",
                "worked",
            ]
            .iter()
            .map(|s| s.to_string()),
        );
    }

    fn initialize_patterns(&mut self) {
        // Quantification patterns (numbers, percentages, money, time)
        let patterns = vec![
            r"\d+%", // Percentages
            r"\$\d+[\d,]*(?:\.\d+)?[KMB]?", // Money
            r"\d+[\d,]*\+?\s*(?:users|customers|clients|people|employees|hours|days|weeks|months|years)", // Counts with units
            r"\d+[\d,]*(?:\.\d+)?\s*(?:million|thousand|billion|hours|minutes|seconds|days|weeks|months|years)", // Large numbers with units
            r"(?:increased|decreased|reduced|improved|saved|generated|grew)\s+(?:by\s+)?\d+", // Change amounts
        ];
        
        self.quantification_patterns = patterns
            .into_iter()
            .filter_map(|pattern| Regex::new(pattern).ok())
            .collect();

        // Achievement patterns (success indicators)
        let achievement_patterns = vec![
            r"(?i)(?:exceeded|surpassed|outperformed|achieved|reached|delivered|completed)",
            r"(?i)(?:award|recognition|promotion|certification|achievement|success)",
        ];
        
        self.achievement_patterns = achievement_patterns
            .into_iter()
            .filter_map(|pattern| Regex::new(pattern).ok())
            .collect();

        // Outcome patterns (result indicators)
        let outcome_patterns = vec![
            r"(?i)resulting in|led to|which resulted in|outcome was|impact was",
            r"(?i)(?:increased|decreased|improved|enhanced|reduced|optimized|streamlined)\s+.+\s+by\s+\d+",
        ];
        
        self.outcome_patterns = outcome_patterns
            .into_iter()
            .filter_map(|pattern| Regex::new(pattern).ok())
            .collect();
    }

    fn initialize_stop_words(&mut self) {
        self.stop_words.extend(
            [
                "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with",
                "by", "from", "up", "about", "into", "through", "during", "before", "after",
                "above", "below", "between", "among", "within", "without", "under", "over",
            ]
            .iter()
            .map(|s| s.to_string()),
        );
    }
}

impl Default for AchievementAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_achievement_analyzer_creation() {
        let analyzer = AchievementAnalyzer::new();
        assert!(!analyzer.strong_action_verbs.is_empty());
        assert!(!analyzer.quantification_patterns.is_empty());
    }

    #[test]
    fn test_action_verb_analysis() {
        let analyzer = AchievementAnalyzer::new();

        // Test strong action verb
        let (has_verb, verb, strength) =
            analyzer.analyze_action_verbs("Led a team of 5 developers");
        assert!(has_verb);
        assert_eq!(verb, Some("led".to_string()));
        assert_eq!(strength, "strong");

        // Test weak action verb
        let (has_verb, verb, strength) =
            analyzer.analyze_action_verbs("Helped with project management");
        assert!(has_verb);
        assert_eq!(verb, Some("helped".to_string()));
        assert_eq!(strength, "weak");
    }

    #[test]
    fn test_quantification_analysis() {
        let analyzer = AchievementAnalyzer::new();

        let (has_quant, quants) =
            analyzer.analyze_quantification("Increased sales by 25% and saved $50K");
        assert!(has_quant);
        assert!(quants.contains(&"25%".to_string()));
        assert!(quants.contains(&"$50K".to_string()));
    }

    #[test]
    fn test_xyz_components_analysis() {
        let analyzer = AchievementAnalyzer::new();

        let xyz = analyzer.analyze_xyz_components(
            "Achieved 25% performance improvement by implementing automated testing",
        );
        assert!(xyz.x_accomplishment.is_some());
        assert!(xyz.y_measurement.is_some());
        assert!(xyz.z_method.is_some());
        assert!(xyz.completeness_score > 0.8);
    }

    #[test]
    fn test_bullet_strength_calculation() {
        let analyzer = AchievementAnalyzer::new();

        let strong_score = analyzer.calculate_bullet_strength("strong", true, true, 1.0);
        assert!(strong_score >= 80.0);

        let weak_score = analyzer.calculate_bullet_strength("weak", false, false, 0.0);
        assert!(weak_score <= 20.0);
    }

    #[test]
    fn test_bullet_point_extraction() {
        let analyzer = AchievementAnalyzer::new();

        let content = "Experience\n• Led development team\n• Improved system performance\n- Reduced costs by 20%";
        let bullets = analyzer.extract_bullet_points(content);

        assert_eq!(bullets.len(), 3);
        assert!(bullets[0].contains("Led development team"));
        assert!(bullets[2].contains("Reduced costs by 20%"));
    }

    #[test]
    fn test_comprehensive_analysis() {
        let analyzer = AchievementAnalyzer::new();

        let resume_content = r#"
        Experience
        • Led cross-functional team of 8 developers, resulting in 30% faster delivery
        • Implemented automated testing framework, reducing bugs by 45%
        • Worked on various projects
        
        Projects
        • Built mobile application with 10K+ downloads
        • Helped improve system performance
        "#;

        let analysis = analyzer.analyze_achievements(resume_content).unwrap();

        assert!(analysis.overall_achievement_score > 0.0);
        assert!(!analysis.strong_achievements.is_empty());
        assert!(!analysis.improvement_opportunities.is_empty());
        assert!(analysis.xyz_formula_compliance >= 0.0);
    }
}
