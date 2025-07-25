use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use log::{error, info, warn};
use sqlx::{Row, SqlitePool};
use std::path::PathBuf;

use crate::models::{
    ATSCompatibilityRule, Analysis, ApplicationStatus, ApplicationStatusCount, CompanyCount,
    IndustryKeyword, JobAnalytics, JobDescription, JobPriority, JobPriorityCount, JobSearchRequest,
    JobSearchResult, JobSortOption, JobStatus, JobStatusCount, LocationCount,
    ModelPerformanceMetrics, Resume, ScoringBenchmark, SortOrder, UserFeedback, UserPreferences,
    UserPreferencesUpdate,
};

/// Helper function to parse timestamps in multiple formats
fn parse_timestamp(timestamp_str: &str) -> Result<DateTime<Utc>> {
    // Try RFC3339 format first
    if let Ok(dt) = timestamp_str.parse::<DateTime<Utc>>() {
        return Ok(dt);
    }

    // Try SQLite datetime format: "YYYY-MM-DD HH:MM:SS"
    if let Ok(naive_dt) = NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S") {
        return Ok(DateTime::from_naive_utc_and_offset(naive_dt, Utc));
    }

    // Try other common formats
    let formats = [
        "%Y-%m-%dT%H:%M:%S%.fZ", // RFC3339 with microseconds
        "%Y-%m-%dT%H:%M:%SZ",    // RFC3339 without microseconds
        "%Y-%m-%d %H:%M:%S%.f",  // SQLite with microseconds
        "%Y-%m-%dT%H:%M:%S",     // ISO 8601 without timezone
    ];

    for format in &formats {
        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(timestamp_str, format) {
            return Ok(DateTime::from_naive_utc_and_offset(naive_dt, Utc));
        }
    }

    Err(anyhow::anyhow!(
        "Unable to parse timestamp '{}' with any known format",
        timestamp_str
    ))
}

