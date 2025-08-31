// Large file to stress test performance
use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read, BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{spawn, JoinHandle};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Configuration for data processing operations
#[derive(Clone, Debug)]
pub struct ProcessorConfig {
    pub batch_size: usize,
    pub compression_enabled: bool,
    pub parallel_processing: bool,
    pub cache_size: usize,
    pub timeout_seconds: u64,
    pub retry_attempts: usize,
    pub buffer_size: usize,
    pub max_memory_usage: usize,
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        Self {
            batch_size: 1000,
            compression_enabled: true,
            parallel_processing: false,
            cache_size: 10_000,
            timeout_seconds: 30,
            retry_attempts: 3,
            buffer_size: 8192,
            max_memory_usage: 1024 * 1024 * 100, // 100MB
        }
    }
}

/// Advanced data processor with caching and parallel processing capabilities
pub struct AdvancedDataProcessor {
    data: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    config: ProcessorConfig,
    cache: Arc<Mutex<BTreeMap<String, CacheEntry>>>,
    stats: Arc<Mutex<ProcessingStats>>,
    thread_pool: Vec<JoinHandle<()>>,
}

#[derive(Clone, Debug)]
struct CacheEntry {
    data: Vec<u8>,
    timestamp: SystemTime,
    access_count: usize,
    checksum: u64,
}

#[derive(Default, Debug)]
struct ProcessingStats {
    files_processed: usize,
    total_bytes: usize,
    cache_hits: usize,
    cache_misses: usize,
    processing_time_ms: u128,
    error_count: usize,
}

impl AdvancedDataProcessor {
    pub fn new(config: ProcessorConfig) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            config,
            cache: Arc::new(Mutex::new(BTreeMap::new())),
            stats: Arc::new(Mutex::new(ProcessingStats::default())),
            thread_pool: Vec::new(),
        }
    }

    pub fn process_file(&self, path: &str) -> io::Result<ProcessingResult> {
        let start_time = Instant::now();

        // Check cache first
        if let Some(cached) = self.get_from_cache(path) {
            let mut stats = self.stats.lock().unwrap();
            stats.cache_hits += 1;
            return Ok(ProcessingResult::from_cache(cached));
        }

        // Read and process file
        let content = std::fs::read(path)?;
        let checksum = self.calculate_checksum(&content);

        // Update cache
        self.update_cache(path, &content, checksum);

        // Store in main data structure
        {
            let mut data = self.data.write().unwrap();
            data.insert(path.to_string(), content.clone());
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.files_processed += 1;
            stats.total_bytes += content.len();
            stats.processing_time_ms += start_time.elapsed().as_millis();
            stats.cache_misses += 1;
        }

        Ok(ProcessingResult::new(path, content, checksum))
    }

    pub fn batch_process(&self, paths: Vec<&str>) -> Vec<Result<ProcessingResult, io::Error>> {
        if self.config.parallel_processing {
            self.batch_process_parallel(paths)
        } else {
            self.batch_process_sequential(paths)
        }
    }

    fn batch_process_sequential(&self, paths: Vec<&str>) -> Vec<Result<ProcessingResult, io::Error>> {
        let mut results = Vec::with_capacity(paths.len());

        for path in paths {
            results.push(self.process_file(path));
        }

        results
    }

    fn batch_process_parallel(&self, paths: Vec<&str>) -> Vec<Result<ProcessingResult, io::Error>> {
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();
        let mut handles = Vec::new();

        for (index, path) in paths.iter().enumerate() {
            let tx_clone = tx.clone();
            let path_owned = path.to_string();
            let processor = self.clone();

            let handle = spawn(move || {
                let result = processor.process_file(&path_owned);
                tx_clone.send((index, result)).unwrap();
            });

            handles.push(handle);
        }

        drop(tx);

        let mut results = vec![Ok(ProcessingResult::empty()); paths.len()];

        for (index, result) in rx {
            results[index] = result;
        }

        for handle in handles {
            handle.join().unwrap();
        }

        results
    }

    fn get_from_cache(&self, path: &str) -> Option<CacheEntry> {
        let mut cache = self.cache.lock().unwrap();

        if let Some(entry) = cache.get_mut(path) {
            entry.access_count += 1;
            Some(entry.clone())
        } else {
            None
        }
    }

    fn update_cache(&self, path: &str, content: &[u8], checksum: u64) {
        let mut cache = self.cache.lock().unwrap();

        // Implement LRU eviction if cache is full
        if cache.len() >= self.config.cache_size {
            self.evict_least_recently_used(&mut cache);
        }

        let entry = CacheEntry {
            data: content.to_vec(),
            timestamp: SystemTime::now(),
            access_count: 1,
            checksum,
        };

        cache.insert(path.to_string(), entry);
    }

    fn evict_least_recently_used(&self, cache: &mut BTreeMap<String, CacheEntry>) {
        if let Some((oldest_key, _)) = cache
            .iter()
            .min_by_key(|(_, entry)| (entry.timestamp, entry.access_count))
            .map(|(k, v)| (k.clone(), v.clone()))
        {
            cache.remove(&oldest_key);
        }
    }

    fn calculate_checksum(&self, data: &[u8]) -> u64 {
        // Simple checksum implementation
        data.iter().enumerate().fold(0u64, |acc, (i, &byte)| {
            acc.wrapping_mul(31).wrapping_add(byte as u64).wrapping_add(i as u64)
        })
    }

    pub fn export_data(&self, format: &str) -> io::Result<String> {
        let data = self.data.read().unwrap();

        match format {
            "json" => Ok(self.to_json(&data)),
            "csv" => Ok(self.to_csv(&data)),
            "xml" => Ok(self.to_xml(&data)),
            "yaml" => Ok(self.to_yaml(&data)),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported export format",
            )),
        }
    }

    fn to_json(&self, data: &HashMap<String, Vec<u8>>) -> String {
        let mut json = String::from("{\n  \"files\": [\n");

        for (i, (key, value)) in data.iter().enumerate() {
            if i > 0 {
                json.push_str(",\n");
            }
            json.push_str(&format!(
                "    {{\n      \"path\": \"{}\",\n      \"size\": {},\n      \"checksum\": \"{:x}\"\n    }}",
                key,
                value.len(),
                self.calculate_checksum(value)
            ));
        }

        json.push_str("\n  ],\n");
        json.push_str(&format!(
            "  \"metadata\": {{\n    \"total_files\": {},\n    \"total_size\": {}\n  }}\n",
            data.len(),
            data.values().map(|v| v.len()).sum::<usize>()
        ));
        json.push_str("}");
        json
    }

    fn to_csv(&self, data: &HashMap<String, Vec<u8>>) -> String {
        let mut csv = String::from("path,size,checksum\n");

        for (key, value) in data {
            csv.push_str(&format!(
                "{},{},{:x}\n",
                key,
                value.len(),
                self.calculate_checksum(value)
            ));
        }

        csv
    }

    fn to_xml(&self, data: &HashMap<String, Vec<u8>>) -> String {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<files>\n");

        for (key, value) in data {
            xml.push_str(&format!(
                "  <file path=\"{}\" size=\"{}\" checksum=\"{:x}\" />\n",
                key,
                value.len(),
                self.calculate_checksum(value)
            ));
        }

        xml.push_str("</files>");
        xml
    }

    fn to_yaml(&self, data: &HashMap<String, Vec<u8>>) -> String {
        let mut yaml = String::from("files:\n");

        for (key, value) in data {
            yaml.push_str(&format!(
                "  - path: \"{}\"\n    size: {}\n    checksum: \"{:x}\"\n",
                key,
                value.len(),
                self.calculate_checksum(value)
            ));
        }

        yaml
    }

    pub fn get_statistics(&self) -> ProcessingStats {
        self.stats.lock().unwrap().clone()
    }

    pub fn clear_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    pub fn optimize_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        let now = SystemTime::now();

        // Remove entries older than 1 hour
        cache.retain(|_, entry| {
            now.duration_since(entry.timestamp)
                .map(|duration| duration.as_secs() < 3600)
                .unwrap_or(false)
        });
    }
}

