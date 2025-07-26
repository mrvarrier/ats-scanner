use serde::Serialize;
use thiserror::Error;

/// Centralized error type for the ATS Scanner application
#[derive(Error, Debug)]
pub enum ATSError {
    #[error("Database operation failed: {message}")]
    Database {
        message: String,
        #[source]
        source: Option<sqlx::Error>,
    },

    #[error("Document parsing failed: {message}")]
    DocumentParsing {
        message: String,
        #[source]
        source: Option<anyhow::Error>,
    },

    #[error("Ollama API error: {message}")]
    OllamaApi {
        message: String,
        #[source]
        source: Option<reqwest::Error>,
    },

    #[error("Security violation: {message}")]
    Security { message: String },

    #[error("Configuration error: {message}")]
    Configuration {
        message: String,
        #[source]
        source: Option<anyhow::Error>,
    },

    #[error("File operation failed: {message}")]
    FileOperation {
        message: String,
        #[source]
        source: Option<std::io::Error>,
    },

    #[error("Memory management error: {message}")]
    Memory { message: String },

    #[error("Validation failed: {message}")]
    Validation { message: String },

    #[error("Plugin execution failed: {message}")]
    Plugin {
        message: String,
        #[source]
        source: Option<anyhow::Error>,
    },

    #[error("Migration error: {message}")]
    Migration {
        message: String,
        #[source]
        source: Option<anyhow::Error>,
    },

    #[error("External service error: {message}")]
    ExternalService {
        message: String,
        #[source]
        source: Option<anyhow::Error>,
    },
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum ErrorSeverity {
    Critical,
    High,
    Medium,
    Low,
}

impl ATSError {
    /// Get the error code for this error type
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::Database { .. } => "DATABASE_ERROR",
            Self::DocumentParsing { .. } => "DOCUMENT_ERROR",
            Self::OllamaApi { .. } => "OLLAMA_ERROR",
            Self::Security { .. } => "SECURITY_ERROR",
            Self::Configuration { .. } => "CONFIG_ERROR",
            Self::FileOperation { .. } => "FILE_ERROR",
            Self::Memory { .. } => "MEMORY_ERROR",
            Self::Validation { .. } => "VALIDATION_ERROR",
            Self::Plugin { .. } => "PLUGIN_ERROR",
            Self::Migration { .. } => "MIGRATION_ERROR",
            Self::ExternalService { .. } => "EXTERNAL_SERVICE_ERROR",
        }
    }

    /// Get the severity level for this error type
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::Security { .. } => ErrorSeverity::Critical,
            Self::Database { .. } => ErrorSeverity::High,
            Self::Configuration { .. } => ErrorSeverity::High,
            Self::Migration { .. } => ErrorSeverity::High,
            Self::Memory { .. } => ErrorSeverity::High,
            Self::OllamaApi { .. } => ErrorSeverity::Medium,
            Self::DocumentParsing { .. } => ErrorSeverity::Medium,
            Self::FileOperation { .. } => ErrorSeverity::Medium,
            Self::Plugin { .. } => ErrorSeverity::Medium,
            Self::ExternalService { .. } => ErrorSeverity::Medium,
            Self::Validation { .. } => ErrorSeverity::Low,
        }
    }

    /// Log the error with appropriate severity level
    pub fn log(&self, context: &str) {
        match self.severity() {
            ErrorSeverity::Critical => log::error!("[CRITICAL] {}: {}", context, self),
            ErrorSeverity::High => log::error!("[HIGH] {}: {}", context, self),
            ErrorSeverity::Medium => log::warn!("[MEDIUM] {}: {}", context, self),
            ErrorSeverity::Low => log::info!("[LOW] {}: {}", context, self),
        }
    }

    /// Create a database error with context
    pub fn database(message: impl Into<String>) -> Self {
        Self::Database {
            message: message.into(),
            source: None,
        }
    }

    /// Create a database error with source
    pub fn database_with_source(message: impl Into<String>, source: sqlx::Error) -> Self {
        Self::Database {
            message: message.into(),
            source: Some(source),
        }
    }

    /// Create a document parsing error
    pub fn document_parsing(message: impl Into<String>) -> Self {
        Self::DocumentParsing {
            message: message.into(),
            source: None,
        }
    }

    /// Create a document parsing error with source
    pub fn document_parsing_with_source(message: impl Into<String>, source: anyhow::Error) -> Self {
        Self::DocumentParsing {
            message: message.into(),
            source: Some(source),
        }
    }

    /// Create an Ollama API error
    pub fn ollama_api(message: impl Into<String>) -> Self {
        Self::OllamaApi {
            message: message.into(),
            source: None,
        }
    }

    /// Create an Ollama API error with source
    pub fn ollama_api_with_source(message: impl Into<String>, source: reqwest::Error) -> Self {
        Self::OllamaApi {
            message: message.into(),
            source: Some(source),
        }
    }

    /// Create a security error
    pub fn security(message: impl Into<String>) -> Self {
        Self::Security {
            message: message.into(),
        }
    }

    /// Create a configuration error
    pub fn configuration(message: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
            source: None,
        }
    }

    /// Create a file operation error
    pub fn file_operation(message: impl Into<String>) -> Self {
        Self::FileOperation {
            message: message.into(),
            source: None,
        }
    }

    /// Create a file operation error with source
    pub fn file_operation_with_source(message: impl Into<String>, source: std::io::Error) -> Self {
        Self::FileOperation {
            message: message.into(),
            source: Some(source),
        }
    }

    /// Create a memory management error
    pub fn memory(message: impl Into<String>) -> Self {
        Self::Memory {
            message: message.into(),
        }
    }

    /// Create a validation error
    #[allow(dead_code)]
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    /// Create a plugin error
    #[allow(dead_code)]
    pub fn plugin(message: impl Into<String>) -> Self {
        Self::Plugin {
            message: message.into(),
            source: None,
        }
    }

    /// Create a migration error
    #[allow(dead_code)]
    pub fn migration(message: impl Into<String>) -> Self {
        Self::Migration {
            message: message.into(),
            source: None,
        }
    }

    /// Create an external service error
    #[allow(dead_code)]
    pub fn external_service(message: impl Into<String>) -> Self {
        Self::ExternalService {
            message: message.into(),
            source: None,
        }
    }
}

