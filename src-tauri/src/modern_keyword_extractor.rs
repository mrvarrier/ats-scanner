use anyhow::Result;
use log::info;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::database::Database;
use crate::ollama::OllamaClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModernKeywordMatch {
    pub keyword: String,
    pub normalized_form: String,
    pub category: String,
    pub confidence_score: f64,
    pub context_relevance: f64,
    pub skill_level: Option<SkillLevel>,
    pub experience_years: Option<i32>,
    pub certifications: Vec<String>,
    pub context_phrases: Vec<String>,
    pub word_position: usize,
    pub semantic_variations: Vec<String>,
    pub industry_relevance: f64,
    pub match_type: MatchType,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchType {
    Exact,         // Direct match
    Semantic,      // Synonym or related term
    Contextual,    // Inferred from context
    Fuzzy,         // Typo or variation
    Compound,      // Multi-word technical phrase
    Certification, // Specific certification
    Framework,     // Technology framework
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionResult {
    pub keyword_matches: Vec<ModernKeywordMatch>,
    pub skill_clusters: Vec<SkillCluster>,
    pub missing_critical_skills: Vec<String>,
    pub emerging_skills: Vec<String>,
    pub confidence_score: f64,
    pub extraction_metadata: ExtractionMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillCluster {
    pub cluster_name: String,
    pub skills: Vec<String>,
    pub completeness_score: f64,
    pub cluster_type: ClusterType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClusterType {
    TechnicalStack,  // e.g., React + Node.js + MongoDB
    CloudPlatform,   // e.g., AWS services
    DataScience,     // e.g., Python + Pandas + Scikit-learn
    DevOps,          // e.g., Docker + Kubernetes + Jenkins
    MobileStack,     // e.g., React Native + Firebase
    WebDevelopment,  // e.g., HTML + CSS + JavaScript
    DatabaseCluster, // e.g., SQL databases
    SecurityStack,   // e.g., Cybersecurity tools
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionMetadata {
    pub total_words_processed: usize,
    pub technical_density: f64,
    pub avg_confidence_score: f64,
    pub processing_time_ms: u64,
    pub nlp_model_used: String,
    pub extraction_version: String,
}

pub struct ModernKeywordExtractor {
    #[allow(dead_code)]
    database: Database,
    ollama_client: OllamaClient,

    // Advanced NLP components
    stemmer: PorterStemmer,
    skill_patterns: HashMap<String, Vec<Regex>>,
    experience_patterns: Vec<Regex>,
    certification_patterns: HashMap<String, Regex>,
    negation_patterns: Vec<Regex>,

    // Modern skill databases
    skill_ontology: SkillOntology,
    trending_skills: TrendingSkillsCache,
    compound_skills: HashMap<String, Vec<String>>,

    // Context analysis
    #[allow(dead_code)]
    context_window: usize,
    confidence_threshold: f64,
}

// Simple Porter Stemmer implementation
pub struct PorterStemmer {
    #[allow(dead_code)]
    vowels: HashSet<char>,
}

impl Default for PorterStemmer {
    fn default() -> Self {
        Self::new()
    }
}

impl PorterStemmer {
    pub fn new() -> Self {
        let vowels = ['a', 'e', 'i', 'o', 'u'].iter().cloned().collect();
        Self { vowels }
    }

    pub fn stem(&self, word: &str) -> String {
        let word = word.to_lowercase();

        // Simple stemming rules - in production, use rust_stemmers crate
        if word.ends_with("ing") && word.len() > 4 {
            word[..word.len() - 3].to_string()
        } else if (word.ends_with("ed") || word.ends_with("er") || word.ends_with("ly"))
            && word.len() > 3
        {
            word[..word.len() - 2].to_string()
        } else if word.ends_with("tion") && word.len() > 5 {
            format!("{}e", &word[..word.len() - 4])
        } else if word.ends_with("ment") && word.len() > 5 {
            word[..word.len() - 4].to_string()
        } else {
            word
        }
    }
}

#[derive(Debug, Clone)]
pub struct SkillOntology {
    pub skill_relationships: HashMap<String, SkillNode>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SkillNode {
    pub skill_name: String,
    pub category: String,
    pub synonyms: Vec<String>,
    pub prerequisites: Vec<String>,
    pub related_skills: Vec<String>,
    pub industry_relevance: HashMap<String, f64>,
    pub difficulty_level: f64,
    pub market_demand: f64,
}

#[derive(Debug, Clone)]
pub struct TrendingSkillsCache {
    pub skills_by_industry: HashMap<String, Vec<TrendingSkill>>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct TrendingSkill {
    pub skill_name: String,
    pub trend_score: f64,
    pub growth_rate: f64,
    pub demand_level: f64,
}

impl ModernKeywordExtractor {
    pub async fn new(database: Database) -> Result<Self> {
        let ollama_client = OllamaClient::new(None)?;
        let stemmer = PorterStemmer::new();

        let mut extractor = Self {
            database,
            ollama_client,
            stemmer,
            skill_patterns: HashMap::new(),
            experience_patterns: Vec::new(),
            certification_patterns: HashMap::new(),
            negation_patterns: Vec::new(),
            skill_ontology: SkillOntology {
                skill_relationships: HashMap::new(),
            },
            trending_skills: TrendingSkillsCache {
                skills_by_industry: HashMap::new(),
                last_updated: chrono::Utc::now(),
            },
            compound_skills: HashMap::new(),
            context_window: 5,
            confidence_threshold: 0.6,
        };

        extractor.initialize_patterns().await?;
        extractor.load_skill_ontology().await?;
        extractor.update_trending_skills().await?;

        Ok(extractor)
    }

    async fn initialize_patterns(&mut self) -> Result<()> {
        info!("Initializing modern NLP patterns for keyword extraction");

        // Experience extraction patterns
        self.experience_patterns = vec![
            Regex::new(
                r"(?i)(\d+)(?:\+)?\s*(?:years?|yrs?)\s*(?:of\s*)?(?:experience|exp)(?:\s*(?:in|with|using))?\s*([^.;]+)",
            )?,
            Regex::new(r"(?i)(\d+)(?:\+)?\s*(?:years?|yrs?)\s*([^.;,]+?)(?:experience|exp)")?,
            Regex::new(
                r"(?i)(?:experienced|expert|proficient)(?:\s*(?:in|with|using))?\s*([^.;,]+?)",
            )?,
            Regex::new(
                r"(?i)(?:senior|lead|principal|staff)\s+([a-zA-Z\s]+?)(?:engineer|developer|architect)",
            )?,
        ];

        // Certification patterns by industry
        self.certification_patterns.insert("technology".to_string(), 
            Regex::new(r"(?i)(aws\s*certified|azure\s*certified|gcp\s*certified|cissp|cism|ceh|comptia|oracle\s*certified|microsoft\s*certified|cisco\s*certified|kubernetes\s*certified)")?);

        self.certification_patterns.insert(
            "project_management".to_string(),
            Regex::new(r"(?i)(pmp|prince2|scrum\s*master|safe|agile\s*certified|csm|psm)")?,
        );

        // Negation patterns to avoid false positives
        self.negation_patterns = vec![
            Regex::new(
                r"(?i)(?:no|not|without|lack(?:ing)?|never|absent)\s+(?:experience\s+(?:in|with))?\s*",
            )?,
            Regex::new(r"(?i)(?:unfamiliar|inexperienced|novice|beginner)\s+(?:with|in)\s*")?,
            Regex::new(r"(?i)(?:limited|minimal|basic)\s+(?:experience\s+(?:in|with))?\s*")?,
        ];

        // Load modern skill patterns from database and trending sources
        self.load_skill_patterns().await?;

        Ok(())
    }

    async fn load_skill_patterns(&mut self) -> Result<()> {
        // Technology patterns - much more comprehensive
        let tech_patterns = vec![
            // Programming languages with context
            Regex::new(
                r"(?i)\b(python|java(?:script)?|typescript|c\+\+|c#|rust|go|kotlin|swift|scala|ruby|php|perl|r\b|matlab)\b(?:\s*(?:programming|development|coding))?",
            )?,
            // Web frameworks with versions
            Regex::new(
                r"(?i)\b(react(?:\.js)?|angular(?:\.js)?|vue(?:\.js)?|svelte|next\.js|nuxt\.js|express(?:\.js)?|fastify)\s*(?:v?\d+(?:\.\d+)*)?",
            )?,
            // Backend frameworks
            Regex::new(
                r"(?i)\b(django|flask|spring(?:\s*boot)?|laravel|rails|asp\.net|node\.js|deno|fastapi)\b",
            )?,
            // Databases with context
            Regex::new(
                r"(?i)\b(postgresql|mysql|mongodb|redis|elasticsearch|cassandra|dynamodb|firestore|sqlite|oracle|sql\s*server)\b(?:\s*(?:database|db))?",
            )?,
            // Cloud platforms with services
            Regex::new(
                r"(?i)\b(aws|amazon\s*web\s*services|azure|google\s*cloud(?:\s*platform)?|gcp|digital\s*ocean|heroku|vercel|netlify)\b",
            )?,
            // DevOps and Infrastructure
            Regex::new(
                r"(?i)\b(docker|kubernetes|k8s|jenkins|gitlab\s*ci|github\s*actions|terraform|ansible|chef|puppet|vagrant)\b",
            )?,
            // AI/ML Technologies
            Regex::new(
                r"(?i)\b(tensorflow|pytorch|scikit-learn|pandas|numpy|jupyter|machine\s*learning|deep\s*learning|neural\s*networks|nlp|computer\s*vision)\b",
            )?,
            // Mobile development
            Regex::new(
                r"(?i)\b(react\s*native|flutter|ionic|xamarin|kotlin|swift|android|ios|mobile\s*development)\b",
            )?,
        ];

        self.skill_patterns
            .insert("technology".to_string(), tech_patterns);

        // Load other industry patterns
        self.load_healthcare_patterns()?;
        self.load_finance_patterns()?;
        self.load_business_patterns()?;

        Ok(())
    }

    fn load_healthcare_patterns(&mut self) -> Result<()> {
        let healthcare_patterns = vec![
            Regex::new(
                r"(?i)\b(epic|cerner|meditech|allscripts|athenahealth|emr|ehr|electronic\s*(?:medical|health)\s*record)\b",
            )?,
            Regex::new(
                r"(?i)\b(hipaa|hitech|fda|clinical\s*trial|medical\s*device|pharmaceutical|biotech)\b",
            )?,
            Regex::new(
                r"(?i)\b(radiology|cardiology|oncology|dermatology|pathology|anesthesiology|surgery)\b",
            )?,
            Regex::new(
                r"(?i)\b(icd-?\d+|cpt|medical\s*coding|medical\s*billing|healthcare\s*analytics)\b",
            )?,
        ];

        self.skill_patterns
            .insert("healthcare".to_string(), healthcare_patterns);
        Ok(())
    }

    fn load_finance_patterns(&mut self) -> Result<()> {
        let finance_patterns = vec![
            Regex::new(
                r"(?i)\b(bloomberg|reuters|factset|morningstar|risk\s*management|portfolio\s*management)\b",
            )?,
            Regex::new(
                r"(?i)\b(financial\s*modeling|valuation|dcf|discounted\s*cash\s*flow|capm|var|value\s*at\s*risk)\b",
            )?,
            Regex::new(
                r"(?i)\b(trading|investment\s*banking|asset\s*management|hedge\s*fund|private\s*equity)\b",
            )?,
            Regex::new(
                r"(?i)\b(cfa|frm|cpa|financial\s*planning|wealth\s*management|compliance)\b",
            )?,
            Regex::new(
                r"(?i)\b(forex|derivatives|fixed\s*income|equity|bonds|securities|commodities)\b",
            )?,
        ];

        self.skill_patterns
            .insert("finance".to_string(), finance_patterns);
        Ok(())
    }

    fn load_business_patterns(&mut self) -> Result<()> {
        let business_patterns = vec![
            Regex::new(
                r"(?i)\b(project\s*management|agile|scrum|kanban|waterfall|lean|six\s*sigma)\b",
            )?,
            Regex::new(
                r"(?i)\b(business\s*analysis|process\s*improvement|change\s*management|stakeholder\s*management)\b",
            )?,
            Regex::new(
                r"(?i)\b(salesforce|hubspot|marketo|tableau|power\s*bi|excel|google\s*analytics)\b",
            )?,
            Regex::new(
                r"(?i)\b(digital\s*marketing|seo|sem|social\s*media|content\s*marketing|email\s*marketing)\b",
            )?,
        ];

        self.skill_patterns
            .insert("business".to_string(), business_patterns);
        Ok(())
    }

    async fn load_skill_ontology(&mut self) -> Result<()> {
        info!("Loading skill ontology and relationships");

        // Build comprehensive skill relationships
        let mut relationships = HashMap::new();

        // Example: React skill node
        relationships.insert(
            "react".to_string(),
            SkillNode {
                skill_name: "React".to_string(),
                category: "frontend_framework".to_string(),
                synonyms: vec!["reactjs".to_string(), "react.js".to_string()],
                prerequisites: vec![
                    "javascript".to_string(),
                    "html".to_string(),
                    "css".to_string(),
                ],
                related_skills: vec![
                    "redux".to_string(),
                    "next.js".to_string(),
                    "typescript".to_string(),
                ],
                industry_relevance: [
                    ("technology".to_string(), 0.95),
                    ("startup".to_string(), 0.9),
                ]
                .iter()
                .cloned()
                .collect(),
                difficulty_level: 0.7,
                market_demand: 0.9,
            },
        );

        // Python skill node
        relationships.insert(
            "python".to_string(),
            SkillNode {
                skill_name: "Python".to_string(),
                category: "programming_language".to_string(),
                synonyms: vec!["py".to_string(), "python3".to_string()],
                prerequisites: vec![],
                related_skills: vec![
                    "django".to_string(),
                    "flask".to_string(),
                    "pandas".to_string(),
                    "tensorflow".to_string(),
                ],
                industry_relevance: [
                    ("technology".to_string(), 0.9),
                    ("data_science".to_string(), 0.95),
                    ("finance".to_string(), 0.8),
                ]
                .iter()
                .cloned()
                .collect(),
                difficulty_level: 0.5,
                market_demand: 0.95,
            },
        );

        // AWS skill node
        relationships.insert(
            "aws".to_string(),
            SkillNode {
                skill_name: "AWS".to_string(),
                category: "cloud_platform".to_string(),
                synonyms: vec!["amazon web services".to_string()],
                prerequisites: vec!["linux".to_string(), "networking".to_string()],
                related_skills: vec![
                    "docker".to_string(),
                    "kubernetes".to_string(),
                    "terraform".to_string(),
                ],
                industry_relevance: [
                    ("technology".to_string(), 0.9),
                    ("enterprise".to_string(), 0.85),
                ]
                .iter()
                .cloned()
                .collect(),
                difficulty_level: 0.8,
                market_demand: 0.9,
            },
        );

        self.skill_ontology.skill_relationships = relationships;

        // Load compound skills (skill combinations that work together)
        self.compound_skills.insert(
            "full_stack".to_string(),
            vec![
                "javascript".to_string(),
                "react".to_string(),
                "node.js".to_string(),
                "mongodb".to_string(),
            ],
        );

        self.compound_skills.insert(
            "data_science".to_string(),
            vec![
                "python".to_string(),
                "pandas".to_string(),
                "numpy".to_string(),
                "scikit-learn".to_string(),
                "jupyter".to_string(),
            ],
        );

        self.compound_skills.insert(
            "devops".to_string(),
            vec![
                "docker".to_string(),
                "kubernetes".to_string(),
                "jenkins".to_string(),
                "aws".to_string(),
                "terraform".to_string(),
            ],
        );

        Ok(())
    }

    async fn update_trending_skills(&mut self) -> Result<()> {
        info!("Updating trending skills cache");

        // In production, this would call external APIs (GitHub, Stack Overflow, job boards)
        // For now, we'll use static data representing 2024-2025 trends

        let tech_trending = vec![
            TrendingSkill {
                skill_name: "rust".to_string(),
                trend_score: 0.95,
                growth_rate: 0.8,
                demand_level: 0.7,
            },
            TrendingSkill {
                skill_name: "webassembly".to_string(),
                trend_score: 0.9,
                growth_rate: 0.9,
                demand_level: 0.6,
            },
            TrendingSkill {
                skill_name: "next.js".to_string(),
                trend_score: 0.88,
                growth_rate: 0.7,
                demand_level: 0.8,
            },
            TrendingSkill {
                skill_name: "svelte".to_string(),
                trend_score: 0.85,
                growth_rate: 0.75,
                demand_level: 0.65,
            },
            TrendingSkill {
                skill_name: "deno".to_string(),
                trend_score: 0.8,
                growth_rate: 0.6,
                demand_level: 0.5,
            },
            TrendingSkill {
                skill_name: "edge_computing".to_string(),
                trend_score: 0.9,
                growth_rate: 0.85,
                demand_level: 0.7,
            },
            TrendingSkill {
                skill_name: "kubernetes".to_string(),
                trend_score: 0.92,
                growth_rate: 0.6,
                demand_level: 0.9,
            },
            TrendingSkill {
                skill_name: "terraform".to_string(),
                trend_score: 0.88,
                growth_rate: 0.65,
                demand_level: 0.85,
            },
        ];

        self.trending_skills
            .skills_by_industry
            .insert("technology".to_string(), tech_trending);
        self.trending_skills.last_updated = chrono::Utc::now();

        Ok(())
    }

    pub async fn extract_keywords(
        &self,
        text: &str,
        target_industry: &str,
        job_description: Option<&str>,
    ) -> Result<ExtractionResult> {
        let start_time = std::time::Instant::now();

        info!(
            "Starting modern keyword extraction for industry: {}",
            target_industry
        );

        // Step 1: Text preprocessing
        let preprocessed_text = self.preprocess_text(text);
        let tokens = self.tokenize(&preprocessed_text);

        // Step 2: AI-powered extraction using Ollama
        let ai_extracted_skills = self
            .ai_extract_skills(&preprocessed_text, target_industry)
            .await?;

        // Step 3: Pattern-based extraction with context
        let pattern_matches = self.pattern_based_extraction(&preprocessed_text, target_industry)?;

        // Step 4: Semantic analysis and matching
        let semantic_matches = self
            .semantic_matching(&preprocessed_text, target_industry)
            .await?;

        // Step 5: Combine and rank all matches
        let mut all_matches =
            self.combine_matches(ai_extracted_skills, pattern_matches, semantic_matches)?;

        // Step 6: Context validation and confidence scoring
        self.validate_with_context(&mut all_matches, &preprocessed_text, job_description)?;

        // Step 7: Skill cluster analysis
        let skill_clusters = self.analyze_skill_clusters(&all_matches)?;

        // Step 8: Identify missing critical skills
        let missing_critical_skills = self
            .identify_missing_skills(&all_matches, target_industry, job_description)
            .await?;

        // Step 9: Identify emerging skills
        let emerging_skills = self.identify_emerging_skills(&all_matches, target_industry)?;

        let processing_time = start_time.elapsed();

        let confidence_score = if all_matches.is_empty() {
            0.0
        } else {
            all_matches.iter().map(|m| m.confidence_score).sum::<f64>() / all_matches.len() as f64
        };

        Ok(ExtractionResult {
            keyword_matches: all_matches,
            skill_clusters,
            missing_critical_skills,
            emerging_skills,
            confidence_score,
            extraction_metadata: ExtractionMetadata {
                total_words_processed: tokens.len(),
                technical_density: self.calculate_technical_density(&tokens),
                avg_confidence_score: confidence_score,
                processing_time_ms: processing_time.as_millis() as u64,
                nlp_model_used: "ollama-qwen2.5:14b".to_string(),
                extraction_version: "2024.1".to_string(),
            },
        })
    }

    fn preprocess_text(&self, text: &str) -> String {
        // Advanced text preprocessing
        let mut processed = text.to_lowercase();

        // Normalize common variations
        processed = processed.replace("node.js", "nodejs");
        processed = processed.replace("react.js", "reactjs");
        processed = processed.replace("vue.js", "vuejs");
        processed = processed.replace("c++", "cplusplus");
        processed = processed.replace("c#", "csharp");

        // Handle common typos and variations
        // (placeholder for future typo corrections)
        processed = processed.replace("postgre sql", "postgresql");
        processed = processed.replace("mongo db", "mongodb");

        // Remove excessive whitespace
        let whitespace_regex = Regex::new(r"\s+").unwrap();
        processed = whitespace_regex.replace_all(&processed, " ").to_string();

        processed
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        // Advanced tokenization that preserves technical terms
        let mut tokens = Vec::new();

        // Split on whitespace and punctuation, but preserve technical patterns
        let token_regex = Regex::new(r"[a-zA-Z0-9_\-\.]+").unwrap();

        for mat in token_regex.find_iter(text) {
            let token = mat.as_str().to_string();

            // Add the original token
            tokens.push(token.clone());

            // Also add stemmed version for better matching
            let stemmed = self.stemmer.stem(&token);
            if stemmed != token {
                tokens.push(stemmed);
            }
        }

        tokens
    }

    async fn ai_extract_skills(
        &self,
        text: &str,
        industry: &str,
    ) -> Result<Vec<ModernKeywordMatch>> {
        let prompt = format!(
            "Extract technical skills, tools, and technologies from this text for the {} industry. \
            Return a JSON array of objects with 'skill', 'category', 'confidence' (0-1), and 'context':\n\n{}",
            industry, text
        );

        match self
            .ollama_client
            .generate_response("qwen2.5:14b", &prompt, None)
            .await
        {
            Ok((response, _)) => self.parse_ai_skills_response(&response),
            Err(_) => {
                // Fallback to empty results if AI fails
                Ok(Vec::new())
            }
        }
    }

    fn parse_ai_skills_response(&self, response: &str) -> Result<Vec<ModernKeywordMatch>> {
        // Parse AI response - simplified implementation
        let mut matches = Vec::new();

        // Look for JSON in response
        if let Some(start) = response.find('[') {
            if let Some(end) = response.rfind(']') {
                let json_str = &response[start..=end];
                if let Ok(parsed) = serde_json::from_str::<Vec<serde_json::Value>>(json_str) {
                    for item in parsed {
                        if let (Some(skill), Some(category), Some(confidence)) = (
                            item["skill"].as_str(),
                            item["category"].as_str(),
                            item["confidence"].as_f64(),
                        ) {
                            matches.push(ModernKeywordMatch {
                                keyword: skill.to_string(),
                                normalized_form: self.stemmer.stem(skill),
                                category: category.to_string(),
                                confidence_score: confidence,
                                context_relevance: 0.8, // Default for AI matches
                                skill_level: None,
                                experience_years: None,
                                certifications: Vec::new(),
                                context_phrases: vec![item["context"]
                                    .as_str()
                                    .unwrap_or("")
                                    .to_string()],
                                word_position: 0,
                                semantic_variations: Vec::new(),
                                industry_relevance: 0.9,
                                match_type: MatchType::Semantic,
                                weight: confidence * 0.9, // Slightly lower weight for AI matches
                            });
                        }
                    }
                }
            }
        }

        Ok(matches)
    }

    fn pattern_based_extraction(
        &self,
        text: &str,
        industry: &str,
    ) -> Result<Vec<ModernKeywordMatch>> {
        let mut matches = Vec::new();

        if let Some(patterns) = self.skill_patterns.get(industry) {
            for pattern in patterns.iter() {
                for mat in pattern.find_iter(text) {
                    let keyword = mat.as_str().trim().to_string();
                    let normalized = self.stemmer.stem(&keyword);

                    matches.push(ModernKeywordMatch {
                        keyword: keyword.clone(),
                        normalized_form: normalized,
                        category: industry.to_string(),
                        confidence_score: 0.8, // High confidence for pattern matches
                        context_relevance: self.calculate_context_relevance(
                            text,
                            mat.start(),
                            mat.end(),
                        ),
                        skill_level: self.extract_skill_level(text, mat.start()),
                        experience_years: self.extract_experience_years(text, mat.start()),
                        certifications: Vec::new(),
                        context_phrases: self.extract_context_phrases(text, mat.start(), mat.end()),
                        word_position: mat.start(),
                        semantic_variations: self.find_semantic_variations(&keyword),
                        industry_relevance: self.get_industry_relevance(&keyword, industry),
                        match_type: MatchType::Exact,
                        weight: 0.9,
                    });
                }
            }
        }

        Ok(matches)
    }

    async fn semantic_matching(
        &self,
        text: &str,
        industry: &str,
    ) -> Result<Vec<ModernKeywordMatch>> {
        let mut matches = Vec::new();

        // Use skill ontology for semantic matching
        for (skill_name, skill_node) in &self.skill_ontology.skill_relationships {
            // Check for synonyms and variations
            for synonym in &skill_node.synonyms {
                if text.contains(&synonym.to_lowercase()) {
                    matches.push(ModernKeywordMatch {
                        keyword: skill_name.clone(),
                        normalized_form: self.stemmer.stem(skill_name),
                        category: skill_node.category.clone(),
                        confidence_score: 0.75, // Lower confidence for semantic matches
                        context_relevance: 0.7,
                        skill_level: None,
                        experience_years: None,
                        certifications: Vec::new(),
                        context_phrases: vec![synonym.clone()],
                        word_position: 0,
                        semantic_variations: skill_node.synonyms.clone(),
                        industry_relevance: skill_node
                            .industry_relevance
                            .get(industry)
                            .copied()
                            .unwrap_or(0.5),
                        match_type: MatchType::Semantic,
                        weight: 0.7,
                    });
                }
            }
        }

        Ok(matches)
    }

    fn combine_matches(
        &self,
        ai_matches: Vec<ModernKeywordMatch>,
        pattern_matches: Vec<ModernKeywordMatch>,
        semantic_matches: Vec<ModernKeywordMatch>,
    ) -> Result<Vec<ModernKeywordMatch>> {
        let mut combined = Vec::new();
        let mut seen_keywords = HashSet::new();

        // Prioritize pattern matches, then AI, then semantic
        for match_set in [pattern_matches, ai_matches, semantic_matches] {
            for mut keyword_match in match_set {
                let key = keyword_match.normalized_form.clone();

                if !seen_keywords.contains(&key) {
                    // Apply confidence boosting for multiple match types
                    if seen_keywords.len() < combined.len() {
                        keyword_match.confidence_score =
                            (keyword_match.confidence_score * 1.1).min(1.0);
                    }

                    combined.push(keyword_match);
                    seen_keywords.insert(key);
                }
            }
        }

        // Sort by confidence and relevance
        combined.sort_by(|a, b| {
            let score_a = a.confidence_score * a.context_relevance * a.weight;
            let score_b = b.confidence_score * b.context_relevance * b.weight;
            score_b
                .partial_cmp(&score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(combined)
    }

    fn validate_with_context(
        &self,
        matches: &mut Vec<ModernKeywordMatch>,
        text: &str,
        job_description: Option<&str>,
    ) -> Result<()> {
        for keyword_match in matches.iter_mut() {
            // Check for negation patterns around the match
            let has_negation = self.check_negation_context(text, keyword_match.word_position);
            if has_negation {
                keyword_match.confidence_score *= 0.3; // Heavily penalize negated matches
            }

            // Boost confidence if mentioned in job description
            if let Some(job_desc) = job_description {
                if job_desc
                    .to_lowercase()
                    .contains(&keyword_match.keyword.to_lowercase())
                {
                    keyword_match.confidence_score =
                        (keyword_match.confidence_score * 1.2).min(1.0);
                    keyword_match.context_relevance =
                        (keyword_match.context_relevance * 1.1).min(1.0);
                }
            }
        }

        // Filter out low-confidence matches
        matches.retain(|m| m.confidence_score >= self.confidence_threshold);

        Ok(())
    }

    fn analyze_skill_clusters(&self, matches: &[ModernKeywordMatch]) -> Result<Vec<SkillCluster>> {
        let mut clusters = Vec::new();

        // Analyze compound skill patterns
        for (cluster_name, required_skills) in &self.compound_skills {
            let mut found_skills = Vec::new();
            let mut total_confidence = 0.0;

            for required_skill in required_skills {
                if let Some(found_match) = matches.iter().find(|m| {
                    m.normalized_form == self.stemmer.stem(required_skill)
                        || m.keyword.to_lowercase() == required_skill.to_lowercase()
                }) {
                    found_skills.push(found_match.keyword.clone());
                    total_confidence += found_match.confidence_score;
                }
            }

            if !found_skills.is_empty() {
                let completeness = found_skills.len() as f64 / required_skills.len() as f64;
                let avg_confidence = total_confidence / found_skills.len() as f64;

                clusters.push(SkillCluster {
                    cluster_name: cluster_name.clone(),
                    skills: found_skills,
                    completeness_score: completeness * avg_confidence,
                    cluster_type: self.determine_cluster_type(cluster_name),
                });
            }
        }

        Ok(clusters)
    }

    async fn identify_missing_skills(
        &self,
        matches: &[ModernKeywordMatch],
        industry: &str,
        job_description: Option<&str>,
    ) -> Result<Vec<String>> {
        let mut missing_skills = Vec::new();
        let found_skills: HashSet<String> =
            matches.iter().map(|m| m.normalized_form.clone()).collect();

        // Check trending skills for the industry
        if let Some(trending) = self.trending_skills.skills_by_industry.get(industry) {
            for trending_skill in trending {
                let normalized_trending = self.stemmer.stem(&trending_skill.skill_name);
                if !found_skills.contains(&normalized_trending) && trending_skill.demand_level > 0.7
                {
                    missing_skills.push(trending_skill.skill_name.clone());
                }
            }
        }

        // Check job description for explicitly mentioned skills
        if let Some(job_desc) = job_description {
            let job_desc_matches = self.pattern_based_extraction(job_desc, industry)?;
            for job_match in job_desc_matches {
                if !found_skills.contains(&job_match.normalized_form) {
                    missing_skills.push(job_match.keyword);
                }
            }
        }

        // Deduplicate and limit results
        missing_skills.sort();
        missing_skills.dedup();
        missing_skills.truncate(10);

        Ok(missing_skills)
    }

    fn identify_emerging_skills(
        &self,
        matches: &[ModernKeywordMatch],
        industry: &str,
    ) -> Result<Vec<String>> {
        let mut emerging = Vec::new();

        if let Some(trending) = self.trending_skills.skills_by_industry.get(industry) {
            for trending_skill in trending {
                if trending_skill.trend_score > 0.8 && trending_skill.growth_rate > 0.7 {
                    // Check if this emerging skill was found
                    let found = matches.iter().any(|m| {
                        m.normalized_form == self.stemmer.stem(&trending_skill.skill_name)
                    });

                    if found {
                        emerging.push(trending_skill.skill_name.clone());
                    }
                }
            }
        }

        Ok(emerging)
    }

    // Helper methods

    fn calculate_context_relevance(&self, text: &str, start: usize, end: usize) -> f64 {
        let context_start = start.saturating_sub(50);
        let context_end = (end + 50).min(text.len());
        let context = &text[context_start..context_end];

        // Look for technical context indicators
        let technical_indicators = [
            "experience",
            "years",
            "proficient",
            "expert",
            "skilled",
            "knowledge",
            "using",
            "with",
        ];
        let indicator_count = technical_indicators
            .iter()
            .filter(|&indicator| context.contains(indicator))
            .count();

        0.5 + (indicator_count as f64 * 0.1).min(0.4)
    }

    fn extract_skill_level(&self, text: &str, position: usize) -> Option<SkillLevel> {
        let context_start = position.saturating_sub(30);
        let context_end = (position + 30).min(text.len());
        let context = &text[context_start..context_end].to_lowercase();

        if context.contains("expert") || context.contains("advanced") {
            Some(SkillLevel::Expert)
        } else if context.contains("senior") || context.contains("lead") {
            Some(SkillLevel::Advanced)
        } else if context.contains("intermediate") || context.contains("proficient") {
            Some(SkillLevel::Intermediate)
        } else if context.contains("beginner") || context.contains("basic") {
            Some(SkillLevel::Beginner)
        } else {
            Some(SkillLevel::Unknown)
        }
    }

    fn extract_experience_years(&self, text: &str, position: usize) -> Option<i32> {
        let context_start = position.saturating_sub(50);
        let context_end = (position + 50).min(text.len());
        let context = &text[context_start..context_end];

        for pattern in &self.experience_patterns {
            if let Some(captures) = pattern.captures(context) {
                if let Some(years_str) = captures.get(1) {
                    if let Ok(years) = years_str.as_str().parse::<i32>() {
                        return Some(years);
                    }
                }
            }
        }

        None
    }

    fn extract_context_phrases(&self, text: &str, start: usize, end: usize) -> Vec<String> {
        let context_start = start.saturating_sub(20);
        let context_end = (end + 20).min(text.len());
        let context = &text[context_start..context_end];

        // Split into phrases and return relevant ones
        context
            .split(&['.', ';', '\n'][..])
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty() && s.len() > 5)
            .take(2)
            .collect()
    }

    fn find_semantic_variations(&self, keyword: &str) -> Vec<String> {
        let normalized = self.stemmer.stem(keyword);

        if let Some(skill_node) = self.skill_ontology.skill_relationships.get(&normalized) {
            skill_node.synonyms.clone()
        } else {
            Vec::new()
        }
    }

    fn get_industry_relevance(&self, keyword: &str, industry: &str) -> f64 {
        let normalized = self.stemmer.stem(keyword);

        if let Some(skill_node) = self.skill_ontology.skill_relationships.get(&normalized) {
            skill_node
                .industry_relevance
                .get(industry)
                .copied()
                .unwrap_or(0.5)
        } else {
            0.5 // Default relevance
        }
    }

    fn check_negation_context(&self, text: &str, position: usize) -> bool {
        let context_start = position.saturating_sub(30);
        let context_end = (position + 10).min(text.len());
        let context = &text[context_start..context_end];

        self.negation_patterns
            .iter()
            .any(|pattern| pattern.is_match(context))
    }

    fn determine_cluster_type(&self, cluster_name: &str) -> ClusterType {
        match cluster_name {
            "full_stack" => ClusterType::WebDevelopment,
            "data_science" => ClusterType::DataScience,
            "devops" => ClusterType::DevOps,
            "mobile" => ClusterType::MobileStack,
            "cloud" => ClusterType::CloudPlatform,
            "security" => ClusterType::SecurityStack,
            "database" => ClusterType::DatabaseCluster,
            _ => ClusterType::TechnicalStack,
        }
    }

    fn calculate_technical_density(&self, tokens: &[String]) -> f64 {
        if tokens.is_empty() {
            return 0.0;
        }

        let technical_count = tokens
            .iter()
            .filter(|token| self.is_technical_term(token))
            .count();

        technical_count as f64 / tokens.len() as f64
    }

    fn is_technical_term(&self, term: &str) -> bool {
        // Check if term matches any known technical patterns
        let term_lower = term.to_lowercase();

        // Check against skill ontology
        if self
            .skill_ontology
            .skill_relationships
            .contains_key(&term_lower)
        {
            return true;
        }

        // Check against common technical indicators
        let technical_patterns = [
            r".*js$",
            r".*py$",
            r".*sql$",
            r".*api$",
            r".*db$",
            r".*framework$",
            r".*library$",
            r".*sdk$",
            r".*cli$",
        ];

        for pattern in &technical_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(&term_lower) {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_porter_stemmer() {
        let stemmer = PorterStemmer::new();

        assert_eq!(stemmer.stem("programming"), "programm");
        assert_eq!(stemmer.stem("developed"), "develop");
        assert_eq!(stemmer.stem("developer"), "develop");
        assert_eq!(stemmer.stem("quickly"), "quick");
        assert_eq!(stemmer.stem("optimization"), "optimize");
    }

    #[tokio::test]
    async fn test_text_preprocessing() {
        let database = crate::database::Database::new(":memory:").await.unwrap();
        let extractor = ModernKeywordExtractor::new(database).await.unwrap();

        let input = "I have experience with React.js, Node.js, and C++";
        let processed = extractor.preprocess_text(input);

        assert!(processed.contains("reactjs"));
        assert!(processed.contains("nodejs"));
        assert!(processed.contains("cplusplus"));
    }
}
