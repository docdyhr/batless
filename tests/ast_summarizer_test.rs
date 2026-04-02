use batless::ast_summarizer::AstSummarizer;
use batless::summary::SummaryLevel;

#[test]
fn test_rust_ast_summary() {
    let code = r#"
fn main() {
    println!("Hello");
}

struct User {
    name: String,
}

impl User {
    fn new(name: String) -> Self {
        Self { name }
    }
}
"#;

    let summary = AstSummarizer::extract_summary(code, Some("Rust"), SummaryLevel::Standard);

    // Debug output
    println!("Summary: {:?}", summary);

    assert!(summary.iter().any(|l| l.line.contains("fn main")));
    assert!(summary.iter().any(|l| l.line.contains("struct User")));
    assert!(summary.iter().any(|l| l.line.contains("impl User")));
    assert!(summary.iter().any(|l| l.line.contains("fn new")));

    // Should NOT contain the body lines (unless they are on the same line as definition)
    assert!(!summary.iter().any(|l| l.line.contains("println!")));
    assert!(!summary.iter().any(|l| l.line.contains("Self { name }")));
}
