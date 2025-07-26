use anyhow::{anyhow, Result};
use chrono::DateTime;
use log::{error, info, warn};
use mime_guess::from_path;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;
use regex::Regex;
use std::io::Cursor;
use std::path::Path;
use zip::ZipArchive;

use crate::models::{
    DocumentContactInfo, DocumentHeading, DocumentInfo, DocumentIssue, DocumentIssueType,
    DocumentMetadata, DocumentQualityMetrics, DocumentSection, DocumentStructure,
    HeadingFormatting, IssueSeverity,
};
use crate::utils::security;

pub struct DocumentParser;

impl DocumentParser {
    pub async fn parse_file(file_path: &str) -> Result<DocumentInfo> {
        info!("Parsing document: {}", file_path);

        // SECURITY: Validate file path to prevent path traversal attacks
        let validated_path = security::validate_file_path(file_path, None).map_err(|e| {
            warn!(
                "Security violation: Invalid file path '{}': {}",
                file_path, e
            );
            anyhow!("Invalid file path")
        })?;

        // Verify file exists and is readable
        if !validated_path.exists() {
            return Err(anyhow!("File does not exist: {}", file_path));
        }

        if !validated_path.is_file() {
            return Err(anyhow!("Path is not a file: {}", file_path));
        }

        let path = Path::new(file_path);
        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mime_type = from_path(path).first_or_octet_stream();
        let file_type = Self::determine_file_type(mime_type.as_ref());

        // Validate file type is allowed for document parsing
        if !Self::is_allowed_file_type(&file_type) {
            warn!(
                "Blocked attempt to parse unsupported file type: {}",
                file_type
            );
            return Err(anyhow!("Unsupported file type: {}", file_type));
        }

        // Read file content using validated path
        let file_content = tokio::fs::read(&validated_path).await?;
        let size = file_content.len();

        // Parse based on file type
        let content = match file_type.as_str() {
            "pdf" => Self::parse_pdf(&file_content).await?,
            "docx" => Self::parse_docx(&file_content).await?,
            "doc" => Self::parse_doc(&file_content).await?,
            "txt" => Self::parse_text(&file_content).await?,
            _ => return Err(anyhow!("Unsupported file type: {}", file_type)),
        };

        // Clean and validate content
        let cleaned_content = Self::clean_text(&content);

        if cleaned_content.trim().is_empty() {
            warn!("No text content extracted from file: {}", filename);
        }

        // Calculate word count and character count
        let word_count = Self::count_words(&cleaned_content);
        let character_count = cleaned_content.chars().count();

        // Extract document metadata
        let metadata = Self::extract_metadata(&file_content, &file_type, file_path).await?;

        // Analyze document structure
        let structure = Self::analyze_document_structure(&cleaned_content);

        // Calculate quality metrics
        let quality_metrics = Self::calculate_quality_metrics(&cleaned_content, &structure);

        Ok(DocumentInfo {
            id: uuid::Uuid::new_v4().to_string(),
            filename,
            file_type,
            size,
            content: cleaned_content,
            word_count,
            character_count,
            metadata,
            structure: Some(structure),
            quality_metrics: Some(quality_metrics),
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
            "docx" => Self::parse_docx(content).await?,
            "doc" => Self::parse_doc(content).await?,
            "txt" => Self::parse_text(content).await?,
            _ => return Err(anyhow!("Unsupported file type: {}", file_type)),
        };

        // Clean and validate content
        let cleaned_content = Self::clean_text(&parsed_content);

        if cleaned_content.trim().is_empty() {
            warn!("No text content extracted from: {}", filename);
        }

        // Calculate word count and character count
        let word_count = Self::count_words(&cleaned_content);
        let character_count = cleaned_content.chars().count();

        // Extract document metadata (limited for content-only parsing)
        let metadata = Self::extract_metadata(content, &file_type, filename).await?;

        // Analyze document structure
        let structure = Self::analyze_document_structure(&cleaned_content);

        // Calculate quality metrics
        let quality_metrics = Self::calculate_quality_metrics(&cleaned_content, &structure);

        Ok(DocumentInfo {
            id: uuid::Uuid::new_v4().to_string(),
            filename: filename.to_string(),
            file_type,
            size,
            content: cleaned_content,
            word_count,
            character_count,
            metadata,
            structure: Some(structure),
            quality_metrics: Some(quality_metrics),
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

    async fn parse_docx(content: &[u8]) -> Result<String> {
        info!("Parsing DOCX document");

        let cursor = Cursor::new(content);
        let mut archive = ZipArchive::new(cursor)
            .map_err(|e| anyhow!("Failed to open DOCX file as ZIP archive: {}", e))?;

        // DOCX files contain the main document in word/document.xml
        let mut document_xml = archive
            .by_name("word/document.xml")
            .map_err(|e| anyhow!("Failed to find document.xml in DOCX file: {}", e))?;

        let mut xml_content = String::new();
        std::io::Read::read_to_string(&mut document_xml, &mut xml_content)
            .map_err(|e| anyhow!("Failed to read document.xml content: {}", e))?;

        // Parse XML and extract text content
        let text = Self::extract_text_from_xml(&xml_content)?;

        if text.trim().is_empty() {
            warn!("No text content found in DOCX document");
        } else {
            info!("Successfully extracted {} characters from DOCX", text.len());
        }

        Ok(text)
    }

    async fn parse_doc(content: &[u8]) -> Result<String> {
        info!("Parsing legacy DOC document");

        // Legacy .doc files use the Compound Document format
        // Try to extract text using a simple approach
        match Self::extract_doc_text_simple(content) {
            Ok(text) => {
                if text.trim().is_empty() {
                    warn!("No text content found in DOC document");
                } else {
                    info!("Successfully extracted {} characters from DOC", text.len());
                }
                Ok(text)
            }
            Err(e) => {
                warn!(
                    "Failed to parse DOC file: {}, falling back to binary text extraction",
                    e
                );
                Self::extract_text_from_binary(content)
            }
        }
    }

    /// Simple DOC text extraction by searching for readable text
    fn extract_doc_text_simple(content: &[u8]) -> Result<String> {
        // DOC files store text in various places, we'll try to find readable chunks
        let mut extracted_text = Vec::new();
        let mut current_text = Vec::new();

        // Look for sequences of printable ASCII characters
        for &byte in content {
            if (32..=126).contains(&byte) || byte == 10 || byte == 13 || byte == 9 {
                // Printable character, newline, carriage return, or tab
                current_text.push(byte);
            } else if byte == 0 && !current_text.is_empty() {
                // Null terminator, end current text chunk
                if current_text.len() > 3 {
                    // Only keep chunks longer than 3 characters
                    if let Ok(text) = String::from_utf8(current_text.clone()) {
                        let cleaned = text.trim();
                        if !cleaned.is_empty() && cleaned.chars().any(|c| c.is_alphabetic()) {
                            extracted_text.push(cleaned.to_string());
                        }
                    }
                }
                current_text.clear();
            } else if !current_text.is_empty()
                && (byte < 32 && byte != 10 && byte != 13 && byte != 9)
            {
                // Non-printable character, end current text chunk
                if current_text.len() > 3 {
                    if let Ok(text) = String::from_utf8(current_text.clone()) {
                        let cleaned = text.trim();
                        if !cleaned.is_empty() && cleaned.chars().any(|c| c.is_alphabetic()) {
                            extracted_text.push(cleaned.to_string());
                        }
                    }
                }
                current_text.clear();
            }
        }

        // Handle any remaining text
        if !current_text.is_empty() && current_text.len() > 3 {
            if let Ok(text) = String::from_utf8(current_text) {
                let cleaned = text.trim();
                if !cleaned.is_empty() && cleaned.chars().any(|c| c.is_alphabetic()) {
                    extracted_text.push(cleaned.to_string());
                }
            }
        }

        let result = extracted_text.join(" ");

        if result.trim().is_empty() {
            return Err(anyhow!("No readable text found in DOC file"));
        }

        Ok(Self::clean_extracted_doc_text(&result))
    }

    /// Extract text from binary content as fallback
    fn extract_text_from_binary(content: &[u8]) -> Result<String> {
        // Try to convert as UTF-8 first
        match String::from_utf8(content.to_vec()) {
            Ok(text) => Ok(Self::clean_text(&text)),
            Err(_) => {
                // Use lossy conversion as fallback
                let text = String::from_utf8_lossy(content);
                Ok(Self::clean_text(&text))
            }
        }
    }

    /// Clean extracted DOC text by removing common artifacts
    fn clean_extracted_doc_text(text: &str) -> String {
        let mut result = text.to_string();

        // Remove common DOC artifacts
        result = result.replace("Microsoft Word", "");
        result = result.replace("Normal.dot", "");
        result = result.replace("Times New Roman", "");
        result = result.replace("Arial", "");

        // Remove excessive repetitions of common words that might be artifacts
        let artifact_patterns = [
            r"\bMicrosoft\b\s*\bWord\b",
            r"\bTimes\b\s*\bNew\b\s*\bRoman\b",
            r"\bArial\b",
            r"\bCalibri\b",
            r"Normal\.dot",
            r"\b_Toc\d+\b",
            r"\bHYPERLINK\b",
        ];

        for pattern in &artifact_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                result = regex.replace_all(&result, "").to_string();
            }
        }

        // Apply standard text cleaning
        Self::clean_text(&result)
    }

    fn extract_text_from_xml(xml_content: &str) -> Result<String> {
        let mut reader = Reader::from_str(xml_content);
        reader.trim_text(true);

        let mut text_content = Vec::new();
        let mut inside_text = false;

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    // <w:t> tags contain the actual text content in DOCX
                    if e.name() == QName(b"w:t") {
                        inside_text = true;
                    }
                }
                Ok(Event::Text(e)) => {
                    if inside_text {
                        let text = e
                            .unescape()
                            .map_err(|e| anyhow!("Failed to decode XML text: {}", e))?;
                        text_content.push(text.to_string());
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name() == QName(b"w:t") {
                        inside_text = false;
                    }
                    // Add line break for paragraph ends
                    if e.name() == QName(b"w:p") {
                        text_content.push("\n".to_string());
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    error!("Error parsing XML: {}", e);
                    return Err(anyhow!("XML parsing error: {}", e));
                }
                _ => {}
            }
        }

        Ok(text_content.join(""))
    }

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
            "application/msword" => "doc".to_string(),
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
            "doc" => "doc".to_string(),
            "txt" | "text" => "txt".to_string(),
            _ => {
                warn!("Unknown file extension: {}, defaulting to txt", extension);
                "txt".to_string()
            }
        }
    }

    /// Validates that the file type is allowed for document parsing
    ///
    /// # Security
    /// This function prevents parsing of potentially dangerous file types
    /// and ensures only document formats are processed
    ///
    /// # Arguments
    /// * `file_type` - The detected file type string
    ///
    /// # Returns
    /// * `true` if the file type is allowed for parsing
    /// * `false` if the file type should be blocked
    fn is_allowed_file_type(file_type: &str) -> bool {
        // Only allow specific document formats that we can safely parse
        matches!(file_type, "pdf" | "docx" | "doc" | "txt")
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

impl DocumentParser {
    /// Count words in the given text
    fn count_words(text: &str) -> usize {
        text.split_whitespace()
            .filter(|word| !word.is_empty())
            .count()
    }

    /// Extract document metadata based on file type
    async fn extract_metadata(
        content: &[u8],
        file_type: &str,
        file_path: &str,
    ) -> Result<DocumentMetadata> {
        let mut metadata = DocumentMetadata::default();

        // Get file system metadata
        if let Ok(file_metadata) = tokio::fs::metadata(file_path).await {
            if let Ok(created) = file_metadata.created() {
                metadata.creation_date = Some(DateTime::from(created));
            }
            if let Ok(modified) = file_metadata.modified() {
                metadata.modification_date = Some(DateTime::from(modified));
            }
        }

        // Extract format-specific metadata
        match file_type {
            "pdf" => {
                metadata = Self::extract_pdf_metadata(content, metadata).await?;
            }
            "docx" => {
                metadata = Self::extract_docx_metadata(content, metadata).await?;
            }
            _ => {
                // For other formats, we have basic file system metadata
            }
        }

        Ok(metadata)
    }

    /// Extract PDF-specific metadata
    async fn extract_pdf_metadata(
        content: &[u8],
        mut metadata: DocumentMetadata,
    ) -> Result<DocumentMetadata> {
        // Use lopdf to extract metadata if available
        match lopdf::Document::load_mem(content) {
            Ok(doc) => {
                if let Ok(info_dict) = doc.trailer.get(b"Info") {
                    if let Ok(info) = info_dict.as_dict() {
                        // Extract common metadata fields
                        if let Ok(lopdf::Object::String(title_bytes, _)) = info.get(b"Title") {
                            if let Ok(title_str) = String::from_utf8(title_bytes.clone()) {
                                metadata.title = Some(title_str);
                            }
                        }
                        if let Ok(lopdf::Object::String(author_bytes, _)) = info.get(b"Author") {
                            if let Ok(author_str) = String::from_utf8(author_bytes.clone()) {
                                metadata.author = Some(author_str);
                            }
                        }
                        if let Ok(lopdf::Object::String(subject_bytes, _)) = info.get(b"Subject") {
                            if let Ok(subject_str) = String::from_utf8(subject_bytes.clone()) {
                                metadata.subject = Some(subject_str);
                            }
                        }
                        if let Ok(lopdf::Object::String(creator_bytes, _)) = info.get(b"Creator") {
                            if let Ok(creator_str) = String::from_utf8(creator_bytes.clone()) {
                                metadata.creator = Some(creator_str);
                            }
                        }
                        if let Ok(lopdf::Object::String(producer_bytes, _)) = info.get(b"Producer")
                        {
                            if let Ok(producer_str) = String::from_utf8(producer_bytes.clone()) {
                                metadata.producer = Some(producer_str);
                            }
                        }
                    }
                }
                // Get page count
                metadata.pages = Some(doc.get_pages().len() as u32);
            }
            Err(e) => {
                warn!("Failed to extract PDF metadata: {}", e);
            }
        }

        Ok(metadata)
    }

    /// Extract DOCX-specific metadata
    async fn extract_docx_metadata(
        content: &[u8],
        mut metadata: DocumentMetadata,
    ) -> Result<DocumentMetadata> {
        let cursor = Cursor::new(content);
        if let Ok(mut archive) = ZipArchive::new(cursor) {
            // Try to read core properties
            if let Ok(mut core_props) = archive.by_name("docProps/core.xml") {
                let mut xml_content = String::new();
                if std::io::Read::read_to_string(&mut core_props, &mut xml_content).is_ok() {
                    metadata = Self::parse_docx_core_properties(&xml_content, metadata)?;
                }
            }

            // Try to read app properties
            if let Ok(mut app_props) = archive.by_name("docProps/app.xml") {
                let mut xml_content = String::new();
                if std::io::Read::read_to_string(&mut app_props, &mut xml_content).is_ok() {
                    metadata = Self::parse_docx_app_properties(&xml_content, metadata)?;
                }
            }
        }

        Ok(metadata)
    }

    /// Parse DOCX core properties XML
    fn parse_docx_core_properties(
        xml_content: &str,
        mut metadata: DocumentMetadata,
    ) -> Result<DocumentMetadata> {
        let mut reader = Reader::from_str(xml_content);
        reader.trim_text(true);

        let mut current_element = String::new();
        let mut buffer = Vec::new();

        loop {
            match reader.read_event_into(&mut buffer) {
                Ok(Event::Start(ref e)) => {
                    current_element = String::from_utf8_lossy(e.name().as_ref()).to_string();
                }
                Ok(Event::Text(e)) => {
                    let text = e.unescape().unwrap_or_default().to_string();
                    match current_element.as_str() {
                        "dc:title" => metadata.title = Some(text),
                        "dc:creator" => metadata.author = Some(text),
                        "dc:subject" => metadata.subject = Some(text),
                        "dc:description" => {} // Could be used for summary
                        "cp:keywords" => metadata.keywords = Some(text),
                        "cp:lastModifiedBy" => {} // Additional author info
                        "dcterms:created" => {
                            if let Ok(date) = dateparser::parse(&text) {
                                metadata.creation_date = Some(date);
                            }
                        }
                        "dcterms:modified" => {
                            if let Ok(date) = dateparser::parse(&text) {
                                metadata.modification_date = Some(date);
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    warn!("Error parsing DOCX core properties: {}", e);
                    break;
                }
                _ => {}
            }
            buffer.clear();
        }

        Ok(metadata)
    }

    /// Parse DOCX app properties XML
    fn parse_docx_app_properties(
        xml_content: &str,
        mut metadata: DocumentMetadata,
    ) -> Result<DocumentMetadata> {
        let mut reader = Reader::from_str(xml_content);
        reader.trim_text(true);

        let mut current_element = String::new();
        let mut buffer = Vec::new();

        loop {
            match reader.read_event_into(&mut buffer) {
                Ok(Event::Start(ref e)) => {
                    current_element = String::from_utf8_lossy(e.name().as_ref()).to_string();
                }
                Ok(Event::Text(e)) => {
                    let text = e.unescape().unwrap_or_default().to_string();
                    match current_element.as_str() {
                        "Application" => metadata.creator = Some(text),
                        "Pages" => {
                            if let Ok(pages) = text.parse::<u32>() {
                                metadata.pages = Some(pages);
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    warn!("Error parsing DOCX app properties: {}", e);
                    break;
                }
                _ => {}
            }
            buffer.clear();
        }

        Ok(metadata)
    }

    /// Analyze document structure to extract sections, headings, etc.
    fn analyze_document_structure(content: &str) -> DocumentStructure {
        info!("Analyzing document structure");

        let lines: Vec<&str> = content.lines().collect();
        let mut sections = Vec::new();
        let mut headings = Vec::new();
        let mut current_position = 0;

        // Extract contact information (usually at the top)
        let contact_info =
            Self::extract_contact_info_enhanced(&lines[..5.min(lines.len())].join("\n"));

        // Identify headings and sections
        for (i, line) in lines.iter().enumerate() {
            let line_trimmed = line.trim();
            if line_trimmed.is_empty() {
                current_position += line.len() + 1;
                continue;
            }

            // Check if this line is a heading
            if Self::is_heading(line_trimmed) {
                let heading = DocumentHeading {
                    text: line_trimmed.to_string(),
                    level: Self::determine_heading_level(line_trimmed),
                    position: current_position,
                    formatting: Self::analyze_heading_formatting(line_trimmed),
                };
                headings.push(heading);

                // Try to extract section content
                if let Some(section) = Self::extract_section_at_position(&lines, i, line_trimmed) {
                    sections.push(section);
                }
            }

            current_position += line.len() + 1;
        }

        // If no sections were found, try to extract common resume sections
        if sections.is_empty() {
            sections = Self::extract_common_sections(content);
        }

        let total_sections = sections.len();
        let has_consistent_formatting = Self::check_formatting_consistency(&headings);

        DocumentStructure {
            sections,
            contact_info,
            headings,
            total_sections,
            has_consistent_formatting,
        }
    }

    /// Check if a line is likely a heading
    fn is_heading(line: &str) -> bool {
        // Existing implementation from is_section_header
        Self::is_section_header(line)
    }

    /// Determine heading level (1-6, with 1 being most important)
    fn determine_heading_level(line: &str) -> u8 {
        // Simple heuristic: all caps = level 1, title case = level 2, etc.
        if line.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) {
            1
        } else if line
            .split_whitespace()
            .all(|word| word.chars().next().unwrap_or('a').is_uppercase())
        {
            2
        } else {
            3
        }
    }

    /// Analyze heading formatting characteristics
    fn analyze_heading_formatting(line: &str) -> HeadingFormatting {
        HeadingFormatting {
            is_bold: false, // Can't detect from plain text
            is_uppercase: line.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()),
            font_size_relative: None, // Can't detect from plain text
            alignment: None,          // Can't detect from plain text
        }
    }

    /// Extract section content starting at a specific position
    fn extract_section_at_position(
        lines: &[&str],
        start_idx: usize,
        section_name: &str,
    ) -> Option<DocumentSection> {
        let mut content_lines = Vec::new();
        let mut bullet_points = Vec::new();
        let mut i = start_idx + 1;

        while i < lines.len() {
            let line = lines[i].trim();

            // Stop if we hit another section header
            if Self::is_section_header(line) {
                break;
            }

            if !line.is_empty() {
                content_lines.push(line);

                // Check if this is a bullet point
                if line.starts_with('•') || line.starts_with("- ") || line.starts_with("* ") {
                    bullet_points.push(line.to_string());
                }
            }

            i += 1;
        }

        if !content_lines.is_empty() {
            Some(DocumentSection {
                name: section_name.to_string(),
                content: content_lines.join("\n"),
                start_position: start_idx,
                end_position: i,
                confidence: 0.8, // Default confidence
                bullet_points,
            })
        } else {
            None
        }
    }

    /// Extract common resume sections using existing logic
    fn extract_common_sections(content: &str) -> Vec<DocumentSection> {
        let mut sections = Vec::new();

        // Use existing extract_section logic
        let section_types = [
            (
                "Experience",
                vec![
                    "experience",
                    "work experience",
                    "professional experience",
                    "employment",
                    "career history",
                    "work history",
                ],
            ),
            (
                "Education",
                vec![
                    "education",
                    "academic background",
                    "qualifications",
                    "degrees",
                ],
            ),
            (
                "Skills",
                vec![
                    "skills",
                    "technical skills",
                    "core competencies",
                    "expertise",
                    "proficiencies",
                    "technologies",
                ],
            ),
            (
                "Summary",
                vec!["summary", "profile", "objective", "about", "overview"],
            ),
        ];

        for (section_name, keywords) in section_types {
            if let Some(section_content) = Self::extract_section(content, &keywords) {
                // Extract bullet points
                let bullet_points: Vec<String> = section_content
                    .lines()
                    .filter(|line| {
                        let trimmed = line.trim();
                        trimmed.starts_with('•')
                            || trimmed.starts_with("- ")
                            || trimmed.starts_with("* ")
                    })
                    .map(|line| line.trim().to_string())
                    .collect();

                let section = DocumentSection {
                    name: section_name.to_string(),
                    content: section_content,
                    start_position: 0, // Would need more sophisticated tracking
                    end_position: 0,
                    confidence: 0.7,
                    bullet_points,
                };
                sections.push(section);
            }
        }

        sections
    }

    /// Check if headings have consistent formatting
    fn check_formatting_consistency(headings: &[DocumentHeading]) -> bool {
        if headings.len() < 2 {
            return true;
        }

        let first_style = &headings[0].formatting;
        headings.iter().all(|h| {
            h.formatting.is_uppercase == first_style.is_uppercase
                && h.formatting.is_bold == first_style.is_bold
        })
    }

    /// Calculate quality metrics for the document
    fn calculate_quality_metrics(
        content: &str,
        structure: &DocumentStructure,
    ) -> DocumentQualityMetrics {
        info!("Calculating document quality metrics");

        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        // ATS Compatibility Score
        let ats_compatibility_score =
            Self::calculate_ats_compatibility(content, structure, &mut issues);

        // Readability Score
        let readability_score = Self::calculate_readability_score(content);

        // Formatting Consistency Score
        let formatting_consistency_score = if structure.has_consistent_formatting {
            90.0
        } else {
            60.0
        };

        // Keyword Density (basic implementation)
        let keyword_density = Self::calculate_keyword_density(content);

        // Section Completeness Score
        let section_completeness_score =
            Self::calculate_section_completeness(structure, &mut recommendations);

        // Contact Info Completeness
        let contact_info_completeness =
            Self::calculate_contact_completeness(&structure.contact_info, &mut issues);

        // Overall Quality Score (weighted average)
        let overall_quality_score = (ats_compatibility_score * 0.3
            + readability_score * 0.2
            + formatting_consistency_score * 0.15
            + section_completeness_score * 0.2
            + contact_info_completeness * 0.15)
            / 1.0;

        DocumentQualityMetrics {
            ats_compatibility_score,
            readability_score,
            formatting_consistency_score,
            keyword_density,
            section_completeness_score,
            contact_info_completeness,
            overall_quality_score,
            issues,
            recommendations,
        }
    }

    /// Calculate ATS compatibility score
    fn calculate_ats_compatibility(
        content: &str,
        structure: &DocumentStructure,
        issues: &mut Vec<DocumentIssue>,
    ) -> f64 {
        let mut score: f64 = 100.0;

        // Check for common ATS issues
        if content.lines().any(|line| line.contains('\t')) {
            score -= 10.0;
            issues.push(DocumentIssue {
                issue_type: DocumentIssueType::AtsCompatibility,
                description: "Document contains tab characters which may cause parsing issues"
                    .to_string(),
                severity: IssueSeverity::Medium,
                location: None,
                suggestion: Some(
                    "Replace tabs with spaces for better ATS compatibility".to_string(),
                ),
            });
        }

        // Check for section headers
        if structure.headings.is_empty() {
            score -= 20.0;
            issues.push(DocumentIssue {
                issue_type: DocumentIssueType::Structure,
                description: "No clear section headers detected".to_string(),
                severity: IssueSeverity::High,
                location: None,
                suggestion: Some(
                    "Add clear section headers like 'EXPERIENCE', 'EDUCATION', etc.".to_string(),
                ),
            });
        }

        // Check for contact information
        if structure.contact_info.email.is_none() {
            score -= 15.0;
            issues.push(DocumentIssue {
                issue_type: DocumentIssueType::ContactInfo,
                description: "No email address found".to_string(),
                severity: IssueSeverity::High,
                location: Some("Header".to_string()),
                suggestion: Some("Include a professional email address in the header".to_string()),
            });
        }

        score.max(0.0)
    }

    /// Calculate readability score
    fn calculate_readability_score(content: &str) -> f64 {
        // Simple readability calculation based on sentence and word length
        let sentences: Vec<&str> = content.split(&['.', '!', '?'][..]).collect();
        let words: Vec<&str> = content.split_whitespace().collect();

        if sentences.is_empty() || words.is_empty() {
            return 50.0;
        }

        let avg_sentence_length = words.len() as f64 / sentences.len() as f64;
        let avg_word_length: f64 =
            words.iter().map(|w| w.len()).sum::<usize>() as f64 / words.len() as f64;

        // Simple scoring: prefer medium-length sentences and words
        let sentence_score = if avg_sentence_length > 15.0 && avg_sentence_length < 25.0 {
            100.0
        } else {
            80.0
        };

        let word_score = if avg_word_length > 4.0 && avg_word_length < 7.0 {
            100.0
        } else {
            80.0
        };

        (sentence_score + word_score) / 2.0
    }

    /// Calculate keyword density
    fn calculate_keyword_density(content: &str) -> f64 {
        let words: Vec<&str> = content.split_whitespace().collect();
        let total_words = words.len() as f64;

        if total_words == 0.0 {
            return 0.0;
        }

        // Count technical/professional keywords (simplified)
        let professional_keywords = [
            "experience",
            "developed",
            "managed",
            "led",
            "created",
            "implemented",
            "designed",
            "coordinated",
            "analyzed",
            "improved",
        ];

        let keyword_count = words
            .iter()
            .filter(|word| {
                professional_keywords
                    .iter()
                    .any(|kw| word.to_lowercase().contains(kw))
            })
            .count() as f64;

        (keyword_count / total_words) * 100.0
    }

    /// Calculate section completeness score
    fn calculate_section_completeness(
        structure: &DocumentStructure,
        recommendations: &mut Vec<String>,
    ) -> f64 {
        let essential_sections = ["experience", "education", "skills"];
        let found_sections: Vec<String> = structure
            .sections
            .iter()
            .map(|s| s.name.to_lowercase())
            .collect();

        let mut found_count = 0;
        for essential in &essential_sections {
            if found_sections
                .iter()
                .any(|section| section.contains(essential))
            {
                found_count += 1;
            } else {
                recommendations.push(format!("Consider adding a {} section", essential));
            }
        }

        (found_count as f64 / essential_sections.len() as f64) * 100.0
    }

    /// Extract enhanced contact information
    fn extract_contact_info_enhanced(text: &str) -> DocumentContactInfo {
        let email_regex =
            Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
        let phone_regex =
            Regex::new(r"(\+?1[-.\s]?)?\(?([0-9]{3})\)?[-.\s]?([0-9]{3})[-.\s]?([0-9]{4})")
                .unwrap();
        let linkedin_regex =
            Regex::new(r"(?:linkedin\.com/in/|linkedin\.com/pub/)([A-Za-z0-9-]+)").unwrap();

        DocumentContactInfo {
            email: email_regex.find(text).map(|m| m.as_str().to_string()),
            phone: phone_regex.find(text).map(|m| m.as_str().to_string()),
            linkedin: linkedin_regex.find(text).map(|m| m.as_str().to_string()),
        }
    }

    /// Calculate contact information completeness
    fn calculate_contact_completeness(
        contact_info: &DocumentContactInfo,
        issues: &mut Vec<DocumentIssue>,
    ) -> f64 {
        let mut score = 0.0;
        let mut total_fields = 0.0;

        // Email (essential)
        total_fields += 2.0; // Weight email more heavily
        if contact_info.email.is_some() {
            score += 2.0;
        } else {
            issues.push(DocumentIssue {
                issue_type: DocumentIssueType::ContactInfo,
                description: "Missing email address".to_string(),
                severity: IssueSeverity::High,
                location: Some("Header".to_string()),
                suggestion: Some("Add a professional email address".to_string()),
            });
        }

        // Phone
        total_fields += 1.0;
        if contact_info.phone.is_some() {
            score += 1.0;
        } else {
            issues.push(DocumentIssue {
                issue_type: DocumentIssueType::ContactInfo,
                description: "Missing phone number".to_string(),
                severity: IssueSeverity::Medium,
                location: Some("Header".to_string()),
                suggestion: Some("Add a phone number for direct contact".to_string()),
            });
        }

        // LinkedIn (nice to have)
        total_fields += 1.0;
        if contact_info.linkedin.is_some() {
            score += 1.0;
        }

        (score / total_fields) * 100.0
    }
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

    const _SAMPLE_JOB_DESCRIPTION: &str = r#"
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
            DocumentParser::determine_file_type_from_filename("document.doc"),
            "doc"
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
    async fn test_extract_text_from_xml() {
        let sample_docx_xml = r#"
        <w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
            <w:body>
                <w:p>
                    <w:r>
                        <w:t>John Doe</w:t>
                    </w:r>
                </w:p>
                <w:p>
                    <w:r>
                        <w:t>Software Engineer</w:t>
                    </w:r>
                </w:p>
                <w:p>
                    <w:r>
                        <w:t>john.doe@email.com</w:t>
                    </w:r>
                </w:p>
            </w:body>
        </w:document>
        "#;

        let result = DocumentParser::extract_text_from_xml(sample_docx_xml);
        assert!(result.is_ok());

        let text = result.unwrap();
        assert!(text.contains("John Doe"));
        assert!(text.contains("Software Engineer"));
        assert!(text.contains("john.doe@email.com"));
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

    // Enhanced functionality tests

    #[tokio::test]
    async fn test_enhanced_document_info_structure() {
        let content = SAMPLE_RESUME_TEXT.as_bytes();
        let result = DocumentParser::parse_content(content, "resume.txt").await;

        assert!(result.is_ok());
        let doc_info = result.unwrap();

        // Test that enhanced fields are populated
        assert!(doc_info.structure.is_some());
        assert!(doc_info.quality_metrics.is_some());

        let structure = doc_info.structure.unwrap();
        assert!(!structure.sections.is_empty());
        assert!(structure.contact_info.email.is_some());

        let quality = doc_info.quality_metrics.unwrap();
        assert!(quality.overall_quality_score > 0.0);
        assert!(quality.ats_compatibility_score > 0.0);
    }

    #[tokio::test]
    async fn test_document_structure_analysis() {
        let structure = DocumentParser::analyze_document_structure(SAMPLE_RESUME_TEXT);

        // Test contact info extraction
        assert!(structure.contact_info.email.is_some());
        assert_eq!(structure.contact_info.email.unwrap(), "john.doe@email.com");
        assert!(structure.contact_info.phone.is_some());
        assert!(structure.contact_info.linkedin.is_some());

        // Test section extraction
        assert!(structure.total_sections > 0);
        assert!(!structure.sections.is_empty());

        // Verify specific sections are found
        let section_names: Vec<String> = structure
            .sections
            .iter()
            .map(|s| s.name.to_lowercase())
            .collect();

        assert!(section_names.iter().any(|name| name.contains("experience")));
        assert!(section_names.iter().any(|name| name.contains("summary")));
    }

    #[tokio::test]
    async fn test_quality_metrics_calculation() {
        let structure = DocumentParser::analyze_document_structure(SAMPLE_RESUME_TEXT);
        let quality = DocumentParser::calculate_quality_metrics(SAMPLE_RESUME_TEXT, &structure);

        // Test quality score ranges
        assert!(quality.overall_quality_score >= 0.0 && quality.overall_quality_score <= 100.0);
        assert!(quality.ats_compatibility_score >= 0.0 && quality.ats_compatibility_score <= 100.0);
        assert!(quality.readability_score >= 0.0 && quality.readability_score <= 100.0);
        assert!(
            quality.section_completeness_score >= 0.0
                && quality.section_completeness_score <= 100.0
        );

        // Test that recommendations are provided for improvements
        assert!(!quality.recommendations.is_empty());
    }

    #[tokio::test]
    async fn test_doc_text_extraction() {
        // Test DOC text extraction with sample binary data
        let sample_doc_data =
            b"PK\x03\x04Microsoft Word Document Text content here\x00\x00Another text chunk\x00";
        let result = DocumentParser::extract_doc_text_simple(sample_doc_data);

        assert!(result.is_ok());
        let text = result.unwrap();
        assert!(text.contains("Text content here"));
        assert!(text.contains("Another text chunk"));
        assert!(!text.contains("Microsoft Word")); // Should be cleaned out
    }

    #[tokio::test]
    async fn test_doc_cleaning() {
        let dirty_text =
            "Microsoft Word Normal.dot Some actual content Times New Roman More content";
        let cleaned = DocumentParser::clean_extracted_doc_text(dirty_text);

        assert!(!cleaned.contains("Microsoft Word"));
        assert!(!cleaned.contains("Normal.dot"));
        assert!(!cleaned.contains("Times New Roman"));
        assert!(cleaned.contains("Some actual content"));
        assert!(cleaned.contains("More content"));
    }

    #[tokio::test]
    async fn test_enhanced_contact_info_extraction() {
        let contact_info = DocumentParser::extract_contact_info_enhanced(SAMPLE_RESUME_TEXT);

        assert!(contact_info.email.is_some());
        assert_eq!(contact_info.email.unwrap(), "john.doe@email.com");

        assert!(contact_info.phone.is_some());
        assert!(contact_info.phone.unwrap().contains("555"));

        assert!(contact_info.linkedin.is_some());
        assert!(contact_info.linkedin.unwrap().contains("johndoe"));
    }

    #[tokio::test]
    async fn test_heading_detection_and_formatting() {
        let test_content = r#"
EXPERIENCE
This is not a heading
Education
SKILLS AND TECHNOLOGIES
Another regular line
Summary
"#;

        let structure = DocumentParser::analyze_document_structure(test_content);

        assert!(!structure.headings.is_empty());

        // Test heading level detection
        let experience_heading = structure
            .headings
            .iter()
            .find(|h| h.text.contains("EXPERIENCE"));
        assert!(experience_heading.is_some());
        assert_eq!(experience_heading.unwrap().level, 1); // All caps should be level 1

        let education_heading = structure
            .headings
            .iter()
            .find(|h| h.text.contains("Education"));
        assert!(education_heading.is_some());
        assert_eq!(education_heading.unwrap().level, 2); // Title case should be level 2
    }

    #[tokio::test]
    async fn test_ats_compatibility_scoring() {
        let structure = DocumentParser::analyze_document_structure(SAMPLE_RESUME_TEXT);
        let mut issues = Vec::new();

        let score = DocumentParser::calculate_ats_compatibility(
            SAMPLE_RESUME_TEXT,
            &structure,
            &mut issues,
        );

        assert!(score >= 0.0 && score <= 100.0);

        // Test with problematic content
        let bad_content = "No sections\tHas tabs\nNo email or phone";
        let bad_structure = DocumentParser::analyze_document_structure(bad_content);
        let mut bad_issues = Vec::new();

        let bad_score = DocumentParser::calculate_ats_compatibility(
            bad_content,
            &bad_structure,
            &mut bad_issues,
        );

        assert!(bad_score < score); // Should score lower
        assert!(!bad_issues.is_empty()); // Should have issues
    }

    #[tokio::test]
    async fn test_keyword_density_calculation() {
        let test_content = "I have experience in software development. I managed teams and led projects. I created innovative solutions and implemented best practices.";
        let density = DocumentParser::calculate_keyword_density(test_content);

        assert!(density > 0.0); // Should find keywords like "experience", "managed", "led", "created", "implemented"
        assert!(density <= 100.0);
    }

    #[tokio::test]
    async fn test_section_completeness_scoring() {
        let good_structure = DocumentParser::analyze_document_structure(SAMPLE_RESUME_TEXT);
        let mut recommendations = Vec::new();

        let score =
            DocumentParser::calculate_section_completeness(&good_structure, &mut recommendations);
        assert!(score > 0.0);

        // Test with missing sections
        let incomplete_content = "John Doe\nSome random content without proper sections";
        let incomplete_structure = DocumentParser::analyze_document_structure(incomplete_content);
        let mut incomplete_recommendations = Vec::new();

        let incomplete_score = DocumentParser::calculate_section_completeness(
            &incomplete_structure,
            &mut incomplete_recommendations,
        );
        assert!(incomplete_score < score);
        assert!(!incomplete_recommendations.is_empty());
    }

    #[tokio::test]
    async fn test_contact_completeness_scoring() {
        // Test complete contact info
        let complete_contact = DocumentContactInfo {
            email: Some("test@example.com".to_string()),
            phone: Some("555-123-4567".to_string()),
            linkedin: Some("linkedin.com/in/test".to_string()),
        };
        let mut issues = Vec::new();
        let complete_score =
            DocumentParser::calculate_contact_completeness(&complete_contact, &mut issues);

        assert_eq!(complete_score, 100.0);
        assert!(issues.is_empty());

        // Test incomplete contact info
        let incomplete_contact = DocumentContactInfo {
            email: None,
            phone: Some("555-123-4567".to_string()),
            linkedin: None,
        };
        let mut incomplete_issues = Vec::new();
        let incomplete_score = DocumentParser::calculate_contact_completeness(
            &incomplete_contact,
            &mut incomplete_issues,
        );

        assert!(incomplete_score < complete_score);
        assert!(!incomplete_issues.is_empty());
    }

    #[tokio::test]
    async fn test_readability_scoring() {
        let readable_text = "This is a well written document. It has good sentence structure. The words are not too long or complex.";
        let score = DocumentParser::calculate_readability_score(readable_text);

        assert!(score >= 0.0 && score <= 100.0);

        let unreadable_text =
            "Thisisaveryverylongwordthatissuperdifficulttoreadeventhoughitcontainsnospaces";
        let bad_score = DocumentParser::calculate_readability_score(unreadable_text);

        assert!(bad_score < score);
    }
}
