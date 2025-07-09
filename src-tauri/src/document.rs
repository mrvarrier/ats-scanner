use anyhow::{anyhow, Result};
use log::{error, info, warn};
use mime_guess::from_path;
use regex::Regex;
use std::path::Path;

use crate::models::DocumentInfo;

pub struct DocumentParser;

impl DocumentParser {
    pub async fn parse_file(file_path: &str) -> Result<DocumentInfo> {
        info!("Parsing document: {}", file_path);

        let path = Path::new(file_path);
        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mime_type = from_path(path).first_or_octet_stream();
        let file_type = Self::determine_file_type(mime_type.as_ref());

        // Read file content
        let file_content = tokio::fs::read(file_path).await?;
        let size = file_content.len();

        // Parse based on file type
        let content = match file_type.as_str() {
            "pdf" => Self::parse_pdf(&file_content).await?,
            "docx" => {
                return Err(anyhow!(
                    "DOCX parsing temporarily disabled due to dependency issues"
                ))
            }
            "txt" => Self::parse_text(&file_content).await?,
            _ => return Err(anyhow!("Unsupported file type: {}", file_type)),
        };

        // Clean and validate content
        let cleaned_content = Self::clean_text(&content);

        if cleaned_content.trim().is_empty() {
            warn!("No text content extracted from file: {}", filename);
        }

        Ok(DocumentInfo {
            id: uuid::Uuid::new_v4().to_string(),
            filename,
            file_type,
            size,
            content: cleaned_content,
        })
    }

    #[allow(dead_code)]
    pub async fn parse_content(content: &[u8], filename: &str) -> Result<DocumentInfo> {
        info!("Parsing document content for: {}", filename);

        let file_type = Self::determine_file_type_from_filename(filename);
        let size = content.len();

        // Parse based on file type
        let parsed_content = match file_type.as_str() {
            "pdf" => Self::parse_pdf(content).await?,
            "docx" => {
                return Err(anyhow!(
                    "DOCX parsing temporarily disabled due to dependency issues"
                ))
            }
            "txt" => Self::parse_text(content).await?,
            _ => return Err(anyhow!("Unsupported file type: {}", file_type)),
        };

        // Clean and validate content
        let cleaned_content = Self::clean_text(&parsed_content);

        if cleaned_content.trim().is_empty() {
            warn!("No text content extracted from: {}", filename);
        }

        Ok(DocumentInfo {
            id: uuid::Uuid::new_v4().to_string(),
            filename: filename.to_string(),
            file_type,
            size,
            content: cleaned_content,
        })
    }

    async fn parse_pdf(content: &[u8]) -> Result<String> {
        info!("Parsing PDF document");

        match pdf_extract::extract_text_from_mem(content) {
            Ok(text) => {
                info!("Successfully extracted text from PDF");
                Ok(text)
            }
            Err(e) => {
                error!("Failed to parse PDF: {}", e);
                Err(anyhow!("Failed to parse PDF: {}", e))
            }
        }
    }

    // Temporarily disabled due to jetscii dependency issues
    // async fn parse_docx(content: &[u8]) -> Result<String> {
    //     info!("Parsing DOCX document");
    //
    //     // For now, use a simple approach - in a real implementation you'd use proper DOCX parsing
    //     // The docx crate API keeps changing, so let's use a more basic approach
    //     match std::str::from_utf8(content) {
    //         Ok(text) => {
    //             info!("Successfully extracted text from DOCX (basic method)");
    //             Ok(text.to_string())
    //         }
    //         Err(_) => {
    //             // Fallback to lossy conversion
    //             warn!("Using lossy conversion for DOCX file");
    //             let text = String::from_utf8_lossy(content).to_string();
    //             Ok(text)
    //         }
    //     }
    // }

    async fn parse_text(content: &[u8]) -> Result<String> {
        info!("Parsing text document");

        // Try UTF-8 first
        if let Ok(text) = String::from_utf8(content.to_vec()) {
            return Ok(text);
        }

        // Try other common encodings
        match encoding_rs::UTF_8.decode(content) {
            (text, _, false) => Ok(text.to_string()),
            _ => {
                // Fallback to lossy UTF-8 conversion
                warn!("Using lossy UTF-8 conversion for text file");
                Ok(String::from_utf8_lossy(content).to_string())
            }
        }
    }

