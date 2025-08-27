//! Performance optimization utilities
//!
//! This module provides utilities for improving performance including
//! caching, memory optimization, and efficient data structures.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Instant;

/// Performance metrics collector
#[derive(Debug, Default, Clone)]
pub struct PerformanceMetrics {
    pub file_processing_time: u128,
    pub highlighting_time: u128,
    pub summarization_time: u128,
    pub tokenization_time: u128,
    pub total_bytes_processed: usize,
    pub total_lines_processed: usize,
}

impl PerformanceMetrics {
    /// Create a new metrics instance
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add file processing time
    pub fn add_processing_time(&mut self, duration_nanos: u128) {
        self.file_processing_time += duration_nanos;
    }
    
    /// Add highlighting time
    pub fn add_highlighting_time(&mut self, duration_nanos: u128) {
        self.highlighting_time += duration_nanos;
    }
    
    /// Add summarization time
    pub fn add_summarization_time(&mut self, duration_nanos: u128) {
        self.summarization_time += duration_nanos;
    }
    
    /// Add tokenization time
    pub fn add_tokenization_time(&mut self, duration_nanos: u128) {
        self.tokenization_time += duration_nanos;
    }
    
    /// Record bytes processed
    pub fn record_bytes_processed(&mut self, bytes: usize) {
        self.total_bytes_processed += bytes;
    }
    
    /// Record lines processed
    pub fn record_lines_processed(&mut self, lines: usize) {
        self.total_lines_processed += lines;
    }
    
    /// Get total processing time
    pub fn total_time(&self) -> u128 {
        self.file_processing_time + self.highlighting_time + self.summarization_time + self.tokenization_time
    }
    
    /// Get processing rate in bytes per second
    pub fn bytes_per_second(&self) -> f64 {
        if self.total_time() == 0 {
            0.0
        } else {
            (self.total_bytes_processed as f64 * 1_000_000_000.0) / self.total_time() as f64
        }
    }
    
    /// Get processing rate in lines per second
    pub fn lines_per_second(&self) -> f64 {
        if self.total_time() == 0 {
            0.0
        } else {
            (self.total_lines_processed as f64 * 1_000_000_000.0) / self.total_time() as f64
        }
    }
}

/// Simple LRU-like cache for language detection results
pub struct LanguageCache {
    cache: RwLock<HashMap<String, String>>,
    max_size: usize,
}

impl LanguageCache {
    /// Create a new language cache
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            max_size,
        }
    }
    
    /// Get cached language for file extension
    pub fn get(&self, extension: &str) -> Option<String> {
        self.cache.read().ok()?.get(extension).cloned()
    }
    
    /// Cache language for file extension
    pub fn insert(&self, extension: String, language: String) {
        if let Ok(mut cache) = self.cache.write() {
            if cache.len() >= self.max_size {
                // Simple eviction: remove first entry
                if let Some(first_key) = cache.keys().next().cloned() {
                    cache.remove(&first_key);
                }
            }
            cache.insert(extension, language);
        }
    }
    
    /// Clear the cache
    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
    }
    
    /// Get cache size
    pub fn size(&self) -> usize {
        self.cache.read().map(|c| c.len()).unwrap_or(0)
    }
}

/// Global language cache instance
static LANGUAGE_CACHE: std::sync::OnceLock<LanguageCache> = std::sync::OnceLock::new();

/// Get or initialize the global language cache
pub fn language_cache() -> &'static LanguageCache {
    LANGUAGE_CACHE.get_or_init(|| LanguageCache::new(1000))
}

/// Performance timer for measuring operation durations
pub struct Timer {
    start: Instant,
    label: String,
}

impl Timer {
    /// Start a new timer
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            label: label.into(),
        }
    }
    
    /// Get elapsed time in nanoseconds
    pub fn elapsed_nanos(&self) -> u128 {
        self.start.elapsed().as_nanos()
    }
    
    /// Get elapsed time in microseconds
    pub fn elapsed_micros(&self) -> u128 {
        self.start.elapsed().as_micros()
    }
    
    /// Get elapsed time in milliseconds
    pub fn elapsed_millis(&self) -> u128 {
        self.start.elapsed().as_millis()
    }
    
    /// Restart the timer
    pub fn restart(&mut self) {
        self.start = Instant::now();
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        if cfg!(debug_assertions) {
            eprintln!("Timer '{}': {}μs", self.label, self.elapsed_micros());
        }
    }
}

