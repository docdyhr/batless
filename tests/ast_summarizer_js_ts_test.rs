use batless::ast_summarizer::AstSummarizer;
use batless::summary::SummaryLevel;

// ========== JavaScript Tests ==========

#[test]
fn test_javascript_basic_summary() {
    let code = r#"
import React from 'react';
import { useState } from 'react';

function App() {
    const [count, setCount] = useState(0);
    return <div>{count}</div>;
}

class Counter {
    constructor() {
        this.count = 0;
    }

    increment() {
        this.count++;
    }
}

export default App;
"#;

    let summary = AstSummarizer::extract_summary(code, Some("JavaScript"), SummaryLevel::Standard);

    println!("JS Summary: {:?}", summary);

    // Should capture imports
    assert!(summary.iter().any(|l| l.contains("import React")));
    assert!(summary.iter().any(|l| l.contains("import { useState }")));

    // Should capture function
    assert!(summary.iter().any(|l| l.contains("function App")));

    // Should capture class and methods
    assert!(summary.iter().any(|l| l.contains("class Counter")));
    assert!(summary.iter().any(|l| l.contains("increment()")));

    // Should capture export
    assert!(summary.iter().any(|l| l.contains("export default")));
}

#[test]
fn test_javascript_arrow_functions() {
    let code = r#"
const add = (a, b) => a + b;

const multiply = (x, y) => {
    return x * y;
};

const Component = () => {
    return <div>Hello</div>;
};
"#;

    let summary = AstSummarizer::extract_summary(code, Some("JavaScript"), SummaryLevel::Standard);

    println!("Arrow function summary: {:?}", summary);

    // Arrow functions should be captured
    assert!(summary.iter().any(|l| l.contains("=>")));
}

#[test]
fn test_javascript_es6_classes() {
    let code = r#"
class Animal {
    constructor(name) {
        this.name = name;
    }

    speak() {
        console.log(`${this.name} makes a sound`);
    }
}

class Dog extends Animal {
    speak() {
        console.log(`${this.name} barks`);
    }
}
"#;

    let summary = AstSummarizer::extract_summary(code, Some("JavaScript"), SummaryLevel::Standard);

    assert!(summary.iter().any(|l| l.contains("class Animal")));
    assert!(summary.iter().any(|l| l.contains("class Dog")));
    assert!(summary.iter().any(|l| l.contains("speak()")));
}

#[test]
fn test_javascript_async_await() {
    let code = r#"
async function fetchData() {
    const response = await fetch('/api/data');
    return response.json();
}

const processData = async () => {
    const data = await fetchData();
    return data;
};
"#;

    let summary = AstSummarizer::extract_summary(code, Some("JavaScript"), SummaryLevel::Standard);

    println!("Async JS summary: {:?}", summary);

    assert!(summary
        .iter()
        .any(|l| l.contains("async function fetchData")));
    assert!(summary.iter().any(|l| l.contains("=>")));
}

#[test]
fn test_javascript_exports() {
    let code = r#"
export function helper() {}
export const API_KEY = 'secret'; // pragma: allowlist secret
export default class Main {}
export { utils };
"#;

    let summary = AstSummarizer::extract_summary(code, Some("JavaScript"), SummaryLevel::Standard);

    // Should capture exports
    assert!(summary.iter().any(|l| l.contains("export")));
}

// ========== TypeScript Tests ==========

#[test]
fn test_typescript_basic_summary() {
    let code = r#"
import { Component } from 'react';

interface User {
    name: string;
    age: number;
}

type UserId = string | number;

class UserService {
    private users: User[] = [];

    addUser(user: User): void {
        this.users.push(user);
    }

    getUser(id: UserId): User | undefined {
        return this.users.find(u => u.name === id);
    }
}

function processUser(user: User): string {
    return `${user.name} is ${user.age} years old`;
}

export { UserService, processUser };
"#;

    let summary = AstSummarizer::extract_summary(code, Some("TypeScript"), SummaryLevel::Standard);

    println!("TS Summary: {:?}", summary);

    // Should capture imports
    assert!(summary.iter().any(|l| l.contains("import")));

    // Should capture interface
    assert!(summary.iter().any(|l| l.contains("interface User")));

    // Should capture type alias
    assert!(summary.iter().any(|l| l.contains("type UserId")));

    // Should capture class and methods
    assert!(summary.iter().any(|l| l.contains("class UserService")));
    assert!(summary.iter().any(|l| l.contains("addUser")));
    assert!(summary.iter().any(|l| l.contains("getUser")));

    // Should capture function
    assert!(summary.iter().any(|l| l.contains("function processUser")));

    // Should capture export
    assert!(summary.iter().any(|l| l.contains("export")));
}

