use anyhow::{anyhow, Result};
use chrono::Utc;
use log::{info, warn};
use serde_json::json;
use std::path::{Component, Path, PathBuf};

use crate::models::Analysis;

/// Security module for path validation and safe file operations
pub mod security {
    use super::*;

    /// Validates a file path to prevent path traversal attacks
    ///
    /// # Security Checks
    /// - Rejects absolute paths
    /// - Blocks parent directory traversal (..)
    /// - Ensures path stays within allowed base directory
    /// - Sanitizes path components
    ///
    /// # Arguments
    /// * `path` - The path to validate
    /// * `allowed_base` - Optional base directory to restrict access to
    ///
    /// # Returns
    /// * `Ok(PathBuf)` - Validated and canonicalized path
    /// * `Err` - Security violation detected
    pub fn validate_file_path(path: &str, allowed_base: Option<&str>) -> Result<PathBuf> {
        // Reject empty paths
        if path.is_empty() {
            return Err(anyhow!("Empty path not allowed"));
        }

        // Reject paths with null bytes (security vulnerability)
        if path.contains('\0') {
            warn!(
                "Path traversal attempt blocked: null byte in path: {:?}",
                path
            );
            return Err(anyhow!("Invalid characters in path"));
        }

        let path = Path::new(path);

        // Reject absolute paths
        if path.is_absolute() {
            warn!("Path traversal attempt blocked: absolute path: {:?}", path);
            return Err(anyhow!("Absolute paths not allowed"));
        }

        // Check each component for security violations
        for component in path.components() {
            match component {
                Component::ParentDir => {
                    warn!(
                        "Path traversal attempt blocked: parent directory traversal in: {:?}",
                        path
                    );
                    return Err(anyhow!("Path traversal not allowed"));
                }
                Component::Normal(s) => {
                    let component_str = s.to_string_lossy();
                    // Block Windows reserved names
                    if is_windows_reserved_name(&component_str) {
                        warn!("Blocked Windows reserved name: {:?}", component_str);
                        return Err(anyhow!("Reserved filename not allowed"));
                    }
                    // Block problematic characters
                    if component_str
                        .chars()
                        .any(|c| matches!(c, '<' | '>' | ':' | '"' | '|' | '?' | '*'))
                    {
                        warn!(
                            "Blocked invalid characters in filename: {:?}",
                            component_str
                        );
                        return Err(anyhow!("Invalid characters in filename"));
                    }
                }
                _ => {} // Allow current dir, root, prefix components
            }
        }

        // If allowed_base is specified, ensure path stays within it
        if let Some(base) = allowed_base {
            let base_path = Path::new(base);
            let resolved_path = base_path.join(path);

            // Try to canonicalize to resolve any remaining .. or symlinks
            match resolved_path.canonicalize() {
                Ok(canonical) => {
                    match base_path.canonicalize() {
                        Ok(canonical_base) => {
                            if !canonical.starts_with(&canonical_base) {
                                warn!(
                                    "Path escape attempt blocked: {:?} outside base {:?}",
                                    canonical, canonical_base
                                );
                                return Err(anyhow!("Path outside allowed directory"));
                            }
                            return Ok(canonical);
                        }
                        Err(_) => {
                            // Base directory doesn't exist yet, validate without canonicalization
                            let absolute_path = if base_path.is_absolute() {
                                base_path.join(path)
                            } else {
                                std::env::current_dir()?.join(base_path).join(path)
                            };
                            return Ok(absolute_path);
                        }
                    }
                }
                Err(_) => {
                    // File doesn't exist yet, validate path structure only
                    let absolute_path = if base_path.is_absolute() {
                        base_path.join(path)
                    } else {
                        std::env::current_dir()?.join(base_path).join(path)
                    };
                    return Ok(absolute_path);
                }
            }
        }

        // Convert relative path to absolute for consistency
        let absolute_path = std::env::current_dir()?.join(path);
        Ok(absolute_path)
    }

    /// Checks if a filename is a Windows reserved name
    fn is_windows_reserved_name(name: &str) -> bool {
        let upper_name = name.to_uppercase();
        let reserved_names = [
            "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
            "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
        ];

        reserved_names.contains(&upper_name.as_str())
            || reserved_names
                .iter()
                .any(|&reserved| upper_name.starts_with(&format!("{}.", reserved)))
    }

    /// Sanitizes a filename for safe use in file operations
    ///
    /// # Arguments
    /// * `filename` - The filename to sanitize
    ///
    /// # Returns
    /// * Sanitized filename safe for file operations
    pub fn sanitize_filename(filename: &str) -> String {
        filename
            .chars()
            .filter(|&c| {
                !matches!(
                    c,
                    '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' | '\0'
                )
            })
            .collect::<String>()
            .trim()
            .to_string()
    }
}

