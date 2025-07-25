use batless::{highlight_content, process_file, BatlessConfig};
use proptest::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn test_process_file_never_panics(
        content in ".*",
        max_lines in 1usize..1000,
        max_bytes in proptest::option::of(1usize..10000)
    ) {
        let mut file = NamedTempFile::new().expect("Failed to create test file");
        file.write_all(content.as_bytes()).expect("Failed to write test content");

        let config = BatlessConfig {
            max_lines,
            max_bytes,
            ..Default::default()
        };

        // Should never panic, even with arbitrary input
        if let Some(path) = file.path().to_str() {
            let _result = process_file(path, &config);
        }
    }

    #[test]
    fn test_highlight_content_deterministic(content in ".*") {
        let config = BatlessConfig::default();

        // Highlighting should be deterministic
        let result1 = highlight_content(&content, "test.rs", &config);
        let result2 = highlight_content(&content, "test.rs", &config);

        prop_assert_eq!(result1.is_ok(), result2.is_ok());

        if let (Ok(r1), Ok(r2)) = (&result1, &result2) {
            prop_assert_eq!(r1, r2);
        }
    }

    #[test]
    fn test_max_lines_respected(
        lines in prop::collection::vec(".*", 1..50),
        max_lines in 1usize..25
    ) {
        let content = lines.join("\n");
        let mut file = NamedTempFile::new().expect("Failed to create test file");
        file.write_all(content.as_bytes()).expect("Failed to write test content");

        let config = BatlessConfig {
            max_lines,
            ..Default::default()
        };

        let file_path = file.path().to_str().expect("Failed to convert path");
        if let Ok(result) = process_file(file_path, &config) {
            prop_assert!(result.lines.len() <= max_lines);
            // The output should contain at most max_lines lines
            prop_assert!(result.lines.len() <= max_lines);
        }
    }

    #[test]
    fn test_max_bytes_respected(
        content in ".*",
        max_bytes in 1usize..1000
    ) {
        let mut file = NamedTempFile::new().expect("Failed to create test file");
        file.write_all(content.as_bytes()).expect("Failed to write test content");

        let config = BatlessConfig {
            max_bytes: Some(max_bytes),
            max_lines: 10000, // High limit so bytes is the constraint
            ..Default::default()
        };

        let file_path = file.path().to_str().expect("Failed to convert path");
        if let Ok(result) = process_file(file_path, &config) {
            prop_assert!(result.total_bytes <= max_bytes + 100); // Allow some margin for line boundaries
        }
    }

    #[test]
    fn test_summary_mode_stability(
        rust_code in r"(fn |struct |impl |use |pub )[a-zA-Z0-9_\s\{\}\(\);]*",
    ) {
        let mut file = NamedTempFile::new().expect("Failed to create test file");
        file.write_all(rust_code.as_bytes()).expect("Failed to write test content");

        let config = BatlessConfig {
            summary_mode: true,
            ..Default::default()
        };

        // Summary mode should not panic and should return a subset of lines
        let file_path = file.path().to_str().expect("Failed to convert path");
        if let Ok(result) = process_file(file_path, &config) {
            if let Some(summary_lines) = &result.summary_lines {
                prop_assert!(summary_lines.len() <= result.total_lines);
                prop_assert_eq!(result.lines, summary_lines.clone());
            }
        }
    }

    #[test]
    fn test_encoding_detection_stability(
        content in prop::collection::vec(any::<u8>(), 0..1000)
    ) {
        let mut file = NamedTempFile::new().expect("Failed to create test file");
        file.write_all(&content).expect("Failed to write test content");

        let config = BatlessConfig::default();

        // Should handle any byte sequence without panicking
        let file_path = file.path().to_str().expect("Failed to convert path");
        let _result = process_file(file_path, &config);
    }
}
