use anyhow::{Context, Result};
use chrono::Utc;
use log::info;
use sqlx::{Row, SqlitePool};

use crate::models::{
    ATSCompatibilityRule, Analysis, IndustryKeyword, ModelPerformanceMetrics, Resume,
    ScoringBenchmark, UserFeedback, UserPreferences, UserPreferencesUpdate,
};

#[derive(Debug, Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self> {
        // Use persistent database file with absolute path
        let current_dir = std::env::current_dir().context("Failed to get current directory")?;
        let data_dir = current_dir.join("data");
        let db_path = data_dir.join("ats_scanner.db");
        let database_url = format!("sqlite:{}", db_path.to_string_lossy());

        info!("Using persistent database: {}", database_url);
        info!("Current directory: {:?}", current_dir);
        info!("Data directory: {:?}", data_dir);

        // Ensure data directory exists
        std::fs::create_dir_all(&data_dir).context("Failed to create data directory")?;

        let pool = SqlitePool::connect(&database_url).await?;

        let db = Database { pool };
        db.run_migrations().await?;
        db.seed_initial_data().await?;

        info!("Database initialized successfully");
        Ok(db)
    }

    pub async fn new_with_url(database_url: &str) -> Result<Self> {
        info!("Connecting to database: {}", database_url);

        // If it's a file-based SQLite database, ensure the parent directory exists
        if database_url.starts_with("sqlite:") && !database_url.contains(":memory:") {
            let db_path = database_url.strip_prefix("sqlite:").unwrap_or(database_url);
            if let Some(parent) = std::path::Path::new(db_path).parent() {
                tokio::fs::create_dir_all(parent).await?;
                info!("Created database directory: {:?}", parent);
            }
        }

        let pool = SqlitePool::connect(database_url).await?;

        let db = Database { pool };
        db.run_migrations().await?;
        db.seed_initial_data().await?;

        info!("Database initialized successfully");
        Ok(db)
    }

    async fn run_migrations(&self) -> Result<()> {
        info!("Running database migrations");

        // Create resumes table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS resumes (
                id TEXT PRIMARY KEY,
                filename TEXT NOT NULL,
                content TEXT NOT NULL,
                file_type TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create analyses table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS analyses (
                id TEXT PRIMARY KEY,
                resume_id TEXT NOT NULL,
                job_description_id TEXT NOT NULL,
                model_used TEXT NOT NULL,
                overall_score REAL NOT NULL,
                skills_score REAL NOT NULL,
                experience_score REAL NOT NULL,
                education_score REAL NOT NULL,
                keywords_score REAL NOT NULL,
                format_score REAL NOT NULL,
                detailed_feedback TEXT NOT NULL,
                missing_keywords TEXT NOT NULL,
                recommendations TEXT NOT NULL,
                processing_time_ms INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (resume_id) REFERENCES resumes (id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create indexes for better performance
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_analyses_resume_id ON analyses(resume_id)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_analyses_created_at ON analyses(created_at)")
            .execute(&self.pool)
            .await?;

        // Create user_preferences table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS user_preferences (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                
                -- Ollama Settings
                ollama_host TEXT NOT NULL DEFAULT 'http://localhost',
                ollama_port INTEGER NOT NULL DEFAULT 11434,
                default_model TEXT,
                connection_timeout_seconds INTEGER NOT NULL DEFAULT 30,
                auto_connect_on_startup BOOLEAN NOT NULL DEFAULT TRUE,
                
                -- Analysis Settings
                default_optimization_level TEXT NOT NULL DEFAULT 'Balanced',
                auto_save_analyses BOOLEAN NOT NULL DEFAULT TRUE,
                analysis_history_retention_days INTEGER NOT NULL DEFAULT 90,
                
                -- UI Preferences
                theme TEXT NOT NULL DEFAULT 'Light',
                language TEXT NOT NULL DEFAULT 'en',
                sidebar_collapsed BOOLEAN NOT NULL DEFAULT FALSE,
                show_advanced_features BOOLEAN NOT NULL DEFAULT FALSE,
                animation_speed TEXT NOT NULL DEFAULT 'Normal',
                
                -- Data & Privacy
                data_storage_location TEXT,
                auto_backup_enabled BOOLEAN NOT NULL DEFAULT FALSE,
                backup_frequency_hours INTEGER NOT NULL DEFAULT 24,
                telemetry_enabled BOOLEAN NOT NULL DEFAULT FALSE,
                
                -- Notifications
                desktop_notifications BOOLEAN NOT NULL DEFAULT TRUE,
                sound_notifications BOOLEAN NOT NULL DEFAULT FALSE,
                email_notifications BOOLEAN NOT NULL DEFAULT FALSE,
                notification_email TEXT,
                
                -- Performance
                max_concurrent_analyses INTEGER NOT NULL DEFAULT 3,
                cache_size_mb INTEGER NOT NULL DEFAULT 256,
                enable_gpu_acceleration BOOLEAN NOT NULL DEFAULT FALSE,
                
                -- Export Settings
                default_export_format TEXT NOT NULL DEFAULT 'JSON',
                include_metadata_in_exports BOOLEAN NOT NULL DEFAULT TRUE,
                compress_exports BOOLEAN NOT NULL DEFAULT FALSE,
                
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                
                UNIQUE(user_id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create index for user preferences
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_user_preferences_user_id ON user_preferences(user_id)",
        )
        .execute(&self.pool)
        .await?;

        // === PHASE 1 ENHANCED SCHEMA ===

        // Create industry_keywords table for industry-specific keyword dictionaries
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS industry_keywords (
                id TEXT PRIMARY KEY,
                industry TEXT NOT NULL,
                keyword TEXT NOT NULL,
                weight REAL DEFAULT 1.0,
                category TEXT, -- 'technical', 'soft_skill', 'certification', etc.
                synonyms TEXT, -- JSON array of synonyms
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create ats_compatibility_rules table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS ats_compatibility_rules (
                id TEXT PRIMARY KEY,
                ats_system TEXT NOT NULL, -- 'greenhouse', 'lever', 'workday', etc.
                rule_type TEXT NOT NULL, -- 'format', 'keyword', 'structure'
                rule_description TEXT,
                penalty_weight REAL DEFAULT 0.0,
                detection_pattern TEXT,
                suggestion TEXT,
                severity TEXT,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create scoring_benchmarks table for industry/role benchmarks
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS scoring_benchmarks (
                id TEXT PRIMARY KEY,
                industry TEXT,
                job_level TEXT, -- 'entry', 'mid', 'senior', 'executive'
                experience_years TEXT,
                benchmark_type TEXT,
                score_threshold REAL,
                description TEXT,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create user_feedback table for continuous learning
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS user_feedback (
                id TEXT PRIMARY KEY,
                analysis_id TEXT,
                user_id TEXT,
                feedback_type TEXT, -- 'accuracy', 'suggestions', 'interview_result'
                rating INTEGER, -- 1-5 scale
                comment TEXT,
                helpful_suggestions TEXT, -- JSON array as string
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create model_performance_metrics table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS model_performance_metrics (
                id TEXT PRIMARY KEY,
                model_name TEXT,
                analysis_id TEXT,
                processing_time_ms INTEGER,
                memory_usage_mb REAL,
                accuracy_score REAL,
                user_satisfaction REAL,
                error_count INTEGER DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create indexes for enhanced performance
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_industry_keywords_industry ON industry_keywords(industry)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_industry_keywords_keyword ON industry_keywords(keyword)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_ats_rules_system ON ats_compatibility_rules(ats_system)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_scoring_benchmarks_industry ON scoring_benchmarks(industry)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_user_feedback_analysis_id ON user_feedback(analysis_id)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_model_performance_model_name ON model_performance_metrics(model_name)")
            .execute(&self.pool)
            .await?;

        info!("Database migrations completed successfully");
        Ok(())
    }

    async fn seed_initial_data(&self) -> Result<()> {
        info!("Seeding initial data");

        // Add some basic industry keywords for common industries
        let tech_keywords = vec![
            ("python", "programming_language", 2.0),
            ("javascript", "programming_language", 2.0),
            ("react", "framework", 1.8),
            ("nodejs", "framework", 1.5),
            ("docker", "devops", 1.5),
            ("kubernetes", "devops", 1.8),
            ("aws", "cloud", 2.0),
            ("git", "version_control", 1.5),
        ];

        for (keyword, category, weight) in tech_keywords {
            let _ = sqlx::query(
                "INSERT OR IGNORE INTO industry_keywords (id, industry, keyword, weight, category, synonyms) VALUES (?, ?, ?, ?, ?, ?)"
            )
            .bind(format!("tech-{}", keyword))
            .bind("technology")
            .bind(keyword)
            .bind(weight)
            .bind(category)
            .bind("[]")
            .execute(&self.pool)
            .await;
        }

        info!("Initial data seeded successfully");
        Ok(())
    }

    // Resume operations
    pub async fn save_resume(&self, resume: &Resume) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO resumes (id, filename, content, file_type, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&resume.id)
        .bind(&resume.filename)
        .bind(&resume.content)
        .bind(&resume.file_type)
        .bind(resume.created_at.to_rfc3339())
        .bind(resume.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        info!("Resume saved with ID: {}", resume.id);
        Ok(())
    }

    pub async fn get_resume(&self, id: &str) -> Result<Option<Resume>> {
        let row = sqlx::query(
            "SELECT id, filename, content, file_type, created_at, updated_at FROM resumes WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let resume = Resume {
                id: row.get("id"),
                filename: row.get("filename"),
                content: row.get("content"),
                file_type: row.get("file_type"),
                created_at: row.get::<String, _>("created_at").parse()?,
                updated_at: row.get::<String, _>("updated_at").parse()?,
            };
            Ok(Some(resume))
        } else {
            Ok(None)
        }
    }

    pub async fn get_all_resumes(&self) -> Result<Vec<Resume>> {
        let rows = sqlx::query(
            "SELECT id, filename, content, file_type, created_at, updated_at FROM resumes ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut resumes = Vec::new();
        for row in rows {
            let resume = Resume {
                id: row.get("id"),
                filename: row.get("filename"),
                content: row.get("content"),
                file_type: row.get("file_type"),
                created_at: row.get::<String, _>("created_at").parse()?,
                updated_at: row.get::<String, _>("updated_at").parse()?,
            };
            resumes.push(resume);
        }

        Ok(resumes)
    }

    // Analysis operations
    pub async fn save_analysis(&self, analysis: &Analysis) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO analyses (
                id, resume_id, job_description_id, model_used, overall_score,
                skills_score, experience_score, education_score, keywords_score, format_score,
                detailed_feedback, missing_keywords, recommendations, processing_time_ms, created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&analysis.id)
        .bind(&analysis.resume_id)
        .bind(&analysis.job_description_id)
        .bind(&analysis.model_used)
        .bind(analysis.overall_score)
        .bind(analysis.skills_score)
        .bind(analysis.experience_score)
        .bind(analysis.education_score)
        .bind(analysis.keywords_score)
        .bind(analysis.format_score)
        .bind(&analysis.detailed_feedback)
        .bind(&analysis.missing_keywords)
        .bind(&analysis.recommendations)
        .bind(analysis.processing_time_ms)
        .bind(analysis.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        info!("Analysis saved with ID: {}", analysis.id);
        Ok(())
    }

    pub async fn get_analysis_history(&self, limit: Option<i64>) -> Result<Vec<Analysis>> {
        let query = if let Some(limit) = limit {
            format!(
                "SELECT * FROM analyses ORDER BY created_at DESC LIMIT {}",
                limit
            )
        } else {
            "SELECT * FROM analyses ORDER BY created_at DESC".to_string()
        };

        let rows = sqlx::query(&query).fetch_all(&self.pool).await?;

        let mut analyses = Vec::new();
        for row in rows {
            let analysis = Analysis {
                id: row.get("id"),
                resume_id: row.get("resume_id"),
                job_description_id: row.get("job_description_id"),
                model_used: row.get("model_used"),
                overall_score: row.get("overall_score"),
                skills_score: row.get("skills_score"),
                experience_score: row.get("experience_score"),
                education_score: row.get("education_score"),
                keywords_score: row.get("keywords_score"),
                format_score: row.get("format_score"),
                detailed_feedback: row.get("detailed_feedback"),
                missing_keywords: row.get("missing_keywords"),
                recommendations: row.get("recommendations"),
                processing_time_ms: row.get("processing_time_ms"),
                created_at: row.get::<String, _>("created_at").parse()?,
            };
            analyses.push(analysis);
        }

        Ok(analyses)
    }

    pub async fn get_analyses_by_resume(&self, resume_id: &str) -> Result<Vec<Analysis>> {
        let rows =
            sqlx::query("SELECT * FROM analyses WHERE resume_id = ? ORDER BY created_at DESC")
                .bind(resume_id)
                .fetch_all(&self.pool)
                .await?;

        let mut analyses = Vec::new();
        for row in rows {
            let analysis = Analysis {
                id: row.get("id"),
                resume_id: row.get("resume_id"),
                job_description_id: row.get("job_description_id"),
                model_used: row.get("model_used"),
                overall_score: row.get("overall_score"),
                skills_score: row.get("skills_score"),
                experience_score: row.get("experience_score"),
                education_score: row.get("education_score"),
                keywords_score: row.get("keywords_score"),
                format_score: row.get("format_score"),
                detailed_feedback: row.get("detailed_feedback"),
                missing_keywords: row.get("missing_keywords"),
                recommendations: row.get("recommendations"),
                processing_time_ms: row.get("processing_time_ms"),
                created_at: row.get::<String, _>("created_at").parse()?,
            };
            analyses.push(analysis);
        }

        Ok(analyses)
    }

    pub async fn delete_resume(&self, id: &str) -> Result<()> {
        // First delete associated analyses
        sqlx::query("DELETE FROM analyses WHERE resume_id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        // Then delete the resume
        sqlx::query("DELETE FROM resumes WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        info!("Resume and associated analyses deleted: {}", id);
        Ok(())
    }

    // Health check for testing
    pub async fn health_check(&self) -> Result<bool> {
        let result = sqlx::query("SELECT 1").fetch_one(&self.pool).await?;
        Ok(result.get::<i32, _>(0) == 1)
    }

    pub async fn get_analysis_stats(&self, days: Option<i32>) -> Result<serde_json::Value> {
        let days = days.unwrap_or(30);
        let cutoff_date = (Utc::now() - chrono::Duration::days(days as i64)).to_rfc3339();

        // Total analyses
        let total_count =
            sqlx::query("SELECT COUNT(*) as count FROM analyses WHERE created_at >= ?")
                .bind(&cutoff_date)
                .fetch_one(&self.pool)
                .await?
                .get::<i32, _>("count");

        // Average scores
        let avg_scores = sqlx::query(
            r#"
            SELECT 
                AVG(overall_score) as avg_overall,
                AVG(skills_score) as avg_skills,
                AVG(experience_score) as avg_experience,
                AVG(education_score) as avg_education,
                AVG(keywords_score) as avg_keywords,
                AVG(format_score) as avg_format
            FROM analyses WHERE created_at >= ?
            "#,
        )
        .bind(&cutoff_date)
        .fetch_one(&self.pool)
        .await?;

        // Model usage stats
        let model_stats = sqlx::query(
            "SELECT model_used, COUNT(*) as count FROM analyses WHERE created_at >= ? GROUP BY model_used ORDER BY count DESC"
        )
        .bind(&cutoff_date)
        .fetch_all(&self.pool)
        .await?;

        // Daily analysis counts
        let daily_stats = sqlx::query(
            r#"
            SELECT 
                DATE(created_at) as date,
                COUNT(*) as count,
                AVG(overall_score) as avg_score
            FROM analyses 
            WHERE created_at >= ? 
            GROUP BY DATE(created_at) 
            ORDER BY date DESC
            "#,
        )
        .bind(&cutoff_date)
        .fetch_all(&self.pool)
        .await?;

        let stats = serde_json::json!({
            "total_analyses": total_count,
            "period_days": days,
            "average_scores": {
                "overall": avg_scores.get::<Option<f64>, _>("avg_overall").unwrap_or(0.0),
                "skills": avg_scores.get::<Option<f64>, _>("avg_skills").unwrap_or(0.0),
                "experience": avg_scores.get::<Option<f64>, _>("avg_experience").unwrap_or(0.0),
                "education": avg_scores.get::<Option<f64>, _>("avg_education").unwrap_or(0.0),
                "keywords": avg_scores.get::<Option<f64>, _>("avg_keywords").unwrap_or(0.0),
                "format": avg_scores.get::<Option<f64>, _>("avg_format").unwrap_or(0.0)
            },
            "model_usage": model_stats.iter().map(|row| {
                serde_json::json!({
                    "model": row.get::<String, _>("model_used"),
                    "count": row.get::<i32, _>("count")
                })
            }).collect::<Vec<_>>(),
            "daily_stats": daily_stats.iter().map(|row| {
                serde_json::json!({
                    "date": row.get::<String, _>("date"),
                    "count": row.get::<i32, _>("count"),
                    "avg_score": row.get::<Option<f64>, _>("avg_score").unwrap_or(0.0)
                })
            }).collect::<Vec<_>>()
        });

        Ok(stats)
    }

    pub async fn get_score_distribution(&self) -> Result<serde_json::Value> {
        let distribution = sqlx::query(
            r#"
            SELECT 
                CASE 
                    WHEN overall_score >= 90 THEN 'Excellent'
                    WHEN overall_score >= 80 THEN 'Good'
                    WHEN overall_score >= 70 THEN 'Average'
                    WHEN overall_score >= 60 THEN 'Below Average'
                    ELSE 'Poor'
                END as score_range,
                COUNT(*) as count
            FROM analyses 
            GROUP BY score_range
            ORDER BY 
                CASE score_range
                    WHEN 'Excellent' THEN 5
                    WHEN 'Good' THEN 4
                    WHEN 'Average' THEN 3
                    WHEN 'Below Average' THEN 2
                    ELSE 1
                END DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let result = serde_json::json!(distribution
            .iter()
            .map(|row| {
                serde_json::json!({
                    "range": row.get::<String, _>("score_range"),
                    "count": row.get::<i32, _>("count")
                })
            })
            .collect::<Vec<_>>());

        Ok(result)
    }

    pub async fn get_improvement_trends(&self) -> Result<serde_json::Value> {
        let trends = sqlx::query(
            r#"
            SELECT 
                r.id as resume_id,
                r.filename,
                COUNT(a.id) as analysis_count,
                MIN(a.overall_score) as first_score,
                MAX(a.overall_score) as latest_score,
                MAX(a.overall_score) - MIN(a.overall_score) as improvement
            FROM resumes r
            JOIN analyses a ON r.id = a.resume_id
            GROUP BY r.id, r.filename
            HAVING COUNT(a.id) > 1
            ORDER BY improvement DESC
            LIMIT 10
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let result = serde_json::json!(trends
            .iter()
            .map(|row| {
                serde_json::json!({
                    "resume_id": row.get::<String, _>("resume_id"),
                    "filename": row.get::<String, _>("filename"),
                    "analysis_count": row.get::<i32, _>("analysis_count"),
                    "first_score": row.get::<f64, _>("first_score"),
                    "latest_score": row.get::<f64, _>("latest_score"),
                    "improvement": row.get::<f64, _>("improvement")
                })
            })
            .collect::<Vec<_>>());

        Ok(result)
    }

    // User Preferences operations
    pub async fn get_user_preferences(&self, user_id: &str) -> Result<Option<UserPreferences>> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, ollama_host, ollama_port, default_model, connection_timeout_seconds,
                   auto_connect_on_startup, default_optimization_level, auto_save_analyses,
                   analysis_history_retention_days, theme, language,
                   sidebar_collapsed, show_advanced_features, animation_speed, data_storage_location,
                   auto_backup_enabled, backup_frequency_hours, telemetry_enabled,
                   desktop_notifications, sound_notifications, email_notifications, notification_email,
                   max_concurrent_analyses, cache_size_mb, enable_gpu_acceleration, default_export_format,
                   include_metadata_in_exports, compress_exports, created_at, updated_at
            FROM user_preferences WHERE user_id = ?
            "#
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let preferences = UserPreferences {
                id: row.get("id"),
                user_id: row.get("user_id"),
                ollama_host: row.get("ollama_host"),
                ollama_port: row.get("ollama_port"),
                default_model: row.get("default_model"),
                connection_timeout_seconds: row.get("connection_timeout_seconds"),
                auto_connect_on_startup: row.get("auto_connect_on_startup"),
                default_optimization_level: serde_json::from_str(
                    &row.get::<String, _>("default_optimization_level"),
                )
                .unwrap_or(crate::models::OptimizationLevel::Balanced),
                auto_save_analyses: row.get("auto_save_analyses"),
                analysis_history_retention_days: row.get("analysis_history_retention_days"),
                theme: serde_json::from_str(&row.get::<String, _>("theme"))
                    .unwrap_or(crate::models::ThemePreference::Light),
                language: row.get("language"),
                sidebar_collapsed: row.get("sidebar_collapsed"),
                show_advanced_features: row.get("show_advanced_features"),
                animation_speed: serde_json::from_str(&row.get::<String, _>("animation_speed"))
                    .unwrap_or(crate::models::AnimationSpeed::Normal),
                data_storage_location: row.get("data_storage_location"),
                auto_backup_enabled: row.get("auto_backup_enabled"),
                backup_frequency_hours: row.get("backup_frequency_hours"),
                telemetry_enabled: row.get("telemetry_enabled"),
                desktop_notifications: row.get("desktop_notifications"),
                sound_notifications: row.get("sound_notifications"),
                email_notifications: row.get("email_notifications"),
                notification_email: row.get("notification_email"),
                max_concurrent_analyses: row.get("max_concurrent_analyses"),
                cache_size_mb: row.get("cache_size_mb"),
                enable_gpu_acceleration: row.get("enable_gpu_acceleration"),
                default_export_format: serde_json::from_str(
                    &row.get::<String, _>("default_export_format"),
                )
                .unwrap_or(crate::models::ExportFormat::Json),
                include_metadata_in_exports: row.get("include_metadata_in_exports"),
                compress_exports: row.get("compress_exports"),
                created_at: row.get::<String, _>("created_at").parse()?,
                updated_at: row.get::<String, _>("updated_at").parse()?,
            };
            Ok(Some(preferences))
        } else {
            Ok(None)
        }
    }

    pub async fn save_user_preferences(&self, preferences: &UserPreferences) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO user_preferences (
                id, user_id, ollama_host, ollama_port, default_model, connection_timeout_seconds,
                auto_connect_on_startup, default_optimization_level, auto_save_analyses,
                analysis_history_retention_days, theme, language,
                sidebar_collapsed, show_advanced_features, animation_speed, data_storage_location,
                auto_backup_enabled, backup_frequency_hours, telemetry_enabled,
                desktop_notifications, sound_notifications, email_notifications, notification_email,
                max_concurrent_analyses, cache_size_mb, enable_gpu_acceleration, default_export_format,
                include_metadata_in_exports, compress_exports, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&preferences.id)
        .bind(&preferences.user_id)
        .bind(&preferences.ollama_host)
        .bind(preferences.ollama_port)
        .bind(&preferences.default_model)
        .bind(preferences.connection_timeout_seconds)
        .bind(preferences.auto_connect_on_startup)
        .bind(serde_json::to_string(&preferences.default_optimization_level).unwrap_or_default())
        .bind(preferences.auto_save_analyses)
        .bind(preferences.analysis_history_retention_days)
        .bind(serde_json::to_string(&preferences.theme).unwrap_or_default())
        .bind(&preferences.language)
        .bind(preferences.sidebar_collapsed)
        .bind(preferences.show_advanced_features)
        .bind(serde_json::to_string(&preferences.animation_speed).unwrap_or_default())
        .bind(&preferences.data_storage_location)
        .bind(preferences.auto_backup_enabled)
        .bind(preferences.backup_frequency_hours)
        .bind(preferences.telemetry_enabled)
        .bind(preferences.desktop_notifications)
        .bind(preferences.sound_notifications)
        .bind(preferences.email_notifications)
        .bind(&preferences.notification_email)
        .bind(preferences.max_concurrent_analyses)
        .bind(preferences.cache_size_mb)
        .bind(preferences.enable_gpu_acceleration)
        .bind(serde_json::to_string(&preferences.default_export_format).unwrap_or_default())
        .bind(preferences.include_metadata_in_exports)
        .bind(preferences.compress_exports)
        .bind(preferences.created_at.to_rfc3339())
        .bind(preferences.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        info!("User preferences saved for user: {}", preferences.user_id);
        Ok(())
    }

    pub async fn update_user_preferences(
        &self,
        user_id: &str,
        updates: &UserPreferencesUpdate,
    ) -> Result<()> {
        let current = self.get_user_preferences(user_id).await?;
        let mut preferences = current.unwrap_or_else(UserPreferences::default);

        // Update only provided fields
        if let Some(host) = &updates.ollama_host {
            preferences.ollama_host = host.clone();
        }
        if let Some(port) = updates.ollama_port {
            preferences.ollama_port = port;
        }
        if let Some(model) = &updates.default_model {
            preferences.default_model = Some(model.clone());
        }
        if let Some(timeout) = updates.connection_timeout_seconds {
            preferences.connection_timeout_seconds = timeout;
        }
        if let Some(auto_connect) = updates.auto_connect_on_startup {
            preferences.auto_connect_on_startup = auto_connect;
        }
        if let Some(opt_level) = &updates.default_optimization_level {
            preferences.default_optimization_level = opt_level.clone();
        }
        if let Some(auto_save) = updates.auto_save_analyses {
            preferences.auto_save_analyses = auto_save;
        }
        if let Some(retention) = updates.analysis_history_retention_days {
            preferences.analysis_history_retention_days = retention;
        }
        if let Some(theme) = &updates.theme {
            preferences.theme = theme.clone();
        }
        if let Some(lang) = &updates.language {
            preferences.language = lang.clone();
        }
        if let Some(collapsed) = updates.sidebar_collapsed {
            preferences.sidebar_collapsed = collapsed;
        }
        if let Some(advanced) = updates.show_advanced_features {
            preferences.show_advanced_features = advanced;
        }
        if let Some(anim_speed) = &updates.animation_speed {
            preferences.animation_speed = anim_speed.clone();
        }
        if let Some(storage_loc) = &updates.data_storage_location {
            preferences.data_storage_location = Some(storage_loc.clone());
        }
        if let Some(backup) = updates.auto_backup_enabled {
            preferences.auto_backup_enabled = backup;
        }
        if let Some(freq) = updates.backup_frequency_hours {
            preferences.backup_frequency_hours = freq;
        }
        if let Some(telemetry) = updates.telemetry_enabled {
            preferences.telemetry_enabled = telemetry;
        }
        if let Some(desktop) = updates.desktop_notifications {
            preferences.desktop_notifications = desktop;
        }
        if let Some(sound) = updates.sound_notifications {
            preferences.sound_notifications = sound;
        }
        if let Some(email) = updates.email_notifications {
            preferences.email_notifications = email;
        }
        if let Some(email_addr) = &updates.notification_email {
            preferences.notification_email = Some(email_addr.clone());
        }
        if let Some(max_concurrent) = updates.max_concurrent_analyses {
            preferences.max_concurrent_analyses = max_concurrent;
        }
        if let Some(cache_size) = updates.cache_size_mb {
            preferences.cache_size_mb = cache_size;
        }
        if let Some(gpu) = updates.enable_gpu_acceleration {
            preferences.enable_gpu_acceleration = gpu;
        }
        if let Some(export_format) = &updates.default_export_format {
            preferences.default_export_format = export_format.clone();
        }
        if let Some(metadata) = updates.include_metadata_in_exports {
            preferences.include_metadata_in_exports = metadata;
        }
        if let Some(compress) = updates.compress_exports {
            preferences.compress_exports = compress;
        }

        preferences.updated_at = Utc::now();
        self.save_user_preferences(&preferences).await?;
        Ok(())
    }

    pub async fn get_or_create_user_preferences(&self, user_id: &str) -> Result<UserPreferences> {
        if let Some(preferences) = self.get_user_preferences(user_id).await? {
            Ok(preferences)
        } else {
            let preferences = UserPreferences {
                user_id: user_id.to_string(),
                ..Default::default()
            };
            self.save_user_preferences(&preferences).await?;
            Ok(preferences)
        }
    }

    // === PHASE 1 ENHANCED DATABASE METHODS ===

    // Industry Keywords operations
    pub async fn save_industry_keyword(&self, keyword: &IndustryKeyword) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO industry_keywords (
                id, industry, keyword, weight, category, synonyms, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&keyword.id)
        .bind(&keyword.industry)
        .bind(&keyword.keyword)
        .bind(keyword.weight)
        .bind(&keyword.category)
        .bind(&keyword.synonyms)
        .bind(keyword.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        info!(
            "Industry keyword saved: {} for {}",
            keyword.keyword, keyword.industry
        );
        Ok(())
    }

    pub async fn get_industry_keywords(&self, industry: &str) -> Result<Vec<IndustryKeyword>> {
        let rows =
            sqlx::query("SELECT * FROM industry_keywords WHERE industry = ? ORDER BY weight DESC")
                .bind(industry)
                .fetch_all(&self.pool)
                .await?;

        let mut keywords = Vec::new();
        for row in rows {
            let keyword = IndustryKeyword {
                id: row.get("id"),
                industry: row.get("industry"),
                keyword: row.get("keyword"),
                weight: row.get("weight"),
                category: row.get("category"),
                synonyms: row.get("synonyms"),
                created_at: row.get::<String, _>("created_at").parse()?,
            };
            keywords.push(keyword);
        }

        Ok(keywords)
    }

    pub async fn get_all_industries(&self) -> Result<Vec<String>> {
        let rows = sqlx::query("SELECT DISTINCT industry FROM industry_keywords ORDER BY industry")
            .fetch_all(&self.pool)
            .await?;

        let industries = rows
            .iter()
            .map(|row| row.get::<String, _>("industry"))
            .collect();

        Ok(industries)
    }

    // ATS Compatibility Rules operations
    pub async fn save_ats_rule(&self, rule: &ATSCompatibilityRule) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO ats_compatibility_rules (
                id, ats_system, rule_type, rule_description, penalty_weight, 
                detection_pattern, suggestion, severity, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&rule.id)
        .bind(&rule.ats_system)
        .bind(&rule.rule_type)
        .bind(&rule.rule_description)
        .bind(rule.penalty_weight)
        .bind(&rule.detection_pattern)
        .bind(&rule.suggestion)
        .bind(&rule.severity)
        .bind(rule.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        info!("ATS rule saved: {} for {}", rule.rule_type, rule.ats_system);
        Ok(())
    }

    pub async fn get_ats_rules(
        &self,
        ats_system: Option<&str>,
    ) -> Result<Vec<ATSCompatibilityRule>> {
        let query = if let Some(system) = ats_system {
            sqlx::query("SELECT * FROM ats_compatibility_rules WHERE ats_system = ? ORDER BY penalty_weight DESC")
                .bind(system)
        } else {
            sqlx::query(
                "SELECT * FROM ats_compatibility_rules ORDER BY ats_system, penalty_weight DESC",
            )
        };

        let rows = query.fetch_all(&self.pool).await?;

        let mut rules = Vec::new();
        for row in rows {
            let rule = ATSCompatibilityRule {
                id: row.get("id"),
                ats_system: row.get("ats_system"),
                rule_type: row.get("rule_type"),
                rule_description: row.get("rule_description"),
                penalty_weight: row.get("penalty_weight"),
                detection_pattern: row.get("detection_pattern"),
                suggestion: row.get("suggestion"),
                severity: row.get("severity"),
                created_at: row.get::<String, _>("created_at").parse()?,
            };
            rules.push(rule);
        }

        Ok(rules)
    }

    // Scoring Benchmarks operations
    pub async fn save_scoring_benchmark(&self, benchmark: &ScoringBenchmark) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO scoring_benchmarks (
                id, industry, job_level, experience_years, benchmark_type, 
                score_threshold, description, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&benchmark.id)
        .bind(&benchmark.industry)
        .bind(&benchmark.job_level)
        .bind(&benchmark.experience_years)
        .bind(&benchmark.benchmark_type)
        .bind(benchmark.score_threshold)
        .bind(&benchmark.description)
        .bind(benchmark.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        info!(
            "Scoring benchmark saved: {} for {} level",
            benchmark.benchmark_type, benchmark.job_level
        );
        Ok(())
    }

    pub async fn get_scoring_benchmarks(
        &self,
        industry: &str,
        job_level: &str,
    ) -> Result<Vec<ScoringBenchmark>> {
        let rows = sqlx::query(
            "SELECT * FROM scoring_benchmarks WHERE industry = ? AND job_level = ? ORDER BY benchmark_type"
        )
        .bind(industry)
        .bind(job_level)
        .fetch_all(&self.pool)
        .await?;

        let mut benchmarks = Vec::new();
        for row in rows {
            let benchmark = ScoringBenchmark {
                id: row.get("id"),
                industry: row.get("industry"),
                job_level: row.get("job_level"),
                experience_years: row.get("experience_years"),
                benchmark_type: row.get("benchmark_type"),
                score_threshold: row.get("score_threshold"),
                description: row.get("description"),
                created_at: row.get::<String, _>("created_at").parse()?,
            };
            benchmarks.push(benchmark);
        }

        Ok(benchmarks)
    }

    // User Feedback operations
    pub async fn save_user_feedback(&self, feedback: &UserFeedback) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO user_feedback (
                id, analysis_id, user_id, feedback_type, rating, comment, 
                helpful_suggestions, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&feedback.id)
        .bind(&feedback.analysis_id)
        .bind(&feedback.user_id)
        .bind(&feedback.feedback_type)
        .bind(feedback.rating)
        .bind(&feedback.comment)
        .bind(&feedback.helpful_suggestions)
        .bind(feedback.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        info!("User feedback saved for analysis: {}", feedback.analysis_id);
        Ok(())
    }

    pub async fn get_feedback_by_analysis(&self, analysis_id: &str) -> Result<Vec<UserFeedback>> {
        let rows = sqlx::query(
            "SELECT * FROM user_feedback WHERE analysis_id = ? ORDER BY created_at DESC",
        )
        .bind(analysis_id)
        .fetch_all(&self.pool)
        .await?;

        let mut feedback_list = Vec::new();
        for row in rows {
            let feedback = UserFeedback {
                id: row.get("id"),
                analysis_id: row.get("analysis_id"),
                user_id: row.get("user_id"),
                feedback_type: row.get("feedback_type"),
                rating: row.get("rating"),
                comment: row.get("comment"),
                helpful_suggestions: row.get("helpful_suggestions"),
                created_at: row.get::<String, _>("created_at").parse()?,
            };
            feedback_list.push(feedback);
        }

        Ok(feedback_list)
    }

    pub async fn get_feedback_stats(&self, days: Option<i32>) -> Result<serde_json::Value> {
        let days = days.unwrap_or(30);
        let cutoff_date = (Utc::now() - chrono::Duration::days(days as i64)).to_rfc3339();

        let stats = sqlx::query(
            r#"
            SELECT 
                AVG(rating) as avg_rating,
                COUNT(*) as total_feedback,
                COUNT(CASE WHEN rating >= 4 THEN 1 END) as positive_feedback,
                COUNT(CASE WHEN rating <= 2 THEN 1 END) as negative_feedback
            FROM user_feedback WHERE created_at >= ?
            "#,
        )
        .bind(&cutoff_date)
        .fetch_one(&self.pool)
        .await?;

        let result = serde_json::json!({
            "avg_rating": stats.get::<Option<f64>, _>("avg_rating").unwrap_or(0.0),
            "total_feedback": stats.get::<i32, _>("total_feedback"),
            "positive_feedback": stats.get::<i32, _>("positive_feedback"),
            "negative_feedback": stats.get::<i32, _>("negative_feedback"),
            "period_days": days
        });

        Ok(result)
    }

    // Model Performance Metrics operations
    pub async fn save_model_performance(&self, metrics: &ModelPerformanceMetrics) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO model_performance_metrics (
                id, model_name, analysis_id, processing_time_ms, memory_usage_mb,
                accuracy_score, user_satisfaction, error_count, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&metrics.id)
        .bind(&metrics.model_name)
        .bind(&metrics.analysis_id)
        .bind(metrics.processing_time_ms)
        .bind(metrics.memory_usage_mb)
        .bind(metrics.accuracy_score)
        .bind(metrics.user_satisfaction)
        .bind(metrics.error_count)
        .bind(metrics.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        info!(
            "Model performance metrics saved for: {}",
            metrics.model_name
        );
        Ok(())
    }

    pub async fn get_model_performance_stats(&self, model_name: &str) -> Result<serde_json::Value> {
        let stats = sqlx::query(
            r#"
            SELECT 
                COUNT(*) as analysis_count,
                AVG(processing_time_ms) as avg_processing_time,
                AVG(memory_usage_mb) as avg_memory_usage,
                AVG(accuracy_score) as avg_accuracy,
                AVG(user_satisfaction) as avg_satisfaction,
                SUM(error_count) as total_errors
            FROM model_performance_metrics WHERE model_name = ?
            "#,
        )
        .bind(model_name)
        .fetch_one(&self.pool)
        .await?;

        let result = serde_json::json!({
            "model_name": model_name,
            "analysis_count": stats.get::<i32, _>("analysis_count"),
            "avg_processing_time_ms": stats.get::<Option<f64>, _>("avg_processing_time").unwrap_or(0.0),
            "avg_memory_usage_mb": stats.get::<Option<f64>, _>("avg_memory_usage").unwrap_or(0.0),
            "avg_accuracy_score": stats.get::<Option<f64>, _>("avg_accuracy").unwrap_or(0.0),
            "avg_user_satisfaction": stats.get::<Option<f64>, _>("avg_satisfaction").unwrap_or(0.0),
            "total_errors": stats.get::<Option<i32>, _>("total_errors").unwrap_or(0)
        });

        Ok(result)
    }

    pub async fn get_all_model_performance(&self) -> Result<Vec<serde_json::Value>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                model_name,
                COUNT(*) as analysis_count,
                AVG(processing_time_ms) as avg_processing_time,
                AVG(memory_usage_mb) as avg_memory_usage,
                AVG(accuracy_score) as avg_accuracy,
                AVG(user_satisfaction) as avg_satisfaction,
                SUM(error_count) as total_errors
            FROM model_performance_metrics 
            GROUP BY model_name
            ORDER BY analysis_count DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let mut results = Vec::new();
        for row in rows {
            let result = serde_json::json!({
                "model_name": row.get::<String, _>("model_name"),
                "analysis_count": row.get::<i32, _>("analysis_count"),
                "avg_processing_time_ms": row.get::<Option<f64>, _>("avg_processing_time").unwrap_or(0.0),
                "avg_memory_usage_mb": row.get::<Option<f64>, _>("avg_memory_usage").unwrap_or(0.0),
                "avg_accuracy_score": row.get::<Option<f64>, _>("avg_accuracy").unwrap_or(0.0),
                "avg_user_satisfaction": row.get::<Option<f64>, _>("avg_satisfaction").unwrap_or(0.0),
                "total_errors": row.get::<Option<i32>, _>("total_errors").unwrap_or(0)
            });
            results.push(result);
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    async fn setup_test_db() -> Database {
        Database::new_with_url("sqlite::memory:").await.unwrap()
    }

    fn create_test_resume() -> Resume {
        Resume {
            id: Uuid::new_v4().to_string(),
            filename: "test_resume.pdf".to_string(),
            content: "Test resume content with skills and experience".to_string(),
            file_type: "pdf".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn create_test_analysis(resume_id: &str) -> Analysis {
        Analysis {
            id: Uuid::new_v4().to_string(),
            resume_id: resume_id.to_string(),
            job_description_id: "test_job_id".to_string(),
            model_used: "test_model".to_string(),
            overall_score: 85.5,
            skills_score: 90.0,
            experience_score: 80.0,
            education_score: 85.0,
            keywords_score: 88.0,
            format_score: 92.0,
            detailed_feedback: "Test feedback".to_string(),
            missing_keywords: "python,react".to_string(),
            recommendations: "Test recommendations".to_string(),
            processing_time_ms: 1500,
            created_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_database_initialization() {
        let db = setup_test_db().await;
        assert!(db.health_check().await.unwrap());
    }

    #[tokio::test]
    async fn test_resume_crud_operations() {
        let db = setup_test_db().await;
        let resume = create_test_resume();

        // Test save
        db.save_resume(&resume).await.unwrap();

        // Test get
        let retrieved = db.get_resume(&resume.id).await.unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.id, resume.id);
        assert_eq!(retrieved.filename, resume.filename);
        assert_eq!(retrieved.content, resume.content);

        // Test get all
        let all_resumes = db.get_all_resumes().await.unwrap();
        assert_eq!(all_resumes.len(), 1);

        // Test delete
        db.delete_resume(&resume.id).await.unwrap();
        let retrieved = db.get_resume(&resume.id).await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_analysis_operations() {
        let db = setup_test_db().await;
        let resume = create_test_resume();
        let analysis = create_test_analysis(&resume.id);

        // Setup dependencies
        db.save_resume(&resume).await.unwrap();

        // Test save analysis
        db.save_analysis(&analysis).await.unwrap();

        // Test get analysis history
        let history = db.get_analysis_history(None).await.unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].id, analysis.id);
        assert_eq!(history[0].overall_score, analysis.overall_score);

        // Test get analyses by resume
        let resume_analyses = db.get_analyses_by_resume(&resume.id).await.unwrap();
        assert_eq!(resume_analyses.len(), 1);
        assert_eq!(resume_analyses[0].resume_id, resume.id);

        // Test limit
        let limited_history = db.get_analysis_history(Some(10)).await.unwrap();
        assert_eq!(limited_history.len(), 1);
    }

    #[tokio::test]
    async fn test_cascading_deletes() {
        let db = setup_test_db().await;
        let resume = create_test_resume();
        let analysis = create_test_analysis(&resume.id);

        // Setup data
        db.save_resume(&resume).await.unwrap();
        db.save_analysis(&analysis).await.unwrap();

        // Verify analysis exists
        let history = db.get_analysis_history(None).await.unwrap();
        assert_eq!(history.len(), 1);

        // Delete resume should cascade to analyses
        db.delete_resume(&resume.id).await.unwrap();
        let history = db.get_analysis_history(None).await.unwrap();
        assert_eq!(history.len(), 0);
    }
}