/// Macro for easy timing of code blocks
#[macro_export]
macro_rules! time_operation {
    ($label:expr, $operation:expr) => {{
        let _timer = $crate::performance::Timer::new($label);
        let result = $operation;
        result
    }};
}

/// String pool for reducing allocations of common strings
pub struct StringPool {
    pool: RwLock<HashMap<String, Arc<String>>>,
}

impl StringPool {
    /// Create a new string pool
    pub fn new() -> Self {
        Self {
            pool: RwLock::new(HashMap::new()),
        }
    }
    
    /// Get or create an Arc<String> from the pool
    pub fn get_or_insert(&self, s: &str) -> Arc<String> {
        // First try to get existing
        if let Ok(pool) = self.pool.read() {
            if let Some(existing) = pool.get(s) {
                return Arc::clone(existing);
            }
        }
        
        // Create new entry
        if let Ok(mut pool) = self.pool.write() {
            // Double-check in case another thread inserted it
            if let Some(existing) = pool.get(s) {
                return Arc::clone(existing);
            }
            
            let arc_string = Arc::new(s.to_string());
            pool.insert(s.to_string(), Arc::clone(&arc_string));
            arc_string
        } else {
            // Fallback if lock fails
            Arc::new(s.to_string())
        }
    }
    
    /// Clear the string pool
    pub fn clear(&self) {
        if let Ok(mut pool) = self.pool.write() {
            pool.clear();
        }
    }
    
    /// Get pool size
    pub fn size(&self) -> usize {
        self.pool.read().map(|p| p.len()).unwrap_or(0)
    }
}

impl Default for StringPool {
    fn default() -> Self {
        Self::new()
    }
}

/// Global string pool for common strings
static STRING_POOL: std::sync::OnceLock<StringPool> = std::sync::OnceLock::new();

/// Get or initialize the global string pool
pub fn string_pool() -> &'static StringPool {
    STRING_POOL.get_or_init(StringPool::new)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_performance_metrics() {
        let mut metrics = PerformanceMetrics::new();
        metrics.add_processing_time(1_000_000); // 1ms
        metrics.record_bytes_processed(1024);
        metrics.record_lines_processed(50);
        
        assert_eq!(metrics.total_time(), 1_000_000);
        assert!(metrics.bytes_per_second() > 0.0);
        assert!(metrics.lines_per_second() > 0.0);
    }
    
    #[test]
    fn test_language_cache() {
        let cache = LanguageCache::new(2);
        
        cache.insert("rs".to_string(), "Rust".to_string());
        assert_eq!(cache.get("rs"), Some("Rust".to_string()));
        assert_eq!(cache.size(), 1);
        
        cache.insert("js".to_string(), "JavaScript".to_string());
        assert_eq!(cache.size(), 2);
        
        // Should evict first entry when at capacity
        cache.insert("py".to_string(), "Python".to_string());
        assert_eq!(cache.size(), 2);
    }
    
    #[test]
    fn test_timer() {
        let mut timer = Timer::new("test");
        thread::sleep(Duration::from_millis(1));
        assert!(timer.elapsed_micros() >= 1000); // At least 1ms
        
        timer.restart();
        let elapsed_after_restart = timer.elapsed_micros();
        assert!(elapsed_after_restart < 1000); // Should be very small
    }
    
    #[test]
    fn test_string_pool() {
        let pool = StringPool::new();
        let s1 = pool.get_or_insert("hello");
        let s2 = pool.get_or_insert("hello");
        
        // Should be the same Arc
        assert!(Arc::ptr_eq(&s1, &s2));
        assert_eq!(pool.size(), 1);
        
        let s3 = pool.get_or_insert("world");
        assert_eq!(pool.size(), 2);
        assert!(!Arc::ptr_eq(&s1, &s3));
    }
    
    #[test]
    fn test_global_caches() {
        let cache = language_cache();
        cache.clear();
        assert_eq!(cache.size(), 0);
        
        let pool = string_pool();
        pool.clear();
        assert_eq!(pool.size(), 0);
    }
}