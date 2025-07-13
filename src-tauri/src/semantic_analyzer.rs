use anyhow::{Context, Result};
use log::info;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::database::Database;
use crate::models::IndustryKeyword;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticAnalysisResult {
    pub keyword_matches: Vec<KeywordMatch>,
    pub semantic_similarity_score: f64,
    pub industry_relevance_score: f64,
    pub skill_gaps: Vec<SkillGap>,
    pub recommended_skills: Vec<String>,
    pub confidence_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordMatch {
    pub keyword: String,
    pub found_in_resume: bool,
    pub semantic_variations: Vec<String>,
    pub relevance_score: f64,
    pub category: String,
    pub weight: f64,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillGap {
    pub missing_skill: String,
    pub importance: f64,
    pub category: String,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryKeywordSet {
    pub industry: String,
    pub keywords: Vec<IndustryKeyword>,
    pub skill_synonyms: HashMap<String, Vec<String>>,
    pub category_weights: HashMap<String, f64>,
}

pub struct SemanticAnalyzer {
    database: Database,
    skill_synonyms: HashMap<String, Vec<String>>,
    stop_words: Vec<String>,
    technical_patterns: Vec<Regex>,
}

impl SemanticAnalyzer {
    pub fn new(database: Database) -> Self {
        let skill_synonyms = Self::build_skill_synonyms_map();
        let stop_words = Self::build_stop_words_list();
        let technical_patterns = Self::build_technical_patterns();

        SemanticAnalyzer {
            database,
            skill_synonyms,
            stop_words,
            technical_patterns,
        }
    }

    pub async fn analyze_semantic_keywords(
        &self,
        resume_content: &str,
        job_description: &str,
        industry: &str,
    ) -> Result<SemanticAnalysisResult> {
        info!("Starting semantic analysis for industry: {}", industry);

        // 1. Check database health before proceeding
        match self.database.health_check().await {
            Ok(true) => info!("Database health check passed in semantic analyzer"),
            Ok(false) => {
                return Err(anyhow::anyhow!(
                    "Database health check failed in semantic analyzer"
                ));
            }
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Database health check error in semantic analyzer: {}",
                    e
                ));
            }
        }

        // 2. Load industry-specific keywords
        let industry_keywords = self
            .database
            .get_industry_keywords(industry)
            .await
            .context(format!("Failed to load industry keywords for industry '{}'. Please check if the database is accessible and the industry is supported.", industry))?;

        info!(
            "Loaded {} keywords for industry '{}'",
            industry_keywords.len(),
            industry
        );

        // 2. Extract keywords from both resume and job description
        let resume_keywords = self.extract_keywords(resume_content);
        let job_keywords = self.extract_keywords(job_description);

        // 3. Perform semantic matching
        let keyword_matches =
            self.perform_semantic_matching(&resume_keywords, &job_keywords, &industry_keywords);

        // 4. Calculate semantic similarity score
        let semantic_similarity_score =
            self.calculate_semantic_similarity(resume_content, job_description);

        // 5. Calculate industry relevance
        let industry_relevance_score =
            self.calculate_industry_relevance(&resume_keywords, &industry_keywords);

        // 6. Identify skill gaps
        let skill_gaps = self.identify_skill_gaps(&keyword_matches, &industry_keywords);

        // 7. Generate skill recommendations
        let recommended_skills =
            self.generate_skill_recommendations(&skill_gaps, &industry_keywords);

        // 8. Calculate overall confidence
        let confidence_score = self.calculate_confidence_score(
            &keyword_matches,
            semantic_similarity_score,
            industry_relevance_score,
        );

        Ok(SemanticAnalysisResult {
            keyword_matches,
            semantic_similarity_score,
            industry_relevance_score,
            skill_gaps,
            recommended_skills,
            confidence_score,
        })
    }

    fn extract_keywords(&self, text: &str) -> Vec<String> {
        let text = text.to_lowercase();
        let mut keywords = Vec::new();

        // Extract technical patterns first
        for pattern in &self.technical_patterns {
            for capture in pattern.captures_iter(&text) {
                if let Some(matched) = capture.get(0) {
                    let keyword = matched.as_str().trim().to_string();
                    if !keyword.is_empty() && keyword.len() > 2 {
                        keywords.push(keyword);
                    }
                }
            }
        }

        // Extract general keywords
        let words: Vec<&str> = text.split_whitespace().collect();
        for window in words
            .windows(1)
            .chain(words.windows(2))
            .chain(words.windows(3))
        {
            let phrase = window.join(" ");
            let cleaned = self.clean_keyword(&phrase);

            if self.is_valid_keyword(&cleaned) {
                keywords.push(cleaned);
            }
        }

        // Remove duplicates and sort by length (longer phrases first)
        keywords.sort_by_key(|b| std::cmp::Reverse(b.len()));
        keywords.dedup();

        keywords
    }

    fn perform_semantic_matching(
        &self,
        resume_keywords: &[String],
        _job_keywords: &[String],
        industry_keywords: &[IndustryKeyword],
    ) -> Vec<KeywordMatch> {
        let mut matches = Vec::new();

        for industry_kw in industry_keywords {
            let mut found_variations = Vec::new();
            let mut found_in_resume = false;
            let mut context = String::new();
            let mut relevance_score = 0.0;

            // Direct match
            if self.contains_keyword(resume_keywords, &industry_kw.keyword) {
                found_in_resume = true;
                found_variations.push(industry_kw.keyword.clone());
                relevance_score += 1.0;
                context = format!("Direct match: {}", industry_kw.keyword);
            }

            // Synonym matching
            if let Ok(synonyms) = serde_json::from_str::<Vec<String>>(&industry_kw.synonyms) {
                for synonym in synonyms {
                    if self.contains_keyword(resume_keywords, &synonym) {
                        found_in_resume = true;
                        found_variations.push(synonym.clone());
                        relevance_score += 0.8;
                        context = format!("Synonym match: {} -> {}", synonym, industry_kw.keyword);
                    }
                }
            }

            // Semantic similarity using built-in synonyms
            if let Some(semantic_synonyms) =
                self.skill_synonyms.get(&industry_kw.keyword.to_lowercase())
            {
                for synonym in semantic_synonyms {
                    if self.contains_keyword(resume_keywords, synonym) {
                        found_in_resume = true;
                        found_variations.push(synonym.clone());
                        relevance_score += 0.6;
                        context = format!("Semantic match: {} -> {}", synonym, industry_kw.keyword);
                    }
                }
            }

            // Partial matching for compound skills
            let partial_score =
                self.calculate_partial_match_score(resume_keywords, &industry_kw.keyword);
            if partial_score > 0.5 {
                relevance_score += partial_score * 0.4;
                if !found_in_resume && partial_score > 0.7 {
                    found_in_resume = true;
                    context = format!(
                        "Partial match: {} (score: {:.2})",
                        industry_kw.keyword, partial_score
                    );
                }
            }

            // Create match entry
            matches.push(KeywordMatch {
                keyword: industry_kw.keyword.clone(),
                found_in_resume,
                semantic_variations: found_variations,
                relevance_score: relevance_score.min(1.0),
                category: industry_kw.category.clone(),
                weight: industry_kw.weight,
                context,
            });
        }

        // Sort by relevance score and weight
        matches.sort_by(|a, b| {
            (b.relevance_score * b.weight)
                .partial_cmp(&(a.relevance_score * a.weight))
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        matches
    }

    fn calculate_semantic_similarity(&self, text1: &str, text2: &str) -> f64 {
        let keywords1 = self.extract_keywords(text1);
        let keywords2 = self.extract_keywords(text2);

        if keywords1.is_empty() || keywords2.is_empty() {
            return 0.0;
        }

        let mut common_keywords = 0;
        let mut total_weight = 0.0;

        for kw1 in &keywords1 {
            for kw2 in &keywords2 {
                let similarity = self.calculate_string_similarity(kw1, kw2);
                if similarity > 0.8 {
                    common_keywords += 1;
                    total_weight += similarity;
                }
            }
        }

        if common_keywords > 0 {
            total_weight / common_keywords as f64
        } else {
            0.0
        }
    }

    fn calculate_industry_relevance(
        &self,
        resume_keywords: &[String],
        industry_keywords: &[IndustryKeyword],
    ) -> f64 {
        if industry_keywords.is_empty() {
            return 0.0;
        }

        let mut total_weight = 0.0;
        let mut matched_weight = 0.0;

        for industry_kw in industry_keywords {
            total_weight += industry_kw.weight;

            if self.contains_keyword(resume_keywords, &industry_kw.keyword) {
                matched_weight += industry_kw.weight;
            } else if let Ok(synonyms) = serde_json::from_str::<Vec<String>>(&industry_kw.synonyms)
            {
                for synonym in synonyms {
                    if self.contains_keyword(resume_keywords, &synonym) {
                        matched_weight += industry_kw.weight * 0.8; // Slightly lower weight for synonyms
                        break;
                    }
                }
            }
        }

        if total_weight > 0.0 {
            matched_weight / total_weight
        } else {
            0.0
        }
    }

    fn identify_skill_gaps(
        &self,
        keyword_matches: &[KeywordMatch],
        industry_keywords: &[IndustryKeyword],
    ) -> Vec<SkillGap> {
        let mut skill_gaps = Vec::new();

        for industry_kw in industry_keywords {
            if let Some(match_result) = keyword_matches
                .iter()
                .find(|m| m.keyword == industry_kw.keyword)
            {
                if !match_result.found_in_resume && industry_kw.weight > 1.0 {
                    let suggestions = self
                        .generate_skill_suggestions(&industry_kw.keyword, &industry_kw.category);

                    skill_gaps.push(SkillGap {
                        missing_skill: industry_kw.keyword.clone(),
                        importance: industry_kw.weight,
                        category: industry_kw.category.clone(),
                        suggestions,
                    });
                }
            }
        }

        // Sort by importance
        skill_gaps.sort_by(|a, b| {
            b.importance
                .partial_cmp(&a.importance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Return top 10 most important gaps
        skill_gaps.into_iter().take(10).collect()
    }

    fn generate_skill_recommendations(
        &self,
        skill_gaps: &[SkillGap],
        industry_keywords: &[IndustryKeyword],
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Recommend skills based on gaps
        for gap in skill_gaps.iter().take(5) {
            recommendations.push(gap.missing_skill.clone());
        }

        // Add trending skills in the same categories
        let gap_categories: Vec<&String> = skill_gaps.iter().map(|g| &g.category).collect();
        for industry_kw in industry_keywords {
            if gap_categories.contains(&&industry_kw.category)
                && industry_kw.weight > 1.5
                && !recommendations.contains(&industry_kw.keyword)
            {
                recommendations.push(industry_kw.keyword.clone());
            }
        }

        recommendations.into_iter().take(8).collect()
    }

    fn calculate_confidence_score(
        &self,
        keyword_matches: &[KeywordMatch],
        semantic_similarity: f64,
        industry_relevance: f64,
    ) -> f64 {
        let match_confidence = if keyword_matches.is_empty() {
            0.0
        } else {
            keyword_matches
                .iter()
                .map(|m| m.relevance_score * m.weight)
                .sum::<f64>()
                / keyword_matches.len() as f64
        };

        // Weighted average of different confidence factors
        (match_confidence * 0.4 + semantic_similarity * 0.3 + industry_relevance * 0.3).min(1.0)
    }

    // Helper methods
    fn contains_keyword(&self, keywords: &[String], target: &str) -> bool {
        let target_lower = target.to_lowercase();
        keywords.iter().any(|kw| {
            let kw_lower = kw.to_lowercase();
            kw_lower.contains(&target_lower) || target_lower.contains(&kw_lower)
        })
    }

    fn calculate_partial_match_score(&self, keywords: &[String], target: &str) -> f64 {
        let target_words: Vec<&str> = target.split_whitespace().collect();
        if target_words.len() <= 1 {
            return 0.0;
        }

        let mut matches = 0;
        for word in &target_words {
            if self.contains_keyword(keywords, word) {
                matches += 1;
            }
        }

        matches as f64 / target_words.len() as f64
    }

    fn calculate_string_similarity(&self, s1: &str, s2: &str) -> f64 {
        let s1_lower = s1.to_lowercase();
        let s2_lower = s2.to_lowercase();

        if s1_lower == s2_lower {
            return 1.0;
        }

        // Simple Levenshtein-based similarity
        let max_len = s1_lower.len().max(s2_lower.len());
        if max_len == 0 {
            return 1.0;
        }

        let distance = self.levenshtein_distance(&s1_lower, &s2_lower);
        1.0 - (distance as f64 / max_len as f64)
    }

    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let len1 = s1.len();
        let len2 = s2.len();
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        for (i, item) in matrix.iter_mut().enumerate().take(len1 + 1) {
            item[0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        for (i, char1) in s1.chars().enumerate() {
            for (j, char2) in s2.chars().enumerate() {
                let cost = if char1 == char2 { 0 } else { 1 };
                matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                    .min(matrix[i + 1][j] + 1)
                    .min(matrix[i][j] + cost);
            }
        }

        matrix[len1][len2]
    }

    fn clean_keyword(&self, keyword: &str) -> String {
        let cleaned = keyword
            .trim()
            .replace(
                &['.', ',', ';', ':', '!', '?', '(', ')', '[', ']', '{', '}'][..],
                "",
            )
            .replace("  ", " ");

        cleaned.trim().to_string()
    }

    fn is_valid_keyword(&self, keyword: &str) -> bool {
        if keyword.len() < 2 || keyword.len() > 50 {
            return false;
        }

        if self.stop_words.contains(&keyword.to_lowercase()) {
            return false;
        }

        // Check if it's mostly alphabetic or contains technical characters
        let alpha_count = keyword.chars().filter(|c| c.is_alphabetic()).count();
        let total_chars = keyword.chars().count();

        alpha_count as f64 / total_chars as f64 > 0.5
    }

    fn generate_skill_suggestions(&self, skill: &str, category: &str) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Add generic suggestions based on category
        match category {
            "programming_language" => {
                suggestions.extend(vec![
                    format!("Learn {} through online courses", skill),
                    format!("Build projects using {}", skill),
                    format!("Get certified in {}", skill),
                ]);
            }
            "framework" => {
                suggestions.extend(vec![
                    format!("Complete {} tutorials", skill),
                    format!("Build a portfolio project with {}", skill),
                ]);
            }
            "soft_skill" => {
                suggestions.extend(vec![
                    format!("Demonstrate {} in project descriptions", skill),
                    format!("Include {} examples in experience section", skill),
                ]);
            }
            _ => {
                suggestions.push(format!("Gain experience with {}", skill));
            }
        }

        suggestions
    }

    fn build_skill_synonyms_map() -> HashMap<String, Vec<String>> {
        let mut synonyms = HashMap::new();

        // Programming languages
        synonyms.insert(
            "javascript".to_string(),
            vec![
                "js".to_string(),
                "node.js".to_string(),
                "nodejs".to_string(),
            ],
        );
        synonyms.insert("typescript".to_string(), vec!["ts".to_string()]);
        synonyms.insert(
            "python".to_string(),
            vec!["py".to_string(), "python3".to_string()],
        );
        synonyms.insert(
            "c++".to_string(),
            vec!["cpp".to_string(), "c plus plus".to_string()],
        );
        synonyms.insert(
            "c#".to_string(),
            vec!["csharp".to_string(), "c sharp".to_string()],
        );

        // Frameworks
        synonyms.insert(
            "react".to_string(),
            vec!["reactjs".to_string(), "react.js".to_string()],
        );
        synonyms.insert(
            "angular".to_string(),
            vec!["angularjs".to_string(), "angular.js".to_string()],
        );
        synonyms.insert(
            "vue".to_string(),
            vec!["vuejs".to_string(), "vue.js".to_string()],
        );

        // Databases
        synonyms.insert(
            "postgresql".to_string(),
            vec!["postgres".to_string(), "psql".to_string()],
        );
        synonyms.insert("mysql".to_string(), vec!["my sql".to_string()]);
        synonyms.insert(
            "mongodb".to_string(),
            vec!["mongo".to_string(), "mongo db".to_string()],
        );

        // Cloud platforms
        synonyms.insert("amazon web services".to_string(), vec!["aws".to_string()]);
        synonyms.insert(
            "google cloud platform".to_string(),
            vec!["gcp".to_string(), "google cloud".to_string()],
        );
        synonyms.insert("microsoft azure".to_string(), vec!["azure".to_string()]);

        // DevOps
        synonyms.insert("kubernetes".to_string(), vec!["k8s".to_string()]);
        synonyms.insert("docker".to_string(), vec!["containerization".to_string()]);

        synonyms
    }

    fn build_stop_words_list() -> Vec<String> {
        vec![
            "the",
            "a",
            "an",
            "and",
            "or",
            "but",
            "in",
            "on",
            "at",
            "to",
            "for",
            "of",
            "with",
            "by",
            "from",
            "up",
            "about",
            "into",
            "through",
            "during",
            "before",
            "after",
            "above",
            "below",
            "up",
            "down",
            "out",
            "off",
            "over",
            "under",
            "again",
            "further",
            "then",
            "once",
            "here",
            "there",
            "when",
            "where",
            "why",
            "how",
            "all",
            "any",
            "both",
            "each",
            "few",
            "more",
            "most",
            "other",
            "some",
            "such",
            "no",
            "nor",
            "not",
            "only",
            "own",
            "same",
            "so",
            "than",
            "too",
            "very",
            "can",
            "will",
            "just",
            "should",
            "now",
            "also",
            "as",
            "well",
            "like",
            "want",
            "need",
            "use",
            "work",
            "make",
            "take",
            "get",
            "go",
            "come",
            "see",
            "know",
            "think",
            "say",
            "tell",
            "ask",
            "become",
            "leave",
            "put",
            "mean",
            "keep",
            "let",
            "begin",
            "seem",
            "help",
            "talk",
            "turn",
            "start",
            "might",
            "show",
            "hear",
            "play",
            "run",
            "move",
            "live",
            "believe",
            "hold",
            "bring",
            "happen",
            "write",
            "provide",
            "sit",
            "stand",
            "lose",
            "pay",
            "meet",
            "include",
            "continue",
            "set",
            "learn",
            "change",
            "lead",
            "understand",
            "watch",
            "follow",
            "stop",
            "create",
            "speak",
            "read",
            "allow",
            "add",
            "spend",
            "grow",
            "open",
            "walk",
            "win",
            "offer",
            "remember",
            "love",
            "consider",
            "appear",
            "buy",
            "wait",
            "serve",
            "die",
            "send",
            "expect",
            "build",
            "stay",
            "fall",
            "cut",
            "reach",
            "kill",
            "remain",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    fn build_technical_patterns() -> Vec<Regex> {
        vec![
            // Programming languages with versions
            Regex::new(r"(?i)\b(python|java|javascript|typescript|c\+\+|c#|php|ruby|go|rust|swift|kotlin)\s*\d*\.?\d*\b").unwrap(),
            // Frameworks with versions
            Regex::new(r"(?i)\b(react|angular|vue|django|flask|spring|express|laravel|rails)\s*\d*\.?\d*\b").unwrap(),
            // Databases
            Regex::new(r"(?i)\b(mysql|postgresql|mongodb|redis|elasticsearch|cassandra|oracle|sql\s*server)\b").unwrap(),
            // Cloud services
            Regex::new(r"(?i)\b(aws|azure|gcp|google\s*cloud|amazon\s*web\s*services|microsoft\s*azure)\b").unwrap(),
            // DevOps tools
            Regex::new(r"(?i)\b(docker|kubernetes|jenkins|gitlab|github|terraform|ansible|chef|puppet)\b").unwrap(),
            // Version numbers
            Regex::new(r"\b\d+\.\d+(?:\.\d+)?\b").unwrap(),
        ]
    }

    #[allow(dead_code)]
    pub fn detect_skill_synonyms(&self, skill: &str) -> Vec<String> {
        let skill_lower = skill.to_lowercase();

        if let Some(synonyms) = self.skill_synonyms.get(&skill_lower) {
            synonyms.clone()
        } else {
            // Try to find synonyms where the skill is a synonym
            for (key, synonyms) in &self.skill_synonyms {
                if synonyms.iter().any(|s| s.to_lowercase() == skill_lower) {
                    let mut result = vec![key.clone()];
                    result.extend(
                        synonyms
                            .iter()
                            .filter(|s| s.to_lowercase() != skill_lower)
                            .cloned(),
                    );
                    return result;
                }
            }
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;

    async fn setup_test_analyzer() -> SemanticAnalyzer {
        let db = Database::new().await.unwrap();
        SemanticAnalyzer::new(db)
    }

    #[tokio::test]
    async fn test_keyword_extraction() {
        let analyzer = setup_test_analyzer().await;
        let text = "I have experience with Python, JavaScript, and React.js development";
        let keywords = analyzer.extract_keywords(text);

        assert!(keywords.contains(&"python".to_string()));
        assert!(keywords.contains(&"javascript".to_string()));
        assert!(keywords.contains(&"react.js".to_string()));
    }

    #[tokio::test]
    async fn test_skill_synonyms() {
        let analyzer = setup_test_analyzer().await;
        let synonyms = analyzer.detect_skill_synonyms("javascript");

        assert!(synonyms.contains(&"js".to_string()));
        assert!(synonyms.contains(&"node.js".to_string()));
    }

    #[tokio::test]
    async fn test_semantic_similarity() {
        let analyzer = setup_test_analyzer().await;
        let text1 = "Python developer with Django experience";
        let text2 = "Looking for Python engineer with web framework knowledge";

        let similarity = analyzer.calculate_semantic_similarity(text1, text2);
        assert!(similarity > 0.5);
    }

    #[tokio::test]
    async fn test_string_similarity() {
        let analyzer = SemanticAnalyzer {
            database: Database::new().await.unwrap(),
            skill_synonyms: HashMap::new(),
            stop_words: Vec::new(),
            technical_patterns: Vec::new(),
        };

        assert_eq!(
            analyzer.calculate_string_similarity("python", "python"),
            1.0
        );
        assert!(analyzer.calculate_string_similarity("python", "python3") > 0.8);
        assert!(analyzer.calculate_string_similarity("react", "angular") < 0.5);
    }
}
