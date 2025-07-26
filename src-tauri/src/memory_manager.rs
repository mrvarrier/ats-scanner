use anyhow::{anyhow, Result};
// use chrono::{DateTime, Utc};
use log::{info, warn};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};

/// Memory management constants for document processing
pub mod limits {
    /// Maximum file size allowed for processing (50MB)
    pub const MAX_DOCUMENT_SIZE: usize = 50 * 1024 * 1024;

    /// Maximum extracted text size to prevent memory exhaustion (10MB)
    pub const MAX_TEXT_SIZE: usize = 10 * 1024 * 1024;

    /// Processing chunk size for streaming operations (64KB)
    pub const CHUNK_SIZE: usize = 64 * 1024;

    /// Warning threshold for large documents (10MB)
    pub const LARGE_DOCUMENT_WARNING: usize = 10 * 1024 * 1024;

    /// Memory pool maximum buffer size (1MB)
    pub const MAX_POOL_BUFFER_SIZE: usize = 1024 * 1024;

    /// Maximum concurrent document processing
    pub const MAX_CONCURRENT_DOCUMENTS: usize = 3;
}

/// Memory usage tracker for document processing
#[derive(Debug, Clone)]
pub struct MemoryTracker {
    current_usage: Arc<AtomicUsize>,
    peak_usage: Arc<AtomicUsize>,
    active_documents: Arc<AtomicUsize>,
}