pub async fn export_data(analyses: &[Analysis], format: &str) -> Result<String> {
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");

    // Sanitize format input to prevent path injection
    let safe_format = security::sanitize_filename(format);
    if safe_format.is_empty() || safe_format != format {
        return Err(anyhow!("Invalid export format"));
    }

    let filename = format!(
        "ats_analysis_export_{}_{}.{}",
        timestamp,
        analyses.len(),
        safe_format
    );

    // Validate the generated filename is safe
    security::validate_file_path(&filename, Some("./exports"))?;

    let file_path = Path::new("./exports").join(&filename);

    // Create exports directory if it doesn't exist
    tokio::fs::create_dir_all("./exports").await?;

    match format.to_lowercase().as_str() {
        "json" => export_json(analyses, &file_path).await?,
        "csv" => export_csv(analyses, &file_path).await?,
        "txt" => export_txt(analyses, &file_path).await?,
        _ => return Err(anyhow!("Unsupported export format: {}", format)),
    }

    info!(
        "Exported {} analyses to {}",
        analyses.len(),
        file_path.display()
    );
    Ok(file_path.to_string_lossy().to_string())
}

async fn export_json(analyses: &[Analysis], file_path: &Path) -> Result<()> {
    let export_data = json!({
        "export_timestamp": Utc::now().to_rfc3339(),
        "total_analyses": analyses.len(),
        "analyses": analyses
    });

    let json_string = serde_json::to_string_pretty(&export_data)?;
    tokio::fs::write(file_path, json_string).await?;

    Ok(())
}

async fn export_csv(analyses: &[Analysis], file_path: &Path) -> Result<()> {
    let mut csv_content = String::new();

    // Header
    csv_content.push_str("ID,Resume ID,Job Description ID,Model Used,Overall Score,Skills Score,Experience Score,Education Score,Keywords Score,Format Score,Processing Time (ms),Created At\n");

    // Data rows
    for analysis in analyses {
        csv_content.push_str(&format!(
            "{},{},{},{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{},{}\n",
            analysis.id,
            analysis.resume_id,
            analysis.job_description_id,
            analysis.model_used,
            analysis.overall_score,
            analysis.skills_score,
            analysis.experience_score,
            analysis.education_score,
            analysis.keywords_score,
            analysis.format_score,
            analysis.processing_time_ms,
            analysis.created_at.to_rfc3339()
        ));
    }

    tokio::fs::write(file_path, csv_content).await?;
    Ok(())
}

async fn export_txt(analyses: &[Analysis], file_path: &Path) -> Result<()> {
    let mut txt_content = String::new();

    txt_content.push_str("ATS Analysis Export Report\n");
    txt_content.push_str(&format!(
        "Generated: {}\n",
        Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));
    txt_content.push_str(&format!("Total Analyses: {}\n\n", analyses.len()));
    txt_content.push_str("=".repeat(80).as_str());
    txt_content.push_str("\n\n");

    for (i, analysis) in analyses.iter().enumerate() {
        txt_content.push_str(&format!("Analysis #{}\n", i + 1));
        txt_content.push_str(&format!("ID: {}\n", analysis.id));
        txt_content.push_str(&format!("Model Used: {}\n", analysis.model_used));
        txt_content.push_str(&format!("Overall Score: {:.1}%\n", analysis.overall_score));
        txt_content.push_str(&format!("  - Skills: {:.1}%\n", analysis.skills_score));
        txt_content.push_str(&format!(
            "  - Experience: {:.1}%\n",
            analysis.experience_score
        ));
        txt_content.push_str(&format!(
            "  - Education: {:.1}%\n",
            analysis.education_score
        ));
        txt_content.push_str(&format!("  - Keywords: {:.1}%\n", analysis.keywords_score));
        txt_content.push_str(&format!("  - Format: {:.1}%\n", analysis.format_score));
        txt_content.push_str(&format!(
            "Processing Time: {}ms\n",
            analysis.processing_time_ms
        ));
        txt_content.push_str(&format!(
            "Created: {}\n",
            analysis.created_at.format("%Y-%m-%d %H:%M:%S UTC")
        ));

        if !analysis.detailed_feedback.is_empty() {
            txt_content.push_str(&format!(
                "\nDetailed Feedback:\n{}\n",
                analysis.detailed_feedback
            ));
        }

        if !analysis.recommendations.is_empty() {
            txt_content.push_str(&format!(
                "\nRecommendations:\n{}\n",
                analysis.recommendations
            ));
        }

        txt_content.push('\n');
        txt_content.push_str("-".repeat(80).as_str());
        txt_content.push_str("\n\n");
    }

    tokio::fs::write(file_path, txt_content).await?;
    Ok(())
}