/// Result type alias for the ATS Scanner application
pub type ATSResult<T> = std::result::Result<T, ATSError>;

/// Helper trait for converting errors to ATSError
pub trait IntoATSError<T> {
    #[allow(dead_code)]
    fn into_ats_error(self, message: &str) -> ATSResult<T>;
    fn into_database_error(self, message: &str) -> ATSResult<T>;
    fn into_file_error(self, message: &str) -> ATSResult<T>;
    fn into_document_error(self, message: &str) -> ATSResult<T>;
    fn into_ollama_error(self, message: &str) -> ATSResult<T>;
}

impl<T> IntoATSError<T> for std::result::Result<T, sqlx::Error> {
    fn into_ats_error(self, message: &str) -> ATSResult<T> {
        self.into_database_error(message)
    }

    fn into_database_error(self, message: &str) -> ATSResult<T> {
        self.map_err(|e| ATSError::database_with_source(message, e))
    }

    fn into_file_error(self, message: &str) -> ATSResult<T> {
        self.map_err(|e| ATSError::database(format!("{}: {}", message, e)))
    }

    fn into_document_error(self, message: &str) -> ATSResult<T> {
        self.map_err(|e| ATSError::database(format!("{}: {}", message, e)))
    }

    fn into_ollama_error(self, message: &str) -> ATSResult<T> {
        self.map_err(|e| ATSError::database(format!("{}: {}", message, e)))
    }
}

impl<T> IntoATSError<T> for std::result::Result<T, std::io::Error> {
    fn into_ats_error(self, message: &str) -> ATSResult<T> {
        self.into_file_error(message)
    }

    fn into_database_error(self, message: &str) -> ATSResult<T> {
        self.map_err(|e| ATSError::file_operation(format!("{}: {}", message, e)))
    }

