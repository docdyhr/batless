//! Technical debt prevention mechanisms
//!
//! This module provides tools and utilities for preventing the accumulation
//! of technical debt in the codebase through automated checks and guidelines.

use std::fs;
use std::path::Path;

/// Code quality metrics and thresholds
#[derive(Debug, Clone)]
pub struct QualityGate {
    pub max_function_lines: usize,
    pub max_module_lines: usize,
    pub max_clone_calls: usize,
    pub max_unwrap_calls: usize,
    pub max_cyclomatic_complexity: usize,
    pub min_test_coverage: f64,
}

impl Default for QualityGate {
    fn default() -> Self {
        Self {
            max_function_lines: 50,
            max_module_lines: 500,
            max_clone_calls: 10,
            max_unwrap_calls: 0,
            max_cyclomatic_complexity: 10,
            min_test_coverage: 80.0,
        }
    }
}

/// Code quality checker
pub struct QualityChecker {
    gates: QualityGate,
}

impl QualityChecker {
    /// Create a new quality checker with default gates
    pub fn new() -> Self {
        Self {
            gates: QualityGate::default(),
        }
    }
    
    /// Create a quality checker with custom gates
    pub fn with_gates(gates: QualityGate) -> Self {
        Self { gates }
    }
    
    /// Check a single file against quality gates
    pub fn check_file(&self, file_path: &Path) -> QualityReport {
        let mut report = QualityReport::new(file_path.to_path_buf());
        
        if let Ok(content) = fs::read_to_string(file_path) {
            let lines: Vec<&str> = content.lines().collect();
            
            // Check module size
            if lines.len() > self.gates.max_module_lines {
                report.violations.push(QualityViolation {
                    rule: "module_size".to_string(),
                    message: format!(
                        "Module has {} lines (max: {})", 
                        lines.len(), 
                        self.gates.max_module_lines
                    ),
                    severity: Severity::High,
                    location: Location::Module,
                });
            }
            
            // Check for clone() usage
            let clone_count = content.matches(".clone()").count();
            if clone_count > self.gates.max_clone_calls {
                report.violations.push(QualityViolation {
                    rule: "clone_usage".to_string(),
                    message: format!(
                        "Found {} clone() calls (max: {})", 
                        clone_count, 
                        self.gates.max_clone_calls
                    ),
                    severity: Severity::Medium,
                    location: Location::Module,
                });
            }
            
            // Check for unwrap() usage in non-test code
            if !file_path.to_string_lossy().contains("test") {
                let unwrap_count = content.matches(".unwrap()").count();
                if unwrap_count > self.gates.max_unwrap_calls {
                    report.violations.push(QualityViolation {
                        rule: "unwrap_usage".to_string(),
                        message: format!(
                            "Found {} unwrap() calls in production code (max: {})", 
                            unwrap_count, 
                            self.gates.max_unwrap_calls
                        ),
                        severity: Severity::High,
                        location: Location::Module,
                    });
                }
            }
            
            // Check for TODO/FIXME comments
            let todo_count = content.matches("TODO").count() + content.matches("FIXME").count();
            if todo_count > 0 {
                report.violations.push(QualityViolation {
                    rule: "todo_comments".to_string(),
                    message: format!("Found {} TODO/FIXME comments", todo_count),
                    severity: Severity::Low,
                    location: Location::Module,
                });
            }
            
            // Check function sizes
            self.check_function_sizes(&content, &mut report);
        }
        
        report
    }
    
    /// Check function sizes in the content
    fn check_function_sizes(&self, content: &str, report: &mut QualityReport) {
        let lines: Vec<&str> = content.lines().collect();
        let mut in_function = false;
        let mut function_start = 0;
        let mut brace_count = 0;
        
        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            // Simple heuristic for function detection
            if (trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") || 
                trimmed.starts_with("async fn ")) && trimmed.contains("(") {
                in_function = true;
                function_start = line_num;
                brace_count = 0;
            }
            
            if in_function {
                brace_count += line.matches('{').count() as i32;
                brace_count -= line.matches('}').count() as i32;
                
                if brace_count == 0 && line.contains('}') {
                    in_function = false;
                    let function_lines = line_num - function_start + 1;
                    
                    if function_lines > self.gates.max_function_lines {
                        report.violations.push(QualityViolation {
                            rule: "function_size".to_string(),
                            message: format!(
                                "Function at line {} has {} lines (max: {})", 
                                function_start + 1, 
                                function_lines, 
                                self.gates.max_function_lines
                            ),
                            severity: Severity::Medium,
                            location: Location::Line(function_start + 1),
                        });
                    }
                }
            }
        }
    }
    
    /// Check an entire directory recursively
    pub fn check_directory(&self, dir_path: &Path) -> Vec<QualityReport> {
        let mut reports = Vec::new();
        
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                    reports.push(self.check_file(&path));
                } else if path.is_dir() && !path.file_name().map_or(false, |name| name == "target") {
                    reports.extend(self.check_directory(&path));
                }
            }
        }
        
        reports
    }
}