impl Clone for AdvancedDataProcessor {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
            config: self.config.clone(),
            cache: Arc::clone(&self.cache),
            stats: Arc::clone(&self.stats),
            thread_pool: Vec::new(), // Don't clone thread handles
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcessingResult {
    pub path: String,
    pub size: usize,
    pub checksum: u64,
    pub processing_time_ms: u128,
    pub from_cache: bool,
}

impl ProcessingResult {
    fn new(path: &str, content: Vec<u8>, checksum: u64) -> Self {
        Self {
            path: path.to_string(),
            size: content.len(),
            checksum,
            processing_time_ms: 0,
            from_cache: false,
        }
    }

    fn from_cache(cache_entry: CacheEntry) -> Self {
        Self {
            path: String::new(),
            size: cache_entry.data.len(),
            checksum: cache_entry.checksum,
            processing_time_ms: 0,
            from_cache: true,
        }
    }

    fn empty() -> Self {
        Self {
            path: String::new(),
            size: 0,
            checksum: 0,
            processing_time_ms: 0,
            from_cache: false,
        }
    }
}

fn main() {
    let config = ProcessorConfig {
        batch_size: 500,
        compression_enabled: true,
        parallel_processing: true,
        cache_size: 5000,
        timeout_seconds: 60,
        retry_attempts: 5,
        buffer_size: 16384,
        max_memory_usage: 1024 * 1024 * 200, // 200MB
    };

    let processor = AdvancedDataProcessor::new(config);
    println!("Advanced data processor initialized with caching and parallel processing");

    // Example usage
    match processor.process_file("example.txt") {
        Ok(result) => println!("Processed {}: {} bytes", result.path, result.size),
        Err(e) => eprintln!("Error processing file: {}", e),
    }

    let stats = processor.get_statistics();
    println!("Processing statistics: {:?}", stats);
}