    fn determine_file_type(mime_type: &str) -> String {
        match mime_type {
            "application/pdf" => "pdf".to_string(),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
                "docx".to_string()
            }
            "text/plain" => "txt".to_string(),
            _ => {
                warn!("Unknown MIME type: {}, defaulting to txt", mime_type);
                "txt".to_string()
            }
        }
    }

    fn determine_file_type_from_filename(filename: &str) -> String {
        let path = Path::new(filename);
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "pdf" => "pdf".to_string(),
            "docx" => "docx".to_string(),
            "txt" | "text" => "txt".to_string(),
            _ => {
                warn!("Unknown file extension: {}, defaulting to txt", extension);
                "txt".to_string()
            }
        }
    }

    fn clean_text(text: &str) -> String {
        // Remove excessive whitespace
        let whitespace_regex = Regex::new(r"\s+").unwrap();
        let cleaned = whitespace_regex.replace_all(text, " ");

        // Remove control characters except newlines and tabs
        let control_regex = Regex::new(r"[\x00-\x08\x0B\x0C\x0E-\x1F\x7F]").unwrap();
        let cleaned = control_regex.replace_all(&cleaned, "");

        // Normalize line breaks
        let line_break_regex = Regex::new(r"\r\n|\r").unwrap();
        let cleaned = line_break_regex.replace_all(&cleaned, "\n");

        // Remove excessive line breaks
        let excessive_breaks_regex = Regex::new(r"\n{3,}").unwrap();
        let cleaned = excessive_breaks_regex.replace_all(&cleaned, "\n\n");

        cleaned.trim().to_string()
    }

    #[allow(dead_code)]
    pub fn extract_sections(content: &str) -> Result<ResumeStructure> {
        info!("Extracting resume sections");

        let _content_lower = content.to_lowercase();
        let lines: Vec<&str> = content.lines().collect();

        let mut structure = ResumeStructure::new();

        // Extract contact information (usually at the top)
        structure.contact_info =
            Self::extract_contact_info(&lines[..5.min(lines.len())].join("\n"));

        // Extract sections based on common headers
        structure.experience = Self::extract_section(
            content,
            &[
                "experience",
                "work experience",
                "professional experience",
                "employment",
                "career history",
                "work history",
            ],
        );

        structure.education = Self::extract_section(
            content,
            &[
                "education",
                "academic background",
                "qualifications",
                "degrees",
            ],
        );

        structure.skills = Self::extract_section(
            content,
            &[
                "skills",
                "technical skills",
                "core competencies",
                "expertise",
                "proficiencies",
                "technologies",
            ],
        );

        structure.summary = Self::extract_section(
            content,
            &["summary", "profile", "objective", "about", "overview"],
        );

        Ok(structure)
    }

    #[allow(dead_code)]
    fn extract_contact_info(text: &str) -> ContactInfo {
        let email_regex =
            Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
        let phone_regex =
            Regex::new(r"(\+?1[-.\s]?)?\(?([0-9]{3})\)?[-.\s]?([0-9]{3})[-.\s]?([0-9]{4})")
                .unwrap();
        let linkedin_regex =
            Regex::new(r"(?:linkedin\.com/in/|linkedin\.com/pub/)([A-Za-z0-9-]+)").unwrap();

        ContactInfo {
            email: email_regex.find(text).map(|m| m.as_str().to_string()),
            phone: phone_regex.find(text).map(|m| m.as_str().to_string()),
            linkedin: linkedin_regex.find(text).map(|m| m.as_str().to_string()),
        }
    }

    #[allow(dead_code)]
    fn extract_section(content: &str, section_headers: &[&str]) -> Option<String> {
        let _content_lower = content.to_lowercase();
        let lines: Vec<&str> = content.lines().collect();

        for header in section_headers {
            let header_regex =
                Regex::new(&format!(r"(?i)^.*{}.*$", regex::escape(header))).unwrap();

            for (i, line) in lines.iter().enumerate() {
                if header_regex.is_match(line) {
                    // Found section header, extract content until next section or end
                    let mut section_content = Vec::new();
                    let mut j = i + 1;

                    while j < lines.len() {
                        let current_line = lines[j].trim();

                        // Check if this line looks like a new section header
                        if Self::is_section_header(current_line) {
                            break;
                        }

                        if !current_line.is_empty() {
                            section_content.push(current_line);
                        }

                        j += 1;
                    }

                    if !section_content.is_empty() {
                        return Some(section_content.join("\n"));
                    }
                }
            }
        }

        None
    }

    #[allow(dead_code)]
    fn is_section_header(line: &str) -> bool {
        let common_headers = [
            "experience",
            "education",
            "skills",
            "summary",
            "objective",
            "work experience",
            "professional experience",
            "academic background",
            "technical skills",
            "core competencies",
            "employment",
            "qualifications",
        ];

        let line_lower = line.to_lowercase();

        // Check if line contains common section headers
        for header in &common_headers {
            if line_lower.contains(header) && line.len() < 50 {
                return true;
            }
        }

        // Check if line is all caps (common for headers)
        if line.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) && line.len() < 30 {
            return true;
        }

        false
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ResumeStructure {
    pub contact_info: ContactInfo,
    pub summary: Option<String>,
    pub experience: Option<String>,
    pub education: Option<String>,
    pub skills: Option<String>,
}

