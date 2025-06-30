use anyhow::{anyhow, Result};
use chrono::Utc;
use log::info;
use serde_json::json;
use std::path::Path;

use crate::models::{Analysis};

pub async fn export_data(analyses: &[Analysis], format: &str) -> Result<String> {
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("ats_analysis_export_{}_{}.{}", timestamp, analyses.len(), format);
    let file_path = Path::new("./exports").join(&filename);
    
    // Create exports directory if it doesn't exist
    tokio::fs::create_dir_all("./exports").await?;
    
    match format.to_lowercase().as_str() {
        "json" => export_json(analyses, &file_path).await?,
        "csv" => export_csv(analyses, &file_path).await?,
        "txt" => export_txt(analyses, &file_path).await?,
        _ => return Err(anyhow!("Unsupported export format: {}", format)),
    }
    
    info!("Exported {} analyses to {}", analyses.len(), file_path.display());
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
    
    txt_content.push_str(&format!("ATS Analysis Export Report\n"));
    txt_content.push_str(&format!("Generated: {}\n", Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
    txt_content.push_str(&format!("Total Analyses: {}\n\n", analyses.len()));
    txt_content.push_str("=".repeat(80).as_str());
    txt_content.push_str("\n\n");
    
    for (i, analysis) in analyses.iter().enumerate() {
        txt_content.push_str(&format!("Analysis #{}\n", i + 1));
        txt_content.push_str(&format!("ID: {}\n", analysis.id));
        txt_content.push_str(&format!("Model Used: {}\n", analysis.model_used));
        txt_content.push_str(&format!("Overall Score: {:.1}%\n", analysis.overall_score));
        txt_content.push_str(&format!("  - Skills: {:.1}%\n", analysis.skills_score));
        txt_content.push_str(&format!("  - Experience: {:.1}%\n", analysis.experience_score));
        txt_content.push_str(&format!("  - Education: {:.1}%\n", analysis.education_score));
        txt_content.push_str(&format!("  - Keywords: {:.1}%\n", analysis.keywords_score));
        txt_content.push_str(&format!("  - Format: {:.1}%\n", analysis.format_score));
        txt_content.push_str(&format!("Processing Time: {}ms\n", analysis.processing_time_ms));
        txt_content.push_str(&format!("Created: {}\n", analysis.created_at.format("%Y-%m-%d %H:%M:%S UTC")));
        
        if !analysis.detailed_feedback.is_empty() {
            txt_content.push_str(&format!("\nDetailed Feedback:\n{}\n", analysis.detailed_feedback));
        }
        
        if !analysis.recommendations.is_empty() {
            txt_content.push_str(&format!("\nRecommendations:\n{}\n", analysis.recommendations));
        }
        
        txt_content.push_str("\n");
        txt_content.push_str("-".repeat(80).as_str());
        txt_content.push_str("\n\n");
    }
    
    tokio::fs::write(file_path, txt_content).await?;
    Ok(())
}