impl Default for MemoryTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryTracker {
    /// Create a new memory tracker
    pub fn new() -> Self {
        Self {
            current_usage: Arc::new(AtomicUsize::new(0)),
            peak_usage: Arc::new(AtomicUsize::new(0)),
            active_documents: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Allocate memory and track usage
    pub fn allocate(&self, size: usize) -> Result<MemoryAllocation> {
        let active_docs = self.active_documents.load(Ordering::Relaxed);
        if active_docs >= limits::MAX_CONCURRENT_DOCUMENTS {
            return Err(anyhow!(
                "Maximum concurrent documents ({}) exceeded",
                limits::MAX_CONCURRENT_DOCUMENTS
            ));
        }

        let current = self.current_usage.fetch_add(size, Ordering::Relaxed);
        let new_total = current + size;

        // Update peak usage if necessary
        let mut peak = self.peak_usage.load(Ordering::Relaxed);
        while new_total > peak {
            match self.peak_usage.compare_exchange_weak(
                peak,
                new_total,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(current_peak) => peak = current_peak,
            }
        }

        if new_total > limits::LARGE_DOCUMENT_WARNING {
            warn!(
                "High memory usage: {} MB allocated for document processing",
                new_total / 1024 / 1024
            );
        }

        Ok(MemoryAllocation {
            size,
            tracker: self.clone(),
        })
    }

    /// Start processing a document
    pub fn start_document(&self) -> DocumentProcessingGuard {
        let count = self.active_documents.fetch_add(1, Ordering::Relaxed);
        info!("Started document processing ({} active)", count + 1);

        DocumentProcessingGuard {
            tracker: self.clone(),
        }
    }

    /// Get current memory usage in bytes
    #[allow(dead_code)]
    pub fn current_usage(&self) -> usize {
        self.current_usage.load(Ordering::Relaxed)
    }

    /// Get peak memory usage in bytes
    #[allow(dead_code)]
    pub fn peak_usage(&self) -> usize {
        self.peak_usage.load(Ordering::Relaxed)
    }

    /// Get number of active documents being processed
    #[allow(dead_code)]
    pub fn active_documents(&self) -> usize {
        self.active_documents.load(Ordering::Relaxed)
    }

    /// Release memory allocation
    fn deallocate(&self, size: usize) {
        self.current_usage.fetch_sub(size, Ordering::Relaxed);
    }

    /// Finish processing a document
    fn finish_document(&self) {
        let count = self.active_documents.fetch_sub(1, Ordering::Relaxed);
        info!(
            "Finished document processing ({} active)",
            count.saturating_sub(1)
        );
    }
}

/// RAII guard for memory allocation tracking
pub struct MemoryAllocation {
    size: usize,
    tracker: MemoryTracker,
}

impl Drop for MemoryAllocation {
    fn drop(&mut self) {
        self.tracker.deallocate(self.size);
    }
}

/// RAII guard for document processing tracking
pub struct DocumentProcessingGuard {
    tracker: MemoryTracker,
}

impl Drop for DocumentProcessingGuard {
    fn drop(&mut self) {
        self.tracker.finish_document();
    }
}

/// Memory pool for reusing buffers during document processing
pub struct DocumentMemoryPool {
    buffer_pool: tokio::sync::Mutex<Vec<Vec<u8>>>,
    string_pool: tokio::sync::Mutex<Vec<String>>,
}

impl Default for DocumentMemoryPool {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentMemoryPool {
    /// Create a new memory pool
    pub fn new() -> Self {
        Self {
            buffer_pool: tokio::sync::Mutex::new(Vec::with_capacity(8)),
            string_pool: tokio::sync::Mutex::new(Vec::with_capacity(8)),
        }
    }

    /// Get a buffer from the pool or allocate a new one
    pub async fn get_buffer(&self, min_size: usize) -> Vec<u8> {
        let mut pool = self.buffer_pool.lock().await;

        // Try to find a suitable buffer from the pool
        for i in 0..pool.len() {
            if pool[i].capacity() >= min_size {
                return pool.swap_remove(i);
            }
        }

        // Allocate a new buffer if none suitable found
        Vec::with_capacity(min_size.max(limits::CHUNK_SIZE))
    }

    /// Return a buffer to the pool for reuse
    pub async fn return_buffer(&self, mut buffer: Vec<u8>) {
        buffer.clear();

        // Only keep reasonably sized buffers in the pool
        if buffer.capacity() <= limits::MAX_POOL_BUFFER_SIZE {
            let mut pool = self.buffer_pool.lock().await;
            if pool.len() < 16 {
                // Limit pool size
                pool.push(buffer);
            }
        }
    }

    /// Get a string from the pool or allocate a new one
    pub async fn get_string(&self, min_capacity: usize) -> String {
        let mut pool = self.string_pool.lock().await;

        // Try to find a suitable string from the pool
        for i in 0..pool.len() {
            if pool[i].capacity() >= min_capacity {
                return pool.swap_remove(i);
            }
        }

        // Allocate a new string if none suitable found
        String::with_capacity(min_capacity.max(4096))
    }

    /// Return a string to the pool for reuse
    #[allow(dead_code)]
    pub async fn return_string(&self, mut string: String) {
        string.clear();

        // Only keep reasonably sized strings in the pool
        if string.capacity() <= limits::MAX_POOL_BUFFER_SIZE {
            let mut pool = self.string_pool.lock().await;
            if pool.len() < 16 {
                // Limit pool size
                pool.push(string);
            }
        }
    }
}

/// Streaming text processor with memory bounds
pub struct StreamingTextProcessor {
    max_text_size: usize,
    chunk_size: usize,
    memory_pool: DocumentMemoryPool,
}

impl Default for StreamingTextProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamingTextProcessor {
    /// Create a new streaming text processor
    pub fn new() -> Self {
        Self {
            max_text_size: limits::MAX_TEXT_SIZE,
            chunk_size: limits::CHUNK_SIZE,
            memory_pool: DocumentMemoryPool::new(),
        }
    }

    /// Create with custom limits
    #[allow(dead_code)]
    pub fn with_limits(max_text_size: usize, chunk_size: usize) -> Self {
        Self {
            max_text_size,
            chunk_size,
            memory_pool: DocumentMemoryPool::new(),
        }
    }

    /// Process content from an async reader with memory bounds
    pub async fn process_reader<R, F>(&self, reader: R, processor: F) -> Result<String>
    where
        R: AsyncRead + Unpin,
        F: Fn(&[u8]) -> Result<String>,
    {
        let mut buf_reader = BufReader::new(reader);
        let mut buffer = self.memory_pool.get_buffer(self.chunk_size).await;
        let mut accumulated_text = self.memory_pool.get_string(4096).await;

        loop {
            // Ensure buffer has the right size
            buffer.resize(self.chunk_size, 0);

            let bytes_read = buf_reader.read(&mut buffer).await?;
            if bytes_read == 0 {
                break; // EOF
            }

            // Process the chunk
            let chunk_text = processor(&buffer[..bytes_read])?;

            // Check memory bounds before accumulating
            if accumulated_text.len() + chunk_text.len() > self.max_text_size {
                warn!(
                    "Text extraction truncated at {} characters (limit: {})",
                    accumulated_text.len(),
                    self.max_text_size
                );
                break;
            }

            accumulated_text.push_str(&chunk_text);
        }

        // Return buffer to pool
        self.memory_pool.return_buffer(buffer).await;

        Ok(accumulated_text)
    }

    /// Process a byte slice with memory bounds
    pub async fn process_bytes<F>(&self, content: &[u8], processor: F) -> Result<String>
    where
        F: Fn(&[u8]) -> Result<String>,
    {
        if content.len() > self.max_text_size {
            warn!(
                "Content size {} exceeds limit {}, processing first {} bytes",
                content.len(),
                self.max_text_size,
                self.max_text_size
            );
        }

        let content_to_process = &content[..content.len().min(self.max_text_size)];

        if content_to_process.len() <= self.chunk_size {
            // Small content, process directly
            return processor(content_to_process);
        }

        // Process in chunks for large content
        let mut accumulated_text = self
            .memory_pool
            .get_string(content_to_process.len() / 2)
            .await;

        for chunk in content_to_process.chunks(self.chunk_size) {
            let chunk_text = processor(chunk)?;

            if accumulated_text.len() + chunk_text.len() > self.max_text_size {
                warn!(
                    "Text extraction truncated at {} characters during chunk processing",
                    accumulated_text.len()
                );
                break;
            }

            accumulated_text.push_str(&chunk_text);
        }

        Ok(accumulated_text)
    }
}

/// Utility functions for memory-conscious document processing
pub mod utils {
    use super::*;

    /// Validate file size before processing
    pub async fn validate_file_size(file_path: &std::path::Path) -> Result<usize> {
        let metadata = tokio::fs::metadata(file_path).await?;
        let size = metadata.len() as usize;

        if size > limits::MAX_DOCUMENT_SIZE {
            return Err(anyhow!(
                "File too large: {} MB (maximum: {} MB)",
                size / 1024 / 1024,
                limits::MAX_DOCUMENT_SIZE / 1024 / 1024
            ));
        }

        if size > limits::LARGE_DOCUMENT_WARNING {
            warn!(
                "Processing large file: {} MB ({})",
                size / 1024 / 1024,
                file_path.display()
            );
        }

        Ok(size)
    }

    /// Validate memory content size
    pub fn validate_content_size(content: &[u8]) -> Result<()> {
        if content.len() > limits::MAX_DOCUMENT_SIZE {
            return Err(anyhow!(
                "Content too large: {} MB (maximum: {} MB)",
                content.len() / 1024 / 1024,
                limits::MAX_DOCUMENT_SIZE / 1024 / 1024
            ));
        }

        Ok(())
    }

    /// Truncate text content to memory limits
    pub fn truncate_text_safely(text: &str, max_size: usize) -> String {
        if text.len() <= max_size {
            return text.to_string();
        }

        // Find a safe UTF-8 boundary near the limit
        let mut truncate_at = max_size;
        while truncate_at > 0 && !text.is_char_boundary(truncate_at) {
            truncate_at -= 1;
        }

        let result = text[..truncate_at].to_string();
        warn!(
            "Text truncated from {} to {} characters due to memory limits",
            text.len(),
            result.len()
        );

        result
    }

    /// Calculate optimal chunk size based on content size
    #[allow(dead_code)]
    pub fn calculate_optimal_chunk_size(content_size: usize) -> usize {
        match content_size {
            0..=1024 => 1024,            // 1KB
            1025..=65536 => 4096,        // 4KB
            65537..=1048576 => 16384,    // 16KB
            1048577..=10485760 => 65536, // 64KB
            _ => limits::CHUNK_SIZE,     // Default chunk size
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[tokio::test]
    async fn test_memory_tracker() {
        let tracker = MemoryTracker::new();

        assert_eq!(tracker.current_usage(), 0);
        assert_eq!(tracker.active_documents(), 0);

        let _allocation = tracker.allocate(1024).unwrap();
        assert_eq!(tracker.current_usage(), 1024);

        let _guard = tracker.start_document();
        assert_eq!(tracker.active_documents(), 1);
    }

    #[tokio::test]
    async fn test_memory_pool() {
        let pool = DocumentMemoryPool::new();

        let buffer = pool.get_buffer(1024).await;
        assert!(buffer.capacity() >= 1024);

        pool.return_buffer(buffer).await;

        let string = pool.get_string(512).await;
        assert!(string.capacity() >= 512);

        pool.return_string(string).await;
    }

    #[tokio::test]
    async fn test_streaming_processor() {
        let processor = StreamingTextProcessor::new();
        let content = b"Hello, world! This is test content.";
        let cursor = Cursor::new(content);

        let result = processor
            .process_reader(cursor, |chunk| {
                Ok(String::from_utf8_lossy(chunk).to_string())
            })
            .await
            .unwrap();

        assert_eq!(result, "Hello, world! This is test content.");
    }

    #[tokio::test]
    async fn test_size_validation() {
        // Test file size validation
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let result = utils::validate_file_size(temp_file.path()).await;
        assert!(result.is_ok());

        // Test content size validation
        let small_content = b"small content";
        assert!(utils::validate_content_size(small_content).is_ok());

        // Test text truncation
        let long_text = "a".repeat(1000);
        let truncated = utils::truncate_text_safely(&long_text, 500);
        assert_eq!(truncated.len(), 500);
    }
}