impl Default for ResumeStructure {
    fn default() -> Self {
        Self::new()
    }
}

impl ResumeStructure {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            contact_info: ContactInfo::default(),
            summary: None,
            experience: None,
            education: None,
            skills: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct ContactInfo {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub linkedin: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_RESUME_TEXT: &str = r#"
John Doe
Software Engineer
john.doe@email.com
(555) 123-4567
linkedin.com/in/johndoe

SUMMARY
Experienced software engineer with 5+ years in web development

EXPERIENCE
Senior Software Engineer - Tech Corp (2020-2023)
• Developed scalable web applications using React and Node.js
• Led team of 4 developers on critical projects
• Improved system performance by 40%

Software Engineer - StartupCo (2018-2020)
• Built REST APIs using Python and Django
• Implemented CI/CD pipelines

EDUCATION
Bachelor of Computer Science - State University (2018)
• GPA: 3.8/4.0

SKILLS
Python, JavaScript, React, Node.js, Docker, AWS
"#;

    const SAMPLE_JOB_DESCRIPTION: &str = r#"
Software Engineer Position
Tech Company Inc.

We are looking for a Software Engineer with experience in:
• Python programming
• React development
• AWS cloud services
• 3+ years experience
• Bachelor's degree in Computer Science

Requirements:
- Strong programming skills
- Experience with microservices
- Knowledge of Docker and Kubernetes
"#;

    #[tokio::test]
    async fn test_parse_text_content() {
        let content = SAMPLE_RESUME_TEXT.as_bytes();
        let result = DocumentParser::parse_content(content, "resume.txt").await;

        assert!(result.is_ok());
        let doc_info = result.unwrap();
        assert_eq!(doc_info.filename, "resume.txt");
        assert_eq!(doc_info.file_type, "txt");
        assert!(doc_info.content.contains("John Doe"));
        assert!(doc_info.content.contains("Software Engineer"));
    }

    #[tokio::test]
    async fn test_determine_file_type_from_filename() {
        assert_eq!(
            DocumentParser::determine_file_type_from_filename("resume.pdf"),
            "pdf"
        );
        assert_eq!(
            DocumentParser::determine_file_type_from_filename("document.docx"),
            "docx"
        );
        assert_eq!(
            DocumentParser::determine_file_type_from_filename("file.txt"),
            "txt"
        );
        assert_eq!(
            DocumentParser::determine_file_type_from_filename("unknown.xyz"),
            "txt"
        );
    }

    #[tokio::test]
    async fn test_clean_text() {
        let dirty_text = "  Multiple   spaces   \n\n\n\nExcessive\r\nLine\rBreaks  ";
        let cleaned = DocumentParser::clean_text(dirty_text);

        assert!(!cleaned.starts_with(' '));
        assert!(!cleaned.ends_with(' '));
        assert!(!cleaned.contains("   ")); // No triple spaces
        assert!(!cleaned.contains("\n\n\n")); // No triple line breaks
    }

    #[tokio::test]
    async fn test_extract_contact_info() {
        let contact = DocumentParser::extract_contact_info(SAMPLE_RESUME_TEXT);

        assert!(contact.email.is_some());
        assert_eq!(contact.email.unwrap(), "john.doe@email.com");

        assert!(contact.phone.is_some());
        assert!(contact.phone.unwrap().contains("555"));

        assert!(contact.linkedin.is_some());
        assert!(contact.linkedin.unwrap().contains("johndoe"));
    }

    #[tokio::test]
    async fn test_extract_sections() {
        let result = DocumentParser::extract_sections(SAMPLE_RESUME_TEXT);
        assert!(result.is_ok());

        let structure = result.unwrap();

        // Test experience section
        assert!(structure.experience.is_some());
        let experience = structure.experience.unwrap();
        assert!(experience.contains("Senior Software Engineer"));
        assert!(experience.contains("Tech Corp"));

        // Test education section
        assert!(structure.education.is_some());
        let education = structure.education.unwrap();
        assert!(education.contains("Bachelor of Computer Science"));
        assert!(education.contains("State University"));

        // Test skills section
        assert!(structure.skills.is_some());
        let skills = structure.skills.unwrap();
        assert!(skills.contains("Python"));
        assert!(skills.contains("JavaScript"));

        // Test summary section
        assert!(structure.summary.is_some());
        let summary = structure.summary.unwrap();
        assert!(summary.contains("Experienced software engineer"));
    }

    #[tokio::test]
    async fn test_is_section_header() {
        assert!(DocumentParser::is_section_header("EXPERIENCE"));
        assert!(DocumentParser::is_section_header("Education"));
        assert!(DocumentParser::is_section_header("Technical Skills"));
        assert!(DocumentParser::is_section_header("WORK EXPERIENCE"));

        assert!(!DocumentParser::is_section_header("This is a very long line that should not be considered a header because it exceeds the length limit"));
        assert!(!DocumentParser::is_section_header("Regular content line"));
    }

    #[tokio::test]
    async fn test_extract_section_with_different_headers() {
        let content_with_work_experience = r#"
WORK EXPERIENCE
Senior Developer at XYZ Corp
Built amazing software

EDUCATION
Computer Science Degree
"#;

        let result = DocumentParser::extract_sections(content_with_work_experience);
        assert!(result.is_ok());

        let structure = result.unwrap();
        assert!(structure.experience.is_some());
        let experience = structure.experience.unwrap();
        assert!(experience.contains("Senior Developer"));
        assert!(experience.contains("XYZ Corp"));
    }

    #[tokio::test]
    async fn test_contact_info_extraction_edge_cases() {
        // Test with no contact info
        let no_contact = "Just some random text without any contact information";
        let contact = DocumentParser::extract_contact_info(no_contact);

        assert!(contact.email.is_none());
        assert!(contact.phone.is_none());
        assert!(contact.linkedin.is_none());

        // Test with partial contact info
        let partial_contact = "Email: test@example.com\nSome other text";
        let contact = DocumentParser::extract_contact_info(partial_contact);

        assert!(contact.email.is_some());
        assert_eq!(contact.email.unwrap(), "test@example.com");
        assert!(contact.phone.is_none());
    }

    #[tokio::test]
    async fn test_different_phone_formats() {
        let phone_formats = vec![
            "(555) 123-4567",
            "555-123-4567",
            "555.123.4567",
            "5551234567",
            "+1 555 123 4567",
        ];

        for phone in phone_formats {
            let text = format!("Contact: {}", phone);
            let contact = DocumentParser::extract_contact_info(&text);
            assert!(
                contact.phone.is_some(),
                "Failed to extract phone: {}",
                phone
            );
        }
    }

    #[tokio::test]
    async fn test_empty_and_whitespace_content() {
        // Test empty content
        let empty_result = DocumentParser::parse_content(b"", "empty.txt").await;
        assert!(empty_result.is_ok());
        let doc = empty_result.unwrap();
        assert!(doc.content.is_empty());

        // Test whitespace-only content
        let whitespace_result =
            DocumentParser::parse_content(b"   \n\n   \t  ", "whitespace.txt").await;
        assert!(whitespace_result.is_ok());
        let doc = whitespace_result.unwrap();
        assert!(doc.content.trim().is_empty());
    }
}