impl Default for QualityChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Quality report for a single file
#[derive(Debug, Clone)]
pub struct QualityReport {
    pub file_path: std::path::PathBuf,
    pub violations: Vec<QualityViolation>,
}

impl QualityReport {
    /// Create a new quality report
    pub fn new(file_path: std::path::PathBuf) -> Self {
        Self {
            file_path,
            violations: Vec::new(),
        }
    }
    
    /// Check if the file passes quality gates
    pub fn passes(&self) -> bool {
        self.violations.is_empty() || 
        self.violations.iter().all(|v| matches!(v.severity, Severity::Low))
    }
    
    /// Get high severity violations
    pub fn high_severity_violations(&self) -> Vec<&QualityViolation> {
        self.violations.iter()
            .filter(|v| matches!(v.severity, Severity::High))
            .collect()
    }
    
    /// Get violations count by severity
    pub fn violations_by_severity(&self, severity: Severity) -> usize {
        self.violations.iter()
            .filter(|v| v.severity == severity)
            .count()
    }
}

/// Quality violation
#[derive(Debug, Clone)]
pub struct QualityViolation {
    pub rule: String,
    pub message: String,
    pub severity: Severity,
    pub location: Location,
}

/// Violation severity levels
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// Location in code where violation occurs
#[derive(Debug, Clone)]
pub enum Location {
    Module,
    Line(usize),
    Function(String),
}

/// Architecture decision record template
pub struct ArchitectureDecisionRecord {
    pub title: String,
    pub status: ADRStatus,
    pub context: String,
    pub decision: String,
    pub consequences: Vec<String>,
    pub date: String,
}

/// ADR status
#[derive(Debug, Clone)]
pub enum ADRStatus {
    Proposed,
    Accepted,
    Deprecated,
    Superseded,
}

impl ArchitectureDecisionRecord {
    /// Create a new ADR
    pub fn new(title: String, context: String, decision: String) -> Self {
        Self {
            title,
            status: ADRStatus::Proposed,
            context,
            decision,
            consequences: Vec::new(),
            date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        }
    }
    
    /// Add a consequence
    pub fn add_consequence(&mut self, consequence: String) {
        self.consequences.push(consequence);
    }
    
    /// Generate markdown format
    pub fn to_markdown(&self) -> String {
        format!(
            r#"# {title}

**Status:** {status:?}
**Date:** {date}

## Context

{context}

## Decision

{decision}

## Consequences

{consequences}
"#,
            title = self.title,
            status = self.status,
            date = self.date,
            context = self.context,
            decision = self.decision,
            consequences = self.consequences
                .iter()
                .map(|c| format!("- {}", c))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_quality_gate_defaults() {
        let gate = QualityGate::default();
        assert_eq!(gate.max_function_lines, 50);
        assert_eq!(gate.max_module_lines, 500);
        assert_eq!(gate.max_unwrap_calls, 0);
    }
    
    #[test]
    fn test_quality_checker() {
        let checker = QualityChecker::new();
        
        // Create a test file with violations
        let mut temp_file = NamedTempFile::new().unwrap();
        let test_content = r#"
fn bad_function() {
    let x = some_value.unwrap();
    x.clone()
}

fn another_function() {
    // TODO: implement this
    let y = value.clone();
}
"#;
        fs::write(temp_file.path(), test_content).unwrap();
        
        let report = checker.check_file(temp_file.path());
        assert!(!report.violations.is_empty());
        
        // Should find unwrap and clone violations
        let has_unwrap_violation = report.violations.iter()
            .any(|v| v.rule == "unwrap_usage");
        assert!(has_unwrap_violation);
    }
    
    #[test]
    fn test_adr_creation() {
        let mut adr = ArchitectureDecisionRecord::new(
            "Use trait objects for formatters".to_string(),
            "We need flexible formatting".to_string(),
            "Use trait objects to enable different formatters".to_string(),
        );
        
        adr.add_consequence("Better testability".to_string());
        adr.add_consequence("More flexible architecture".to_string());
        
        let markdown = adr.to_markdown();
        assert!(markdown.contains("Use trait objects"));
        assert!(markdown.contains("Better testability"));
    }
}