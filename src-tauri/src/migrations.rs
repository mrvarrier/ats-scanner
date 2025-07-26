use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use std::collections::HashMap;

/// Represents a single database migration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Migration {
    pub version: i32,
    pub name: String,
    pub description: String,
    pub up_sql: String,
    pub down_sql: String,
    pub checksum: String,
}

/// Migration execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationResult {
    pub version: i32,
    pub success: bool,
    pub executed_at: DateTime<Utc>,
    pub execution_time_ms: i64,
    pub error_message: Option<String>,
}

/// Database schema version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaVersion {
    pub current_version: i32,
    pub latest_available: i32,
    pub is_up_to_date: bool,
    pub pending_migrations: Vec<Migration>,
    pub applied_migrations: Vec<MigrationRecord>,
}

/// Record of an applied migration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRecord {
    pub version: i32,
    pub name: String,
    pub checksum: String,
    pub applied_at: DateTime<Utc>,
    pub execution_time_ms: i64,
}

/// Migration manager for handling database schema evolution
pub struct MigrationManager {
    pool: SqlitePool,
    migrations: HashMap<i32, Migration>,
}

impl MigrationManager {
    /// Create a new migration manager
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            migrations: HashMap::new(),
        }
    }

    /// Initialize the migration system by creating the schema_migrations table
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing migration system");

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                checksum TEXT NOT NULL,
                applied_at TEXT NOT NULL,
                execution_time_ms INTEGER NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .context("Failed to create schema_migrations table")?;

        // Create index for performance
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_migrations_version ON schema_migrations(version)",
        )
        .execute(&self.pool)
        .await
        .context("Failed to create migration version index")?;

        info!("Migration system initialized successfully");
        Ok(())
    }

    /// Register migrations for execution
    pub fn register_migrations(&mut self) {
        info!("Registering database migrations");

        // Migration 1: Add missing indexes for performance
        self.register_migration(Migration {
            version: 1,
            name: "add_performance_indexes".to_string(),
            description: "Add database indexes for improved query performance".to_string(),
            up_sql: r#"
                CREATE INDEX IF NOT EXISTS idx_job_descriptions_created_at ON job_descriptions(created_at);
                CREATE INDEX IF NOT EXISTS idx_user_feedback_created_at ON user_feedback(created_at);
                CREATE INDEX IF NOT EXISTS idx_model_performance_updated_at ON model_performance_metrics(updated_at);
                CREATE INDEX IF NOT EXISTS idx_ats_rules_category ON ats_compatibility_rules(category);
                CREATE INDEX IF NOT EXISTS idx_industry_keywords_industry ON industry_keywords(industry);
            "#.to_string(),
            down_sql: r#"
                DROP INDEX IF EXISTS idx_job_descriptions_created_at;
                DROP INDEX IF EXISTS idx_user_feedback_created_at;
                DROP INDEX IF EXISTS idx_model_performance_updated_at;
                DROP INDEX IF EXISTS idx_ats_rules_category;
                DROP INDEX IF EXISTS idx_industry_keywords_industry;
            "#.to_string(),
            checksum: "performance_indexes_v1".to_string(),
        });

        // Migration 2: Add dynamic keyword database tables
        self.register_migration(Migration {
            version: 2,
            name: "add_dynamic_keywords".to_string(),
            description: "Add tables for dynamic keyword tracking and market analysis".to_string(),
            up_sql: r#"
                CREATE TABLE IF NOT EXISTS dynamic_keywords (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    keyword TEXT NOT NULL UNIQUE,
                    industry TEXT NOT NULL,
                    frequency_score REAL NOT NULL DEFAULT 0.0,
                    trend_score REAL NOT NULL DEFAULT 0.0,
                    market_demand REAL NOT NULL DEFAULT 0.0,
                    salary_correlation REAL NOT NULL DEFAULT 0.0,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS keyword_trends (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    keyword_id INTEGER NOT NULL,
                    trend_date TEXT NOT NULL,
                    frequency_count INTEGER NOT NULL,
                    job_postings_count INTEGER NOT NULL,
                    average_salary REAL,
                    FOREIGN KEY (keyword_id) REFERENCES dynamic_keywords (id)
                );

                CREATE TABLE IF NOT EXISTS skill_relationships (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    source_skill TEXT NOT NULL,
                    target_skill TEXT NOT NULL,
                    relationship_type TEXT NOT NULL,
                    strength REAL NOT NULL,
                    industry_context TEXT,
                    created_at TEXT NOT NULL
                );

                CREATE INDEX IF NOT EXISTS idx_dynamic_keywords_industry ON dynamic_keywords(industry);
                CREATE INDEX IF NOT EXISTS idx_keyword_trends_date ON keyword_trends(trend_date);
                CREATE INDEX IF NOT EXISTS idx_skill_relationships_source ON skill_relationships(source_skill);
            "#.to_string(),
            down_sql: r#"
                DROP INDEX IF EXISTS idx_skill_relationships_source;
                DROP INDEX IF EXISTS idx_keyword_trends_date;
                DROP INDEX IF EXISTS idx_dynamic_keywords_industry;
                DROP TABLE IF EXISTS skill_relationships;
                DROP TABLE IF EXISTS keyword_trends;
                DROP TABLE IF EXISTS dynamic_keywords;
            "#.to_string(),
            checksum: "dynamic_keywords_v1".to_string(),
        });

        // Migration 3: Add ML optimization tracking tables
        self.register_migration(Migration {
            version: 3,
            name: "add_ml_optimization".to_string(),
            description: "Add tables for ML optimization tracking and user learning patterns".to_string(),
            up_sql: r#"
                CREATE TABLE IF NOT EXISTS user_optimization_history (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    user_id TEXT NOT NULL,
                    optimization_type TEXT NOT NULL,
                    parameters_before TEXT NOT NULL, -- JSON
                    parameters_after TEXT NOT NULL,  -- JSON
                    performance_improvement REAL NOT NULL,
                    created_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS learning_patterns (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    user_id TEXT NOT NULL,
                    pattern_type TEXT NOT NULL,
                    pattern_data TEXT NOT NULL, -- JSON
                    confidence_score REAL NOT NULL,
                    last_updated TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS model_performance_tracking (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    model_name TEXT NOT NULL,
                    metric_name TEXT NOT NULL,
                    metric_value REAL NOT NULL,
                    measurement_context TEXT, -- JSON
                    measured_at TEXT NOT NULL
                );

                CREATE INDEX IF NOT EXISTS idx_optimization_history_user ON user_optimization_history(user_id);
                CREATE INDEX IF NOT EXISTS idx_learning_patterns_user ON learning_patterns(user_id);
                CREATE INDEX IF NOT EXISTS idx_model_tracking_name ON model_performance_tracking(model_name);
            "#.to_string(),
            down_sql: r#"
                DROP INDEX IF EXISTS idx_model_tracking_name;
                DROP INDEX IF EXISTS idx_learning_patterns_user;
                DROP INDEX IF EXISTS idx_optimization_history_user;
                DROP TABLE IF EXISTS model_performance_tracking;
                DROP TABLE IF EXISTS learning_patterns;
                DROP TABLE IF EXISTS user_optimization_history;
            "#.to_string(),
            checksum: "ml_optimization_v1".to_string(),
        });

        // Migration 4: Add context-aware matching result caching
        self.register_migration(Migration {
            version: 4,
            name: "add_context_matching_cache".to_string(),
            description: "Add caching tables for context-aware matching results".to_string(),
            up_sql: r#"
                CREATE TABLE IF NOT EXISTS context_match_cache (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    resume_hash TEXT NOT NULL,
                    job_description_hash TEXT NOT NULL,
                    industry TEXT NOT NULL,
                    match_result TEXT NOT NULL, -- JSON
                    confidence_score REAL NOT NULL,
                    cached_at TEXT NOT NULL,
                    expires_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS semantic_analysis_cache (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    content_hash TEXT NOT NULL UNIQUE,
                    analysis_type TEXT NOT NULL,
                    analysis_result TEXT NOT NULL, -- JSON
                    cached_at TEXT NOT NULL,
                    expires_at TEXT NOT NULL
                );

                CREATE INDEX IF NOT EXISTS idx_context_cache_hashes ON context_match_cache(resume_hash, job_description_hash);
                CREATE INDEX IF NOT EXISTS idx_context_cache_expires ON context_match_cache(expires_at);
                CREATE INDEX IF NOT EXISTS idx_semantic_cache_expires ON semantic_analysis_cache(expires_at);
            "#.to_string(),
            down_sql: r#"
                DROP INDEX IF EXISTS idx_semantic_cache_expires;
                DROP INDEX IF EXISTS idx_context_cache_expires;
                DROP INDEX IF EXISTS idx_context_cache_hashes;
                DROP TABLE IF EXISTS semantic_analysis_cache;
                DROP TABLE IF EXISTS context_match_cache;
            "#.to_string(),
            checksum: "context_matching_cache_v1".to_string(),
        });

        // Migration 5: Add document parsing metadata and version tracking
        self.register_migration(Migration {
            version: 5,
            name: "add_document_versioning".to_string(),
            description: "Add version tracking and metadata for parsed documents".to_string(),
            up_sql: r#"
                ALTER TABLE resumes ADD COLUMN version INTEGER DEFAULT 1;
                ALTER TABLE resumes ADD COLUMN document_hash TEXT;
                ALTER TABLE resumes ADD COLUMN metadata TEXT; -- JSON for document metadata
                ALTER TABLE resumes ADD COLUMN parsing_errors TEXT; -- JSON for any parsing issues

                CREATE TABLE IF NOT EXISTS document_versions (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    resume_id TEXT NOT NULL,
                    version INTEGER NOT NULL,
                    content TEXT NOT NULL,
                    changes_summary TEXT,
                    created_at TEXT NOT NULL,
                    FOREIGN KEY (resume_id) REFERENCES resumes (id)
                );

                CREATE INDEX IF NOT EXISTS idx_resumes_hash ON resumes(document_hash);
                CREATE INDEX IF NOT EXISTS idx_document_versions_resume ON document_versions(resume_id, version);
            "#.to_string(),
            down_sql: r#"
                DROP INDEX IF EXISTS idx_document_versions_resume;
                DROP INDEX IF EXISTS idx_resumes_hash;
                DROP TABLE IF EXISTS document_versions;
                -- Note: Cannot drop columns in SQLite, they would remain
            "#.to_string(),
            checksum: "document_versioning_v1".to_string(),
        });

        info!("Registered {} migrations", self.migrations.len());
    }

    /// Register a single migration
    fn register_migration(&mut self, migration: Migration) {
        self.migrations.insert(migration.version, migration);
    }

    /// Get current schema version
    pub async fn get_current_version(&self) -> Result<i32> {
        let row = sqlx::query("SELECT COALESCE(MAX(version), 0) as version FROM schema_migrations")
            .fetch_one(&self.pool)
            .await
            .context("Failed to fetch current schema version")?;

        Ok(row.get("version"))
    }

    /// Get all applied migrations
    pub async fn get_applied_migrations(&self) -> Result<Vec<MigrationRecord>> {
        let rows = sqlx::query(
            r#"
            SELECT version, name, checksum, applied_at, execution_time_ms
            FROM schema_migrations
            ORDER BY version ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch applied migrations")?;

        let mut migrations = Vec::new();
        for row in rows {
            let applied_at_str: String = row.get("applied_at");
            let applied_at = applied_at_str
                .parse::<DateTime<Utc>>()
                .context("Failed to parse applied_at timestamp")?;

            migrations.push(MigrationRecord {
                version: row.get("version"),
                name: row.get("name"),
                checksum: row.get("checksum"),
                applied_at,
                execution_time_ms: row.get("execution_time_ms"),
            });
        }

        Ok(migrations)
    }

    /// Get schema version information
    pub async fn get_schema_version(&self) -> Result<SchemaVersion> {
        let current_version = self.get_current_version().await?;
        let applied_migrations = self.get_applied_migrations().await?;

        let latest_available = self.migrations.keys().max().copied().unwrap_or(0);
        let is_up_to_date = current_version == latest_available;

        let pending_migrations = self
            .migrations
            .values()
            .filter(|m| m.version > current_version)
            .cloned()
            .collect();

        Ok(SchemaVersion {
            current_version,
            latest_available,
            is_up_to_date,
            pending_migrations,
            applied_migrations,
        })
    }

    /// Run all pending migrations
    pub async fn migrate(&self) -> Result<Vec<MigrationResult>> {
        info!("Starting database migration");

        let current_version = self.get_current_version().await?;
        let mut results = Vec::new();

        // Get migrations to apply (sorted by version)
        let mut pending_migrations: Vec<_> = self
            .migrations
            .values()
            .filter(|m| m.version > current_version)
            .collect();
        pending_migrations.sort_by_key(|m| m.version);

        if pending_migrations.is_empty() {
            info!("No pending migrations to apply");
            return Ok(results);
        }

        info!("Applying {} pending migrations", pending_migrations.len());

        for migration in pending_migrations {
            let start_time = chrono::Utc::now();
            let result = self.apply_migration(migration).await;
            let execution_time = chrono::Utc::now().signed_duration_since(start_time);

            let migration_result = match result {
                Ok(()) => {
                    info!("Migration {} applied successfully", migration.version);
                    MigrationResult {
                        version: migration.version,
                        success: true,
                        executed_at: start_time,
                        execution_time_ms: execution_time.num_milliseconds(),
                        error_message: None,
                    }
                }
                Err(e) => {
                    warn!("Migration {} failed: {}", migration.version, e);
                    MigrationResult {
                        version: migration.version,
                        success: false,
                        executed_at: start_time,
                        execution_time_ms: execution_time.num_milliseconds(),
                        error_message: Some(e.to_string()),
                    }
                }
            };

            let should_continue = migration_result.success;
            results.push(migration_result);

            if !should_continue {
                warn!("Stopping migration due to failure");
                break;
            }
        }

        info!("Migration completed with {} results", results.len());
        Ok(results)
    }

    /// Apply a single migration
    async fn apply_migration(&self, migration: &Migration) -> Result<()> {
        info!(
            "Applying migration {}: {}",
            migration.version, migration.name
        );

        // Start transaction for atomic migration
        let mut tx = self
            .pool
            .begin()
            .await
            .context("Failed to start migration transaction")?;

        // Check if migration was already applied (safety check)
        let existing = sqlx::query("SELECT version FROM schema_migrations WHERE version = ?")
            .bind(migration.version)
            .fetch_optional(&mut *tx)
            .await
            .context("Failed to check existing migration")?;

        if existing.is_some() {
            return Err(anyhow!(
                "Migration {} is already applied",
                migration.version
            ));
        }

        // Execute migration SQL
        for statement in migration.up_sql.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() {
                sqlx::query(statement)
                    .execute(&mut *tx)
                    .await
                    .with_context(|| {
                        format!("Failed to execute migration statement: {}", statement)
                    })?;
            }
        }

        // Record migration in schema_migrations table
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            r#"
            INSERT INTO schema_migrations (version, name, checksum, applied_at, execution_time_ms)
            VALUES (?, ?, ?, ?, 0)
            "#,
        )
        .bind(migration.version)
        .bind(&migration.name)
        .bind(&migration.checksum)
        .bind(&now)
        .execute(&mut *tx)
        .await
        .context("Failed to record migration")?;

        // Commit transaction
        tx.commit()
            .await
            .context("Failed to commit migration transaction")?;

        Ok(())
    }

    /// Rollback a migration (if down_sql is provided)
    pub async fn rollback(&self, version: i32) -> Result<MigrationResult> {
        info!("Rolling back migration {}", version);

        let migration = self
            .migrations
            .get(&version)
            .ok_or_else(|| anyhow!("Migration {} not found", version))?;

        if migration.down_sql.trim().is_empty() {
            return Err(anyhow!("Migration {} has no rollback SQL", version));
        }

        let start_time = chrono::Utc::now();
        let result = self.apply_rollback(migration).await;
        let execution_time = chrono::Utc::now().signed_duration_since(start_time);

        let migration_result = match result {
            Ok(()) => {
                info!("Migration {} rolled back successfully", version);
                MigrationResult {
                    version,
                    success: true,
                    executed_at: start_time,
                    execution_time_ms: execution_time.num_milliseconds(),
                    error_message: None,
                }
            }
            Err(e) => {
                warn!("Migration {} rollback failed: {}", version, e);
                MigrationResult {
                    version,
                    success: false,
                    executed_at: start_time,
                    execution_time_ms: execution_time.num_milliseconds(),
                    error_message: Some(e.to_string()),
                }
            }
        };

        Ok(migration_result)
    }

    /// Apply rollback for a migration
    async fn apply_rollback(&self, migration: &Migration) -> Result<()> {
        // Start transaction for atomic rollback
        let mut tx = self
            .pool
            .begin()
            .await
            .context("Failed to start rollback transaction")?;

        // Execute rollback SQL
        for statement in migration.down_sql.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() {
                sqlx::query(statement)
                    .execute(&mut *tx)
                    .await
                    .with_context(|| {
                        format!("Failed to execute rollback statement: {}", statement)
                    })?;
            }
        }

        // Remove migration record
        sqlx::query("DELETE FROM schema_migrations WHERE version = ?")
            .bind(migration.version)
            .execute(&mut *tx)
            .await
            .context("Failed to remove migration record")?;

        // Commit transaction
        tx.commit()
            .await
            .context("Failed to commit rollback transaction")?;

        Ok(())
    }

    /// Verify migration integrity by checking checksums
    pub async fn verify_integrity(&self) -> Result<Vec<String>> {
        info!("Verifying migration integrity");

        let applied = self.get_applied_migrations().await?;
        let mut issues = Vec::new();

        for applied_migration in applied {
            if let Some(registered_migration) = self.migrations.get(&applied_migration.version) {
                if registered_migration.checksum != applied_migration.checksum {
                    issues.push(format!(
                        "Migration {} checksum mismatch: expected '{}', found '{}'",
                        applied_migration.version,
                        registered_migration.checksum,
                        applied_migration.checksum
                    ));
                }
            } else {
                issues.push(format!(
                    "Applied migration {} not found in registered migrations",
                    applied_migration.version
                ));
            }
        }

        if issues.is_empty() {
            info!("Migration integrity verification passed");
        } else {
            warn!("Found {} migration integrity issues", issues.len());
        }

        Ok(issues)
    }

    /// Clean up expired cache entries (utility function)
    pub async fn cleanup_expired_cache(&self) -> Result<u64> {
        info!("Cleaning up expired cache entries");

        let now = Utc::now().to_rfc3339();
        let mut total_cleaned = 0u64;

        // Clean context match cache
        let result = sqlx::query("DELETE FROM context_match_cache WHERE expires_at < ?")
            .bind(&now)
            .execute(&self.pool)
            .await;

        if let Ok(result) = result {
            total_cleaned += result.rows_affected();
        }

        // Clean semantic analysis cache
        let result = sqlx::query("DELETE FROM semantic_analysis_cache WHERE expires_at < ?")
            .bind(&now)
            .execute(&self.pool)
            .await;

        if let Ok(result) = result {
            total_cleaned += result.rows_affected();
        }

        info!("Cleaned up {} expired cache entries", total_cleaned);
        Ok(total_cleaned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    use tempfile::NamedTempFile;

    async fn create_test_db() -> SqlitePool {
        let temp_file = NamedTempFile::new().unwrap();
        let database_url = format!("sqlite://{}", temp_file.path().display());
        SqlitePool::connect(&database_url).await.unwrap()
    }

    #[tokio::test]
    async fn test_migration_system_initialization() {
        let pool = create_test_db().await;
        let migration_manager = MigrationManager::new(pool);

        let result = migration_manager.initialize().await;
        assert!(result.is_ok());

        let version = migration_manager.get_current_version().await.unwrap();
        assert_eq!(version, 0);
    }

    #[tokio::test]
    async fn test_migration_registration_and_execution() {
        let pool = create_test_db().await;
        let mut migration_manager = MigrationManager::new(pool);

        migration_manager.initialize().await.unwrap();
        migration_manager.register_migrations();

        let schema_version = migration_manager.get_schema_version().await.unwrap();
        assert!(!schema_version.pending_migrations.is_empty());

        let results = migration_manager.migrate().await.unwrap();
        assert!(!results.is_empty());

        let final_version = migration_manager.get_current_version().await.unwrap();
        assert!(final_version > 0);
    }
}