    fn into_file_error(self, message: &str) -> ATSResult<T> {
        self.map_err(|e| ATSError::file_operation_with_source(message, e))
    }

    fn into_document_error(self, message: &str) -> ATSResult<T> {
        self.map_err(|e| ATSError::document_parsing(format!("{}: {}", message, e)))
    }

    fn into_ollama_error(self, message: &str) -> ATSResult<T> {
        self.map_err(|e| ATSError::file_operation(format!("{}: {}", message, e)))
    }
}

impl<T> IntoATSError<T> for std::result::Result<T, reqwest::Error> {
    fn into_ats_error(self, message: &str) -> ATSResult<T> {
        self.into_ollama_error(message)
    }

    fn into_database_error(self, message: &str) -> ATSResult<T> {
        self.map_err(|e| ATSError::ollama_api(format!("{}: {}", message, e)))
    }

    fn into_file_error(self, message: &str) -> ATSResult<T> {
        self.map_err(|e| ATSError::ollama_api(format!("{}: {}", message, e)))
    }

    fn into_document_error(self, message: &str) -> ATSResult<T> {
        self.map_err(|e| ATSError::ollama_api(format!("{}: {}", message, e)))
    }

    fn into_ollama_error(self, message: &str) -> ATSResult<T> {
        self.map_err(|e| ATSError::ollama_api_with_source(message, e))
    }
}

impl<T> IntoATSError<T> for std::result::Result<T, anyhow::Error> {
    fn into_ats_error(self, message: &str) -> ATSResult<T> {
        self.into_document_error(message)
    }

    fn into_database_error(self, message: &str) -> ATSResult<T> {
        self.map_err(|e| ATSError::database(format!("{}: {}", message, e)))
    }

    fn into_file_error(self, message: &str) -> ATSResult<T> {
        self.map_err(|e| ATSError::file_operation(format!("{}: {}", message, e)))
    }

    fn into_document_error(self, message: &str) -> ATSResult<T> {
        self.map_err(|e| ATSError::document_parsing_with_source(message, e))
    }

    fn into_ollama_error(self, message: &str) -> ATSResult<T> {
        self.map_err(|e| ATSError::ollama_api(format!("{}: {}", message, e)))
    }
}

/// Conversion from anyhow::Error to ATSError
impl From<anyhow::Error> for ATSError {
    fn from(err: anyhow::Error) -> Self {
        // Try to determine the error type from the error chain
        let error_string = err.to_string().to_lowercase();

        if error_string.contains("database") || error_string.contains("sql") {
            ATSError::database(err.to_string())
        } else if error_string.contains("file") || error_string.contains("io") {
            ATSError::file_operation(err.to_string())
        } else if error_string.contains("security") || error_string.contains("permission") {
            ATSError::security(err.to_string())
        } else if error_string.contains("config") {
            ATSError::configuration(err.to_string())
        } else if error_string.contains("memory") {
            ATSError::memory(err.to_string())
        } else {
            ATSError::document_parsing_with_source("Unknown error", err)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        assert_eq!(ATSError::database("test").error_code(), "DATABASE_ERROR");
        assert_eq!(
            ATSError::document_parsing("test").error_code(),
            "DOCUMENT_ERROR"
        );
        assert_eq!(ATSError::security("test").error_code(), "SECURITY_ERROR");
    }

    #[test]
    fn test_error_severity() {
        assert!(matches!(
            ATSError::security("test").severity(),
            ErrorSeverity::Critical
        ));
        assert!(matches!(
            ATSError::database("test").severity(),
            ErrorSeverity::High
        ));
        assert!(matches!(
            ATSError::validation("test").severity(),
            ErrorSeverity::Low
        ));
    }

    #[test]
    fn test_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let result: std::result::Result<(), _> = Err(io_error);
        let ats_result = result.into_file_error("Failed to read file");

        assert!(ats_result.is_err());
        if let Err(ats_error) = ats_result {
            assert_eq!(ats_error.error_code(), "FILE_ERROR");
        }
    }
}