#[test]
fn test_typescript_interfaces() {
    let code = r#"
interface Animal {
    name: string;
    age: number;
    speak(): void;
}

interface Dog extends Animal {
    breed: string;
    bark(): void;
}

interface Cat extends Animal {
    indoor: boolean;
    meow(): void;
}
"#;

    let summary = AstSummarizer::extract_summary(code, Some("TypeScript"), SummaryLevel::Standard);

    assert!(summary.iter().any(|l| l.contains("interface Animal")));
    assert!(summary.iter().any(|l| l.contains("interface Dog")));
    assert!(summary.iter().any(|l| l.contains("interface Cat")));
}

#[test]
fn test_typescript_enums() {
    let code = r#"
enum Color {
    Red,
    Green,
    Blue
}

enum Status {
    Pending = 'PENDING',
    Success = 'SUCCESS',
    Error = 'ERROR'
}

function getColor(): Color {
    return Color.Red;
}
"#;

    let summary = AstSummarizer::extract_summary(code, Some("TypeScript"), SummaryLevel::Detailed);

    println!("Enum summary: {:?}", summary);

    assert!(summary.iter().any(|l| l.contains("enum Color")));
    assert!(summary.iter().any(|l| l.contains("enum Status")));
    assert!(summary.iter().any(|l| l.contains("function getColor")));
}

#[test]
fn test_typescript_generics() {
    let code = r#"
interface Box<T> {
    value: T;
}

class Container<T> {
    private items: T[] = [];

    add(item: T): void {
        this.items.push(item);
    }

    get(index: number): T | undefined {
        return this.items[index];
    }
}

function identity<T>(arg: T): T {
    return arg;
}
"#;

    let summary = AstSummarizer::extract_summary(code, Some("TypeScript"), SummaryLevel::Standard);

    assert!(summary.iter().any(|l| l.contains("interface Box")));
    assert!(summary.iter().any(|l| l.contains("class Container")));
    assert!(summary.iter().any(|l| l.contains("add")));
    assert!(summary.iter().any(|l| l.contains("function identity")));
}

#[test]
fn test_typescript_decorators() {
    let code = r#"
function Component(target: any) {
    // decorator logic
}

@Component
class MyComponent {
    @observable
    private value: string = '';

    @computed
    get displayValue(): string {
        return this.value.toUpperCase();
    }

    @action
    setValue(newValue: string): void {
        this.value = newValue;
    }
}
"#;

    let summary = AstSummarizer::extract_summary(code, Some("TypeScript"), SummaryLevel::Standard);

    println!("Decorator summary: {:?}", summary);

    assert!(summary.iter().any(|l| l.contains("function Component")));
    assert!(summary.iter().any(|l| l.contains("class MyComponent")));
}

#[test]
fn test_typescript_react_component() {
    let code = r#"
import React, { FC, useState } from 'react';

interface Props {
    title: string;
    count?: number;
}

const Counter: FC<Props> = ({ title, count = 0 }) => {
    const [value, setValue] = useState(count);

    return (
        <div>
            <h1>{title}</h1>
            <p>Count: {value}</p>
        </div>
    );
};

export default Counter;
"#;

    let summary = AstSummarizer::extract_summary(code, Some("TypeScript"), SummaryLevel::Standard);

    assert!(summary.iter().any(|l| l.contains("import React")));
    assert!(summary.iter().any(|l| l.contains("interface Props")));
    assert!(summary.iter().any(|l| l.contains("=>")));
    assert!(summary.iter().any(|l| l.contains("export default")));
}

#[test]
fn test_javascript_minimal_level() {
    let code = r#"
import utils from './utils';

function main() {}
class App {}
const handler = () => {};
"#;

    let summary = AstSummarizer::extract_summary(code, Some("JavaScript"), SummaryLevel::Minimal);

    // Minimal should have functions, classes, and arrow functions
    assert!(summary.iter().any(|l| l.contains("function main")));
    assert!(summary.iter().any(|l| l.contains("class App")));
    assert!(summary.iter().any(|l| l.contains("=>")));

    // Minimal should NOT include imports
    assert!(!summary.iter().any(|l| l.contains("import")));
}

#[test]
fn test_typescript_minimal_level() {
    let code = r#"
import { User } from './types';

interface Config {}
class Service {}
function process() {}
"#;

    let summary = AstSummarizer::extract_summary(code, Some("TypeScript"), SummaryLevel::Minimal);

    // Minimal should have interfaces, classes, functions
    assert!(summary.iter().any(|l| l.contains("interface Config")));
    assert!(summary.iter().any(|l| l.contains("class Service")));
    assert!(summary.iter().any(|l| l.contains("function process")));

    // Minimal should NOT include imports
    assert!(!summary.iter().any(|l| l.contains("import")));
}

#[test]
fn test_javascript_empty_file() {
    let code = "";
    let summary = AstSummarizer::extract_summary(code, Some("JavaScript"), SummaryLevel::Standard);
    assert!(summary.is_empty());
}

#[test]
fn test_typescript_empty_file() {
    let code = "";
    let summary = AstSummarizer::extract_summary(code, Some("TypeScript"), SummaryLevel::Standard);
    assert!(summary.is_empty());
}