#[derive(Debug, Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    // Create a new Database instance that shares the same connection pool
    pub fn shared_clone(&self) -> Self {
        Database {
            pool: self.pool.clone(),
        }
    }

    pub fn get_pool(&self) -> &SqlitePool {
        &self.pool
    }
    pub async fn new() -> Result<Self> {
        // Use a fallback approach since we don't have access to Tauri app handle here
        // First try current directory approach, then try home directory fallback
        let result = Self::try_current_directory_database().await;

        match result {
            Ok(db) => {
                info!("Database initialized successfully using current directory");
                Ok(db)
            }
            Err(e) => {
                warn!(
                    "Failed to initialize database using current directory: {}",
                    e
                );
                info!("Attempting fallback to home directory");
                Self::try_home_directory_database().await
            }
        }
    }

    async fn try_current_directory_database() -> Result<Self> {
        // Use persistent database file with absolute path
        let current_dir = std::env::current_dir().context("Failed to get current directory")?;
        let data_dir = current_dir.join("data");
        let db_path = data_dir.join("ats_scanner.db");
        let database_url = format!("sqlite:{}", db_path.to_string_lossy());

        info!("Trying current directory database: {}", database_url);
        info!("Current directory: {:?}", current_dir);
        info!("Data directory: {:?}", data_dir);

        // Ensure data directory exists
        std::fs::create_dir_all(&data_dir).context("Failed to create data directory")?;

        // Try to connect with specific SQLite error handling
        let pool = Self::connect_with_error_handling(&database_url, &data_dir, &db_path).await?;

        let db = Database { pool };
        db.run_migrations().await?;
        db.seed_initial_data().await?;

        Ok(db)
    }

    async fn try_home_directory_database() -> Result<Self> {
        // Fallback to home directory approach
        let home_dir = dirs::home_dir().context("Failed to get home directory")?;
        let app_data_dir = home_dir.join(".ats-scanner");
        let db_path = app_data_dir.join("ats_scanner.db");
        let database_url = format!("sqlite:{}", db_path.to_string_lossy());

        info!("Trying home directory database: {}", database_url);
        info!("Home directory: {:?}", home_dir);
        info!("App data directory: {:?}", app_data_dir);

        // Ensure app data directory exists
        std::fs::create_dir_all(&app_data_dir).context("Failed to create app data directory")?;

        // Try to connect with specific SQLite error handling
        let pool =
            Self::connect_with_error_handling(&database_url, &app_data_dir, &db_path).await?;

        let db = Database { pool };
        db.run_migrations().await?;
        db.seed_initial_data().await?;

        info!("Database initialized successfully using home directory fallback");
        Ok(db)
    }

    async fn connect_with_error_handling(
        database_url: &str,
        data_dir: &PathBuf,
        db_path: &PathBuf,
    ) -> Result<SqlitePool> {
        match SqlitePool::connect(database_url).await {
            Ok(pool) => Ok(pool),
            Err(e) => {
                error!("SQLite connection failed: {}", e);

                // Check if this is the specific error code 14
                if e.to_string().contains("code: 14")
                    || e.to_string().contains("unable to open database file")
                {
                    error!("SQLite error code 14 detected: unable to open database file");
                    error!("Database path: {}", database_url);
                    error!("Data directory exists: {}", data_dir.exists());
                    error!("Database file exists: {}", db_path.exists());

                    // Try to provide more diagnostic information
                    if let Ok(metadata) = std::fs::metadata(data_dir) {
                        error!("Data directory permissions: {:?}", metadata.permissions());
                    }

                    if db_path.exists() {
                        if let Ok(metadata) = std::fs::metadata(db_path) {
                            error!("Database file permissions: {:?}", metadata.permissions());
                            error!("Database file size: {}", metadata.len());
                        }
                    }

                    return Err(anyhow::anyhow!(
                        "SQLite error code 14: Unable to open database file at '{}'. This could be due to:\n\
                        1. Permission issues - check if the application has write access to the directory\n\
                        2. Disk space issues - ensure sufficient disk space is available\n\
                        3. File system corruption - check the file system integrity\n\
                        4. Concurrent access conflicts - ensure no other application instances are running\n\
                        5. Path resolution issues - verify the path is correct and accessible\n\
                        6. Directory creation failed - ensure parent directory can be created\n\n\
                        Database path: {}\n\
                        Directory exists: {}\n\
                        File exists: {}\n\n\
                        Please check the application logs for more details and try restarting the application.",
                        database_url,
                        database_url,
                        data_dir.exists(),
                        db_path.exists()
                    ));
                }

                Err(e.into())
            }
        }
    }

    pub async fn new_with_app_handle(app_handle: &tauri::AppHandle) -> Result<Self> {
        // Use Tauri's app data directory for more reliable path resolution
        let app_data_dir = app_handle
            .path_resolver()
            .app_data_dir()
            .context("Failed to get app data directory")?;

        let db_path = app_data_dir.join("ats_scanner.db");
        let database_url = format!("sqlite:{}", db_path.to_string_lossy());

        info!("Using Tauri app data directory: {}", database_url);
        info!("App data directory: {:?}", app_data_dir);

        // Ensure app data directory exists
        tokio::fs::create_dir_all(&app_data_dir)
            .await
            .context("Failed to create app data directory")?;

        // Try to connect with specific SQLite error handling
        let pool =
            Self::connect_with_error_handling(&database_url, &app_data_dir, &db_path).await?;

        let db = Database { pool };
        db.run_migrations().await?;
        db.seed_initial_data().await?;

        info!("Database initialized successfully using Tauri app handle");
        Ok(db)
    }

    pub async fn new_with_url(database_url: &str) -> Result<Self> {
        info!("Connecting to database: {}", database_url);

        // If it's a file-based SQLite database, ensure the parent directory exists
        if database_url.starts_with("sqlite:") && !database_url.contains(":memory:") {
            let db_path_str = database_url.strip_prefix("sqlite:").unwrap_or(database_url);
            let db_path = PathBuf::from(db_path_str);

            if let Some(parent) = db_path.parent() {
                tokio::fs::create_dir_all(parent).await?;
                info!("Created database directory: {:?}", parent);

                // Try to connect with specific SQLite error handling
                let pool = Self::connect_with_error_handling(
                    database_url,
                    &parent.to_path_buf(),
                    &db_path,
                )
                .await?;

                let db = Database { pool };
                db.run_migrations().await?;
                db.seed_initial_data().await?;

                info!("Database initialized successfully");
                return Ok(db);
            }
        }

        // For in-memory databases or other cases
        let pool = SqlitePool::connect(database_url).await?;

        let db = Database { pool };
        db.run_migrations().await?;
        db.seed_initial_data().await?;

        info!("Database initialized successfully");
        Ok(db)
    }

    async fn run_migrations(&self) -> Result<()> {
        info!("Running database migrations");

        // Start transaction for atomic migrations
        let mut tx = self.pool.begin().await?;

        // Create resumes table
        info!("Creating resumes table");
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
        .execute(&mut *tx)
        .await
        .context("Failed to create resumes table")?;

        // Create analyses table
        info!("Creating analyses table");
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
        .execute(&mut *tx)
        .await
        .context("Failed to create analyses table")?;

        // Create indexes for better performance
        info!("Creating database indexes");
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_analyses_resume_id ON analyses(resume_id)")
            .execute(&mut *tx)
            .await
            .context("Failed to create idx_analyses_resume_id index")?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_analyses_created_at ON analyses(created_at)")
            .execute(&mut *tx)
            .await
            .context("Failed to create idx_analyses_created_at index")?;

        // Create user_preferences table
        info!("Creating user_preferences table");
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
        .execute(&mut *tx)
        .await
        .context("Failed to create user_preferences table")?;

        // Create index for user preferences
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_user_preferences_user_id ON user_preferences(user_id)",
        )
        .execute(&mut *tx)
        .await
        .context("Failed to create idx_user_preferences_user_id index")?;

        // === PHASE 1 ENHANCED SCHEMA ===

        // Create industry_keywords table for industry-specific keyword dictionaries
        info!("Creating industry_keywords table");
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
        .execute(&mut *tx)
        .await
        .context("Failed to create industry_keywords table")?;

        // Create ats_compatibility_rules table
        info!("Creating ats_compatibility_rules table");
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
        .execute(&mut *tx)
        .await
        .context("Failed to create ats_compatibility_rules table")?;

        // Create scoring_benchmarks table for industry/role benchmarks
        info!("Creating scoring_benchmarks table");
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
        .execute(&mut *tx)
        .await
        .context("Failed to create scoring_benchmarks table")?;

        // Create user_feedback table for continuous learning
        info!("Creating user_feedback table");
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
        .execute(&mut *tx)
        .await
        .context("Failed to create user_feedback table")?;

        // Create model_performance_metrics table
        info!("Creating model_performance_metrics table");
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
        .execute(&mut *tx)
        .await
        .context("Failed to create model_performance_metrics table")?;

        // Create indexes for enhanced performance
        info!("Creating enhanced performance indexes");
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_industry_keywords_industry ON industry_keywords(industry)")
            .execute(&mut *tx)
            .await
            .context("Failed to create idx_industry_keywords_industry index")?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_industry_keywords_keyword ON industry_keywords(keyword)")
            .execute(&mut *tx)
            .await
            .context("Failed to create idx_industry_keywords_keyword index")?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_ats_rules_system ON ats_compatibility_rules(ats_system)")
            .execute(&mut *tx)
            .await
            .context("Failed to create idx_ats_rules_system index")?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_scoring_benchmarks_industry ON scoring_benchmarks(industry)")
            .execute(&mut *tx)
            .await
            .context("Failed to create idx_scoring_benchmarks_industry index")?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_user_feedback_analysis_id ON user_feedback(analysis_id)")
            .execute(&mut *tx)
            .await
            .context("Failed to create idx_user_feedback_analysis_id index")?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_model_performance_model_name ON model_performance_metrics(model_name)")
            .execute(&mut *tx)
            .await
            .context("Failed to create idx_model_performance_model_name index")?;

        // Create job_descriptions table for job management
        info!("Creating job_descriptions table");
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS job_descriptions (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                company TEXT NOT NULL,
                content TEXT NOT NULL,
                requirements TEXT NOT NULL DEFAULT '[]', -- JSON array as string
                preferred_qualifications TEXT, -- JSON array as string  
                salary_range_min INTEGER,
                salary_range_max INTEGER,
                salary_currency TEXT DEFAULT 'USD',
                location TEXT NOT NULL DEFAULT '',
                remote_options TEXT NOT NULL DEFAULT 'OnSite', -- RemoteWorkType enum
                employment_type TEXT NOT NULL DEFAULT 'FullTime', -- EmploymentType enum
                experience_level TEXT NOT NULL DEFAULT 'MidLevel', -- ExperienceLevel enum
                posted_date TEXT, -- DateTime<Utc> as string
                application_deadline TEXT, -- DateTime<Utc> as string
                job_url TEXT,
                keywords TEXT NOT NULL DEFAULT '[]', -- JSON array as string
                industry TEXT,
                department TEXT,
                status TEXT NOT NULL DEFAULT 'Draft', -- JobStatus enum
                priority TEXT NOT NULL DEFAULT 'Medium', -- JobPriority enum
                notes TEXT,
                application_status TEXT NOT NULL DEFAULT 'NotApplied', -- ApplicationStatus enum
                application_date TEXT, -- DateTime<Utc> as string
                interview_date TEXT, -- DateTime<Utc> as string
                response_deadline TEXT, -- DateTime<Utc> as string
                contact_person TEXT,
                contact_email TEXT,
                tags TEXT NOT NULL DEFAULT '[]', -- JSON array as string
                source TEXT NOT NULL DEFAULT 'Manual', -- JobSource enum
                is_archived BOOLEAN NOT NULL DEFAULT FALSE,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&mut *tx)
        .await
        .context("Failed to create job_descriptions table")?;

        // Create indexes for job_descriptions table performance
        info!("Creating job_descriptions indexes");
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_job_descriptions_company ON job_descriptions(company)",
        )
        .execute(&mut *tx)
        .await
        .context("Failed to create idx_job_descriptions_company index")?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_job_descriptions_location ON job_descriptions(location)")
            .execute(&mut *tx)
            .await
            .context("Failed to create idx_job_descriptions_location index")?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_job_descriptions_status ON job_descriptions(status)",
        )
        .execute(&mut *tx)
        .await
        .context("Failed to create idx_job_descriptions_status index")?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_job_descriptions_priority ON job_descriptions(priority)")
            .execute(&mut *tx)
            .await
            .context("Failed to create idx_job_descriptions_priority index")?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_job_descriptions_application_status ON job_descriptions(application_status)")
            .execute(&mut *tx)
            .await
            .context("Failed to create idx_job_descriptions_application_status index")?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_job_descriptions_created_at ON job_descriptions(created_at)")
            .execute(&mut *tx)
            .await
            .context("Failed to create idx_job_descriptions_created_at index")?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_job_descriptions_updated_at ON job_descriptions(updated_at)")
            .execute(&mut *tx)
            .await
            .context("Failed to create idx_job_descriptions_updated_at index")?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_job_descriptions_posted_date ON job_descriptions(posted_date)")
            .execute(&mut *tx)
            .await
            .context("Failed to create idx_job_descriptions_posted_date index")?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_job_descriptions_application_deadline ON job_descriptions(application_deadline)")
            .execute(&mut *tx)
            .await
            .context("Failed to create idx_job_descriptions_application_deadline index")?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_job_descriptions_salary_range ON job_descriptions(salary_range_min, salary_range_max)")
            .execute(&mut *tx)
            .await
            .context("Failed to create idx_job_descriptions_salary_range index")?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_job_descriptions_is_archived ON job_descriptions(is_archived)")
            .execute(&mut *tx)
            .await
            .context("Failed to create idx_job_descriptions_is_archived index")?;

        // Commit transaction to ensure all migrations are successful
        tx.commit()
            .await
            .context("Failed to commit database migrations")?;

        info!("Database migrations completed successfully - all tables and indexes created");
        Ok(())
    }

    async fn seed_initial_data(&self) -> Result<()> {
        info!("Starting initial data seeding process");

        // Verify all required tables exist before seeding
        self.verify_required_tables().await?;

        // Technology industry keywords
        let tech_keywords = vec![
            (
                "python",
                "programming_language",
                2.0,
                r#"["py", "python3"]"#,
            ),
            (
                "javascript",
                "programming_language",
                2.0,
                r#"["js", "node.js", "nodejs"]"#,
            ),
            ("react", "framework", 1.8, r#"["reactjs", "react.js"]"#),
            ("nodejs", "framework", 1.5, r#"["node.js", "node"]"#),
            ("docker", "devops", 1.5, r#"["containerization"]"#),
            ("kubernetes", "devops", 1.8, r#"["k8s"]"#),
            ("aws", "cloud", 2.0, r#"["amazon web services"]"#),
            ("git", "version_control", 1.5, r#"["github", "gitlab"]"#),
            ("java", "programming_language", 2.0, r#"["jvm", "spring"]"#),
            ("typescript", "programming_language", 1.8, r#"["ts"]"#),
            (
                "sql",
                "database",
                1.8,
                r#"["mysql", "postgresql", "sqlite"]"#,
            ),
            ("api", "technical", 1.5, r#"["rest", "restful", "graphql"]"#),
            ("agile", "methodology", 1.3, r#"["scrum", "kanban"]"#),
            (
                "microservices",
                "architecture",
                1.6,
                r#"["micro-services"]"#,
            ),
            (
                "ci/cd",
                "devops",
                1.5,
                r#"["continuous integration", "continuous deployment"]"#,
            ),
        ];

        // Healthcare industry keywords
        let healthcare_keywords = vec![
            (
                "hipaa",
                "regulation",
                2.0,
                r#"["health insurance portability"]"#,
            ),
            (
                "ehr",
                "software",
                1.8,
                r#"["electronic health record", "emr"]"#,
            ),
            (
                "clinical",
                "domain",
                1.5,
                r#"["clinical research", "clinical trial"]"#,
            ),
            (
                "medical device",
                "domain",
                1.8,
                r#"["medical devices", "fda"]"#,
            ),
            ("nursing", "role", 1.5, r#"["rn", "registered nurse"]"#),
            (
                "patient care",
                "soft_skill",
                1.4,
                r#"["patient safety", "patient outcomes"]"#,
            ),
            (
                "pharmacy",
                "domain",
                1.6,
                r#"["pharmacist", "medication management"]"#,
            ),
            (
                "healthcare analytics",
                "technical",
                1.7,
                r#"["health data", "medical data"]"#,
            ),
            (
                "telemedicine",
                "domain",
                1.5,
                r#"["telehealth", "remote care"]"#,
            ),
            (
                "cpt",
                "standard",
                1.4,
                r#"["current procedural terminology"]"#,
            ),
            (
                "icd",
                "standard",
                1.4,
                r#"["international classification of diseases"]"#,
            ),
            (
                "gdpr",
                "regulation",
                1.6,
                r#"["general data protection regulation"]"#,
            ),
            (
                "quality assurance",
                "methodology",
                1.3,
                r#"["qa", "quality control"]"#,
            ),
            (
                "medical coding",
                "technical",
                1.5,
                r#"["medical coder", "coding"]"#,
            ),
            (
                "healthcare management",
                "leadership",
                1.4,
                r#"["health administration"]"#,
            ),
        ];

        // Finance industry keywords
        let finance_keywords = vec![
            (
                "financial modeling",
                "technical",
                1.8,
                r#"["financial models", "modeling"]"#,
            ),
            (
                "risk management",
                "domain",
                1.7,
                r#"["risk assessment", "credit risk"]"#,
            ),
            (
                "compliance",
                "regulation",
                1.6,
                r#"["regulatory compliance", "sox"]"#,
            ),
            (
                "investment",
                "domain",
                1.5,
                r#"["investment analysis", "portfolio management"]"#,
            ),
            (
                "banking",
                "domain",
                1.4,
                r#"["retail banking", "commercial banking"]"#,
            ),
            ("fintech", "domain", 1.6, r#"["financial technology"]"#),
            (
                "trading",
                "domain",
                1.5,
                r#"["equity trading", "derivatives"]"#,
            ),
            ("accounting", "domain", 1.4, r#"["cpa", "gaap"]"#),
            (
                "excel",
                "technical",
                1.3,
                r#"["microsoft excel", "spreadsheet"]"#,
            ),
            ("bloomberg", "technical", 1.5, r#"["bloomberg terminal"]"#),
            (
                "financial reporting",
                "technical",
                1.4,
                r#"["financial statements"]"#,
            ),
            ("kyc", "regulation", 1.5, r#"["know your customer", "aml"]"#),
            (
                "securities",
                "domain",
                1.4,
                r#"["securities analysis", "equity"]"#,
            ),
            (
                "credit analysis",
                "technical",
                1.5,
                r#"["credit scoring", "underwriting"]"#,
            ),
            (
                "quantitative analysis",
                "technical",
                1.6,
                r#"["quant", "quantitative finance"]"#,
            ),
        ];

        // Education industry keywords
        let education_keywords = vec![
            (
                "curriculum",
                "domain",
                1.5,
                r#"["curriculum development", "course design"]"#,
            ),
            (
                "learning management",
                "technical",
                1.4,
                r#"["lms", "blackboard", "canvas"]"#,
            ),
            (
                "student assessment",
                "methodology",
                1.3,
                r#"["assessment", "evaluation"]"#,
            ),
            (
                "educational technology",
                "technical",
                1.5,
                r#"["edtech", "e-learning"]"#,
            ),
            (
                "pedagogy",
                "methodology",
                1.4,
                r#"["teaching methods", "instructional design"]"#,
            ),
            (
                "special education",
                "domain",
                1.3,
                r#"["special needs", "iep"]"#,
            ),
            (
                "academic research",
                "methodology",
                1.4,
                r#"["research methods", "publications"]"#,
            ),
            (
                "student affairs",
                "domain",
                1.2,
                r#"["student services", "student life"]"#,
            ),
            (
                "higher education",
                "domain",
                1.3,
                r#"["university", "college"]"#,
            ),
            (
                "k-12",
                "domain",
                1.3,
                r#"["elementary", "secondary education"]"#,
            ),
            (
                "distance learning",
                "methodology",
                1.4,
                r#"["online learning", "remote education"]"#,
            ),
            (
                "educational leadership",
                "leadership",
                1.5,
                r#"["academic leadership", "administration"]"#,
            ),
            (
                "data analysis",
                "technical",
                1.4,
                r#"["educational data", "student data"]"#,
            ),
            (
                "grant writing",
                "technical",
                1.3,
                r#"["funding", "proposals"]"#,
            ),
            (
                "accreditation",
                "regulation",
                1.2,
                r#"["accreditation standards"]"#,
            ),
        ];

        // Manufacturing industry keywords
        let manufacturing_keywords = vec![
            (
                "lean manufacturing",
                "methodology",
                1.6,
                r#"["lean", "continuous improvement"]"#,
            ),
            (
                "six sigma",
                "methodology",
                1.5,
                r#"["quality improvement", "dmaic"]"#,
            ),
            (
                "quality control",
                "methodology",
                1.4,
                r#"["qc", "quality assurance"]"#,
            ),
            (
                "supply chain",
                "domain",
                1.5,
                r#"["supply chain management", "logistics"]"#,
            ),
            (
                "production planning",
                "technical",
                1.4,
                r#"["production scheduling", "planning"]"#,
            ),
            ("safety", "domain", 1.3, r#"["workplace safety", "osha"]"#),
            (
                "process improvement",
                "methodology",
                1.5,
                r#"["process optimization"]"#,
            ),
            (
                "inventory management",
                "technical",
                1.3,
                r#"["inventory control", "stock management"]"#,
            ),
            (
                "manufacturing engineering",
                "technical",
                1.6,
                r#"["industrial engineering"]"#,
            ),
            (
                "automation",
                "technical",
                1.5,
                r#"["process automation", "robotics"]"#,
            ),
            (
                "kaizen",
                "methodology",
                1.4,
                r#"["continuous improvement"]"#,
            ),
            (
                "erp",
                "software",
                1.4,
                r#"["enterprise resource planning", "sap"]"#,
            ),
            (
                "iso",
                "standard",
                1.3,
                r#"["iso 9001", "quality standards"]"#,
            ),
            (
                "maintenance",
                "technical",
                1.2,
                r#"["preventive maintenance", "equipment maintenance"]"#,
            ),
            (
                "operations",
                "domain",
                1.3,
                r#"["operations management", "plant operations"]"#,
            ),
        ];

        // Retail industry keywords
        let retail_keywords = vec![
            (
                "customer service",
                "soft_skill",
                1.5,
                r#"["customer support", "customer experience"]"#,
            ),
            (
                "sales",
                "domain",
                1.4,
                r#"["sales management", "retail sales"]"#,
            ),
            (
                "inventory",
                "technical",
                1.3,
                r#"["inventory management", "stock control"]"#,
            ),
            (
                "merchandising",
                "domain",
                1.4,
                r#"["visual merchandising", "product placement"]"#,
            ),
            (
                "pos",
                "technical",
                1.2,
                r#"["point of sale", "cash register"]"#,
            ),
            (
                "e-commerce",
                "domain",
                1.5,
                r#"["online retail", "digital commerce"]"#,
            ),
            (
                "brand management",
                "marketing",
                1.4,
                r#"["branding", "brand strategy"]"#,
            ),
            (
                "market research",
                "methodology",
                1.3,
                r#"["consumer research", "market analysis"]"#,
            ),
            (
                "supply chain",
                "domain",
                1.4,
                r#"["logistics", "distribution"]"#,
            ),
            (
                "pricing",
                "technical",
                1.3,
                r#"["pricing strategy", "revenue management"]"#,
            ),
            (
                "loss prevention",
                "domain",
                1.2,
                r#"["security", "theft prevention"]"#,
            ),
            (
                "buying",
                "domain",
                1.3,
                r#"["procurement", "vendor management"]"#,
            ),
            (
                "store management",
                "leadership",
                1.4,
                r#"["retail management", "store operations"]"#,
            ),
            (
                "omnichannel",
                "strategy",
                1.5,
                r#"["multichannel", "cross-channel"]"#,
            ),
            (
                "customer analytics",
                "technical",
                1.4,
                r#"["customer data", "retail analytics"]"#,
            ),
        ];

        // Consulting industry keywords
        let consulting_keywords = vec![
            (
                "project management",
                "methodology",
                1.6,
                r#"["pmp", "project planning"]"#,
            ),
            (
                "business analysis",
                "technical",
                1.5,
                r#"["business analyst", "requirements gathering"]"#,
            ),
            (
                "strategy",
                "domain",
                1.5,
                r#"["strategic planning", "business strategy"]"#,
            ),
            (
                "change management",
                "methodology",
                1.4,
                r#"["organizational change", "transformation"]"#,
            ),
            (
                "stakeholder management",
                "soft_skill",
                1.4,
                r#"["stakeholder engagement"]"#,
            ),
            (
                "process mapping",
                "technical",
                1.3,
                r#"["process analysis", "workflow"]"#,
            ),
            (
                "data analysis",
                "technical",
                1.4,
                r#"["data analytics", "business intelligence"]"#,
            ),
            (
                "client management",
                "soft_skill",
                1.3,
                r#"["client relationship", "account management"]"#,
            ),
            (
                "presentation",
                "soft_skill",
                1.2,
                r#"["presentations", "public speaking"]"#,
            ),
            (
                "problem solving",
                "soft_skill",
                1.4,
                r#"["analytical thinking", "critical thinking"]"#,
            ),
            (
                "facilitation",
                "soft_skill",
                1.3,
                r#"["workshop facilitation", "meeting facilitation"]"#,
            ),
            (
                "requirements analysis",
                "technical",
                1.4,
                r#"["requirements gathering", "business requirements"]"#,
            ),
            (
                "implementation",
                "technical",
                1.3,
                r#"["solution implementation", "deployment"]"#,
            ),
            (
                "training",
                "soft_skill",
                1.2,
                r#"["training delivery", "knowledge transfer"]"#,
            ),
            (
                "process improvement",
                "methodology",
                1.4,
                r#"["process optimization", "efficiency"]"#,
            ),
        ];

        // Insert all keywords into the database
        let all_keywords = vec![
            ("technology", tech_keywords),
            ("healthcare", healthcare_keywords),
            ("finance", finance_keywords),
            ("education", education_keywords),
            ("manufacturing", manufacturing_keywords),
            ("retail", retail_keywords),
            ("consulting", consulting_keywords),
        ];

        let industry_count = all_keywords.len();
        let total_keywords: usize = all_keywords
            .iter()
            .map(|(_, keywords)| keywords.len())
            .sum();

        for (industry, keywords) in all_keywords {
            for (keyword, category, weight, synonyms) in keywords {
                let _ = sqlx::query(
                    "INSERT OR IGNORE INTO industry_keywords (id, industry, keyword, weight, category, synonyms) VALUES (?, ?, ?, ?, ?, ?)"
                )
                .bind(format!("{}-{}", industry, keyword.replace(" ", "_")))
                .bind(industry)
                .bind(keyword)
                .bind(weight)
                .bind(category)
                .bind(synonyms)
                .execute(&self.pool)
                .await;
            }
        }

        info!(
            "Initial data seeding completed successfully with {} industries and {} total keywords",
            industry_count, total_keywords
        );
        Ok(())
    }

    async fn verify_required_tables(&self) -> Result<()> {
        let required_tables = vec![
            "resumes",
            "analyses",
            "user_preferences",
            "industry_keywords",
            "ats_compatibility_rules",
            "scoring_benchmarks",
            "user_feedback",
            "model_performance_metrics",
        ];

        info!("Verifying {} required tables exist", required_tables.len());

        for table in &required_tables {
            let exists = sqlx::query(&format!(
                "SELECT name FROM sqlite_master WHERE type='table' AND name='{}'",
                table
            ))
            .fetch_optional(&self.pool)
            .await?
            .is_some();

            if !exists {
                return Err(anyhow::anyhow!(
                    "Required table '{}' does not exist in database",
                    table
                ));
            }
        }

        info!(
            "All {} required tables verified successfully",
            required_tables.len()
        );
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
                created_at: parse_timestamp(&row.get::<String, _>("created_at"))?,
                updated_at: parse_timestamp(&row.get::<String, _>("updated_at"))?,
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
                created_at: parse_timestamp(&row.get::<String, _>("created_at"))?,
                updated_at: parse_timestamp(&row.get::<String, _>("updated_at"))?,
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
                created_at: parse_timestamp(&row.get::<String, _>("created_at"))?,
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
                created_at: parse_timestamp(&row.get::<String, _>("created_at"))?,
            };
            analyses.push(analysis);
        }

        Ok(analyses)
    }

    pub async fn delete_analysis(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM analyses WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        info!("Analysis deleted: {}", id);
        Ok(())
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

    // Comprehensive health check with detailed information
    pub async fn comprehensive_health_check(&self) -> Result<serde_json::Value> {
        let mut health_info = std::collections::HashMap::new();

        // Basic connectivity check
        let basic_check = match sqlx::query("SELECT 1").fetch_one(&self.pool).await {
            Ok(result) => result.get::<i32, _>(0) == 1,
            Err(e) => {
                health_info.insert(
                    "basic_connectivity".to_string(),
                    serde_json::json!({
                        "status": "failed",
                        "error": e.to_string()
                    }),
                );
                false
            }
        };

        health_info.insert(
            "basic_connectivity".to_string(),
            serde_json::json!({
                "status": if basic_check { "healthy" } else { "failed" },
                "description": "Basic database connectivity test"
            }),
        );

        // Table existence check
        let table_check = self.check_table_existence().await;
        health_info.insert(
            "table_existence".to_string(),
            serde_json::json!(table_check),
        );

        // Database size check
        let db_size = self.get_database_size().await;
        health_info.insert("database_size".to_string(), serde_json::json!(db_size));

        // Connection pool status
        let pool_info = self.get_connection_pool_info();
        health_info.insert("connection_pool".to_string(), serde_json::json!(pool_info));

        // Performance check
        let performance_check = self.check_database_performance().await;
        health_info.insert(
            "performance".to_string(),
            serde_json::json!(performance_check),
        );

        // Calculate overall health
        let overall_healthy = basic_check
            && table_check
                .get("all_tables_exist")
                .unwrap_or(&serde_json::Value::Bool(false))
                .as_bool()
                .unwrap_or(false);

        health_info.insert("overall_status".to_string(), serde_json::json!({
            "healthy": overall_healthy,
            "timestamp": Utc::now().to_rfc3339(),
            "summary": if overall_healthy { "Database is healthy" } else { "Database has issues" }
        }));

        Ok(serde_json::json!(health_info))
    }

    async fn check_table_existence(&self) -> serde_json::Value {
        let required_tables = vec![
            "resumes",
            "analyses",
            "user_preferences",
            "industry_keywords",
            "ats_compatibility_rules",
            "scoring_benchmarks",
            "user_feedback",
            "model_performance_metrics",
        ];

        let mut table_status = std::collections::HashMap::new();
        let mut all_exist = true;

        for table in &required_tables {
            let exists = match sqlx::query(&format!(
                "SELECT name FROM sqlite_master WHERE type='table' AND name='{}'",
                table
            ))
            .fetch_optional(&self.pool)
            .await
            {
                Ok(Some(_)) => true,
                Ok(None) => false,
                Err(e) => {
                    table_status
                        .insert(format!("{}_error", table), serde_json::json!(e.to_string()));
                    false
                }
            };

            table_status.insert(table.to_string(), serde_json::json!(exists));
            if !exists {
                all_exist = false;
            }
        }

        table_status.insert("all_tables_exist".to_string(), serde_json::json!(all_exist));
        serde_json::json!(table_status)
    }

    async fn get_database_size(&self) -> serde_json::Value {
        let page_count = sqlx::query("PRAGMA page_count")
            .fetch_one(&self.pool)
            .await
            .map(|row| row.get::<i32, _>(0))
            .unwrap_or(0);

        let page_size = sqlx::query("PRAGMA page_size")
            .fetch_one(&self.pool)
            .await
            .map(|row| row.get::<i32, _>(0))
            .unwrap_or(0);

        let total_size = (page_count as i64) * (page_size as i64);

        serde_json::json!({
            "page_count": page_count,
            "page_size": page_size,
            "total_size_bytes": total_size,
            "total_size_mb": total_size as f64 / 1024.0 / 1024.0
        })
    }

    fn get_connection_pool_info(&self) -> serde_json::Value {
        serde_json::json!({
            "size": self.pool.size(),
            "idle": self.pool.num_idle(),
            "description": "SQLite connection pool information"
        })
    }

    async fn check_database_performance(&self) -> serde_json::Value {
        let start_time = std::time::Instant::now();

        // Run a simple query to test performance
        let query_result = sqlx::query("SELECT COUNT(*) FROM sqlite_master")
            .fetch_one(&self.pool)
            .await;

        let duration = start_time.elapsed();

        match query_result {
            Ok(_) => serde_json::json!({
                "status": "healthy",
                "query_time_ms": duration.as_millis(),
                "description": "Database query performance check"
            }),
            Err(e) => serde_json::json!({
                "status": "failed",
                "error": e.to_string(),
                "query_time_ms": duration.as_millis(),
                "description": "Database query performance check failed"
            }),
        }
    }

    // Database maintenance and optimization
    pub async fn optimize_database(&self) -> Result<serde_json::Value> {
        let mut results = std::collections::HashMap::new();

        // Run VACUUM to reclaim space
        let vacuum_start = std::time::Instant::now();
        match sqlx::query("VACUUM").execute(&self.pool).await {
            Ok(_) => {
                results.insert(
                    "vacuum".to_string(),
                    serde_json::json!({
                        "status": "success",
                        "duration_ms": vacuum_start.elapsed().as_millis(),
                        "description": "Database vacuum completed successfully"
                    }),
                );
            }
            Err(e) => {
                results.insert(
                    "vacuum".to_string(),
                    serde_json::json!({
                        "status": "failed",
                        "error": e.to_string(),
                        "description": "Database vacuum failed"
                    }),
                );
            }
        }

        // Run ANALYZE to update statistics
        let analyze_start = std::time::Instant::now();
        match sqlx::query("ANALYZE").execute(&self.pool).await {
            Ok(_) => {
                results.insert(
                    "analyze".to_string(),
                    serde_json::json!({
                        "status": "success",
                        "duration_ms": analyze_start.elapsed().as_millis(),
                        "description": "Database analysis completed successfully"
                    }),
                );
            }
            Err(e) => {
                results.insert(
                    "analyze".to_string(),
                    serde_json::json!({
                        "status": "failed",
                        "error": e.to_string(),
                        "description": "Database analysis failed"
                    }),
                );
            }
        }

        // Check integrity
        let integrity_check = match sqlx::query("PRAGMA integrity_check")
            .fetch_one(&self.pool)
            .await
        {
            Ok(row) => {
                let result: String = row.get(0);
                serde_json::json!({
                    "status": if result == "ok" { "healthy" } else { "issues_found" },
                    "result": result,
                    "description": "Database integrity check"
                })
            }
            Err(e) => serde_json::json!({
                "status": "failed",
                "error": e.to_string(),
                "description": "Database integrity check failed"
            }),
        };

        results.insert("integrity_check".to_string(), integrity_check);

        Ok(serde_json::json!({
            "timestamp": Utc::now().to_rfc3339(),
            "results": results
        }))
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
                created_at: parse_timestamp(&row.get::<String, _>("created_at"))?,
                updated_at: parse_timestamp(&row.get::<String, _>("updated_at"))?,
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
            // Safe parsing of created_at with proper error handling for multiple timestamp formats
            let created_at_str: String = row.get("created_at");
            let created_at = parse_timestamp(&created_at_str).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to parse created_at timestamp '{}' for industry '{}': {}. This indicates corrupted database data.",
                    created_at_str,
                    industry,
                    e
                )
            })?;

            let keyword = IndustryKeyword {
                id: row.get("id"),
                industry: row.get("industry"),
                keyword: row.get("keyword"),
                weight: row.get("weight"),
                category: row.get("category"),
                synonyms: row.get("synonyms"),
                created_at,
            };
            keywords.push(keyword);
        }

        // If no keywords found for the specific industry, provide fallback keywords
        if keywords.is_empty() {
            info!(
                "No keywords found for industry '{}', using fallback keywords",
                industry
            );
            keywords = self.get_fallback_keywords(industry).await;
        }

        Ok(keywords)
    }

    async fn get_fallback_keywords(&self, industry: &str) -> Vec<IndustryKeyword> {
        let now = chrono::Utc::now();

        // Generic keywords that apply to most industries
        let generic_keywords = vec![
            (
                "communication",
                "soft_skill",
                1.3,
                r#"["verbal communication", "written communication"]"#,
            ),
            (
                "teamwork",
                "soft_skill",
                1.2,
                r#"["collaboration", "team player"]"#,
            ),
            (
                "leadership",
                "soft_skill",
                1.4,
                r#"["team leadership", "management"]"#,
            ),
            (
                "problem solving",
                "soft_skill",
                1.4,
                r#"["analytical thinking", "critical thinking"]"#,
            ),
            (
                "project management",
                "methodology",
                1.3,
                r#"["project planning", "project coordination"]"#,
            ),
            (
                "data analysis",
                "technical",
                1.3,
                r#"["data analytics", "analysis"]"#,
            ),
            (
                "microsoft office",
                "technical",
                1.1,
                r#"["excel", "word", "powerpoint"]"#,
            ),
            (
                "customer service",
                "soft_skill",
                1.2,
                r#"["customer support", "client service"]"#,
            ),
            (
                "time management",
                "soft_skill",
                1.1,
                r#"["organization", "prioritization"]"#,
            ),
            (
                "attention to detail",
                "soft_skill",
                1.1,
                r#"["accuracy", "detail-oriented"]"#,
            ),
        ];

        // Industry-specific fallback keywords for common industries
        let industry_specific_fallback = match industry.to_lowercase().as_str() {
            "business" | "general" | "administration" => vec![
                (
                    "business analysis",
                    "technical",
                    1.3,
                    r#"["business planning", "strategic analysis"]"#,
                ),
                (
                    "office administration",
                    "technical",
                    1.2,
                    r#"["administrative support", "office management"]"#,
                ),
                (
                    "customer relations",
                    "soft_skill",
                    1.2,
                    r#"["client relations", "relationship management"]"#,
                ),
            ],
            "sales" | "marketing" => vec![
                (
                    "sales experience",
                    "domain",
                    1.4,
                    r#"["sales performance", "revenue generation"]"#,
                ),
                (
                    "marketing",
                    "domain",
                    1.3,
                    r#"["digital marketing", "brand marketing"]"#,
                ),
                (
                    "customer acquisition",
                    "technical",
                    1.3,
                    r#"["lead generation", "prospecting"]"#,
                ),
            ],
            "operations" | "logistics" => vec![
                (
                    "operations management",
                    "technical",
                    1.4,
                    r#"["operational efficiency", "process management"]"#,
                ),
                (
                    "supply chain",
                    "domain",
                    1.3,
                    r#"["logistics", "distribution"]"#,
                ),
                (
                    "inventory management",
                    "technical",
                    1.2,
                    r#"["inventory control", "stock management"]"#,
                ),
            ],
            "human resources" | "hr" => vec![
                (
                    "recruitment",
                    "technical",
                    1.4,
                    r#"["hiring", "talent acquisition"]"#,
                ),
                (
                    "employee relations",
                    "soft_skill",
                    1.3,
                    r#"["hr management", "personnel management"]"#,
                ),
                (
                    "performance management",
                    "technical",
                    1.3,
                    r#"["performance evaluation", "employee development"]"#,
                ),
            ],
            _ => vec![
                (
                    "industry experience",
                    "domain",
                    1.2,
                    r#"["sector knowledge", "domain expertise"]"#,
                ),
                (
                    "professional development",
                    "soft_skill",
                    1.1,
                    r#"["continuous learning", "skill development"]"#,
                ),
                (
                    "compliance",
                    "regulation",
                    1.2,
                    r#"["regulatory compliance", "standards"]"#,
                ),
            ],
        };

        // Combine generic and industry-specific keywords
        let mut all_fallback_keywords = generic_keywords;
        all_fallback_keywords.extend(industry_specific_fallback);

        // Convert to IndustryKeyword structs
        all_fallback_keywords
            .into_iter()
            .map(|(keyword, category, weight, synonyms)| IndustryKeyword {
                id: format!("fallback-{}-{}", industry, keyword.replace(" ", "_")),
                industry: industry.to_string(),
                keyword: keyword.to_string(),
                weight,
                category: category.to_string(),
                synonyms: synonyms.to_string(),
                created_at: now,
            })
            .collect()
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
                created_at: parse_timestamp(&row.get::<String, _>("created_at"))?,
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
                created_at: parse_timestamp(&row.get::<String, _>("created_at"))?,
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
                created_at: parse_timestamp(&row.get::<String, _>("created_at"))?,
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

    // Job Description CRUD Operations
    pub async fn save_job_description(&self, job: &JobDescription) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO job_descriptions (
                id, title, company, content, requirements, preferred_qualifications,
                salary_range_min, salary_range_max, salary_currency, location,
                remote_options, employment_type, experience_level, posted_date,
                application_deadline, job_url, keywords, industry, department,
                status, priority, notes, application_status, application_date,
                interview_date, response_deadline, contact_person, contact_email,
                tags, source, is_archived, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&job.id)
        .bind(&job.title)
        .bind(&job.company)
        .bind(&job.content)
        .bind(&job.requirements)
        .bind(&job.preferred_qualifications)
        .bind(job.salary_range_min)
        .bind(job.salary_range_max)
        .bind(&job.salary_currency)
        .bind(&job.location)
        .bind(serde_json::to_string(&job.remote_options).unwrap_or_default())
        .bind(serde_json::to_string(&job.employment_type).unwrap_or_default())
        .bind(serde_json::to_string(&job.experience_level).unwrap_or_default())
        .bind(job.posted_date.map(|d| d.to_rfc3339()))
        .bind(job.application_deadline.map(|d| d.to_rfc3339()))
        .bind(&job.job_url)
        .bind(&job.keywords)
        .bind(&job.industry)
        .bind(&job.department)
        .bind(serde_json::to_string(&job.status).unwrap_or_default())
        .bind(serde_json::to_string(&job.priority).unwrap_or_default())
        .bind(&job.notes)
        .bind(serde_json::to_string(&job.application_status).unwrap_or_default())
        .bind(job.application_date.map(|d| d.to_rfc3339()))
        .bind(job.interview_date.map(|d| d.to_rfc3339()))
        .bind(job.response_deadline.map(|d| d.to_rfc3339()))
        .bind(&job.contact_person)
        .bind(&job.contact_email)
        .bind(&job.tags)
        .bind(serde_json::to_string(&job.source).unwrap_or_default())
        .bind(job.is_archived)
        .bind(job.created_at.to_rfc3339())
        .bind(job.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        info!("Job description saved with ID: {}", job.id);
        Ok(())
    }

    pub async fn get_job_description(&self, id: &str) -> Result<Option<JobDescription>> {
        let row = sqlx::query(
            r#"
            SELECT id, title, company, content, requirements, preferred_qualifications,
                   salary_range_min, salary_range_max, salary_currency, location,
                   remote_options, employment_type, experience_level, posted_date,
                   application_deadline, job_url, keywords, industry, department,
                   status, priority, notes, application_status, application_date,
                   interview_date, response_deadline, contact_person, contact_email,
                   tags, source, is_archived, created_at, updated_at
            FROM job_descriptions WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let job = JobDescription {
                id: row.get("id"),
                title: row.get("title"),
                company: row.get("company"),
                content: row.get("content"),
                requirements: row.get("requirements"),
                preferred_qualifications: row.get("preferred_qualifications"),
                salary_range_min: row.get("salary_range_min"),
                salary_range_max: row.get("salary_range_max"),
                salary_currency: row.get("salary_currency"),
                location: row.get("location"),
                remote_options: serde_json::from_str(&row.get::<String, _>("remote_options"))
                    .unwrap_or_default(),
                employment_type: serde_json::from_str(&row.get::<String, _>("employment_type"))
                    .unwrap_or_default(),
                experience_level: serde_json::from_str(&row.get::<String, _>("experience_level"))
                    .unwrap_or_default(),
                posted_date: row
                    .get::<Option<String>, _>("posted_date")
                    .and_then(|s| parse_timestamp(&s).ok()),
                application_deadline: row
                    .get::<Option<String>, _>("application_deadline")
                    .and_then(|s| parse_timestamp(&s).ok()),
                job_url: row.get("job_url"),
                keywords: row.get("keywords"),
                industry: row.get("industry"),
                department: row.get("department"),
                status: serde_json::from_str(&row.get::<String, _>("status")).unwrap_or_default(),
                priority: serde_json::from_str(&row.get::<String, _>("priority"))
                    .unwrap_or_default(),
                notes: row.get("notes"),
                application_status: serde_json::from_str(
                    &row.get::<String, _>("application_status"),
                )
                .unwrap_or_default(),
                application_date: row
                    .get::<Option<String>, _>("application_date")
                    .and_then(|s| parse_timestamp(&s).ok()),
                interview_date: row
                    .get::<Option<String>, _>("interview_date")
                    .and_then(|s| parse_timestamp(&s).ok()),
                response_deadline: row
                    .get::<Option<String>, _>("response_deadline")
                    .and_then(|s| parse_timestamp(&s).ok()),
                contact_person: row.get("contact_person"),
                contact_email: row.get("contact_email"),
                tags: row.get("tags"),
                source: serde_json::from_str(&row.get::<String, _>("source")).unwrap_or_default(),
                is_archived: row.get("is_archived"),
                created_at: parse_timestamp(&row.get::<String, _>("created_at"))?,
                updated_at: parse_timestamp(&row.get::<String, _>("updated_at"))?,
            };
            Ok(Some(job))
        } else {
            Ok(None)
        }
    }

    pub async fn update_job_description(&self, job: &JobDescription) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE job_descriptions SET
                title = ?, company = ?, content = ?, requirements = ?, preferred_qualifications = ?,
                salary_range_min = ?, salary_range_max = ?, salary_currency = ?, location = ?,
                remote_options = ?, employment_type = ?, experience_level = ?, posted_date = ?,
                application_deadline = ?, job_url = ?, keywords = ?, industry = ?, department = ?,
                status = ?, priority = ?, notes = ?, application_status = ?, application_date = ?,
                interview_date = ?, response_deadline = ?, contact_person = ?, contact_email = ?,
                tags = ?, source = ?, is_archived = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&job.title)
        .bind(&job.company)
        .bind(&job.content)
        .bind(&job.requirements)
        .bind(&job.preferred_qualifications)
        .bind(job.salary_range_min)
        .bind(job.salary_range_max)
        .bind(&job.salary_currency)
        .bind(&job.location)
        .bind(serde_json::to_string(&job.remote_options).unwrap_or_default())
        .bind(serde_json::to_string(&job.employment_type).unwrap_or_default())
        .bind(serde_json::to_string(&job.experience_level).unwrap_or_default())
        .bind(job.posted_date.map(|d| d.to_rfc3339()))
        .bind(job.application_deadline.map(|d| d.to_rfc3339()))
        .bind(&job.job_url)
        .bind(&job.keywords)
        .bind(&job.industry)
        .bind(&job.department)
        .bind(serde_json::to_string(&job.status).unwrap_or_default())
        .bind(serde_json::to_string(&job.priority).unwrap_or_default())
        .bind(&job.notes)
        .bind(serde_json::to_string(&job.application_status).unwrap_or_default())
        .bind(job.application_date.map(|d| d.to_rfc3339()))
        .bind(job.interview_date.map(|d| d.to_rfc3339()))
        .bind(job.response_deadline.map(|d| d.to_rfc3339()))
        .bind(&job.contact_person)
        .bind(&job.contact_email)
        .bind(&job.tags)
        .bind(serde_json::to_string(&job.source).unwrap_or_default())
        .bind(job.is_archived)
        .bind(job.updated_at.to_rfc3339())
        .bind(&job.id)
        .execute(&self.pool)
        .await?;

        info!("Job description updated with ID: {}", job.id);
        Ok(())
    }

    pub async fn delete_job_description(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM job_descriptions WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        info!("Job description deleted with ID: {}", id);
        Ok(())
    }

    pub async fn get_all_job_descriptions(
        &self,
        include_archived: bool,
    ) -> Result<Vec<JobDescription>> {
        let query = if include_archived {
            "SELECT * FROM job_descriptions ORDER BY updated_at DESC"
        } else {
            "SELECT * FROM job_descriptions WHERE is_archived = FALSE ORDER BY updated_at DESC"
        };

        let rows = sqlx::query(query).fetch_all(&self.pool).await?;

        let mut jobs = Vec::new();
        for row in rows {
            let job = JobDescription {
                id: row.get("id"),
                title: row.get("title"),
                company: row.get("company"),
                content: row.get("content"),
                requirements: row.get("requirements"),
                preferred_qualifications: row.get("preferred_qualifications"),
                salary_range_min: row.get("salary_range_min"),
                salary_range_max: row.get("salary_range_max"),
                salary_currency: row.get("salary_currency"),
                location: row.get("location"),
                remote_options: serde_json::from_str(&row.get::<String, _>("remote_options"))
                    .unwrap_or_default(),
                employment_type: serde_json::from_str(&row.get::<String, _>("employment_type"))
                    .unwrap_or_default(),
                experience_level: serde_json::from_str(&row.get::<String, _>("experience_level"))
                    .unwrap_or_default(),
                posted_date: row
                    .get::<Option<String>, _>("posted_date")
                    .and_then(|s| parse_timestamp(&s).ok()),
                application_deadline: row
                    .get::<Option<String>, _>("application_deadline")
                    .and_then(|s| parse_timestamp(&s).ok()),
                job_url: row.get("job_url"),
                keywords: row.get("keywords"),
                industry: row.get("industry"),
                department: row.get("department"),
                status: serde_json::from_str(&row.get::<String, _>("status")).unwrap_or_default(),
                priority: serde_json::from_str(&row.get::<String, _>("priority"))
                    .unwrap_or_default(),
                notes: row.get("notes"),
                application_status: serde_json::from_str(
                    &row.get::<String, _>("application_status"),
                )
                .unwrap_or_default(),
                application_date: row
                    .get::<Option<String>, _>("application_date")
                    .and_then(|s| parse_timestamp(&s).ok()),
                interview_date: row
                    .get::<Option<String>, _>("interview_date")
                    .and_then(|s| parse_timestamp(&s).ok()),
                response_deadline: row
                    .get::<Option<String>, _>("response_deadline")
                    .and_then(|s| parse_timestamp(&s).ok()),
                contact_person: row.get("contact_person"),
                contact_email: row.get("contact_email"),
                tags: row.get("tags"),
                source: serde_json::from_str(&row.get::<String, _>("source")).unwrap_or_default(),
                is_archived: row.get("is_archived"),
                created_at: parse_timestamp(&row.get::<String, _>("created_at"))?,
                updated_at: parse_timestamp(&row.get::<String, _>("updated_at"))?,
            };
            jobs.push(job);
        }

        Ok(jobs)
    }

    pub async fn search_job_descriptions(
        &self,
        request: &JobSearchRequest,
    ) -> Result<JobSearchResult> {
        let mut query = String::from("SELECT * FROM job_descriptions WHERE 1=1");
        let mut count_query =
            String::from("SELECT COUNT(*) as total FROM job_descriptions WHERE 1=1");
        let mut params: Vec<String> = Vec::new();

        // Build WHERE conditions
        if let Some(query_text) = &request.query {
            query.push_str(" AND (title LIKE ?1 OR company LIKE ?1 OR content LIKE ?1)");
            count_query.push_str(" AND (title LIKE ?1 OR company LIKE ?1 OR content LIKE ?1)");
            params.push(format!("%{}%", query_text));
        }

        if let Some(company) = &request.company {
            let param_idx = params.len() + 1;
            query.push_str(&format!(" AND company LIKE ?{}", param_idx));
            count_query.push_str(&format!(" AND company LIKE ?{}", param_idx));
            params.push(format!("%{}%", company));
        }

        if let Some(location) = &request.location {
            let param_idx = params.len() + 1;
            query.push_str(&format!(" AND location LIKE ?{}", param_idx));
            count_query.push_str(&format!(" AND location LIKE ?{}", param_idx));
            params.push(format!("%{}%", location));
        }

        if let Some(salary_min) = request.salary_min {
            let param_idx = params.len() + 1;
            query.push_str(&format!(" AND salary_range_max >= ?{}", param_idx));
            count_query.push_str(&format!(" AND salary_range_max >= ?{}", param_idx));
            params.push(salary_min.to_string());
        }

        if let Some(salary_max) = request.salary_max {
            let param_idx = params.len() + 1;
            query.push_str(&format!(" AND salary_range_min <= ?{}", param_idx));
            count_query.push_str(&format!(" AND salary_range_min <= ?{}", param_idx));
            params.push(salary_max.to_string());
        }

        if let Some(industry) = &request.industry {
            let param_idx = params.len() + 1;
            query.push_str(&format!(" AND industry LIKE ?{}", param_idx));
            count_query.push_str(&format!(" AND industry LIKE ?{}", param_idx));
            params.push(format!("%{}%", industry));
        }

        if let Some(include_archived) = request.include_archived {
            if !include_archived {
                query.push_str(" AND is_archived = FALSE");
                count_query.push_str(" AND is_archived = FALSE");
            }
        } else {
            query.push_str(" AND is_archived = FALSE");
            count_query.push_str(" AND is_archived = FALSE");
        }

        // Add ORDER BY clause
        if let Some(sort_by) = &request.sort_by {
            let order = match request.sort_order.as_ref().unwrap_or(&SortOrder::Desc) {
                SortOrder::Asc => "ASC",
                SortOrder::Desc => "DESC",
            };

            let column = match sort_by {
                JobSortOption::CreatedAt => "created_at",
                JobSortOption::UpdatedAt => "updated_at",
                JobSortOption::PostedDate => "posted_date",
                JobSortOption::ApplicationDeadline => "application_deadline",
                JobSortOption::Priority => "priority",
                JobSortOption::Title => "title",
                JobSortOption::Company => "company",
                JobSortOption::SalaryMin => "salary_range_min",
                JobSortOption::SalaryMax => "salary_range_max",
            };

            query.push_str(&format!(" ORDER BY {} {}", column, order));
        } else {
            query.push_str(" ORDER BY updated_at DESC");
        }

        // Add LIMIT and OFFSET
        if let Some(limit) = request.limit {
            let param_idx = params.len() + 1;
            query.push_str(&format!(" LIMIT ?{}", param_idx));
            params.push(limit.to_string());
        }

        if let Some(offset) = request.offset {
            let param_idx = params.len() + 1;
            query.push_str(&format!(" OFFSET ?{}", param_idx));
            params.push(offset.to_string());
        }

        // Get total count
        let mut count_sql_query = sqlx::query(&count_query);
        for (i, param) in params.iter().enumerate() {
            if i < params.len() - 2 {
                // Exclude LIMIT and OFFSET params from count query
                count_sql_query = count_sql_query.bind(param);
            } else {
                break;
            }
        }

        let count_row = count_sql_query.fetch_one(&self.pool).await?;
        let total_count: i64 = count_row.get("total");

        // Execute search query
        let mut sql_query = sqlx::query(&query);
        for param in &params {
            sql_query = sql_query.bind(param);
        }

        let rows = sql_query.fetch_all(&self.pool).await?;

        let mut jobs = Vec::new();
        for row in rows {
            let job = JobDescription {
                id: row.get("id"),
                title: row.get("title"),
                company: row.get("company"),
                content: row.get("content"),
                requirements: row.get("requirements"),
                preferred_qualifications: row.get("preferred_qualifications"),
                salary_range_min: row.get("salary_range_min"),
                salary_range_max: row.get("salary_range_max"),
                salary_currency: row.get("salary_currency"),
                location: row.get("location"),
                remote_options: serde_json::from_str(&row.get::<String, _>("remote_options"))
                    .unwrap_or_default(),
                employment_type: serde_json::from_str(&row.get::<String, _>("employment_type"))
                    .unwrap_or_default(),
                experience_level: serde_json::from_str(&row.get::<String, _>("experience_level"))
                    .unwrap_or_default(),
                posted_date: row
                    .get::<Option<String>, _>("posted_date")
                    .and_then(|s| parse_timestamp(&s).ok()),
                application_deadline: row
                    .get::<Option<String>, _>("application_deadline")
                    .and_then(|s| parse_timestamp(&s).ok()),
                job_url: row.get("job_url"),
                keywords: row.get("keywords"),
                industry: row.get("industry"),
                department: row.get("department"),
                status: serde_json::from_str(&row.get::<String, _>("status")).unwrap_or_default(),
                priority: serde_json::from_str(&row.get::<String, _>("priority"))
                    .unwrap_or_default(),
                notes: row.get("notes"),
                application_status: serde_json::from_str(
                    &row.get::<String, _>("application_status"),
                )
                .unwrap_or_default(),
                application_date: row
                    .get::<Option<String>, _>("application_date")
                    .and_then(|s| parse_timestamp(&s).ok()),
                interview_date: row
                    .get::<Option<String>, _>("interview_date")
                    .and_then(|s| parse_timestamp(&s).ok()),
                response_deadline: row
                    .get::<Option<String>, _>("response_deadline")
                    .and_then(|s| parse_timestamp(&s).ok()),
                contact_person: row.get("contact_person"),
                contact_email: row.get("contact_email"),
                tags: row.get("tags"),
                source: serde_json::from_str(&row.get::<String, _>("source")).unwrap_or_default(),
                is_archived: row.get("is_archived"),
                created_at: parse_timestamp(&row.get::<String, _>("created_at"))?,
                updated_at: parse_timestamp(&row.get::<String, _>("updated_at"))?,
            };
            jobs.push(job);
        }

        let has_more = if let Some(limit) = request.limit {
            jobs.len() as i64 == limit
        } else {
            false
        };

        Ok(JobSearchResult {
            jobs,
            total_count,
            has_more,
        })
    }

    pub async fn get_job_analytics(&self) -> Result<JobAnalytics> {
        // Get total jobs count
        let total_jobs: i64 =
            sqlx::query("SELECT COUNT(*) as count FROM job_descriptions WHERE is_archived = FALSE")
                .fetch_one(&self.pool)
                .await?
                .get("count");

        // Get jobs by status
        let status_rows = sqlx::query("SELECT status, COUNT(*) as count FROM job_descriptions WHERE is_archived = FALSE GROUP BY status")
            .fetch_all(&self.pool)
            .await?;
        let mut jobs_by_status = Vec::new();
        for row in status_rows {
            let status_str: String = row.get("status");
            let status: JobStatus = serde_json::from_str(&status_str).unwrap_or(JobStatus::Draft);
            jobs_by_status.push(JobStatusCount {
                status,
                count: row.get("count"),
            });
        }

        // Get jobs by priority
        let priority_rows = sqlx::query("SELECT priority, COUNT(*) as count FROM job_descriptions WHERE is_archived = FALSE GROUP BY priority")
            .fetch_all(&self.pool)
            .await?;
        let mut jobs_by_priority = Vec::new();
        for row in priority_rows {
            let priority_str: String = row.get("priority");
            let priority: JobPriority =
                serde_json::from_str(&priority_str).unwrap_or(JobPriority::Medium);
            jobs_by_priority.push(JobPriorityCount {
                priority,
                count: row.get("count"),
            });
        }

        // Get jobs by application status
        let app_status_rows = sqlx::query("SELECT application_status, COUNT(*) as count FROM job_descriptions WHERE is_archived = FALSE GROUP BY application_status")
            .fetch_all(&self.pool)
            .await?;
        let mut jobs_by_application_status = Vec::new();
        for row in app_status_rows {
            let app_status_str: String = row.get("application_status");
            let app_status: ApplicationStatus =
                serde_json::from_str(&app_status_str).unwrap_or(ApplicationStatus::NotApplied);
            jobs_by_application_status.push(ApplicationStatusCount {
                status: app_status,
                count: row.get("count"),
            });
        }

        // Get top companies
        let company_rows = sqlx::query("SELECT company, COUNT(*) as count FROM job_descriptions WHERE is_archived = FALSE GROUP BY company ORDER BY count DESC LIMIT 10")
            .fetch_all(&self.pool)
            .await?;
        let mut top_companies = Vec::new();
        for row in company_rows {
            top_companies.push(CompanyCount {
                company: row.get("company"),
                count: row.get("count"),
            });
        }

        // Get top locations
        let location_rows = sqlx::query("SELECT location, COUNT(*) as count FROM job_descriptions WHERE is_archived = FALSE AND location != '' GROUP BY location ORDER BY count DESC LIMIT 10")
            .fetch_all(&self.pool)
            .await?;
        let mut top_locations = Vec::new();
        for row in location_rows {
            top_locations.push(LocationCount {
                location: row.get("location"),
                count: row.get("count"),
            });
        }

        // Calculate success and response rates (basic implementation)
        let applied_count: i64 = sqlx::query("SELECT COUNT(*) as count FROM job_descriptions WHERE application_status NOT IN ('\"NotApplied\"') AND is_archived = FALSE")
            .fetch_one(&self.pool)
            .await?
            .get("count");

        let responded_count: i64 = sqlx::query("SELECT COUNT(*) as count FROM job_descriptions WHERE application_status IN ('\"PhoneScreen\"', '\"TechnicalInterview\"', '\"OnSiteInterview\"', '\"FinalRound\"', '\"OfferReceived\"', '\"OfferAccepted\"') AND is_archived = FALSE")
            .fetch_one(&self.pool)
            .await?
            .get("count");

        let success_count: i64 = sqlx::query("SELECT COUNT(*) as count FROM job_descriptions WHERE application_status IN ('\"OfferReceived\"', '\"OfferAccepted\"') AND is_archived = FALSE")
            .fetch_one(&self.pool)
            .await?
            .get("count");

        let success_rate = if applied_count > 0 {
            success_count as f64 / applied_count as f64 * 100.0
        } else {
            0.0
        };

        let response_rate = if applied_count > 0 {
            responded_count as f64 / applied_count as f64 * 100.0
        } else {
            0.0
        };

        Ok(JobAnalytics {
            total_jobs,
            jobs_by_status,
            jobs_by_priority,
            jobs_by_application_status,
            average_salary_range: None, // TODO: Implement salary calculations
            top_companies,
            top_locations,
            application_timeline: Vec::new(), // TODO: Implement timeline
            success_rate,
            response_rate,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    async fn setup_test_db() -> Result<Database> {
        Database::new_with_url("sqlite::memory:").await
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
    async fn test_database_initialization() -> Result<()> {
        let db = setup_test_db().await?;
        assert!(db.health_check().await?);
        Ok(())
    }

    #[tokio::test]
    async fn test_resume_crud_operations() -> Result<()> {
        let db = setup_test_db().await?;
        let resume = create_test_resume();

        // Test save
        db.save_resume(&resume).await?;

        // Test get
        let retrieved = db.get_resume(&resume.id).await?;
        assert!(retrieved.is_some());
        let retrieved = retrieved.context("Resume should exist after save")?;
        assert_eq!(retrieved.id, resume.id);
        assert_eq!(retrieved.filename, resume.filename);
        assert_eq!(retrieved.content, resume.content);

        // Test get all
        let all_resumes = db.get_all_resumes().await?;
        assert_eq!(all_resumes.len(), 1);

        // Test delete
        db.delete_resume(&resume.id).await?;
        let retrieved = db.get_resume(&resume.id).await?;
        assert!(retrieved.is_none());

        Ok(())
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
