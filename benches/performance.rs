use batless::{highlight_content, process_file, BatlessConfig, LanguageDetector, ThemeManager};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use std::io::Write;
use tempfile::NamedTempFile;

fn create_test_file(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file
}

fn benchmark_process_file(c: &mut Criterion) {
    let mut group = c.benchmark_group("process_file");

    // Test with different file sizes
    let sizes = vec![1000, 10000, 100000];

    for size in sizes {
        let content = "fn main() {\n    println!(\"Hello, world!\");\n}\n".repeat(size / 50);
        let file = create_test_file(&content);
        let config = BatlessConfig::default();

        group.throughput(Throughput::Bytes(content.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("file_size", size),
            &(file.path().to_str().unwrap(), &config),
            |b, (path, config)| b.iter(|| black_box(process_file(path, config).unwrap())),
        );
    }

    group.finish();
}

fn benchmark_highlight_content(c: &mut Criterion) {
    let mut group = c.benchmark_group("highlight_content");

    let test_cases = vec![
        (
            "rust",
            "fn main() {\n    println!(\"Hello, world!\");\n}",
            "test.rs",
        ),
        (
            "python",
            "def main():\n    print('Hello, world!')\n",
            "test.py",
        ),
        ("json", r#"{"hello": "world", "number": 42}"#, "test.json"),
        (
            "plain",
            "This is plain text\nwith multiple lines\n",
            "test.txt",
        ),
    ];

    for (lang, content, filename) in test_cases {
        let config = BatlessConfig::default();
        group.bench_function(lang, |b| {
            b.iter(|| black_box(highlight_content(content, filename, &config).unwrap()))
        });
    }

    group.finish();
}

fn benchmark_summary_mode(c: &mut Criterion) {
    let rust_code = r#"
use std::io;

pub struct Config {
    pub debug: bool,
    pub timeout: u64,
}

impl Config {
    pub fn new() -> Self {
        Self { debug: false, timeout: 30 }
    }
}

pub fn process_data(data: &str) -> Result<String, io::Error> {
    // Process the data
    Ok(data.to_uppercase())
}

fn main() {
    let config = Config::new();
    match process_data("hello") {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
"#
    .repeat(10); // Repeat to make it larger

    let file = create_test_file(&rust_code);

    let summary_config = BatlessConfig {
        summary_mode: true,
        ..Default::default()
    };

    let regular_config = BatlessConfig {
        summary_mode: false,
        ..Default::default()
    };

    c.bench_function("summary_mode_enabled", |b| {
        b.iter(|| black_box(process_file(file.path().to_str().unwrap(), &summary_config).unwrap()))
    });

    c.bench_function("summary_mode_disabled", |b| {
        b.iter(|| black_box(process_file(file.path().to_str().unwrap(), &regular_config).unwrap()))
    });
}

fn benchmark_max_lines_limits(c: &mut Criterion) {
    let large_content = "Line of text\n".repeat(10000);
    let file = create_test_file(&large_content);

    let mut group = c.benchmark_group("max_lines");

    for max_lines in [100, 1000, 5000, 10000].iter() {
        let config = BatlessConfig {
            max_lines: *max_lines,
            ..Default::default()
        };

        group.bench_with_input(
            BenchmarkId::new("limit", max_lines),
            &(file.path().to_str().unwrap(), &config),
            |b, (path, config)| b.iter(|| black_box(process_file(path, config).unwrap())),
        );
    }

    group.finish();
}

fn benchmark_startup_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("startup_operations");

    // Benchmark operations that should be fast and not load heavy syntax sets
    group.bench_function("list_languages", |b| {
        b.iter(|| black_box(LanguageDetector::list_languages()))
    });

    group.bench_function("list_themes", |b| {
        b.iter(|| black_box(ThemeManager::list_themes()))
    });

    // Benchmark config loading with precedence
    group.bench_function("config_default", |b| {
        b.iter(|| black_box(BatlessConfig::default()))
    });

    group.bench_function("config_load_with_precedence", |b| {
        b.iter(|| black_box(BatlessConfig::load_with_precedence().unwrap()))
    });

    // Benchmark validation operations
    group.bench_function("validate_theme", |b| {
        b.iter(|| black_box(ThemeManager::validate_theme("base16-ocean.dark").unwrap()))
    });

    group.bench_function("validate_language", |b| {
        b.iter(|| black_box(LanguageDetector::validate_language("rust").unwrap()))
    });

    group.finish();
}

fn benchmark_config_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("config_operations");

    // Test config validation performance
    let configs = vec![
        ("default", BatlessConfig::default()),
        ("with_limits", BatlessConfig::default().with_max_lines(5000).with_max_bytes(Some(1_000_000))),
        ("with_summary", BatlessConfig::default().with_summary_mode(true)),
        ("with_tokens", BatlessConfig::default().with_include_tokens(true)),
    ];

    for (name, config) in configs {
        group.bench_function(name, |b| {
            b.iter(|| black_box(config.validate().unwrap()))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_process_file,
    benchmark_highlight_content,
    benchmark_summary_mode,
    benchmark_max_lines_limits,
    benchmark_startup_operations,
    benchmark_config_operations
);
criterion_main!(benches);
