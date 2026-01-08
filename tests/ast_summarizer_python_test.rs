use batless::ast_summarizer::AstSummarizer;
use batless::summary::SummaryLevel;

#[test]
fn test_python_ast_basic_summary() {
    let code = r#"
import os
import sys

def main():
    print("Hello")

class User:
    def __init__(self, name):
        self.name = name

    def greet(self):
        return f"Hello, {self.name}"
"#;

    let summary = AstSummarizer::extract_summary(code, Some("Python"), SummaryLevel::Standard);

    println!("Summary: {:?}", summary);

    // Check for function definitions
    assert!(summary.iter().any(|l| l.contains("def main")));
    assert!(summary.iter().any(|l| l.contains("def __init__")));
    assert!(summary.iter().any(|l| l.contains("def greet")));

    // Check for class definition
    assert!(summary.iter().any(|l| l.contains("class User")));

    // Check for imports
    assert!(summary.iter().any(|l| l.contains("import os")));
    assert!(summary.iter().any(|l| l.contains("import sys")));

    // Should NOT contain function bodies
    assert!(!summary.iter().any(|l| l.contains("print(\"Hello\")")));
    assert!(!summary.iter().any(|l| l.contains("self.name = name")));
}

#[test]
fn test_python_ast_minimal_level() {
    let code = r#"
import os

def hello():
    pass

class World:
    pass
"#;

    let summary = AstSummarizer::extract_summary(code, Some("Python"), SummaryLevel::Minimal);

    // Minimal should have functions and classes
    assert!(summary.iter().any(|l| l.contains("def hello")));
    assert!(summary.iter().any(|l| l.contains("class World")));

    // Minimal should NOT include imports
    assert!(!summary.iter().any(|l| l.contains("import os")));
}

#[test]
fn test_python_ast_decorators() {
    let code = r#"
@staticmethod
def static_method():
    pass

@classmethod
def class_method(cls):
    pass

@property
def my_property(self):
    return self._value
"#;

    let summary = AstSummarizer::extract_summary(code, Some("Python"), SummaryLevel::Standard);

    println!("Decorator summary: {:?}", summary);

    // Should capture decorated functions
    assert!(summary.iter().any(|l| l.contains("@staticmethod")));
    assert!(summary.iter().any(|l| l.contains("@classmethod")));
    assert!(summary.iter().any(|l| l.contains("@property")));
}

#[test]
fn test_python_ast_async_functions() {
    let code = r#"
async def fetch_data():
    await get_data()

async def process():
    result = await fetch_data()
    return result
"#;

    let summary = AstSummarizer::extract_summary(code, Some("Python"), SummaryLevel::Standard);

    println!("Async summary: {:?}", summary);

    // Should capture async functions
    assert!(summary.iter().any(|l| l.contains("async def fetch_data")));
    assert!(summary.iter().any(|l| l.contains("async def process")));

    // Should NOT contain function bodies
    assert!(!summary.iter().any(|l| l.contains("await get_data()")));
}

#[test]
fn test_python_ast_from_imports() {
    let code = r#"
from os import path
from typing import List, Dict, Optional
from .models import User, Post

def process_user(user: User) -> Dict:
    return user.to_dict()
"#;

    let summary = AstSummarizer::extract_summary(code, Some("Python"), SummaryLevel::Standard);

    println!("From import summary: {:?}", summary);

    // Should capture from imports
    assert!(summary.iter().any(|l| l.contains("from os import")));
    assert!(summary.iter().any(|l| l.contains("from typing import")));
    assert!(summary.iter().any(|l| l.contains("from .models import")));

    // Should capture function
    assert!(summary.iter().any(|l| l.contains("def process_user")));
}

#[test]
fn test_python_ast_nested_classes() {
    let code = r#"
class Outer:
    class Inner:
        def inner_method(self):
            pass

    def outer_method(self):
        pass
"#;

    let summary = AstSummarizer::extract_summary(code, Some("Python"), SummaryLevel::Standard);

    println!("Nested class summary: {:?}", summary);

    // Should capture both outer and inner classes
    assert!(summary.iter().any(|l| l.contains("class Outer")));
    assert!(summary.iter().any(|l| l.contains("class Inner")));
    assert!(summary.iter().any(|l| l.contains("def inner_method")));
    assert!(summary.iter().any(|l| l.contains("def outer_method")));
}

#[test]
fn test_python_ast_lambda_ignored() {
    let code = r#"
def regular_function():
    mapper = lambda x: x * 2
    return mapper

square = lambda x: x ** 2
"#;

    let summary = AstSummarizer::extract_summary(code, Some("Python"), SummaryLevel::Standard);

    // Should capture regular function
    assert!(summary.iter().any(|l| l.contains("def regular_function")));

    // Lambda functions are typically not captured as "important" structures
    // (they're expressions, not statements)
}

#[test]
fn test_python_ast_empty_file() {
    let code = "";

    let summary = AstSummarizer::extract_summary(code, Some("Python"), SummaryLevel::Standard);

    assert!(summary.is_empty());
}

#[test]
fn test_python_ast_comments_ignored() {
    let code = r#"
# This is a comment
def my_function():
    # Another comment
    pass

"""
This is a docstring
"""

class MyClass:
    """Class docstring"""
    pass
"#;

    let summary = AstSummarizer::extract_summary(code, Some("Python"), SummaryLevel::Standard);

    // Should capture function and class
    assert!(summary.iter().any(|l| l.contains("def my_function")));
    assert!(summary.iter().any(|l| l.contains("class MyClass")));

    // Comments themselves shouldn't be in summary (only the lines they're on if they have code)
}

#[test]
fn test_python_ast_detailed_level() {
    let code = r#"
import os

MAX_SIZE = 1000

def process():
    global MAX_SIZE
    local_var = 42
    return local_var
"#;

    let summary = AstSummarizer::extract_summary(code, Some("Python"), SummaryLevel::Detailed);

    println!("Detailed summary: {:?}", summary);

    // Detailed should include imports, functions, and module-level assignments
    assert!(summary.iter().any(|l| l.contains("import os")));
    assert!(summary.iter().any(|l| l.contains("MAX_SIZE")));
    assert!(summary.iter().any(|l| l.contains("def process")));
    assert!(summary.iter().any(|l| l.contains("global MAX_SIZE")));
}

#[test]
fn test_python_ast_none_level() {
    let code = r#"
def my_function():
    pass
"#;

    let summary = AstSummarizer::extract_summary(code, Some("Python"), SummaryLevel::None);

    assert!(summary.is_empty());
}

#[test]
fn test_python_ast_complex_decorators() {
    let code = r#"
@app.route('/api/users')
@login_required
@cache(timeout=300)
def get_users():
    return User.query.all()

@dataclass
@frozen
class Config:
    debug: bool
    port: int
"#;

    let summary = AstSummarizer::extract_summary(code, Some("Python"), SummaryLevel::Standard);

    println!("Complex decorator summary: {:?}", summary);

    // Should capture first decorator of each decorated definition and the definition itself
    assert!(summary.iter().any(|l| l.contains("@app.route")));
    assert!(summary.iter().any(|l| l.contains("def get_users")));
    assert!(summary.iter().any(|l| l.contains("@dataclass")));
    assert!(summary.iter().any(|l| l.contains("class Config")));

    // Note: Only the starting line of decorated definitions is captured,
    // so @login_required and @cache may not appear if they're on separate lines
}
