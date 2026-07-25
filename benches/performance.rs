use batless::{process_file, BatlessConfig, LanguageDetector};
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
    let sizes = vec![1000, 10_000, 100_000];

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

fn benchmark_max_lines_limits(c: &mut Criterion) {
    let large_content = "Line of text\n".repeat(10000);
    let file = create_test_file(&large_content);

    let mut group = c.benchmark_group("max_lines");

    for max_lines in &[100, 1000, 5000, 10000] {
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

    group.bench_function("list_languages", |b| {
        b.iter(|| black_box(LanguageDetector::list_languages()));
    });

    group.bench_function("config_default", |b| {
        b.iter(|| black_box(BatlessConfig::default()));
    });

    group.bench_function("config_load_with_precedence", |b| {
        b.iter(|| black_box(BatlessConfig::load_with_precedence().unwrap()));
    });

    group.bench_function("validate_language", |b| {
        b.iter(|| {
            LanguageDetector::validate_language("rust").unwrap();
            black_box(());
        });
    });

    group.finish();
}

fn benchmark_config_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("config_operations");

    // Test config validation performance
    let configs = vec![
        ("default", BatlessConfig::default()),
        (
            "with_limits",
            BatlessConfig::default()
                .with_max_lines(5000)
                .with_max_bytes(Some(1_000_000)),
        ),
    ];

    for (name, config) in configs {
        group.bench_function(name, |b| {
            b.iter(|| {
                config.validate().unwrap();
                black_box(());
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_process_file,
    benchmark_max_lines_limits,
    benchmark_startup_operations,
    benchmark_config_operations
);
criterion_main!(benches);
