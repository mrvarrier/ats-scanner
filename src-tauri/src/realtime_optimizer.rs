use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::achievement_analyzer::{AchievementAnalyzer, BulletAnalysis};
use crate::database::Database;
use crate::semantic_analyzer::SemanticAnalyzer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveSuggestions {
    pub current_section: Section,
    pub context_suggestions: Vec<ContextualSuggestion>,
    pub priority_improvements: Vec<PriorityImprovement>,
    pub real_time_score: f64,
    pub score_change: f64,
    pub typing_feedback: TypingFeedback,
    pub section_strength: f64,
    pub completion_percentage: f64,
    pub next_recommended_action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualSuggestion {
    pub suggestion_id: String,
    pub type_: String, // "xyz_improvement", "keyword_integration", "action_verb", "quantification"
    pub title: String,
    pub description: String,
    pub suggestion: String,
    pub confidence: f64,
    pub applicable_text: String,
    pub replacement_text: Option<String>,
    pub explanation: String,
    pub impact_score: f64,
    pub urgency: String, // "immediate", "soon", "later"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityImprovement {
    pub improvement_id: String,
    pub category: String, // "achievement", "keyword", "format", "structure"
    pub title: String,
    pub description: String,
    pub current_issues: Vec<String>,
    pub suggested_fixes: Vec<String>,
    pub impact_score: f64,
    pub implementation_effort: String,
    pub example_before: Option<String>,
    pub example_after: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingFeedback {
    pub current_bullet_analysis: Option<BulletAnalysis>,
    pub live_suggestions: Vec<LiveSuggestion>,
    pub word_count: usize,
    pub character_count: usize,
    pub estimated_reading_time: String,
    pub tone_analysis: ToneAnalysis,
    pub clarity_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveSuggestion {
    pub position: usize, // Character position in text
    pub suggestion_type: String,
    pub text: String,
    pub confidence: f64,
    pub auto_apply: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToneAnalysis {
    pub professionalism_score: f64,
    pub confidence_level: f64,
    pub action_orientation: f64,
    pub specificity_score: f64,
    pub overall_tone: String, // "professional", "confident", "passive", "aggressive"
    pub tone_suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Section {
    Summary,
    Experience,
    Education,
    Skills,
    Projects,
    Achievements,
    Certifications,
    Unknown,
}

pub struct RealtimeOptimizer {
    achievement_analyzer: AchievementAnalyzer,
    #[allow(dead_code)]
    semantic_analyzer: SemanticAnalyzer,
    #[allow(dead_code)]
    suggestion_cache: HashMap<String, Vec<ContextualSuggestion>>,
    change_tracker: ChangeTracker,
}

#[derive(Debug, Clone)]
struct ChangeTracker {
    previous_content: String,
    change_history: Vec<ContentChange>,
    performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ContentChange {
    timestamp: chrono::DateTime<chrono::Utc>,
    change_type: String,
    old_text: String,
    new_text: String,
    section: String,
    impact_score: f64,
}

#[derive(Debug, Clone)]
struct PerformanceMetrics {
    total_changes: usize,
    #[allow(dead_code)]
    improvements_applied: usize,
    score_trajectory: Vec<f64>,
    #[allow(dead_code)]
    session_duration: std::time::Duration,
}

impl RealtimeOptimizer {
    pub fn new(database: Database) -> Self {
        Self {
            achievement_analyzer: AchievementAnalyzer::new(),
            semantic_analyzer: SemanticAnalyzer::new(database),
            suggestion_cache: HashMap::new(),
            change_tracker: ChangeTracker {
                previous_content: String::new(),
                change_history: Vec::new(),
                performance_metrics: PerformanceMetrics {
                    total_changes: 0,
                    improvements_applied: 0,
                    score_trajectory: Vec::new(),
                    session_duration: std::time::Duration::from_secs(0),
                },
            },
        }
    }

    pub async fn get_live_suggestions(
        &mut self,
        current_content: &str,
        job_description: &str,
        cursor_position: usize,
    ) -> Result<LiveSuggestions> {
        info!(
            "Generating live suggestions for content at position {}",
            cursor_position
        );

        // 1. Identify current section being edited
        let current_section = self.identify_current_section(current_content, cursor_position);

        // 2. Track changes since last analysis
        self.track_content_changes(current_content);

        // 3. Generate context-aware suggestions
        let context_suggestions = self
            .generate_context_suggestions(
                current_content,
                job_description,
                &current_section,
                cursor_position,
            )
            .await?;

        // 4. Identify priority improvements for the current section
        let priority_improvements = self
            .identify_priority_improvements_for_section(current_content, &current_section)
            .await?;

        // 5. Calculate real-time score and feedback
        let real_time_score = self
            .calculate_real_time_score(current_content, job_description)
            .await?;
        let score_change = self.calculate_score_change(real_time_score);

        // 6. Generate typing feedback
        let typing_feedback = self
            .generate_typing_feedback(current_content, cursor_position)
            .await?;

        // 7. Calculate section strength and completion
        let section_strength = self.calculate_section_strength(current_content);
        let completion_percentage =
            self.calculate_completion_percentage(current_content, &current_section);

        // 8. Determine next recommended action
        let next_recommended_action =
            self.determine_next_action(&context_suggestions, &priority_improvements);

        Ok(LiveSuggestions {
            current_section,
            context_suggestions,
            priority_improvements,
            real_time_score,
            score_change,
            typing_feedback,
            section_strength,
            completion_percentage,
            next_recommended_action,
        })
    }

    async fn generate_context_suggestions(
        &self,
        content: &str,
        job_description: &str,
        section: &Section,
        cursor_position: usize,
    ) -> Result<Vec<ContextualSuggestion>> {
        let mut suggestions = Vec::new();

        match section {
            Section::Experience => {
                suggestions.extend(
                    self.generate_experience_suggestions(content, job_description, cursor_position)
                        .await?,
                );
            }
            Section::Summary => {
                suggestions.extend(
                    self.generate_summary_suggestions(content, job_description, cursor_position)
                        .await?,
                );
            }
            Section::Skills => {
                suggestions.extend(
                    self.generate_skills_suggestions(content, job_description, cursor_position)
                        .await?,
                );
            }
            Section::Education => {
                suggestions.extend(
                    self.generate_education_suggestions(content, cursor_position)
                        .await?,
                );
            }
            Section::Projects => {
                suggestions.extend(
                    self.generate_projects_suggestions(content, job_description, cursor_position)
                        .await?,
                );
            }
            _ => {
                suggestions.extend(
                    self.generate_general_suggestions(content, cursor_position)
                        .await?,
                );
            }
        }

        // Sort by confidence and impact
        suggestions.sort_by(|a, b| {
            let a_score = a.confidence * a.impact_score;
            let b_score = b.confidence * b.impact_score;
            b_score.partial_cmp(&a_score).unwrap()
        });

        // Limit to top 5 suggestions to avoid overwhelming user
        Ok(suggestions.into_iter().take(5).collect())
    }

    async fn generate_experience_suggestions(
        &self,
        content: &str,
        job_description: &str,
        cursor_position: usize,
    ) -> Result<Vec<ContextualSuggestion>> {
        let mut suggestions = Vec::new();

        // Extract current bullet point being edited
        let current_bullet = self.extract_current_bullet(content, cursor_position);

        if let Some(bullet) = current_bullet {
            // Analyze current bullet for weaknesses
            let bullet_analysis = self.achievement_analyzer.analyze_achievements(&bullet)?;

            // Generate X-Y-Z improvement suggestions
            if !bullet_analysis.improvement_opportunities.is_empty() {
                for opportunity in &bullet_analysis.improvement_opportunities {
                    suggestions.push(ContextualSuggestion {
                        suggestion_id: format!("xyz_{}", suggestions.len()),
                        type_: "xyz_improvement".to_string(),
                        title: "Enhance with X-Y-Z Formula".to_string(),
                        description: "Structure this achievement using 'Accomplished [X] as measured by [Y], by doing [Z]'".to_string(),
                        suggestion: opportunity.improved_version.clone(),
                        confidence: 0.85,
                        applicable_text: opportunity.original.clone(),
                        replacement_text: Some(opportunity.improved_version.clone()),
                        explanation: opportunity.explanation.clone(),
                        impact_score: opportunity.improvement_impact,
                        urgency: "immediate".to_string(),
                    });
                }
            }

            // Check for missing quantification
            if !bullet.contains(char::is_numeric) {
                suggestions.push(ContextualSuggestion {
                    suggestion_id: format!("quant_{}", suggestions.len()),
                    type_: "quantification".to_string(),
                    title: "Add Quantifiable Results".to_string(),
                    description:
                        "Include specific numbers, percentages, or metrics to demonstrate impact"
                            .to_string(),
                    suggestion: self.suggest_quantification_for_bullet(&bullet),
                    confidence: 0.75,
                    applicable_text: bullet.clone(),
                    replacement_text: None,
                    explanation: "Quantified achievements are more compelling and memorable"
                        .to_string(),
                    impact_score: 20.0,
                    urgency: "soon".to_string(),
                });
            }
        }

        // Check for missing keywords from job description
        let missing_keywords = self
            .identify_missing_keywords_in_section(content, job_description, "experience")
            .await?;
        for keyword in missing_keywords.iter().take(2) {
            suggestions.push(ContextualSuggestion {
                suggestion_id: format!("keyword_{}", suggestions.len()),
                type_: "keyword_integration".to_string(),
                title: format!("Add Missing Keyword: {}", keyword),
                description: format!(
                    "Consider integrating '{}' into a relevant bullet point",
                    keyword
                ),
                suggestion: self
                    .suggest_keyword_integration_for_experience(content, keyword)
                    .await,
                confidence: 0.70,
                applicable_text: self.find_best_integration_spot(content, keyword),
                replacement_text: None,
                explanation: format!(
                    "'{}' appears in the job description but not in your experience",
                    keyword
                ),
                impact_score: 15.0,
                urgency: "soon".to_string(),
            });
        }

        Ok(suggestions)
    }

    async fn generate_summary_suggestions(
        &self,
        content: &str,
        job_description: &str,
        _cursor_position: usize,
    ) -> Result<Vec<ContextualSuggestion>> {
        let mut suggestions = Vec::new();

        // Analyze summary for key elements
        let summary_section = self.extract_summary_section(content);

        if let Some(summary) = summary_section {
            // Check if summary includes years of experience
            if !self.contains_years_of_experience(&summary) {
                suggestions.push(ContextualSuggestion {
                    suggestion_id: "summary_experience".to_string(),
                    type_: "summary_enhancement".to_string(),
                    title: "Add Years of Experience".to_string(),
                    description: "Include your years of relevant experience in the summary"
                        .to_string(),
                    suggestion: "Consider starting with '[X] years of experience in...'"
                        .to_string(),
                    confidence: 0.80,
                    applicable_text: summary.clone(),
                    replacement_text: None,
                    explanation: "Years of experience help recruiters quickly assess your level"
                        .to_string(),
                    impact_score: 18.0,
                    urgency: "soon".to_string(),
                });
            }

            // Check for missing key skills from job description
            let missing_skills = self
                .identify_missing_summary_keywords(&summary, job_description)
                .await?;
            if !missing_skills.is_empty() {
                suggestions.push(ContextualSuggestion {
                    suggestion_id: "summary_keywords".to_string(),
                    type_: "keyword_integration".to_string(),
                    title: "Add Key Skills to Summary".to_string(),
                    description: format!("Consider adding: {}", missing_skills.join(", ")),
                    suggestion: format!("Integrate these skills: {}", missing_skills.join(", ")),
                    confidence: 0.75,
                    applicable_text: summary.clone(),
                    replacement_text: None,
                    explanation: "Summary should highlight your most relevant skills".to_string(),
                    impact_score: 22.0,
                    urgency: "immediate".to_string(),
                });
            }
        }

        Ok(suggestions)
    }

    async fn generate_skills_suggestions(
        &self,
        content: &str,
        job_description: &str,
        _cursor_position: usize,
    ) -> Result<Vec<ContextualSuggestion>> {
        let mut suggestions = Vec::new();

        // Extract skills from content
        let skills_list = self.extract_skills_section(content).await?;
        let skills_text = skills_list.join(", ");

        if !skills_list.is_empty() {
            // Find missing technical skills
            let missing_technical_skills = self
                .identify_missing_technical_skills(&skills_text, job_description)
                .await?;

            for skill in missing_technical_skills.iter().take(3) {
                suggestions.push(ContextualSuggestion {
                    suggestion_id: format!("skill_{}", skill),
                    type_: "skill_addition".to_string(),
                    title: format!("Add Skill: {}", skill),
                    description: format!(
                        "Consider adding '{}' if you have experience with it",
                        skill
                    ),
                    suggestion: format!("Add '{}' to your skills list", skill),
                    confidence: 0.65,
                    applicable_text: skills_text.clone(),
                    replacement_text: None,
                    explanation: format!("'{}' is mentioned in the job requirements", skill),
                    impact_score: 12.0,
                    urgency: "later".to_string(),
                });
            }

            // Check for skills organization
            if !self.is_skills_section_organized(&skills_text) {
                suggestions.push(ContextualSuggestion {
                    suggestion_id: "skills_organization".to_string(),
                    type_: "structure_improvement".to_string(),
                    title: "Organize Skills by Category".to_string(),
                    description: "Group skills into categories (Technical, Programming Languages, Tools, etc.)".to_string(),
                    suggestion: "Consider organizing: Technical Skills, Programming Languages, Frameworks, Tools".to_string(),
                    confidence: 0.70,
                    applicable_text: skills_text.clone(),
                    replacement_text: None,
                    explanation: "Organized skills are easier for recruiters to scan".to_string(),
                    impact_score: 15.0,
                    urgency: "later".to_string(),
                });
            }
        }

        Ok(suggestions)
    }

    async fn generate_education_suggestions(
        &self,
        content: &str,
        _cursor_position: usize,
    ) -> Result<Vec<ContextualSuggestion>> {
        let mut suggestions = Vec::new();

        let education_section = self.extract_education_section(content);

        if let Some(education) = education_section {
            // Check for missing GPA (if recent graduate)
            if self.appears_to_be_recent_graduate(&education) && !self.contains_gpa(&education) {
                suggestions.push(ContextualSuggestion {
                    suggestion_id: "education_gpa".to_string(),
                    type_: "education_enhancement".to_string(),
                    title: "Consider Adding GPA".to_string(),
                    description: "If your GPA is 3.5 or higher, consider including it".to_string(),
                    suggestion: "Add 'GPA: X.X' if 3.5 or above".to_string(),
                    confidence: 0.60,
                    applicable_text: education.clone(),
                    replacement_text: None,
                    explanation: "Strong GPA can be valuable for recent graduates".to_string(),
                    impact_score: 8.0,
                    urgency: "later".to_string(),
                });
            }

            // Check for relevant coursework
            if !self.contains_relevant_coursework(&education) {
                suggestions.push(ContextualSuggestion {
                    suggestion_id: "education_coursework".to_string(),
                    type_: "education_enhancement".to_string(),
                    title: "Add Relevant Coursework".to_string(),
                    description: "Include 3-4 most relevant courses to the target role".to_string(),
                    suggestion: "Add 'Relevant Coursework: [Course1], [Course2], [Course3]'"
                        .to_string(),
                    confidence: 0.65,
                    applicable_text: education.clone(),
                    replacement_text: None,
                    explanation: "Relevant coursework shows specific preparation for the role"
                        .to_string(),
                    impact_score: 10.0,
                    urgency: "later".to_string(),
                });
            }
        }

        Ok(suggestions)
    }

    async fn generate_projects_suggestions(
        &self,
        content: &str,
        job_description: &str,
        _cursor_position: usize,
    ) -> Result<Vec<ContextualSuggestion>> {
        let mut suggestions = Vec::new();

        let projects_section = self.extract_projects_section(content);

        if let Some(projects) = projects_section {
            // Check for missing project links
            if !self.contains_project_links(&projects) {
                suggestions.push(ContextualSuggestion {
                    suggestion_id: "project_links".to_string(),
                    type_: "project_enhancement".to_string(),
                    title: "Add Project Links".to_string(),
                    description: "Include GitHub repository or live demo links".to_string(),
                    suggestion:
                        "Add 'GitHub: github.com/username/project' or 'Live Demo: project-url.com'"
                            .to_string(),
                    confidence: 0.75,
                    applicable_text: projects.clone(),
                    replacement_text: None,
                    explanation: "Project links allow recruiters to see your actual work"
                        .to_string(),
                    impact_score: 20.0,
                    urgency: "soon".to_string(),
                });
            }

            // Check for missing technologies used
            let project_tech_gaps = self
                .identify_missing_project_technologies(&projects, job_description)
                .await?;
            if !project_tech_gaps.is_empty() {
                suggestions.push(ContextualSuggestion {
                    suggestion_id: "project_tech".to_string(),
                    type_: "technology_highlighting".to_string(),
                    title: "Highlight Relevant Technologies".to_string(),
                    description: format!(
                        "Emphasize these technologies: {}",
                        project_tech_gaps.join(", ")
                    ),
                    suggestion: format!(
                        "Include these in project descriptions: {}",
                        project_tech_gaps.join(", ")
                    ),
                    confidence: 0.70,
                    applicable_text: projects.clone(),
                    replacement_text: None,
                    explanation: "Highlighting relevant technologies improves keyword matching"
                        .to_string(),
                    impact_score: 16.0,
                    urgency: "soon".to_string(),
                });
            }
        }

        Ok(suggestions)
    }

    async fn generate_general_suggestions(
        &self,
        content: &str,
        _cursor_position: usize,
    ) -> Result<Vec<ContextualSuggestion>> {
        let mut suggestions = Vec::new();

        // General formatting suggestions
        if self.has_formatting_issues(content) {
            suggestions.push(ContextualSuggestion {
                suggestion_id: "general_formatting".to_string(),
                type_: "formatting".to_string(),
                title: "Improve Formatting".to_string(),
                description: "Ensure consistent formatting throughout the document".to_string(),
                suggestion: "Use consistent bullet points, spacing, and font styles".to_string(),
                confidence: 0.60,
                applicable_text: content.to_string(),
                replacement_text: None,
                explanation: "Consistent formatting improves readability and ATS parsing"
                    .to_string(),
                impact_score: 10.0,
                urgency: "later".to_string(),
            });
        }

        Ok(suggestions)
    }

    async fn identify_priority_improvements_for_section(
        &self,
        content: &str,
        section: &Section,
    ) -> Result<Vec<PriorityImprovement>> {
        let mut improvements = Vec::new();

        match section {
            Section::Experience => {
                // Analyze achievement strength in experience section
                let experience_content = self.extract_section_content(content, "experience");
                if let Some(exp_content) = experience_content {
                    let achievement_analysis = self
                        .achievement_analyzer
                        .analyze_achievements(&exp_content)?;

                    if achievement_analysis.xyz_formula_compliance < 50.0 {
                        improvements.push(PriorityImprovement {
                            improvement_id: "experience_xyz".to_string(),
                            category: "achievement".to_string(),
                            title: "Improve Achievement Structure".to_string(),
                            description: "Many bullet points lack the X-Y-Z formula structure".to_string(),
                            current_issues: vec![
                                "Low X-Y-Z compliance".to_string(),
                                "Missing quantification".to_string(),
                                "Weak action verbs".to_string(),
                            ],
                            suggested_fixes: vec![
                                "Rewrite bullets using 'Accomplished [X] as measured by [Y], by doing [Z]'".to_string(),
                                "Add specific numbers and percentages".to_string(),
                                "Start with strong action verbs".to_string(),
                            ],
                            impact_score: 25.0,
                            implementation_effort: "medium".to_string(),
                            example_before: Some("Worked on improving system performance".to_string()),
                            example_after: Some("Improved system performance by 40% as measured by response time reduction, by implementing caching and database optimization".to_string()),
                        });
                    }
                }
            }
            Section::Summary => {
                let summary_content = self.extract_section_content(content, "summary");
                if summary_content.is_none() || summary_content.as_ref().unwrap().len() < 50 {
                    improvements.push(PriorityImprovement {
                        improvement_id: "summary_missing".to_string(),
                        category: "structure".to_string(),
                        title: "Add Professional Summary".to_string(),
                        description: "Resume lacks a compelling professional summary".to_string(),
                        current_issues: vec!["No summary section".to_string()],
                        suggested_fixes: vec![
                            "Add 2-3 sentence professional summary".to_string(),
                            "Highlight years of experience and key skills".to_string(),
                            "Include career objective".to_string(),
                        ],
                        impact_score: 20.0,
                        implementation_effort: "easy".to_string(),
                        example_before: None,
                        example_after: Some("Experienced software engineer with 5+ years developing scalable web applications using React, Node.js, and AWS. Proven track record of improving system performance and leading cross-functional teams.".to_string()),
                    });
                }
            }
            _ => {}
        }

        Ok(improvements)
    }

    async fn generate_typing_feedback(
        &self,
        content: &str,
        cursor_position: usize,
    ) -> Result<TypingFeedback> {
        // Analyze current bullet being typed
        let current_bullet_analysis =
            if let Some(bullet) = self.extract_current_bullet(content, cursor_position) {
                let analysis = self.achievement_analyzer.analyze_achievements(&bullet)?;
                analysis.strong_achievements.first().cloned()
            } else {
                None
            };

        // Generate live suggestions for current typing
        let live_suggestions = self.generate_live_typing_suggestions(content, cursor_position);

        // Calculate text metrics
        let word_count = content.split_whitespace().count();
        let character_count = content.chars().count();
        let estimated_reading_time = format!("{} seconds", (word_count / 200).max(1));

        // Analyze tone
        let tone_analysis = self.analyze_tone(content);

        // Calculate clarity score
        let clarity_score = self.calculate_clarity_score(content);

        Ok(TypingFeedback {
            current_bullet_analysis,
            live_suggestions,
            word_count,
            character_count,
            estimated_reading_time,
            tone_analysis,
            clarity_score,
        })
    }

    // Helper methods for real-time analysis
    fn identify_current_section(&self, content: &str, cursor_position: usize) -> Section {
        let lines: Vec<&str> = content.lines().collect();
        let mut char_count = 0;
        let mut current_section = Section::Unknown;

        for line in lines {
            if char_count >= cursor_position {
                break;
            }

            let line_lower = line.trim().to_lowercase();
            if line_lower.contains("summary") || line_lower.contains("objective") {
                current_section = Section::Summary;
            } else if line_lower.contains("experience") || line_lower.contains("employment") {
                current_section = Section::Experience;
            } else if line_lower.contains("education") || line_lower.contains("academic") {
                current_section = Section::Education;
            } else if line_lower.contains("skills") || line_lower.contains("competencies") {
                current_section = Section::Skills;
            } else if line_lower.contains("projects") {
                current_section = Section::Projects;
            } else if line_lower.contains("achievements") || line_lower.contains("awards") {
                current_section = Section::Achievements;
            } else if line_lower.contains("certifications") || line_lower.contains("licenses") {
                current_section = Section::Certifications;
            }

            char_count += line.len() + 1; // +1 for newline
        }

        current_section
    }

    fn track_content_changes(&mut self, current_content: &str) {
        if self.change_tracker.previous_content != current_content {
            // Record the change
            let change = ContentChange {
                timestamp: chrono::Utc::now(),
                change_type: "edit".to_string(),
                old_text: self.change_tracker.previous_content.clone(),
                new_text: current_content.to_string(),
                section: "unknown".to_string(), // Could be enhanced to detect section
                impact_score: 0.0,              // Could be calculated
            };

            self.change_tracker.change_history.push(change);
            self.change_tracker.previous_content = current_content.to_string();
            self.change_tracker.performance_metrics.total_changes += 1;
        }
    }

    async fn calculate_real_time_score(&self, content: &str, job_description: &str) -> Result<f64> {
        // Quick scoring algorithm for real-time feedback
        let mut score = 0.0;

        // Achievement analysis weight: 40%
        if let Ok(achievement_analysis) = self.achievement_analyzer.analyze_achievements(content) {
            score += achievement_analysis.overall_achievement_score * 0.4;
        }

        // Keyword matching weight: 30%
        let keyword_score = self.quick_keyword_match(content, job_description);
        score += keyword_score * 0.3;

        // Format score weight: 30%
        let format_score = self.quick_format_score(content);
        score += format_score * 0.3;

        Ok(score)
    }

    fn calculate_score_change(&mut self, current_score: f64) -> f64 {
        let previous_score = self
            .change_tracker
            .performance_metrics
            .score_trajectory
            .last()
            .copied()
            .unwrap_or(0.0);
        self.change_tracker
            .performance_metrics
            .score_trajectory
            .push(current_score);

        // Keep only last 10 scores for memory efficiency
        if self
            .change_tracker
            .performance_metrics
            .score_trajectory
            .len()
            > 10
        {
            self.change_tracker
                .performance_metrics
                .score_trajectory
                .remove(0);
        }

        current_score - previous_score
    }

    // Placeholder implementations for helper methods
    fn extract_current_bullet(&self, content: &str, cursor_position: usize) -> Option<String> {
        // Find the bullet point that contains the cursor position
        let lines: Vec<&str> = content.lines().collect();
        let mut char_count = 0;

        for line in lines {
            if char_count <= cursor_position
                && cursor_position <= char_count + line.len()
                && (line.trim_start().starts_with('•') || line.trim_start().starts_with('-'))
            {
                return Some(line.trim().to_string());
            }
            char_count += line.len() + 1;
        }
        None
    }

    fn suggest_quantification_for_bullet(&self, _bullet: &str) -> String {
        "Consider adding specific numbers: 'by X%', 'reduced by Y minutes', 'increased by Z units'"
            .to_string()
    }

    async fn identify_missing_keywords_in_section(
        &self,
        content: &str,
        job_description: &str,
        section: &str,
    ) -> Result<Vec<String>> {
        use crate::ollama::OllamaClient;

        let section_keyword_prompt = format!(
            "Analyze the {} section of this resume against the job description to identify missing important keywords.

{} Section Content:
{}

Job Description:
{}

Focus on identifying keywords that:
1. Are mentioned in the job description but missing from this resume section
2. Are relevant to the {} section specifically
3. Would improve ATS keyword matching
4. Are industry-standard terms for this type of role

For {} section, look for:
{}

Return missing keywords as a JSON array, prioritizing the most important ones.
Example: [\"Project Management\", \"Stakeholder Communication\", \"Process Improvement\"]",
            section, section, content, job_description, section, section,
            match section {
                "experience" => "- Technical skills used in projects\n- Methodologies and processes\n- Tools and technologies\n- Leadership and management terms\n- Industry-specific terminology",
                "summary" => "- Key role titles and functions\n- Core competencies\n- Industry expertise areas\n- Leadership qualities\n- Technical specializations",
                "skills" => "- Technical skills and tools\n- Programming languages\n- Frameworks and libraries\n- Certifications\n- Software proficiency",
                _ => "- Relevant professional terminology\n- Industry-standard keywords\n- Role-specific skills\n- Technical capabilities"
            }
        );

        let ollama_client = OllamaClient::new(None)?;
        let response = ollama_client
            .generate_ml_analysis("qwen2.5:14b", &section_keyword_prompt, "keyword_analysis")
            .await?;

        match serde_json::from_str::<Vec<String>>(&response) {
            Ok(keywords) => {
                info!(
                    "ML identified {} missing keywords for {} section",
                    keywords.len(),
                    section
                );
                Ok(keywords.into_iter().take(5).collect()) // Limit to top 5 per section
            }
            Err(e) => {
                log::warn!(
                    "ML section keyword analysis failed for {}: {}, using fallback",
                    section,
                    e
                );
                self.fallback_section_keyword_analysis(content, job_description, section)
            }
        }
    }

    fn fallback_section_keyword_analysis(
        &self,
        content: &str,
        job_description: &str,
        section: &str,
    ) -> Result<Vec<String>> {
        let content_lower = content.to_lowercase();
        let job_lower = job_description.to_lowercase();

        let section_keywords = match section {
            "experience" => vec![
                "project management",
                "team leadership",
                "agile",
                "scrum",
                "ci/cd",
                "architecture",
                "scalability",
                "performance",
                "optimization",
                "automation",
                "collaboration",
                "stakeholder",
                "requirements",
                "implementation",
                "deployment",
            ],
            "summary" => vec![
                "senior",
                "lead",
                "principal",
                "architect",
                "manager",
                "director",
                "expertise",
                "specialization",
                "proficiency",
                "leadership",
                "strategic",
                "innovative",
                "results-driven",
                "cross-functional",
                "technical leadership",
            ],
            "skills" => vec![
                "programming",
                "development",
                "frameworks",
                "databases",
                "cloud",
                "devops",
                "testing",
                "monitoring",
                "security",
                "apis",
                "microservices",
            ],
            _ => vec![
                "professional",
                "technical",
                "analytical",
                "problem-solving",
                "communication",
            ],
        };

        let mut missing_keywords = Vec::new();

        for keyword in section_keywords {
            if job_lower.contains(keyword) && !content_lower.contains(keyword) {
                // Convert to title case
                let title_case = keyword
                    .split_whitespace()
                    .map(|word| {
                        let mut chars = word.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => {
                                first.to_uppercase().collect::<String>() + chars.as_str()
                            }
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" ");
                missing_keywords.push(title_case);
            }
        }

        info!(
            "Fallback found {} missing keywords for {} section",
            missing_keywords.len(),
            section
        );
        Ok(missing_keywords.into_iter().take(4).collect())
    }

    async fn suggest_keyword_integration_for_experience(
        &self,
        content: &str,
        keyword: &str,
    ) -> String {
        use crate::ollama::OllamaClient;

        let integration_prompt = format!(
            "Suggest how to naturally integrate the keyword '{}' into this experience section without keyword stuffing.

Current Experience Content:
{}

Provide a specific, actionable suggestion for where and how to incorporate '{}' naturally. 
Focus on:
1. Identifying the most relevant existing bullet point or accomplishment
2. Suggesting specific wording that feels natural
3. Maintaining the authenticity of the experience

Provide a concise integration suggestion.",
            keyword, content, keyword
        );

        let ollama_client = match OllamaClient::new(None) {
            Ok(client) => client,
            Err(_) => return self.fallback_keyword_integration_suggestion(content, keyword),
        };

        match ollama_client
            .generate_ml_analysis("mistral:latest", &integration_prompt, "keyword_integration")
            .await
        {
            Ok(response) => {
                info!(
                    "Generated ML keyword integration suggestion for: {}",
                    keyword
                );
                response
            }
            Err(_) => self.fallback_keyword_integration_suggestion(content, keyword),
        }
    }

    fn fallback_keyword_integration_suggestion(&self, content: &str, keyword: &str) -> String {
        // Analyze content to find best integration point
        let lines: Vec<&str> = content.lines().collect();

        for line in &lines {
            let line_lower = line.to_lowercase();
            let _keyword_lower = keyword.to_lowercase();

            // Look for related contexts
            if line_lower.contains("project")
                || line_lower.contains("develop")
                || line_lower.contains("implement")
            {
                return format!("Consider adding '{}' to this accomplishment: '{}'. For example, you could specify that you used {} in this project or how {} contributed to the outcome.", keyword, line.trim(), keyword, keyword);
            }
        }

        // Generic suggestion if no specific context found
        format!("Consider mentioning '{}' in a relevant project description or accomplishment where you actually used this skill/technology. Be specific about how {} contributed to your results.", keyword, keyword)
    }

    fn find_best_integration_spot(&self, _content: &str, _keyword: &str) -> String {
        "Best integration location".to_string()
    }

    fn extract_summary_section(&self, content: &str) -> Option<String> {
        // Extract summary section from content
        let lines: Vec<&str> = content.lines().collect();
        let mut in_summary = false;
        let mut summary_lines = Vec::new();

        for line in lines {
            let line_lower = line.trim().to_lowercase();
            if line_lower.contains("summary") || line_lower.contains("objective") {
                in_summary = true;
                continue;
            }
            if in_summary {
                if line.trim().is_empty() {
                    continue;
                }
                if line_lower.contains("experience")
                    || line_lower.contains("education")
                    || line_lower.contains("skills")
                {
                    break;
                }
                summary_lines.push(line);
            }
        }

        if summary_lines.is_empty() {
            None
        } else {
            Some(summary_lines.join("\n"))
        }
    }

    fn contains_years_of_experience(&self, summary: &str) -> bool {
        summary.contains("year")
            && (summary.contains("experience")
                || summary.contains("developer")
                || summary.contains("engineer"))
    }

    async fn identify_missing_summary_keywords(
        &self,
        _summary: &str,
        _job_description: &str,
    ) -> Result<Vec<String>> {
        Ok(vec!["leadership".to_string(), "agile".to_string()]) // Placeholder
    }

    async fn extract_skills_section(&self, content: &str) -> Result<Vec<String>> {
        use crate::ollama::OllamaClient;

        let skill_extraction_prompt = format!(
            "Extract technical skills, programming languages, frameworks, tools, and technologies from this resume text.

Focus on extracting:
- Programming languages (Python, Java, JavaScript, Rust, Go, C++, etc.)
- Web frameworks (React, Angular, Vue, Django, Flask, Express, etc.)
- Database technologies (MySQL, PostgreSQL, MongoDB, Redis, etc.)
- Cloud platforms (AWS, Azure, GCP, Docker, Kubernetes, etc.)
- Development tools (Git, Jenkins, CI/CD, Terraform, etc.)
- Data technologies (Pandas, NumPy, TensorFlow, PyTorch, etc.)
- Other technical tools and methodologies

Resume text:
{}

Extract skills and return as a JSON array of strings. Only include technical skills, not soft skills.
Example: [\"Python\", \"React\", \"AWS\", \"Docker\", \"PostgreSQL\"]",
            content
        );

        let ollama_client = OllamaClient::new(None)?;
        let response = ollama_client
            .generate_ml_analysis(
                "mistral:latest",
                &skill_extraction_prompt,
                "skill_extraction",
            )
            .await?;

        // Parse JSON response or fallback to regex extraction
        match serde_json::from_str::<Vec<String>>(&response) {
            Ok(skills) => {
                info!("Successfully extracted {} skills using ML", skills.len());
                Ok(skills)
            }
            Err(e) => {
                log::warn!("ML skill extraction failed: {}, using fallback", e);
                self.fallback_skill_extraction(content)
            }
        }
    }

    fn fallback_skill_extraction(&self, content: &str) -> Result<Vec<String>> {
        let mut skills = std::collections::HashSet::new();
        let content_lower = content.to_lowercase();

        // Comprehensive skill patterns optimized for common resume formats
        let skill_patterns = vec![
            // Programming languages
            (
                r"(?i)\b(python|java|javascript|typescript|rust|go|golang|c\+\+|c#|php|ruby|swift|kotlin|scala|r\b|matlab)\b",
                "Programming Languages",
            ),
            // Web frameworks
            (
                r"(?i)\b(react|angular|vue|svelte|django|flask|spring|express|laravel|rails|nextjs|nuxt)\b",
                "Web Frameworks",
            ),
            // Databases
            (
                r"(?i)\b(mysql|postgresql|postgres|mongodb|redis|elasticsearch|cassandra|dynamodb|sqlite|oracle)\b",
                "Databases",
            ),
            // Cloud and DevOps
            (
                r"(?i)\b(aws|azure|gcp|google cloud|docker|kubernetes|k8s|jenkins|terraform|ansible|vagrant|helm)\b",
                "Cloud/DevOps",
            ),
            // Data and ML
            (
                r"(?i)\b(pandas|numpy|tensorflow|pytorch|scikit-learn|spark|hadoop|tableau|powerbi|jupyter)\b",
                "Data/ML",
            ),
            // Tools
            (
                r"(?i)\b(git|github|gitlab|jira|confluence|slack|figma|sketch|photoshop|illustrator)\b",
                "Tools",
            ),
        ];

        for (pattern, _category) in skill_patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                for mat in regex.find_iter(&content_lower) {
                    let skill = mat.as_str().to_string();
                    // Capitalize first letter for consistent formatting
                    let formatted_skill = skill
                        .chars()
                        .enumerate()
                        .map(|(i, c)| {
                            if i == 0 {
                                c.to_uppercase().collect::<String>()
                            } else {
                                c.to_string()
                            }
                        })
                        .collect::<String>();
                    skills.insert(formatted_skill);
                }
            }
        }

        let skills_vec: Vec<String> = skills.into_iter().collect();
        info!(
            "Fallback skill extraction found {} skills",
            skills_vec.len()
        );
        Ok(skills_vec)
    }

    async fn identify_missing_technical_skills(
        &self,
        resume_skills: &str,
        job_description: &str,
    ) -> Result<Vec<String>> {
        use crate::ollama::OllamaClient;

        let keyword_analysis_prompt = format!(
            "Compare the technical skills in this resume with the job requirements and identify missing technical skills.

Resume Skills Section:
{}

Job Description:
{}

Analyze what technical skills, tools, frameworks, or technologies are mentioned in the job description but missing from the resume skills.

Focus on:
- Programming languages required but not listed
- Frameworks or libraries mentioned in job but not in resume
- Tools and technologies required by the position
- Cloud platforms or DevOps tools needed
- Database technologies required
- Methodologies (Agile, CI/CD, TDD) if mentioned in job

Return missing technical skills as a JSON array.
Example: [\"Docker\", \"Kubernetes\", \"Terraform\", \"CI/CD\"]",
            resume_skills, job_description
        );

        let ollama_client = OllamaClient::new(None)?;
        let response = ollama_client
            .generate_ml_analysis("qwen2.5:14b", &keyword_analysis_prompt, "keyword_analysis")
            .await?;

        match serde_json::from_str::<Vec<String>>(&response) {
            Ok(missing_skills) => {
                info!(
                    "ML identified {} missing technical skills",
                    missing_skills.len()
                );
                Ok(missing_skills.into_iter().take(8).collect()) // Limit to top 8 most important
            }
            Err(e) => {
                log::warn!("ML keyword analysis failed: {}, using fallback", e);
                self.fallback_missing_skills_analysis(resume_skills, job_description)
            }
        }
    }

    fn fallback_missing_skills_analysis(
        &self,
        resume_skills: &str,
        job_description: &str,
    ) -> Result<Vec<String>> {
        let resume_lower = resume_skills.to_lowercase();
        let job_lower = job_description.to_lowercase();

        // Common technical skills to check for
        let common_tech_skills = vec![
            "docker",
            "kubernetes",
            "terraform",
            "ansible",
            "jenkins",
            "git",
            "ci/cd",
            "aws",
            "azure",
            "gcp",
            "python",
            "java",
            "javascript",
            "typescript",
            "go",
            "rust",
            "react",
            "angular",
            "vue",
            "nodejs",
            "express",
            "django",
            "flask",
            "spring",
            "mysql",
            "postgresql",
            "mongodb",
            "redis",
            "elasticsearch",
            "agile",
            "scrum",
            "tdd",
            "microservices",
            "api",
            "rest",
            "graphql",
        ];

        let mut missing_skills = Vec::new();

        for skill in common_tech_skills {
            if job_lower.contains(skill) && !resume_lower.contains(skill) {
                // Capitalize first letter
                let formatted_skill = skill
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if i == 0 {
                            c.to_uppercase().collect::<String>()
                        } else {
                            c.to_string()
                        }
                    })
                    .collect::<String>();
                missing_skills.push(formatted_skill);
            }
        }

        info!(
            "Fallback analysis found {} missing skills",
            missing_skills.len()
        );
        Ok(missing_skills.into_iter().take(6).collect())
    }

    fn is_skills_section_organized(&self, skills: &str) -> bool {
        let skills_lower = skills.to_lowercase();
        let mut organization_score = 0;

        // Check for bullet points or clear structure
        if skills.contains("•") || skills.contains("-") || skills.contains("*") {
            organization_score += 2;
        }

        // Check for categories
        let category_indicators = vec![
            "programming languages",
            "frameworks",
            "databases",
            "tools",
            "technologies",
            "frontend",
            "backend",
            "devops",
            "cloud",
            "languages",
            "technical skills",
        ];

        for indicator in category_indicators {
            if skills_lower.contains(indicator) {
                organization_score += 3;
                break;
            }
        }

        // Check for consistent formatting (colons, sections)
        if skills.contains(":") {
            organization_score += 1;
        }

        // Check for line breaks indicating structure
        let lines: Vec<&str> = skills.lines().collect();
        if lines.len() >= 3 {
            organization_score += 1;
        }

        // Check if skills are not just a long comma-separated list
        let comma_count = skills.matches(',').count();
        let word_count = skills.split_whitespace().count();

        if comma_count > 10 && word_count > 20 && !skills.contains('\n') {
            // Likely an unorganized comma list
            organization_score -= 2;
        }

        // Well organized if score >= 3
        organization_score >= 3
    }

    fn extract_education_section(&self, content: &str) -> Option<String> {
        // Common education section headers
        let education_patterns = vec![
            r"(?i)\b(education|academic|university|college|degree|certification)\b.*?:",
            r"(?i)^\s*(education|academic background|educational background)",
            r"(?i)^\s*#{1,3}\s*(education|academic|university)",
        ];

        for pattern in education_patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                if let Some(mat) = regex.find(content) {
                    let start = mat.start();
                    let remaining_content = &content[start..];

                    // Find the end of the section (next major section or end of content)
                    let section_end_patterns = vec![
                        r"(?i)\n\s*(experience|work history|employment|skills|projects|certifications)\s*[:\n]",
                        r"(?i)\n\s*#{1,3}\s*(experience|work|skills|projects)",
                    ];

                    let mut section_end = remaining_content.len();
                    for end_pattern in section_end_patterns {
                        if let Ok(end_regex) = regex::Regex::new(end_pattern) {
                            if let Some(end_match) = end_regex.find(remaining_content) {
                                section_end = section_end.min(end_match.start());
                            }
                        }
                    }

                    let education_content = &remaining_content[..section_end];
                    if education_content.trim().len() > 10 {
                        return Some(education_content.trim().to_string());
                    }
                }
            }
        }

        // Fallback: look for degree keywords anywhere in the content
        let degree_keywords = vec![
            "bachelor",
            "master",
            "phd",
            "doctorate",
            "degree",
            "university",
            "college",
        ];
        let content_lower = content.to_lowercase();

        for keyword in degree_keywords {
            if content_lower.contains(keyword) {
                // Extract a reasonable chunk around the keyword
                if let Some(pos) = content_lower.find(keyword) {
                    let start = pos.saturating_sub(100);
                    let end = (pos + 200).min(content.len());
                    return Some(content[start..end].trim().to_string());
                }
            }
        }

        None
    }

    fn appears_to_be_recent_graduate(&self, education: &str) -> bool {
        let education_lower = education.to_lowercase();
        use chrono::Datelike;
        let current_year = chrono::Utc::now().year();

        // Look for graduation years in the last 3 years
        for year in (current_year - 3)..=current_year {
            if education_lower.contains(&year.to_string()) {
                return true;
            }
        }

        // Look for terms indicating recent graduation
        let recent_grad_indicators = vec![
            "recent graduate",
            "new graduate",
            "fresh graduate",
            "graduating",
            "expected graduation",
            "anticipated graduation",
            "bachelor's degree",
            "master's degree",
            "bs ",
            "ms ",
            "ba ",
            "ma ",
        ];

        for indicator in recent_grad_indicators {
            if education_lower.contains(indicator) {
                return true;
            }
        }

        // Look for student-related terms
        let student_indicators = [
            "student",
            "gpa",
            "dean's list",
            "honors",
            "cum laude",
            "thesis",
            "capstone",
            "coursework",
            "relevant courses",
        ];

        let student_indicator_count = student_indicators
            .iter()
            .filter(|&indicator| education_lower.contains(indicator))
            .count();

        // If multiple student indicators, likely recent graduate
        if student_indicator_count >= 2 {
            return true;
        }

        // Check for lack of extensive work experience (suggesting recent grad)
        let experience_indicators = [
            "years experience",
            "senior",
            "lead",
            "manager",
            "director",
            "principal",
            "architect",
            "10+",
            "15+",
            "20+",
        ];

        let has_extensive_experience = experience_indicators
            .iter()
            .any(|&indicator| education_lower.contains(indicator));

        !has_extensive_experience && (student_indicator_count > 0 || education.len() > 50)
    }

    fn contains_gpa(&self, education: &str) -> bool {
        education.to_lowercase().contains("gpa")
    }

    fn contains_relevant_coursework(&self, education: &str) -> bool {
        education.to_lowercase().contains("coursework")
            || education.to_lowercase().contains("courses")
    }

    fn extract_projects_section(&self, content: &str) -> Option<String> {
        // Common project section headers
        let project_patterns = vec![
            r"(?i)\b(projects|portfolio|work samples|personal projects)\b.*?:",
            r"(?i)^\s*(projects|portfolio|personal projects|side projects)",
            r"(?i)^\s*#{1,3}\s*(projects|portfolio|work)",
        ];

        for pattern in project_patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                if let Some(mat) = regex.find(content) {
                    let start = mat.start();
                    let remaining_content = &content[start..];

                    // Find the end of the section
                    let section_end_patterns = vec![
                        r"(?i)\n\s*(experience|education|skills|certifications)\s*[:)
]",
                        r"(?i)\n\s*#{1,3}\s*(experience|education|skills)",
                    ];

                    let mut section_end = remaining_content.len();
                    for end_pattern in section_end_patterns {
                        if let Ok(end_regex) = regex::Regex::new(end_pattern) {
                            if let Some(end_match) = end_regex.find(remaining_content) {
                                section_end = section_end.min(end_match.start());
                            }
                        }
                    }

                    let projects_content = &remaining_content[..section_end];
                    if projects_content.trim().len() > 10 {
                        return Some(projects_content.trim().to_string());
                    }
                }
            }
        }

        // Fallback: look for project-related keywords
        let project_keywords = vec![
            "github",
            "repository",
            "demo",
            "built",
            "developed",
            "created",
            "implemented",
        ];
        let content_lower = content.to_lowercase();

        for keyword in project_keywords {
            if content_lower.contains(keyword) {
                if let Some(pos) = content_lower.find(keyword) {
                    let start = pos.saturating_sub(150);
                    let end = (pos + 300).min(content.len());
                    let potential_content = content[start..end].trim();

                    // Check if this looks like a project description
                    if potential_content.len() > 50
                        && (potential_content.to_lowercase().contains("project")
                            || potential_content.to_lowercase().contains("application")
                            || potential_content.to_lowercase().contains("system"))
                    {
                        return Some(potential_content.to_string());
                    }
                }
            }
        }

        None
    }

    fn contains_project_links(&self, projects: &str) -> bool {
        projects.contains("github") || projects.contains("http") || projects.contains("demo")
    }

    async fn identify_missing_project_technologies(
        &self,
        projects: &str,
        job_description: &str,
    ) -> Result<Vec<String>> {
        use crate::ollama::OllamaClient;

        let tech_analysis_prompt = format!(
            "Analyze the technologies mentioned in these project descriptions against the job requirements.

Project Descriptions:
{}

Job Requirements:
{}

Identify technologies, frameworks, tools, or platforms mentioned in the job description that are missing from the project descriptions.

Focus on:
- Programming languages and frameworks
- Database technologies
- Cloud platforms and services
- Development tools and methodologies
- APIs and integrations
- DevOps and deployment tools

Return missing technologies as a JSON array of strings.
Example: [\"Docker\", \"Kubernetes\", \"Redis\", \"CI/CD\"]",
            projects, job_description
        );

        let ollama_client = OllamaClient::new(None)?;
        let response = ollama_client
            .generate_ml_analysis("mistral:latest", &tech_analysis_prompt, "tech_gap_analysis")
            .await?;

        match serde_json::from_str::<Vec<String>>(&response) {
            Ok(missing_techs) => {
                info!(
                    "ML identified {} missing project technologies",
                    missing_techs.len()
                );
                Ok(missing_techs.into_iter().take(6).collect()) // Limit to top 6
            }
            Err(e) => {
                log::warn!("ML tech analysis failed: {}, using fallback", e);
                self.fallback_missing_project_technologies(projects, job_description)
            }
        }
    }

    fn fallback_missing_project_technologies(
        &self,
        projects: &str,
        job_description: &str,
    ) -> Result<Vec<String>> {
        let projects_lower = projects.to_lowercase();
        let job_lower = job_description.to_lowercase();

        // Common project technologies to check
        let common_tech = vec![
            "docker",
            "kubernetes",
            "jenkins",
            "terraform",
            "ansible",
            "mongodb",
            "postgresql",
            "redis",
            "elasticsearch",
            "aws",
            "azure",
            "gcp",
            "firebase",
            "heroku",
            "react",
            "angular",
            "vue",
            "nextjs",
            "gatsby",
            "nodejs",
            "express",
            "django",
            "flask",
            "spring",
            "graphql",
            "rest api",
            "microservices",
            "websockets",
            "ci/cd",
            "github actions",
            "gitlab ci",
            "testing",
            "jest",
        ];

        let mut missing_technologies = Vec::new();

        for tech in common_tech {
            if job_lower.contains(tech) && !projects_lower.contains(tech) {
                // Capitalize first letter
                let formatted_tech = tech
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if i == 0 {
                            c.to_uppercase().collect::<String>()
                        } else {
                            c.to_string()
                        }
                    })
                    .collect::<String>();
                missing_technologies.push(formatted_tech);
            }
        }

        info!(
            "Fallback analysis found {} missing project technologies",
            missing_technologies.len()
        );
        Ok(missing_technologies.into_iter().take(5).collect())
    }

    fn has_formatting_issues(&self, content: &str) -> bool {
        let mut issue_count = 0;

        // Check for inconsistent spacing
        if content.contains("  ") || content.contains("\t") {
            issue_count += 1;
        }

        // Check for ALL CAPS sections (usually bad formatting)
        let lines: Vec<&str> = content.lines().collect();
        let all_caps_lines = lines
            .iter()
            .filter(|line| {
                line.len() > 10
                    && line.chars().filter(|c| c.is_alphabetic()).count() > 5
                    && line.chars().filter(|c| c.is_uppercase()).count() as f64
                        / line.chars().filter(|c| c.is_alphabetic()).count() as f64
                        > 0.8
            })
            .count();

        if all_caps_lines > 0 {
            issue_count += 1;
        }

        // Check for poor bullet point consistency
        let bullet_types = ["•", "*", "-", "◦", "‣"];
        let bullet_count: usize = bullet_types
            .iter()
            .map(|bullet| content.matches(bullet).count())
            .sum();

        let different_bullet_types = bullet_types
            .iter()
            .filter(|bullet| content.contains(*bullet))
            .count();

        if bullet_count > 5 && different_bullet_types > 2 {
            issue_count += 1;
        }

        // Check for excessive line breaks
        let consecutive_breaks = content.matches("\n\n\n").count();
        if consecutive_breaks > 2 {
            issue_count += 1;
        }

        // Check for lack of structure (very long paragraphs)
        let long_paragraphs = lines
            .iter()
            .filter(|line| line.len() > 200 && !line.contains('.') && !line.contains(';'))
            .count();

        if long_paragraphs > 0 {
            issue_count += 1;
        }

        // Check for mixed date formats
        let date_formats = [
            regex::Regex::new(r"\d{1,2}/\d{1,2}/\d{4}").unwrap(),
            regex::Regex::new(r"\d{4}-\d{1,2}-\d{1,2}").unwrap(),
            regex::Regex::new(r"[A-Za-z]+ \d{4}").unwrap(),
        ];

        let format_count = date_formats
            .iter()
            .filter(|regex| regex.is_match(content))
            .count();

        if format_count > 1 {
            issue_count += 1;
        }

        // Consider it having formatting issues if 2+ issues found
        issue_count >= 2
    }

    fn extract_section_content(&self, content: &str, section: &str) -> Option<String> {
        let section_lower = section.to_lowercase();

        // Create patterns for the requested section
        let section_patterns = vec![
            format!(r"(?i)\b{}\b.*?:", regex::escape(&section_lower)),
            format!(r"(?i)^\s*{}", regex::escape(&section_lower)),
            format!(r"(?i)^\s*#{{1,3}}\s*{}", regex::escape(&section_lower)),
            format!(r"(?i)\b{}\s*section\b", regex::escape(&section_lower)),
        ];

        for pattern in section_patterns {
            if let Ok(regex) = regex::Regex::new(&pattern) {
                if let Some(mat) = regex.find(content) {
                    let start = mat.start();
                    let remaining_content = &content[start..];

                    // Find the end of the section (next major section)
                    let common_sections = vec![
                        "experience",
                        "education",
                        "skills",
                        "projects",
                        "certifications",
                        "summary",
                        "contact",
                    ];
                    let mut section_end = remaining_content.len();

                    for other_section in common_sections {
                        if other_section != section_lower {
                            let end_patterns = vec![
                                format!(r"(?i)\n\s*{}\s*[:)\n]", regex::escape(other_section)),
                                format!(r"(?i)\n\s*#{{1,3}}\s*{}", regex::escape(other_section)),
                            ];

                            for end_pattern in end_patterns {
                                if let Ok(end_regex) = regex::Regex::new(&end_pattern) {
                                    if let Some(end_match) = end_regex.find(remaining_content) {
                                        section_end = section_end.min(end_match.start());
                                    }
                                }
                            }
                        }
                    }

                    let section_content = &remaining_content[..section_end];
                    if section_content.trim().len() > 5 {
                        return Some(section_content.trim().to_string());
                    }
                }
            }
        }

        // Fallback: simple keyword search
        let content_lower = content.to_lowercase();
        if let Some(pos) = content_lower.find(&section_lower) {
            let start = pos.saturating_sub(50);
            let end = (pos + 200).min(content.len());
            let potential_content = content[start..end].trim();

            if potential_content.len() > 20 {
                return Some(potential_content.to_string());
            }
        }

        None
    }

    fn generate_live_typing_suggestions(
        &self,
        content: &str,
        position: usize,
    ) -> Vec<LiveSuggestion> {
        let mut suggestions = Vec::new();

        if position >= content.len() {
            return suggestions;
        }

        let current_line_start = content[..position]
            .rfind('\n')
            .map(|pos| pos + 1)
            .unwrap_or(0);
        let current_line_end = content[position..]
            .find('\n')
            .map(|pos| position + pos)
            .unwrap_or(content.len());
        let current_line = &content[current_line_start..current_line_end];
        let cursor_in_line = position - current_line_start;

        // Analyze context around cursor
        let words_before: Vec<&str> = content[..position].split_whitespace().collect();
        let current_word_start = content[..position]
            .rfind(' ')
            .map(|pos| pos + 1)
            .unwrap_or(0);
        let current_partial = &content[current_word_start..position];

        // Suggest action verbs if starting a bullet point or sentence
        if (current_line.trim_start().starts_with("•")
            || current_line.trim_start().starts_with("- "))
            && (cursor_in_line <= 10 || current_partial.is_empty())
        {
            let action_verbs = vec![
                "Developed",
                "Implemented",
                "Created",
                "Built",
                "Designed",
                "Managed",
                "Led",
                "Coordinated",
                "Executed",
                "Delivered",
                "Optimized",
                "Improved",
                "Increased",
                "Reduced",
                "Streamlined",
                "Automated",
                "Integrated",
            ];

            for verb in action_verbs.iter().take(5) {
                if verb
                    .to_lowercase()
                    .starts_with(&current_partial.to_lowercase())
                {
                    suggestions.push(LiveSuggestion {
                        position: current_word_start,
                        suggestion_type: "action_verb".to_string(),
                        text: verb.to_string(),
                        confidence: 0.8,
                        auto_apply: false,
                    });
                }
            }
        }

        // Suggest quantifiers when appropriate
        if words_before.len() > 2 {
            let last_words = &words_before[words_before.len().saturating_sub(3)..];
            let context = last_words.join(" ").to_lowercase();

            if (context.contains("increased")
                || context.contains("improved")
                || context.contains("reduced"))
                && !context.contains('%')
                && !context.contains('$')
            {
                suggestions.push(LiveSuggestion {
                    position,
                    suggestion_type: "quantifier".to_string(),
                    text: "by X%".to_string(),
                    confidence: 0.7,
                    auto_apply: false,
                });
            }
        }

        // Suggest technical terms based on context
        if current_partial.len() >= 2 {
            let tech_terms = vec![
                "API",
                "REST",
                "GraphQL",
                "microservices",
                "Docker",
                "Kubernetes",
                "AWS",
                "Azure",
                "GCP",
                "CI/CD",
                "Jenkins",
                "Git",
                "React",
                "Node.js",
                "Python",
                "JavaScript",
                "TypeScript",
                "PostgreSQL",
                "MongoDB",
                "Redis",
            ];

            for term in tech_terms {
                if term
                    .to_lowercase()
                    .starts_with(&current_partial.to_lowercase())
                {
                    suggestions.push(LiveSuggestion {
                        position: current_word_start,
                        suggestion_type: "technology".to_string(),
                        text: term.to_string(),
                        confidence: 0.9,
                        auto_apply: false,
                    });
                }
            }
        }

        // Suggest improvements for weak language
        let weak_phrases = vec![
            (
                "responsible for",
                "Led",
                "Use active language to show ownership",
            ),
            (
                "worked on",
                "Developed",
                "Be specific about your contributions",
            ),
            (
                "helped",
                "Collaborated to",
                "Show your active role in the team",
            ),
            (
                "participated in",
                "Contributed to",
                "Highlight your specific involvement",
            ),
        ];

        for (weak, strong, _explanation) in weak_phrases {
            if current_line.to_lowercase().contains(weak) {
                suggestions.push(LiveSuggestion {
                    position,
                    suggestion_type: "improvement".to_string(),
                    text: format!("Replace '{}' with '{}'", weak, strong),
                    confidence: 0.6,
                    auto_apply: false,
                });
            }
        }

        // Limit to top 3 most relevant suggestions
        suggestions.into_iter().take(3).collect()
    }

    fn analyze_tone(&self, content: &str) -> ToneAnalysis {
        let content_lower = content.to_lowercase();
        let words: Vec<&str> = content.split_whitespace().collect();
        let total_words = words.len() as f64;

        if total_words == 0.0 {
            return ToneAnalysis {
                professionalism_score: 0.0,
                confidence_level: 0.0,
                action_orientation: 0.0,
                specificity_score: 0.0,
                overall_tone: "insufficient_content".to_string(),
                tone_suggestions: vec!["Add more content to analyze tone".to_string()],
            };
        }

        // Calculate professionalism score
        let professional_words = vec![
            "accomplished",
            "achieved",
            "developed",
            "implemented",
            "managed",
            "coordinated",
            "supervised",
            "collaborated",
            "executed",
            "delivered",
            "optimized",
            "enhanced",
            "streamlined",
            "strategic",
            "analytical",
        ];

        let unprofessional_indicators = [
            "awesome",
            "cool",
            "stuff",
            "things",
            "pretty good",
            "kinda",
            "sorta",
        ];

        let professional_count = professional_words
            .iter()
            .filter(|&word| content_lower.contains(word))
            .count() as f64;

        let unprofessional_count = unprofessional_indicators
            .iter()
            .filter(|&word| content_lower.contains(word))
            .count() as f64;

        let professionalism_score = ((professional_count / total_words * 100.0)
            - (unprofessional_count / total_words * 50.0))
            .clamp(0.0, 100.0)
            .max(50.0);

        // Calculate confidence level
        let confident_words = [
            "led",
            "spearheaded",
            "pioneered",
            "established",
            "created",
            "built",
            "designed",
            "architected",
            "founded",
            "initiated",
        ];

        let weak_words = [
            "helped",
            "assisted",
            "participated",
            "contributed",
            "involved in",
            "responsible for",
            "worked on",
            "tried",
            "attempted",
        ];

        let confident_count = confident_words
            .iter()
            .filter(|&word| content_lower.contains(word))
            .count() as f64;

        let weak_count = weak_words
            .iter()
            .filter(|&word| content_lower.contains(word))
            .count() as f64;

        let confidence_level = ((confident_count / total_words * 100.0)
            - (weak_count / total_words * 30.0))
            .clamp(0.0, 100.0)
            .max(40.0);

        // Calculate action orientation
        let action_verbs = vec![
            "developed",
            "implemented",
            "created",
            "built",
            "designed",
            "managed",
            "led",
            "coordinated",
            "executed",
            "delivered",
            "optimized",
            "improved",
            "increased",
            "reduced",
            "streamlined",
            "automated",
            "integrated",
        ];

        let action_count = action_verbs
            .iter()
            .filter(|&verb| content_lower.contains(verb))
            .count() as f64;

        let action_orientation = (action_count / total_words * 200.0)
            .clamp(0.0, 100.0)
            .max(30.0);

        // Calculate specificity score
        let specific_indicators = vec![
            "%",
            "$",
            "million",
            "thousand",
            "users",
            "customers",
            "team",
            "time",
            "months",
            "weeks",
            "projects",
            "systems",
            "applications",
        ];

        let number_pattern = regex::Regex::new(r"\d+").unwrap();
        let number_count = number_pattern.find_iter(content).count() as f64;

        let specific_count = specific_indicators
            .iter()
            .filter(|&indicator| content_lower.contains(indicator))
            .count() as f64;

        let specificity_score = ((specific_count + number_count) / total_words * 150.0)
            .clamp(0.0, 100.0)
            .max(25.0);

        // Determine overall tone
        let avg_score =
            (professionalism_score + confidence_level + action_orientation + specificity_score)
                / 4.0;
        let overall_tone = match avg_score {
            s if s >= 80.0 => "excellent",
            s if s >= 70.0 => "professional",
            s if s >= 60.0 => "good",
            s if s >= 50.0 => "adequate",
            _ => "needs_improvement",
        }
        .to_string();

        // Generate suggestions
        let mut suggestions = Vec::new();

        if action_orientation < 60.0 {
            suggestions.push("Use more action verbs to show your impact".to_string());
        }
        if confidence_level < 60.0 {
            suggestions.push("Replace passive language with confident statements".to_string());
        }
        if specificity_score < 50.0 {
            suggestions.push("Add specific metrics and quantifiable achievements".to_string());
        }
        if professionalism_score < 70.0 {
            suggestions.push("Use more professional terminology".to_string());
        }

        if suggestions.is_empty() {
            suggestions
                .push("Your tone is strong - maintain this professional approach".to_string());
        }

        ToneAnalysis {
            professionalism_score,
            confidence_level,
            action_orientation,
            specificity_score,
            overall_tone,
            tone_suggestions: suggestions,
        }
    }

    fn calculate_clarity_score(&self, content: &str) -> f64 {
        let mut clarity_score = 100.0;
        let sentences: Vec<&str> = content.split(". ").collect();
        let words: Vec<&str> = content.split_whitespace().collect();
        let total_words = words.len() as f64;

        if total_words == 0.0 {
            return 0.0;
        }

        // Check average sentence length (ideal: 15-20 words)
        let avg_sentence_length = total_words / sentences.len() as f64;
        if avg_sentence_length > 25.0 {
            clarity_score -= 15.0; // Too long sentences
        } else if avg_sentence_length < 8.0 {
            clarity_score -= 10.0; // Too short sentences
        }

        // Check for complex words (more than 3 syllables approximation)
        let complex_words = words
            .iter()
            .filter(|word| word.len() > 12 || word.matches(['a', 'e', 'i', 'o', 'u']).count() > 4)
            .count() as f64;
        let complex_ratio = complex_words / total_words;
        if complex_ratio > 0.15 {
            clarity_score -= 20.0;
        }

        // Check for jargon and buzzwords
        let jargon_words = [
            "synergy",
            "leverage",
            "paradigm",
            "optimize",
            "streamline",
            "robust",
            "scalable",
            "dynamic",
            "innovative",
            "cutting-edge",
            "state-of-the-art",
        ];
        let jargon_count = jargon_words
            .iter()
            .filter(|&word| content.to_lowercase().contains(word))
            .count() as f64;
        if jargon_count > 3.0 {
            clarity_score -= 15.0;
        }

        // Check for passive voice indicators
        let passive_indicators = ["was", "were", "been", "being", "is", "are"];
        let passive_count = passive_indicators
            .iter()
            .filter(|&indicator| content.to_lowercase().contains(indicator))
            .count() as f64;
        let passive_ratio = passive_count / total_words;
        if passive_ratio > 0.1 {
            clarity_score -= 10.0;
        }

        // Check for clear structure (bullet points, numbers)
        if content.contains("•") || content.contains("- ") || content.contains("1.") {
            clarity_score += 5.0;
        }

        // Check for specific metrics and numbers
        let number_pattern = regex::Regex::new(r"\d+").unwrap();
        let number_count = number_pattern.find_iter(content).count() as f64;
        if number_count > 0.0 {
            clarity_score += (number_count / total_words * 50.0).min(10.0);
        }

        // Check for action verbs
        let action_verbs = [
            "achieved",
            "developed",
            "implemented",
            "created",
            "built",
            "designed",
            "managed",
            "led",
            "improved",
            "increased",
            "reduced",
            "optimized",
        ];
        let action_count = action_verbs
            .iter()
            .filter(|&verb| content.to_lowercase().contains(verb))
            .count() as f64;
        if action_count > 0.0 {
            clarity_score += (action_count / total_words * 100.0).min(10.0);
        }

        clarity_score.clamp(0.0, 100.0)
    }

    fn quick_keyword_match(&self, content: &str, job_description: &str) -> f64 {
        // Simple keyword matching for real-time feedback
        let content_words: std::collections::HashSet<String> = content
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let job_words: std::collections::HashSet<String> = job_description
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let matches = content_words.intersection(&job_words).count();
        let total_job_words = job_words.len();

        if total_job_words > 0 {
            (matches as f64 / total_job_words as f64) * 100.0
        } else {
            0.0
        }
    }

    fn quick_format_score(&self, content: &str) -> f64 {
        let mut score: f64 = 100.0;

        // Quick format checks
        if content.contains("<table") {
            score -= 25.0;
        }
        if content.contains("•") || content.contains("-") {
            score += 10.0;
        }
        if content.len() > 1000 {
            score += 5.0;
        }

        score.max(0.0)
    }

    fn calculate_section_strength(&self, content: &str) -> f64 {
        let mut strength_score = 0.0;
        let content_lower = content.to_lowercase();
        let words: Vec<&str> = content.split_whitespace().collect();
        let total_words = words.len() as f64;

        if total_words == 0.0 {
            return 0.0;
        }

        // Base score from content length (ideal range: 50-200 words per section)
        let length_score = match total_words {
            w if w < 20.0 => 20.0,
            w if w < 50.0 => 40.0 + (w - 20.0) * 1.5,
            w if (50.0..=150.0).contains(&w) => 85.0,
            w if w > 150.0 && w <= 200.0 => 85.0 - (w - 150.0) * 0.3,
            _ => 70.0,
        };
        strength_score += length_score * 0.3;

        // Check for strong action verbs
        let strong_verbs = vec![
            "led",
            "managed",
            "developed",
            "implemented",
            "created",
            "built",
            "designed",
            "architected",
            "optimized",
            "improved",
            "increased",
            "reduced",
            "achieved",
            "delivered",
            "executed",
            "spearheaded",
        ];

        let verb_count = strong_verbs
            .iter()
            .filter(|&verb| content_lower.contains(verb))
            .count() as f64;
        strength_score += (verb_count / total_words * 200.0).min(25.0);

        // Check for quantifiable achievements
        let achievement_indicators = vec![
            "%",
            "$",
            "million",
            "thousand",
            "increase",
            "decrease",
            "improve",
            "save",
            "generate",
            "revenue",
            "cost",
            "efficiency",
            "time",
            "users",
        ];

        let achievement_count = achievement_indicators
            .iter()
            .filter(|&indicator| content_lower.contains(indicator))
            .count() as f64;
        strength_score += (achievement_count / total_words * 150.0).min(20.0);

        // Check for technical depth
        let technical_keywords = [
            "system",
            "application",
            "database",
            "api",
            "framework",
            "architecture",
            "integration",
            "deployment",
            "testing",
            "performance",
            "security",
            "scalability",
        ];

        let technical_count = technical_keywords
            .iter()
            .filter(|&keyword| content_lower.contains(keyword))
            .count() as f64;
        strength_score += (technical_count / total_words * 100.0).min(15.0);

        // Check for specific technologies and tools
        let technologies = vec![
            "python",
            "java",
            "javascript",
            "react",
            "node",
            "sql",
            "aws",
            "docker",
            "kubernetes",
            "git",
            "api",
            "rest",
            "graphql",
            "microservices",
        ];

        let tech_count = technologies
            .iter()
            .filter(|&tech| content_lower.contains(tech))
            .count() as f64;
        strength_score += (tech_count / total_words * 80.0).min(10.0);

        // Bonus for clear structure
        if content.contains("•") || content.contains("- ") {
            strength_score += 5.0;
        }

        // Bonus for numbers and metrics
        let number_pattern = regex::Regex::new(r"\d+").unwrap();
        let number_count = number_pattern.find_iter(content).count() as f64;
        if number_count > 0.0 {
            strength_score += (number_count / total_words * 50.0).min(5.0);
        }

        strength_score.clamp(0.0, 100.0)
    }

    fn calculate_completion_percentage(&self, content: &str, section: &Section) -> f64 {
        let content_lower = content.to_lowercase();
        let words: Vec<&str> = content.split_whitespace().collect();
        let total_words = words.len() as f64;

        if total_words == 0.0 {
            return 0.0;
        }

        let completion_score: f64 = match section {
            Section::Experience => {
                // Check for essential experience elements
                let mut elements_present = 0;

                // Job titles or roles
                let role_indicators = [
                    "engineer",
                    "developer",
                    "analyst",
                    "manager",
                    "lead",
                    "specialist",
                ];
                if role_indicators
                    .iter()
                    .any(|&role| content_lower.contains(role))
                {
                    elements_present += 1;
                }

                // Company names or organization context
                if content_lower.contains("company")
                    || content_lower.contains("organization")
                    || content.chars().filter(|c| c.is_uppercase()).count() > 5
                {
                    elements_present += 1;
                }

                // Time periods
                let time_indicators = [
                    "2019", "2020", "2021", "2022", "2023", "2024", "2025", "months", "years",
                ];
                if time_indicators
                    .iter()
                    .any(|&time| content_lower.contains(time))
                {
                    elements_present += 1;
                }

                // Achievements and responsibilities
                let achievement_verbs = [
                    "developed",
                    "implemented",
                    "managed",
                    "led",
                    "created",
                    "improved",
                ];
                if achievement_verbs
                    .iter()
                    .any(|&verb| content_lower.contains(verb))
                {
                    elements_present += 1;
                }

                // Technologies used
                let tech_terms = ["python", "java", "javascript", "sql", "aws", "react", "api"];
                if tech_terms.iter().any(|&tech| content_lower.contains(tech)) {
                    elements_present += 1;
                }

                (elements_present as f64 / 5.0) * 100.0
            }

            Section::Education => {
                let mut elements_present = 0;

                // Degree type
                let degrees = [
                    "bachelor",
                    "master",
                    "phd",
                    "doctorate",
                    "degree",
                    "diploma",
                ];
                if degrees.iter().any(|&degree| content_lower.contains(degree)) {
                    elements_present += 1;
                }

                // Institution
                if content_lower.contains("university")
                    || content_lower.contains("college")
                    || content_lower.contains("institute")
                {
                    elements_present += 1;
                }

                // Field of study
                let fields = [
                    "computer science",
                    "engineering",
                    "business",
                    "science",
                    "arts",
                    "technology",
                ];
                if fields.iter().any(|&field| content_lower.contains(field)) {
                    elements_present += 1;
                }

                // Graduation year or time period
                let years = ["2019", "2020", "2021", "2022", "2023", "2024", "2025"];
                if years.iter().any(|&year| content_lower.contains(year)) {
                    elements_present += 1;
                }

                (elements_present as f64 / 4.0) * 100.0
            }

            Section::Skills => {
                let mut elements_present = 0;

                // Programming languages
                let languages = [
                    "python",
                    "java",
                    "javascript",
                    "typescript",
                    "go",
                    "rust",
                    "c++",
                ];
                if languages.iter().any(|&lang| content_lower.contains(lang)) {
                    elements_present += 1;
                }

                // Frameworks/tools
                let frameworks = ["react", "angular", "vue", "django", "express", "spring"];
                if frameworks
                    .iter()
                    .any(|&framework| content_lower.contains(framework))
                {
                    elements_present += 1;
                }

                // Cloud/DevOps
                let cloud_tools = ["aws", "azure", "docker", "kubernetes", "jenkins"];
                if cloud_tools.iter().any(|&tool| content_lower.contains(tool)) {
                    elements_present += 1;
                }

                // Databases
                let databases = ["sql", "mysql", "postgresql", "mongodb", "redis"];
                if databases.iter().any(|&db| content_lower.contains(db)) {
                    elements_present += 1;
                }

                // Organization (categories or structure)
                if content.contains("•") || content.contains(":") || content.contains("-") {
                    elements_present += 1;
                }

                (elements_present as f64 / 5.0) * 100.0
            }

            Section::Projects => {
                let mut elements_present = 0;

                // Project names or descriptions
                if content_lower.contains("project")
                    || content_lower.contains("application")
                    || content_lower.contains("system")
                {
                    elements_present += 1;
                }

                // Technologies used
                let tech_terms = ["python", "react", "node", "sql", "api", "database"];
                if tech_terms.iter().any(|&tech| content_lower.contains(tech)) {
                    elements_present += 1;
                }

                // Project outcomes or features
                let outcome_words = ["built", "developed", "created", "implemented", "features"];
                if outcome_words
                    .iter()
                    .any(|&outcome| content_lower.contains(outcome))
                {
                    elements_present += 1;
                }

                // Links or repositories
                if content_lower.contains("github")
                    || content_lower.contains("demo")
                    || content_lower.contains("http")
                {
                    elements_present += 1;
                }

                (elements_present as f64 / 4.0) * 100.0
            }

            _ => {
                // Generic completion for other sections
                match total_words {
                    w if w < 10.0 => 30.0,
                    w if w < 25.0 => 60.0,
                    w if w >= 25.0 => 90.0,
                    _ => 80.0,
                }
            }
        };

        // Adjust based on content length
        let length_factor = match total_words {
            w if w < 10.0 => 0.5,
            w if w < 25.0 => 0.8,
            w if w >= 25.0 => 1.0,
            _ => 1.0,
        };

        (completion_score * length_factor).clamp(0.0, 100.0)
    }

    fn determine_next_action(
        &self,
        suggestions: &[ContextualSuggestion],
        improvements: &[PriorityImprovement],
    ) -> String {
        if let Some(urgent_suggestion) = suggestions.iter().find(|s| s.urgency == "immediate") {
            format!("Immediate: {}", urgent_suggestion.title)
        } else if let Some(high_priority) = improvements.iter().find(|i| i.impact_score > 20.0) {
            format!("Priority: {}", high_priority.title)
        } else if let Some(suggestion) = suggestions.first() {
            format!("Next: {}", suggestion.title)
        } else {
            "Continue writing - content looks good!".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;

    #[tokio::test]
    async fn test_realtime_optimizer_creation() {
        let db = Database::new().await.unwrap();
        let _optimizer = RealtimeOptimizer::new(db);
        assert!(true); // Basic creation test
    }

    #[tokio::test]
    async fn test_live_suggestions_generation() {
        let db = Database::new().await.unwrap();
        let mut optimizer = RealtimeOptimizer::new(db);

        let content =
            "Experience\n• Worked on various projects\n• Helped improve system performance";
        let job_description = "Looking for software engineer with Python experience";

        let suggestions = optimizer
            .get_live_suggestions(content, job_description, 50)
            .await;
        assert!(suggestions.is_ok());

        let live_suggestions = suggestions.unwrap();
        assert!(live_suggestions.real_time_score >= 0.0);
        assert!(live_suggestions.real_time_score <= 100.0);
    }

    #[test]
    fn test_section_identification() {
        let db = futures::executor::block_on(Database::new()).unwrap();
        let optimizer = RealtimeOptimizer::new(db);

        let content = "Summary\nExperienced developer\n\nExperience\n• Led development team";

        let section = optimizer.identify_current_section(content, 10); // In summary section
        assert!(matches!(section, Section::Summary));

        let section = optimizer.identify_current_section(content, 50); // In experience section
        assert!(matches!(section, Section::Experience));
    }

    #[test]
    fn test_current_bullet_extraction() {
        let db = futures::executor::block_on(Database::new()).unwrap();
        let optimizer = RealtimeOptimizer::new(db);

        let content = "Experience\n• Led development team\n• Improved system performance";
        let bullet = optimizer.extract_current_bullet(content, 25);

        assert!(bullet.is_some());
        assert!(bullet.unwrap().contains("Led development team"));
    }

    #[tokio::test]
    async fn test_real_time_scoring() {
        let db = Database::new().await.unwrap();
        let optimizer = RealtimeOptimizer::new(db);

        let content = "Experience\n• Led team of 5 developers, resulting in 25% faster delivery\n• Implemented automated testing, reducing bugs by 40%";
        let job_description = "Software engineer with leadership and testing experience";

        let score = optimizer
            .calculate_real_time_score(content, job_description)
            .await
            .unwrap();
        assert!(score > 0.0);
        assert!(score <= 100.0);
    }
}
