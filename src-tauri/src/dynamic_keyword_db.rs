use anyhow::Result;
use chrono::{DateTime, Utc};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::collections::HashMap;
use tokio::time::{interval, Duration};

use crate::database::Database;
use crate::ollama::OllamaClient;

/// Dynamic keyword database that learns and adapts in real-time
pub struct DynamicKeywordDatabase {
    database: Database,
    ollama_client: OllamaClient,

    // Real-time caches
    trending_keywords: HashMap<String, TrendingKeywordData>,
    industry_keywords: HashMap<String, Vec<DynamicKeyword>>,
    #[allow(dead_code)]
    skill_relationships: HashMap<String, SkillRelationshipData>,
    #[allow(dead_code)]
    market_demand_cache: HashMap<String, MarketDemandData>,

    // Configuration
    update_interval_hours: u64,
    confidence_threshold: f64,
    #[allow(dead_code)]
    max_keywords_per_industry: usize,
    last_full_update: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicKeyword {
    pub keyword: String,
    pub normalized_form: String,
    pub industry: String,
    pub category: String,
    pub confidence_score: f64,
    pub market_frequency: f64,
    pub growth_rate: f64,
    pub synonyms: Vec<String>,
    pub related_skills: Vec<String>,
    pub difficulty_level: f32,
    pub salary_impact: f32,
    pub geographic_relevance: HashMap<String, f64>,
    pub last_updated: DateTime<Utc>,
    pub source: KeywordSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeywordSource {
    JobPosting,
    UserFeedback,
    AIAnalysis,
    IndustryReport,
    TrendAnalysis,
    ManualCuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingKeywordData {
    pub keyword: String,
    pub trend_score: f64,
    pub velocity: f64, // Rate of change
    pub momentum: f64, // Acceleration of trend
    pub peak_prediction: Option<DateTime<Utc>>,
    pub decline_prediction: Option<DateTime<Utc>>,
    pub adoption_stage: AdoptionStage,
    pub industries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdoptionStage {
    Emerging,   // Just appearing
    Growing,    // Gaining traction
    Mainstream, // Widely adopted
    Mature,     // Established standard
    Declining,  // Losing relevance
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRelationshipData {
    pub primary_skill: String,
    pub prerequisites: Vec<WeightedSkill>,
    pub complementary_skills: Vec<WeightedSkill>,
    pub alternative_skills: Vec<WeightedSkill>,
    pub career_progressions: Vec<WeightedSkill>,
    pub co_occurrence_frequency: HashMap<String, f64>,
    pub learning_pathways: Vec<LearningPathway>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightedSkill {
    pub skill: String,
    pub weight: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPathway {
    pub pathway_name: String,
    pub steps: Vec<LearningStep>,
    pub estimated_duration: String,
    pub difficulty: f32,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStep {
    pub skill: String,
    pub order: u32,
    pub prerequisites: Vec<String>,
    pub resources: Vec<LearningResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningResource {
    pub resource_type: ResourceType,
    pub title: String,
    pub provider: String,
    pub url: Option<String>,
    pub cost: String,
    pub duration: String,
    pub rating: f32,
    pub completion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    Course,
    Certification,
    Book,
    Tutorial,
    Project,
    Bootcamp,
    Workshop,
    Conference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDemandData {
    pub skill: String,
    pub demand_score: f64,
    pub supply_score: f64,
    pub competition_ratio: f64,
    pub salary_trends: SalaryTrendData,
    pub job_postings_count: u64,
    pub geographic_demand: HashMap<String, f64>,
    pub industry_demand: HashMap<String, f64>,
    pub growth_projection: GrowthProjection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryTrendData {
    pub current_average: f64,
    pub six_month_growth: f64,
    pub yearly_growth: f64,
    pub geographic_variance: HashMap<String, f64>,
    pub experience_multipliers: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthProjection {
    pub next_quarter: f64,
    pub next_year: f64,
    pub three_year: f64,
    pub confidence_interval: (f64, f64),
    pub key_drivers: Vec<String>,
}

impl DynamicKeywordDatabase {
    pub async fn new(database: Database) -> Result<Self> {
        let ollama_client = OllamaClient::new(None)?;

        let mut db = Self {
            database,
            ollama_client,
            trending_keywords: HashMap::new(),
            industry_keywords: HashMap::new(),
            skill_relationships: HashMap::new(),
            market_demand_cache: HashMap::new(),
            update_interval_hours: 6, // Update every 6 hours
            confidence_threshold: 0.6,
            max_keywords_per_industry: 500,
            last_full_update: Utc::now() - chrono::Duration::hours(24), // Force initial update
        };

        // Initialize database tables
        db.initialize_database_schema().await?;

        // Load existing data from database
        db.load_cached_data().await?;

        // Start background update task
        db.start_background_updates().await?;

        info!("Dynamic keyword database initialized successfully");
        Ok(db)
    }

    async fn initialize_database_schema(&self) -> Result<()> {
        info!("Initializing dynamic keyword database schema");

        // Dynamic keywords table
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS dynamic_keywords (
                id TEXT PRIMARY KEY,
                keyword TEXT NOT NULL,
                normalized_form TEXT NOT NULL,
                industry TEXT NOT NULL,
                category TEXT NOT NULL,
                confidence_score REAL NOT NULL,
                market_frequency REAL NOT NULL,
                growth_rate REAL NOT NULL,
                synonyms TEXT NOT NULL, -- JSON array
                related_skills TEXT NOT NULL, -- JSON array
                difficulty_level REAL NOT NULL,
                salary_impact REAL NOT NULL,
                geographic_relevance TEXT NOT NULL, -- JSON object
                last_updated TEXT NOT NULL,
                source TEXT NOT NULL,
                UNIQUE(keyword, industry)
            );
        ",
        )
        .execute(self.database.get_pool())
        .await?;

        // Trending keywords table
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS trending_keywords (
                id TEXT PRIMARY KEY,
                keyword TEXT NOT NULL UNIQUE,
                trend_score REAL NOT NULL,
                velocity REAL NOT NULL,
                momentum REAL NOT NULL,
                peak_prediction TEXT,
                decline_prediction TEXT,
                adoption_stage TEXT NOT NULL,
                industries TEXT NOT NULL, -- JSON array
                last_updated TEXT NOT NULL
            );
        ",
        )
        .execute(self.database.get_pool())
        .await?;

        // Skill relationships table
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS skill_relationships (
                id TEXT PRIMARY KEY,
                primary_skill TEXT NOT NULL UNIQUE,
                prerequisites TEXT NOT NULL, -- JSON array
                complementary_skills TEXT NOT NULL, -- JSON array
                alternative_skills TEXT NOT NULL, -- JSON array
                career_progressions TEXT NOT NULL, -- JSON array
                co_occurrence_frequency TEXT NOT NULL, -- JSON object
                learning_pathways TEXT NOT NULL, -- JSON array
                last_updated TEXT NOT NULL
            );
        ",
        )
        .execute(self.database.get_pool())
        .await?;

        // Market demand data table
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS market_demand_data (
                id TEXT PRIMARY KEY,
                skill TEXT NOT NULL UNIQUE,
                demand_score REAL NOT NULL,
                supply_score REAL NOT NULL,
                competition_ratio REAL NOT NULL,
                salary_trends TEXT NOT NULL, -- JSON object
                job_postings_count INTEGER NOT NULL,
                geographic_demand TEXT NOT NULL, -- JSON object
                industry_demand TEXT NOT NULL, -- JSON object
                growth_projection TEXT NOT NULL, -- JSON object
                last_updated TEXT NOT NULL
            );
        ",
        )
        .execute(self.database.get_pool())
        .await?;

        // Create indexes for performance
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_dynamic_keywords_industry ON dynamic_keywords(industry);")
            .execute(self.database.get_pool()).await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_dynamic_keywords_category ON dynamic_keywords(category);")
            .execute(self.database.get_pool()).await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_trending_keywords_score ON trending_keywords(trend_score);")
            .execute(self.database.get_pool()).await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_market_demand_score ON market_demand_data(demand_score);")
            .execute(self.database.get_pool()).await?;

        info!("Dynamic keyword database schema initialized");
        Ok(())
    }

    async fn load_cached_data(&mut self) -> Result<()> {
        info!("Loading cached keyword data from database");

        // Load dynamic keywords
        let keyword_rows = sqlx::query(
            "
            SELECT keyword, normalized_form, industry, category, confidence_score, 
                   market_frequency, growth_rate, synonyms, related_skills, 
                   difficulty_level, salary_impact, geographic_relevance, 
                   last_updated, source
            FROM dynamic_keywords 
            WHERE confidence_score >= ?
            ORDER BY market_frequency DESC
        ",
        )
        .bind(self.confidence_threshold)
        .fetch_all(self.database.get_pool())
        .await?;

        for row in keyword_rows {
            let industry: String = row.get("industry");
            let keyword = DynamicKeyword {
                keyword: row.get("keyword"),
                normalized_form: row.get("normalized_form"),
                industry: industry.clone(),
                category: row.get("category"),
                confidence_score: row.get("confidence_score"),
                market_frequency: row.get("market_frequency"),
                growth_rate: row.get("growth_rate"),
                synonyms: serde_json::from_str::<Vec<String>>(&row.get::<String, _>("synonyms"))
                    .unwrap_or_default(),
                related_skills: serde_json::from_str::<Vec<String>>(
                    &row.get::<String, _>("related_skills"),
                )
                .unwrap_or_default(),
                difficulty_level: row.get("difficulty_level"),
                salary_impact: row.get("salary_impact"),
                geographic_relevance: serde_json::from_str::<HashMap<String, f64>>(
                    &row.get::<String, _>("geographic_relevance"),
                )
                .unwrap_or_default(),
                last_updated: row.get::<String, _>("last_updated").parse()?,
                source: serde_json::from_str(&format!("\"{}\"", row.get::<String, _>("source")))
                    .unwrap_or(KeywordSource::ManualCuration),
            };

            self.industry_keywords
                .entry(industry)
                .or_default()
                .push(keyword);
        }

        // Load trending keywords
        let trending_rows = sqlx::query(
            "
            SELECT keyword, trend_score, velocity, momentum, peak_prediction, 
                   decline_prediction, adoption_stage, industries, last_updated
            FROM trending_keywords 
            ORDER BY trend_score DESC
        ",
        )
        .fetch_all(self.database.get_pool())
        .await?;

        for row in trending_rows {
            let keyword: String = row.get("keyword");
            let trending_data = TrendingKeywordData {
                keyword: keyword.clone(),
                trend_score: row.get("trend_score"),
                velocity: row.get("velocity"),
                momentum: row.get("momentum"),
                peak_prediction: row
                    .get::<Option<String>, _>("peak_prediction")
                    .and_then(|s| s.parse().ok()),
                decline_prediction: row
                    .get::<Option<String>, _>("decline_prediction")
                    .and_then(|s| s.parse().ok()),
                adoption_stage: serde_json::from_str(&format!(
                    "\"{}\"",
                    row.get::<String, _>("adoption_stage")
                ))
                .unwrap_or(AdoptionStage::Emerging),
                industries: serde_json::from_str::<Vec<String>>(
                    &row.get::<String, _>("industries"),
                )
                .unwrap_or_default(),
            };

            self.trending_keywords.insert(keyword, trending_data);
        }

        info!(
            "Loaded {} industry keyword sets and {} trending keywords",
            self.industry_keywords.len(),
            self.trending_keywords.len()
        );

        Ok(())
    }

    async fn start_background_updates(&self) -> Result<()> {
        let database = self.database.clone();
        let ollama_client = self.ollama_client.clone();
        let update_interval = self.update_interval_hours;

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(update_interval * 3600));

            loop {
                interval.tick().await;

                if let Err(e) = Self::perform_background_update(&database, &ollama_client).await {
                    warn!("Background keyword update failed: {}", e);
                }
            }
        });

        info!(
            "Background keyword update task started (interval: {} hours)",
            update_interval
        );
        Ok(())
    }

    async fn perform_background_update(
        database: &Database,
        ollama_client: &OllamaClient,
    ) -> Result<()> {
        info!("Starting background keyword database update");

        // Update trending keywords from multiple sources
        Self::update_trending_keywords_from_ai(database, ollama_client).await?;

        // Update market demand data
        Self::update_market_demand_data(database, ollama_client).await?;

        // Update skill relationships
        Self::update_skill_relationships(database, ollama_client).await?;

        // Clean up outdated data
        Self::cleanup_outdated_data(database).await?;

        info!("Background keyword database update completed");
        Ok(())
    }

    async fn update_trending_keywords_from_ai(
        database: &Database,
        ollama_client: &OllamaClient,
    ) -> Result<()> {
        let prompt = r#"Analyze current technology trends for 2024-2025 and provide trending skills/keywords.
        
Focus on:
1. Emerging technologies gaining adoption
2. Programming languages showing growth
3. Frameworks becoming popular
4. Cloud and DevOps tools trending
5. AI/ML technologies in demand
6. Data science and analytics tools
7. Cybersecurity technologies
8. Mobile and web development trends

Return a JSON array of trending keywords with this structure:
[
  {
    "keyword": "edge computing",
    "trend_score": 0.92,
    "velocity": 0.15,
    "momentum": 0.08,
    "adoption_stage": "Growing",
    "industries": ["technology", "enterprise", "startup"]
  }
]

Include 20-30 most significant trending keywords."#;

        match ollama_client
            .generate_response("qwen2.5:14b", prompt, None)
            .await
        {
            Ok((response, _)) => {
                if let Some(json_start) = response.find('[') {
                    if let Some(json_end) = response.rfind(']') {
                        let json_str = &response[json_start..=json_end];

                        if let Ok(trending_data) =
                            serde_json::from_str::<Vec<serde_json::Value>>(json_str)
                        {
                            let trending_data_len = trending_data.len();
                            for item in trending_data {
                                if let (
                                    Some(keyword),
                                    Some(trend_score),
                                    Some(velocity),
                                    Some(momentum),
                                    Some(adoption_stage),
                                    Some(industries),
                                ) = (
                                    item["keyword"].as_str(),
                                    item["trend_score"].as_f64(),
                                    item["velocity"].as_f64(),
                                    item["momentum"].as_f64(),
                                    item["adoption_stage"].as_str(),
                                    item["industries"].as_array(),
                                ) {
                                    let industries_vec: Vec<String> = industries
                                        .iter()
                                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                        .collect();

                                    // Insert or update trending keyword
                                    sqlx::query("
                                        INSERT OR REPLACE INTO trending_keywords 
                                        (id, keyword, trend_score, velocity, momentum, adoption_stage, industries, last_updated)
                                        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                                    ")
                                    .bind(uuid::Uuid::new_v4().to_string())
                                    .bind(keyword)
                                    .bind(trend_score)
                                    .bind(velocity)
                                    .bind(momentum)
                                    .bind(adoption_stage)
                                    .bind(serde_json::to_string(&industries_vec).unwrap_or_default())
                                    .bind(Utc::now().to_rfc3339())
                                    .execute(database.get_pool())
                                    .await?;
                                }
                            }

                            info!(
                                "Updated {} trending keywords from AI analysis",
                                trending_data_len
                            );
                        }
                    }
                }
            }
            Err(e) => {
                warn!("Failed to get trending keywords from AI: {}", e);
            }
        }

        Ok(())
    }

    async fn update_market_demand_data(
        database: &Database,
        ollama_client: &OllamaClient,
    ) -> Result<()> {
        // Get current trending keywords to analyze market demand
        let trending_keywords = sqlx::query(
            "
            SELECT keyword FROM trending_keywords 
            WHERE trend_score > 0.7 
            ORDER BY trend_score DESC 
            LIMIT 20
        ",
        )
        .fetch_all(database.get_pool())
        .await?;

        for row in trending_keywords {
            let keyword: String = row.get("keyword");

            let prompt = format!(
                r#"Analyze the current job market demand for the skill: "{}"

Provide market analysis data in JSON format:
{{
  "skill": "{}",
  "demand_score": 0.85,
  "supply_score": 0.65,
  "competition_ratio": 1.3,
  "job_postings_estimate": 15000,
  "salary_trends": {{
    "current_average": 95000,
    "six_month_growth": 0.08,
    "yearly_growth": 0.15
  }},
  "geographic_demand": {{
    "San Francisco": 0.95,
    "New York": 0.88,
    "Seattle": 0.92,
    "Austin": 0.78,
    "Remote": 0.85
  }},
  "industry_demand": {{
    "technology": 0.95,
    "finance": 0.72,
    "healthcare": 0.65,
    "startup": 0.88
  }},
  "growth_projection": {{
    "next_quarter": 0.05,
    "next_year": 0.18,
    "three_year": 0.45,
    "confidence_interval": [0.35, 0.55]
  }}
}}

Focus on realistic market data for 2024-2025."#,
                keyword, keyword
            );

            if let Ok((response, _)) = ollama_client
                .generate_response("qwen2.5:14b", &prompt, None)
                .await
            {
                if let Some(json_start) = response.find('{') {
                    if let Some(json_end) = response.rfind('}') {
                        let json_str = &response[json_start..=json_end];

                        if let Ok(market_data) = serde_json::from_str::<serde_json::Value>(json_str)
                        {
                            sqlx::query(
                                "
                                INSERT OR REPLACE INTO market_demand_data 
                                (id, skill, demand_score, supply_score, competition_ratio, 
                                 salary_trends, job_postings_count, geographic_demand, 
                                 industry_demand, growth_projection, last_updated)
                                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                            ",
                            )
                            .bind(uuid::Uuid::new_v4().to_string())
                            .bind(&keyword)
                            .bind(market_data["demand_score"].as_f64().unwrap_or(0.5))
                            .bind(market_data["supply_score"].as_f64().unwrap_or(0.5))
                            .bind(market_data["competition_ratio"].as_f64().unwrap_or(1.0))
                            .bind(market_data["salary_trends"].to_string())
                            .bind(
                                market_data["job_postings_estimate"]
                                    .as_u64()
                                    .unwrap_or(1000) as i64,
                            )
                            .bind(market_data["geographic_demand"].to_string())
                            .bind(market_data["industry_demand"].to_string())
                            .bind(market_data["growth_projection"].to_string())
                            .bind(Utc::now().to_rfc3339())
                            .execute(database.get_pool())
                            .await?;
                        }
                    }
                }
            }
        }

        info!("Updated market demand data for trending keywords");
        Ok(())
    }

    async fn update_skill_relationships(
        database: &Database,
        ollama_client: &OllamaClient,
    ) -> Result<()> {
        // Get high-demand skills to analyze relationships
        let skills = sqlx::query(
            "
            SELECT skill FROM market_demand_data 
            WHERE demand_score > 0.7 
            ORDER BY demand_score DESC 
            LIMIT 15
        ",
        )
        .fetch_all(database.get_pool())
        .await?;

        for row in skills {
            let skill: String = row.get("skill");

            let prompt = format!(
                r#"Analyze skill relationships and learning pathways for: "{}"

Provide comprehensive relationship data in JSON format:
{{
  "primary_skill": "{}",
  "prerequisites": [
    {{"skill": "javascript", "weight": 0.9, "confidence": 0.95}},
    {{"skill": "html", "weight": 0.8, "confidence": 0.9}}
  ],
  "complementary_skills": [
    {{"skill": "redux", "weight": 0.85, "confidence": 0.8}},
    {{"skill": "typescript", "weight": 0.9, "confidence": 0.85}}
  ],
  "alternative_skills": [
    {{"skill": "vue", "weight": 0.7, "confidence": 0.75}},
    {{"skill": "angular", "weight": 0.6, "confidence": 0.7}}
  ],
  "career_progressions": [
    {{"skill": "react native", "weight": 0.8, "confidence": 0.8}},
    {{"skill": "next.js", "weight": 0.9, "confidence": 0.85}}
  ],
  "learning_pathways": [
    {{
      "pathway_name": "Frontend Mastery",
      "steps": [
        {{
          "skill": "html css basics",
          "order": 1,
          "prerequisites": [],
          "resources": [
            {{
              "resource_type": "Course",
              "title": "HTML & CSS Fundamentals",
              "provider": "FreeCodeCamp",
              "cost": "Free",
              "duration": "2 weeks",
              "rating": 4.5,
              "completion_rate": 0.85
            }}
          ]
        }}
      ],
      "estimated_duration": "3-4 months",
      "difficulty": 0.7,
      "success_rate": 0.8
    }}
  ]
}}

Focus on practical, current skill relationships for 2024-2025."#,
                skill, skill
            );

            if let Ok((response, _)) = ollama_client
                .generate_response("qwen2.5:14b", &prompt, None)
                .await
            {
                if let Some(json_start) = response.find('{') {
                    if let Some(json_end) = response.rfind('}') {
                        let json_str = &response[json_start..=json_end];

                        if let Ok(relationship_data) =
                            serde_json::from_str::<serde_json::Value>(json_str)
                        {
                            sqlx::query(
                                "
                                INSERT OR REPLACE INTO skill_relationships 
                                (id, primary_skill, prerequisites, complementary_skills, 
                                 alternative_skills, career_progressions, co_occurrence_frequency, 
                                 learning_pathways, last_updated)
                                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                            ",
                            )
                            .bind(uuid::Uuid::new_v4().to_string())
                            .bind(&skill)
                            .bind(relationship_data["prerequisites"].to_string())
                            .bind(relationship_data["complementary_skills"].to_string())
                            .bind(relationship_data["alternative_skills"].to_string())
                            .bind(relationship_data["career_progressions"].to_string())
                            .bind("{}".to_string()) // Empty co-occurrence for now
                            .bind(relationship_data["learning_pathways"].to_string())
                            .bind(Utc::now().to_rfc3339())
                            .execute(database.get_pool())
                            .await?;
                        }
                    }
                }
            }
        }

        info!("Updated skill relationships for high-demand skills");
        Ok(())
    }

    async fn cleanup_outdated_data(database: &Database) -> Result<()> {
        let cutoff_date = (Utc::now() - chrono::Duration::days(30)).to_rfc3339();

        // Remove outdated dynamic keywords with low confidence
        let removed_keywords_result = sqlx::query(
            "
            DELETE FROM dynamic_keywords 
            WHERE last_updated < ? AND confidence_score < 0.4
        ",
        )
        .bind(&cutoff_date)
        .execute(database.get_pool())
        .await?;

        // Remove outdated trending keywords with low trend scores
        let removed_trending_result = sqlx::query(
            "
            DELETE FROM trending_keywords 
            WHERE last_updated < ? AND trend_score < 0.3
        ",
        )
        .bind(&cutoff_date)
        .execute(database.get_pool())
        .await?;

        info!(
            "Cleaned up {} outdated keywords and {} outdated trending keywords",
            removed_keywords_result.rows_affected(),
            removed_trending_result.rows_affected()
        );

        Ok(())
    }

    /// Get keywords for a specific industry with real-time updates
    pub async fn get_industry_keywords(&mut self, industry: &str) -> Result<Vec<DynamicKeyword>> {
        // Check if we need to refresh data
        if self.should_refresh_data() {
            self.refresh_industry_data(industry).await?;
        }

        Ok(self
            .industry_keywords
            .get(industry)
            .cloned()
            .unwrap_or_default())
    }

    /// Get trending keywords across all industries
    pub fn get_trending_keywords(&self, limit: Option<usize>) -> Vec<&TrendingKeywordData> {
        let mut trending: Vec<_> = self.trending_keywords.values().collect();
        trending.sort_by(|a, b| {
            b.trend_score
                .partial_cmp(&a.trend_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if let Some(limit) = limit {
            trending.truncate(limit);
        }

        trending
    }

    /// Get market demand data for a specific skill
    pub async fn get_market_demand(&self, skill: &str) -> Result<Option<MarketDemandData>> {
        let rows = sqlx::query(
            "
            SELECT demand_score, supply_score, competition_ratio, salary_trends, 
                   job_postings_count, geographic_demand, industry_demand, 
                   growth_projection, last_updated
            FROM market_demand_data 
            WHERE skill = ?
        ",
        )
        .bind(skill)
        .fetch_all(self.database.get_pool())
        .await?;

        if let Some(row) = rows.first() {
            let market_data = MarketDemandData {
                skill: skill.to_string(),
                demand_score: row.get("demand_score"),
                supply_score: row.get("supply_score"),
                competition_ratio: row.get("competition_ratio"),
                salary_trends: serde_json::from_str::<SalaryTrendData>(
                    &row.get::<String, _>("salary_trends"),
                )
                .unwrap_or(SalaryTrendData {
                    current_average: 75000.0,
                    six_month_growth: 0.05,
                    yearly_growth: 0.1,
                    geographic_variance: HashMap::new(),
                    experience_multipliers: HashMap::new(),
                }),
                job_postings_count: row.get::<i64, _>("job_postings_count") as u64,
                geographic_demand: serde_json::from_str::<HashMap<String, f64>>(
                    &row.get::<String, _>("geographic_demand"),
                )
                .unwrap_or_default(),
                industry_demand: serde_json::from_str::<HashMap<String, f64>>(
                    &row.get::<String, _>("industry_demand"),
                )
                .unwrap_or_default(),
                growth_projection: serde_json::from_str::<GrowthProjection>(
                    &row.get::<String, _>("growth_projection"),
                )
                .unwrap_or(GrowthProjection {
                    next_quarter: 0.02,
                    next_year: 0.08,
                    three_year: 0.25,
                    confidence_interval: (0.15, 0.35),
                    key_drivers: vec!["market demand".to_string()],
                }),
            };

            Ok(Some(market_data))
        } else {
            Ok(None)
        }
    }

    /// Add user feedback to improve keyword accuracy
    pub async fn add_user_feedback(
        &mut self,
        keyword: &str,
        industry: &str,
        feedback: UserFeedback,
    ) -> Result<()> {
        info!(
            "Adding user feedback for keyword '{}' in industry '{}'",
            keyword, industry
        );

        // Update confidence scores based on feedback
        let confidence_adjustment = match feedback.rating {
            5 => 0.1,
            4 => 0.05,
            3 => 0.0,
            2 => -0.05,
            1 => -0.1,
            _ => 0.0,
        };

        sqlx::query(
            "
            UPDATE dynamic_keywords 
            SET confidence_score = MIN(1.0, MAX(0.0, confidence_score + ?)),
                last_updated = ?
            WHERE keyword = ? AND industry = ?
        ",
        )
        .bind(confidence_adjustment)
        .bind(Utc::now().to_rfc3339())
        .bind(keyword)
        .bind(industry)
        .execute(self.database.get_pool())
        .await?;

        // Reload cached data to reflect changes
        self.load_cached_data().await?;

        Ok(())
    }

    fn should_refresh_data(&self) -> bool {
        let hours_since_update = (Utc::now() - self.last_full_update).num_hours();
        hours_since_update >= self.update_interval_hours as i64
    }

    async fn refresh_industry_data(&mut self, industry: &str) -> Result<()> {
        info!("Refreshing keyword data for industry: {}", industry);

        // Trigger background update for specific industry
        Self::perform_background_update(&self.database, &self.ollama_client).await?;

        // Reload cached data
        self.load_cached_data().await?;

        self.last_full_update = Utc::now();

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFeedback {
    pub keyword: String,
    pub industry: String,
    pub rating: u8, // 1-5 scale
    pub comment: Option<String>,
    pub context: Option<String>, // Where this keyword was found
}

impl DynamicKeywordDatabase {
    /// Creates a new DynamicKeywordDatabase instance with default configuration
    /// Note: This replaces the Default trait implementation to avoid blocking operations
    #[allow(dead_code)]
    pub fn new_default() -> Result<Self> {
        // Use the proper async constructor instead of blocking operations
        Err(anyhow::anyhow!(
            "Use DynamicKeywordDatabase::new() instead of default construction to avoid blocking operations"
        ))
    }
}

// Remove Default implementation to prevent blocking operations
// Note: If Default is required elsewhere, use new_default() and handle the error appropriately

// Helper function to be used by the modern keyword extractor
impl DynamicKeywordDatabase {
    pub async fn enhance_extraction_with_live_data(
        &self,
        extracted_keywords: &mut [crate::modern_keyword_extractor::ModernKeywordMatch],
        industry: &str,
    ) -> Result<()> {
        for keyword_match in extracted_keywords.iter_mut() {
            // Enhance with trending data
            if let Some(trending) = self.trending_keywords.get(&keyword_match.keyword) {
                // Boost confidence for trending keywords
                let trend_boost = trending.trend_score * 0.1;
                keyword_match.confidence_score =
                    (keyword_match.confidence_score + trend_boost).min(1.0);

                // Add adoption stage information
                match trending.adoption_stage {
                    AdoptionStage::Emerging => keyword_match.weight *= 1.1,
                    AdoptionStage::Growing => keyword_match.weight *= 1.2,
                    AdoptionStage::Mainstream => keyword_match.weight *= 1.0,
                    AdoptionStage::Mature => keyword_match.weight *= 0.9,
                    AdoptionStage::Declining => keyword_match.weight *= 0.7,
                }
            }

            // Enhance with market demand data
            if let Ok(Some(market_data)) = self.get_market_demand(&keyword_match.keyword).await {
                // Adjust industry relevance based on market demand
                let industry_demand = market_data
                    .industry_demand
                    .get(industry)
                    .copied()
                    .unwrap_or(0.5);
                keyword_match.industry_relevance =
                    (keyword_match.industry_relevance + industry_demand) / 2.0;

                // Add salary impact information (convert to normalized scale)
                let salary_impact = (market_data.salary_trends.current_average / 150000.0).min(1.0);
                keyword_match.weight = (keyword_match.weight + salary_impact) / 2.0;
            }
        }

        Ok(())
    }
}
