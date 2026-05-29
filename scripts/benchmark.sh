#!/usr/bin/env bash
# Benchmark script for batless — called by the rust-benchmark reusable workflow.
# Expects: hyperfine on PATH, ./target/release/batless built.
# Writes:  benchmark_results.md in the working directory.
set -euo pipefail

mkdir -p benchmark_files

cat > benchmark_files/medium.rs << 'RUST'
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

pub struct DataProcessor {
    data: HashMap<String, Vec<u8>>,
    config: ProcessorConfig,
}

pub struct ProcessorConfig {
    batch_size: usize,
    compression: bool,
}

impl DataProcessor {
    pub fn new(config: ProcessorConfig) -> Self {
        Self { data: HashMap::new(), config }
    }

    pub fn process_file(&mut self, path: &str) -> io::Result<()> {
        let content = fs::read(path)?;
        self.data.insert(path.to_string(), content);
        Ok(())
    }

    pub fn export_data(&self, format: &str) -> String {
        match format {
            "json" => self.to_json(),
            "csv"  => self.to_csv(),
            _      => "Unsupported format".to_string(),
        }
    }

    fn to_json(&self) -> String {
        let mut json = String::from("{\n");
        for (k, v) in &self.data {
            json.push_str(&format!("  \"{}\": {},\n", k, v.len()));
        }
        json.push('}');
        json
    }

    fn to_csv(&self) -> String {
        let mut csv = String::from("file,size\n");
        for (k, v) in &self.data { csv.push_str(&format!("{},{}\n", k, v.len())); }
        csv
    }
}

fn main() {
    let config = ProcessorConfig { batch_size: 1000, compression: true };
    let mut processor = DataProcessor::new(config);
    println!("Data processor initialized");
}
RUST

BINARY=./target/release/batless

echo "## Performance Benchmark Results"  > benchmark_results.md
echo ""                                  >> benchmark_results.md

for mode in plain json summary index; do
  echo "### ${mode^} Mode" >> benchmark_results.md
  hyperfine --warmup 3 --runs 10 \
    "${BINARY} benchmark_files/medium.rs --mode=${mode}" \
    --export-markdown "${mode}_bench.md"
  cat "${mode}_bench.md" >> benchmark_results.md
  echo "" >> benchmark_results.md
done

echo "Benchmark complete. Results written to benchmark_results.md"
