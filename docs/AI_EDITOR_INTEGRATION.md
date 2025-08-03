# 🤖 AI Assistant & Editor Integration Guide

Complete guide for optimizing `batless` with AI assistants in modern editors like Zed and VS Code.

## 🎯 Quick Setup (TL;DR)

```bash
# Best practice: Use GH_PAGER for GitHub CLI
export GH_PAGER="batless --plain --max-lines=500 --summary-level=standard"

# Add to ~/.zshrc for permanent setup
echo 'export GH_PAGER="batless --plain --max-lines=500 --summary-level=standard"' >> ~/.zshrc
```

## 📚 Table of Contents

- [Quick Setup](#-quick-setup-tldr)
- [Why Not Aliases?](#-why-not-aliases)
- [Environment Variables](#-environment-variables)
- [AI-Optimized Profiles & CLI Tools](#-ai-optimized-profiles--cli-tools)
- [CLI AI Coding Assistant Integrations](#%EF%B8%8F-cli-ai-coding-assistant-integrations)
- [Web-Based AI Assistant Integrations](#-web-based-ai-assistant-integrations)
- [Universal AI Helper Functions](#-universal-ai-helper-functions)
- [Editor Integration](#-editor-integration)
  - [Zed Editor](#zed-editor)
  - [VS Code](#vs-code)
- [Advanced Workflows](#-advanced-workflows)
- [GitHub CLI Integration](#-github-cli-integration)
- [Troubleshooting](#-troubleshooting)

## 🚫 Why Not Aliases?

**Don't use simple aliases** like `alias cat=batless` - use these superior approaches instead:

### ❌ Avoid: Simple Aliases

```bash
# Problems: breaks compatibility, too broad, conflicts with other tools
alias cat="batless --plain"
alias less="batless --plain"
```

### ✅ Recommended: Smart Environment Variables

```bash
# Targeted, compatible, and AI-optimized
export PAGER="batless --plain"
export GH_PAGER="batless --plain --max-lines=500 --summary-level=standard"
```

### ✅ Advanced: Smart Functions

```bash
# Adaptive behavior based on context
ai_view() {
    if [[ -t 1 ]]; then
        # Interactive terminal - use syntax highlighting
        batless --profile=claude "$@"
    else
        # Piped to AI tool - use JSON output
        batless --profile=copilot "$@"
    fi
}
```

## 🌍 Environment Variables

### **GH_PAGER (Recommended for GitHub CLI)**

```bash
# Basic setup
export GH_PAGER="batless --plain --max-lines=500"

# AI-optimized setup
export GH_PAGER="batless --plain --max-lines=500 --summary-level=standard"

# Advanced with context fitting
export GH_PAGER="batless --profile=claude --max-lines=300 --fit-context"
```

### **PAGER (General Purpose)**

```bash
# Conservative approach
export PAGER="batless --plain"

# AI-friendly approach
export PAGER="batless --plain --max-lines=300 --color=never"
```

### **Multiple Configuration Setup**

```bash
# Add to ~/.zshrc
export GH_PAGER_DEFAULT="batless --plain --max-lines=500"
export GH_PAGER_AI="batless --profile=claude --max-lines=300 --fit-context"
export GH_PAGER_JSON="batless --mode=json --validate-json --max-lines=200"

# Set default
export GH_PAGER="$GH_PAGER_DEFAULT"

# Quick switchers
alias gh-ai="export GH_PAGER=\"$GH_PAGER_AI\""
alias gh-json="export GH_PAGER=\"$GH_PAGER_JSON\""
alias gh-default="export GH_PAGER=\"$GH_PAGER_DEFAULT\""
```

## 🎨 AI-Optimized Profiles & CLI Tools

### **Claude (Anthropic) - Best Overall**

```bash
# Use built-in profile
batless --profile=claude file.rs

# Equivalent manual settings:
batless --mode=summary --max-lines=4000 --summary-level=standard file.rs

# With context optimization:
batless --profile=claude --fit-context --ai-model=claude35-sonnet file.rs
```

### **Claude Code (claude.ai/code) - Web Interface**

```bash
# Optimized for Claude Code web interface
batless --mode=summary --max-lines=3000 --summary-level=detailed file.rs

# Copy-paste ready format
batless --color=never --mode=summary --max-lines=2500 file.rs

# Multi-file context for Claude Code
for file in src/*.rs; do
  echo "=== File: $file ==="
  batless --mode=summary --max-lines=200 "$file"
  echo ""
done
```

### **Google Gemini - Advanced Reasoning**

```bash
# Gemini-optimized context (1M token context window)
batless --fit-context --ai-model=generic --prompt-tokens=1000 --max-lines=8000 file.rs

# Structured output for Gemini
batless --mode=json --include-tokens --summary-level=detailed file.rs

# Large codebase analysis for Gemini
batless --streaming-json --streaming-chunk-size=2000 --max-lines=10000 large-project/
```

### **GitHub Copilot - Code Completion**

```bash
# Use built-in profile
batless --profile=copilot file.rs

# Equivalent manual settings:
batless --mode=json --include-tokens --max-lines=2000 file.rs

# Enhanced for Copilot Chat
batless --mode=json --include-tokens --summary-level=standard file.rs
```

### **ChatGPT/OpenAI - General Purpose**

```bash
# Use built-in profile
batless --profile=chatgpt file.rs

# With model-specific optimization:
batless --profile=chatgpt --ai-model=gpt4-turbo --fit-context file.rs

# For o1 models (reasoning-focused)
batless --mode=summary --summary-level=detailed --max-lines=4000 file.rs
```

## 🛠️ CLI AI Coding Assistant Integrations

### **Aider - AI Pair Programming**

```bash
# Perfect for aider's file analysis
export AIDER_PAGER="batless --mode=summary --max-lines=500 --color=never"

# Aider-optimized viewing function
aider_view() {
    batless --mode=summary --max-lines=800 --summary-level=standard --color=never "$@"
}

# Multi-file context for aider
aider_context() {
    echo "=== Aider Context Generation ==="
    for file in "$@"; do
        echo "File: $file"
        batless --mode=summary --max-lines=300 --color=never "$file"
        echo "---"
    done
}

# Usage: aider_context src/*.py
```

### **Cursor - AI Code Editor**

```bash
# Cursor-optimized settings
cursor_view() {
    batless --mode=json --include-tokens --summary-level=standard \
           --max-lines=1500 --ai-model=gpt4-turbo "$@"
}

# For Cursor's composer feature
cursor_composer() {
    batless --mode=summary --summary-level=detailed \
           --max-lines=2000 --color=never "$@"
}

# Cursor Tab integration
export CURSOR_PAGER="batless --mode=json --include-tokens --max-lines=1000"
```

### **Continue.dev - VS Code Extension**

```bash
# Continue.dev optimized profile
continue_view() {
    batless --mode=json --include-tokens --summary-level=standard \
           --fit-context --ai-model=claude35-sonnet "$@"
}

# For Continue's slash commands
continue_context() {
    batless --mode=summary --max-lines=1000 --summary-level=detailed \
           --color=never "$@" | head -100
}

# Environment setup for Continue.dev
export CONTINUE_PAGER="batless --mode=json --max-lines=800 --include-tokens"
```

### **Cline (Claude in VS Code)**

```bash
# Cline-specific optimization
cline_view() {
    batless --profile=claude --fit-context --ai-model=claude35-sonnet \
           --max-lines=2500 "$@"
}

# For Cline's file exploration
cline_explore() {
    find "$1" -type f -name "*.${2:-*}" | head -10 | while read file; do
        echo "=== $file ==="
        batless --mode=summary --max-lines=150 --summary-level=minimal "$file"
    done
}

# Usage: cline_explore src/ py
```

### **Windsurf Editor**

```bash
# Windsurf AI integration
windsurf_view() {
    batless --mode=json --include-tokens --summary-level=standard \
           --max-lines=2000 --ai-model=claude35-sonnet "$@"
}

# For Windsurf's cascade feature
windsurf_cascade() {
    batless --mode=summary --summary-level=detailed \
           --max-lines=1500 --color=never "$@"
}
```

### **Bolt.new / StackBlitz**

```bash
# Optimized for web-based AI coding
bolt_view() {
    batless --color=never --mode=summary --summary-level=standard \
           --max-lines=1000 "$@"
}

# Copy-paste friendly format
bolt_context() {
    echo "```"
    batless --color=never --mode=plain --max-lines=800 "$@"
    echo "```"
}
```

### **Replit Agent**

```bash
# Replit-optimized viewing
replit_view() {
    batless --mode=json --include-tokens --max-lines=1200 \
           --summary-level=standard "$@"
}

# For Replit's bounty analysis
replit_analyze() {
    batless --mode=summary --summary-level=detailed \
           --max-lines=800 --color=never "$@"
}
```

### **Codium AI (PR-Agent)**

```bash
# Perfect for PR analysis
codium_pr() {
    batless --mode=summary --summary-level=detailed \
           --max-lines=1000 --color=never "$@"
}

# For code review context
codium_review() {
    git diff --name-only | while read file; do
        echo "=== Changed: $file ==="
        batless --mode=summary --max-lines=200 "$file"
    done
}
```

### **Tabnine**

```bash
# Tabnine context optimization
tabnine_view() {
    batless --mode=json --include-tokens --max-lines=1500 \
           --ai-model=generic "$@"
}

# For Tabnine Chat
tabnine_chat() {
    batless --mode=summary --summary-level=standard \
           --max-lines=1000 --color=never "$@"
}
```

## � Web-Based AI Assistant Integrations

### **ChatGPT Web Interface**

```bash
# Optimized for ChatGPT web copy-paste
chatgpt_web() {
    echo "=== Code for ChatGPT Analysis ==="
    echo '```'
    batless --color=never --mode=plain --max-lines=2000 "$@"
    echo '```'
    echo ""
    echo "Summary:"
    batless --mode=summary --summary-level=standard --color=never --max-lines=500 "$@"
}

# For ChatGPT Code Interpreter
chatgpt_data() {
    batless --mode=json --include-tokens --max-lines=1500 "$@"
}
```

### **Claude Web Interface (claude.ai)**

```bash
# Perfect for Claude web conversations
claude_web() {
    echo "Here's the code for analysis:"
    echo ""
    echo '```'
    batless --color=never --mode=plain --max-lines=3000 "$@"
    echo '```'
    echo ""
    echo "Key structures:"
    batless --mode=summary --summary-level=detailed --color=never --max-lines=800 "$@"
}

# For Claude's artifact generation
claude_artifacts() {
    batless --mode=summary --summary-level=detailed \
           --max-lines=2500 --color=never "$@"
}
```

### **Google Gemini Web**

```bash
# Gemini web interface optimization
gemini_web() {
    echo "=== Code Analysis Request ==="
    batless --color=never --mode=plain --max-lines=5000 "$@"
    echo ""
    echo "=== Structure Summary ==="
    batless --mode=summary --summary-level=detailed --color=never "$@"
}

# For Gemini's large context capabilities
gemini_large() {
    batless --color=never --mode=plain --max-lines=10000 "$@"
}
```

### **Perplexity AI**

```bash
# Perplexity-optimized context
perplexity_context() {
    echo "Please analyze this code:"
    echo ""
    batless --color=never --mode=summary --summary-level=standard \
           --max-lines=1500 "$@"
}
```

### **Poe (Multiple Models)**

```bash
# Universal function for Poe platform
poe_context() {
    local model="${1:-claude}"
    local file="$2"

    case "$model" in
        "claude")
            batless --mode=summary --summary-level=detailed --color=never --max-lines=2500 "$file"
            ;;
        "gpt4")
            batless --mode=json --include-tokens --color=never --max-lines=2000 "$file"
            ;;
        "gemini")
            batless --color=never --mode=plain --max-lines=4000 "$file"
            ;;
        *)
            batless --mode=summary --color=never --max-lines=2000 "$file"
            ;;
    esac
}

# Usage: poe_context claude file.rs
```

### **Universal Web AI Function**

```bash
# One function for all web-based AI tools
web_ai_context() {
    local file="$1"
    local format="${2:-auto}"

    echo "=== AI Analysis Context ==="
    echo "File: $file"
    echo "Generated: $(date)"
    echo ""

    case "$format" in
        "raw"|"plain")
            echo '```'
            batless --color=never --mode=plain --max-lines=3000 "$file"
            echo '```'
            ;;
        "summary")
            batless --mode=summary --summary-level=detailed --color=never --max-lines=2000 "$file"
            ;;
        "json")
            batless --mode=json --include-tokens --max-lines=2000 "$file"
            ;;
        "auto"|*)
            echo "## Code:"
            echo '```'
            batless --color=never --mode=plain --max-lines=2000 "$file"
            echo '```'
            echo ""
            echo "## Structure:"
            batless --mode=summary --summary-level=standard --color=never --max-lines=800 "$file"
            ;;
    esac
}

# Usage examples:
# web_ai_context file.rs          # Auto format (code + summary)
# web_ai_context file.rs raw      # Raw code only
# web_ai_context file.rs summary  # Summary only
# web_ai_context file.rs json     # JSON format
```

## �🎯 Universal AI Helper Functions

### **Smart AI Context Builder**

```bash
# Universal function that adapts to different AI tools
ai_smart_context() {
    local tool="${1:-auto}"
    local file="$2"

    case "$tool" in
        "claude"|"claude-code")
            batless --profile=claude --fit-context --ai-model=claude35-sonnet "$file"
            ;;
        "gemini")
            batless --fit-context --ai-model=generic --max-lines=8000 "$file"
            ;;
        "copilot"|"cursor")
            batless --profile=copilot "$file"
            ;;
        "chatgpt"|"openai")
            batless --profile=chatgpt --ai-model=gpt4-turbo "$file"
            ;;
        "aider")
            batless --mode=summary --max-lines=800 --color=never "$file"
            ;;
        "continue"|"cline")
            batless --profile=claude --max-lines=2000 "$file"
            ;;
        "auto"|*)
            # Auto-detect based on environment or default to Claude
            if [[ -n "$CURSOR_EDITOR" ]]; then
                batless --profile=copilot "$file"
            elif [[ -n "$AIDER_SESSION" ]]; then
                batless --mode=summary --max-lines=800 --color=never "$file"
            else
                batless --profile=claude "$file"
            fi
            ;;
    esac
}

# Usage examples:
# ai_smart_context claude file.rs
# ai_smart_context gemini large-file.py
# ai_smart_context auto file.js  # Auto-detects environment
```

### **Multi-Tool Context Generator**

```bash
# Generate context for multiple AI tools at once
ai_multi_context() {
    local file="$1"
    echo "=== Multi-AI Context for: $file ==="

    echo "## Claude/Claude Code:"
    batless --profile=claude --max-lines=300 "$file"

    echo -e "\n## GitHub Copilot:"
    batless --profile=copilot --max-lines=200 "$file"

    echo -e "\n## Aider/CLI Tools:"
    batless --mode=summary --max-lines=200 --color=never "$file"

    echo -e "\n## Copy-Paste Format:"
    echo '```'
    batless --color=never --mode=plain --max-lines=150 "$file"
    echo '```'
}
```

### **AI Tool Detection**

```bash
# Detect which AI tools are available and set optimal defaults
setup_ai_environment() {
    echo "🔍 Detecting AI coding assistants..."

    # Check for various AI tools
    [[ -n "$(which aider 2>/dev/null)" ]] && echo "✅ Aider detected"
    [[ -n "$(which cursor 2>/dev/null)" ]] && echo "✅ Cursor detected"
    [[ -f ".continue/config.json" ]] && echo "✅ Continue.dev detected"
    [[ -n "$REPLIT_DB_URL" ]] && echo "✅ Replit environment detected"

    # Set up universal aliases
    alias ai-claude="ai_smart_context claude"
    alias ai-gemini="ai_smart_context gemini"
    alias ai-copilot="ai_smart_context copilot"
    alias ai-aider="ai_smart_context aider"
    alias ai-auto="ai_smart_context auto"

    echo "✅ AI environment configured! Use: ai-claude file.rs"
}
```

### **Custom AI Workflows**

```bash
# Smart context fitting for large files
batless --fit-context --ai-model=claude35-sonnet --prompt-tokens=800 large-file.py

# Token analysis for context planning
batless --count-tokens --ai-model=gpt4 --mode=json file.py

# Streaming for large codebases
batless --streaming-json --streaming-chunk-size=500 --enable-resume large-project/
```

## 🎯 Editor Integration

### **Zed Editor**

#### Configuration (`~/.config/zed/settings.json`)

```json
{
  "terminal": {
    "env": {
      "PAGER": "batless --plain",
      "GH_PAGER": "batless --profile=claude --max-lines=300"
    }
  },
  "assistant": {
    "default_model": "claude-3-5-sonnet",
    "custom_commands": {
      "analyze_file": "batless --profile=claude",
      "get_context": "batless --mode=json --include-tokens --summary-level=standard",
      "quick_view": "batless --max-lines=50 --mode=summary"
    }
  }
}
```

#### Zed Workflow

1. **File Analysis**: Use Cmd+Shift+P → "analyze_file" with batless
2. **Context Building**: Select code → use batless with `--profile=claude`
3. **AI Chat**: Pipe batless output directly to Zed's Claude integration

#### Zed Tasks (`.zed/tasks.json`)

```json
[
  {
    "label": "AI Context: Claude",
    "command": "batless",
    "args": ["--profile=claude", "$ZED_FILE"],
    "use_new_terminal": true
  },
  {
    "label": "AI Context: Quick Summary",
    "command": "batless",
    "args": ["--mode=summary", "--max-lines=100", "$ZED_FILE"]
  }
]
```

### **VS Code**

#### Configuration (`.vscode/settings.json`)

```json
{
  "terminal.integrated.env.osx": {
    "PAGER": "batless --plain",
    "GH_PAGER": "batless --profile=claude --max-lines=300"
  },
  "terminal.integrated.env.linux": {
    "PAGER": "batless --plain",
    "GH_PAGER": "batless --profile=claude --max-lines=300"
  },
  "github.copilot.advanced": {
    "authProvider": "github",
    "inlineSuggest.enable": true
  }
}
```

#### VS Code Tasks (`.vscode/tasks.json`)

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "AI Context: Claude",
      "type": "shell",
      "command": "batless",
      "args": ["--profile=claude", "${file}"],
      "group": "build",
      "presentation": {
        "echo": true,
        "reveal": "always",
        "panel": "new"
      }
    },
    {
      "label": "AI Context: Copilot",
      "type": "shell",
      "command": "batless",
      "args": ["--profile=copilot", "${file}"],
      "group": "build"
    },
    {
      "label": "AI Context: Token Count",
      "type": "shell",
      "command": "batless",
      "args": ["--count-tokens", "--ai-model=gpt4", "${file}"],
      "group": "build"
    }
  ]
}
```

#### VS Code Snippets

```json
{
  "Batless AI Context": {
    "prefix": "batless-ai",
    "body": [
      "batless --profile=${1|claude,copilot,chatgpt,assistant|} --max-lines=${2:300} ${3:file}"
    ],
    "description": "Generate AI context with batless"
  }
}
```

## 🚀 Advanced Workflows

### **1. Quick File Analysis**

```bash
# Get immediate AI-friendly overview
batless --mode=summary --max-lines=100 src/main.rs

# With line numbers for reference
batless --mode=summary --max-lines=100 -n src/main.rs
```

### **2. Large Codebase Exploration**

```bash
# Smart chunking for AI processing
find src/ -name "*.rs" | head -5 | xargs batless --profile=claude

# Streaming analysis
batless --streaming-json --max-lines=1000 --summary-level=minimal large-file.py
```

### **3. Error Investigation**

```bash
# Non-blocking error context (perfect for CI/CD)
batless --mode=summary --color=never --max-lines=50 failing-test.rs

# JSON output for automated processing
batless --mode=json --validate-json error-prone-file.js
```

### **4. AI Chat Preparation**

```bash
# Perfect Claude context
batless --profile=claude --fit-context --ai-model=claude35-sonnet complex-algorithm.py

# Multi-file context building
for file in src/*.rs; do
  echo "=== $file ==="
  batless --profile=claude --max-lines=200 "$file"
done
```

### **5. Context-Aware Functions**

```bash
# Add to ~/.zshrc
# Smart AI viewer that adapts to terminal vs pipe
ai_context() {
    local file="$1"
    if [[ -t 1 ]]; then
        # Interactive terminal - full output with highlighting
        batless --profile=claude --max-lines=400 "$file"
    else
        # Piped to AI - structured JSON
        batless --mode=json --include-tokens --summary-level=standard --max-lines=300 "$file"
    fi
}

# Model-specific context builders
claude_context() {
    batless --profile=claude --fit-context --ai-model=claude35-sonnet "$@"
}

copilot_context() {
    batless --profile=copilot --include-tokens "$@"
}

gpt_context() {
    batless --profile=chatgpt --ai-model=gpt4-turbo --fit-context "$@"
}
```

## 🐙 GitHub CLI Integration

### **Basic Setup**

```bash
# Essential setup
export GH_PAGER="batless --plain --max-lines=500"

# AI-optimized setup
export GH_PAGER="batless --plain --max-lines=500 --summary-level=standard"
```

### **Advanced GH_PAGER Configurations**

#### **For Different Use Cases**

```bash
# PR Reviews
export GH_PAGER_PR="batless --plain --max-lines=500 --summary-level=standard"

# Large Repository Exploration
export GH_PAGER_REPO="batless --plain --max-lines=200 --fit-context --ai-model=claude35-sonnet"

# JSON API Responses
export GH_PAGER_API="batless --mode=json --validate-json --max-lines=300"

# Quick File Previews
export GH_PAGER_FILES="batless --plain --max-lines=100 -n"
```

#### **Smart GH_PAGER Switcher**

```bash
# Add to ~/.zshrc
gh_smart_pager() {
    case "${1:-default}" in
        "pr"|"pull")
            export GH_PAGER="batless --plain --max-lines=500 --summary-level=standard"
            ;;
        "api"|"json")
            export GH_PAGER="batless --mode=json --validate-json --max-lines=200"
            ;;
        "ai"|"claude")
            export GH_PAGER="batless --profile=claude --max-lines=300 --fit-context"
            ;;
        "files"|"file")
            export GH_PAGER="batless --plain -n --max-lines=200"
            ;;
        *)
            export GH_PAGER="batless --plain --max-lines=500"
            ;;
    esac
    echo "GH_PAGER set to: $GH_PAGER"
}

# Usage examples:
# gh_smart_pager pr    # Optimize for PR viewing
# gh_smart_pager ai    # Optimize for AI analysis
# gh_smart_pager api   # Optimize for JSON responses
```

### **GitHub CLI Aliases with Batless**

```bash
# Add to ~/.zshrc
alias ghpr="GH_PAGER='batless --profile=claude --max-lines=300' gh pr view"
alias ghfile="GH_PAGER='batless --plain -n --max-lines=200' gh api"
alias ghdiff="GH_PAGER='batless --plain --max-lines=400' gh pr diff"
alias ghissue="GH_PAGER='batless --mode=summary --max-lines=150' gh issue view"
alias ghrepo="GH_PAGER='batless --profile=claude --max-lines=200' gh repo view"

# AI-specific aliases
alias ghpr-ai="GH_PAGER='batless --profile=claude --fit-context' gh pr view"
alias ghapi-json="GH_PAGER='batless --mode=json --validate-json' gh api"
```

### **Real-World Usage Examples**

```bash
# Your current setup works great for:
gh pr view 42                    # View PR with 500-line limit
gh issue list                    # List issues with clean output
gh repo view owner/repo          # Repository overview

# Enhanced with AI features:
GH_PAGER="batless --profile=claude --max-lines=300" gh pr diff 42
GH_PAGER="batless --mode=json --include-tokens" gh api repos/owner/repo/contents/file.rs

# Context-aware usage:
gh_smart_pager ai && gh pr view 42    # Switch to AI mode, then view PR
gh_smart_pager json && gh api repos/owner/repo/releases/latest
```

## 🎨 Theme & Display Optimization

### **For AI Assistants (Plain Text)**

```bash
# Clean output for AI processing
batless --mode=plain --color=never file.rs

# Or use cat replacement mode
batless --plain file.rs
```

### **For Human Review**

```bash
# Beautiful syntax highlighting
batless --theme="InspiredGitHub" --mode=highlight file.rs

# Dark mode friendly
batless --theme="base16-ocean.dark" file.rs
```

### **Context-Specific Themes**

```bash
# AI processing - no colors
export BATLESS_AI_FLAGS="--color=never --plain"

# Human review - nice themes
export BATLESS_HUMAN_FLAGS="--theme=InspiredGitHub"

# Function to switch contexts
batless_mode() {
    case "$1" in
        "ai")
            batless $BATLESS_AI_FLAGS "${@:2}"
            ;;
        "human")
            batless $BATLESS_HUMAN_FLAGS "${@:2}"
            ;;
        *)
            batless "$@"
            ;;
    esac
}
```

## 🔍 Troubleshooting

### **Common Issues**

#### **1. Environment Variables Not Working**

```bash
# Check current settings
echo $GH_PAGER
echo $PAGER

# Reload shell configuration
source ~/.zshrc

# Test with temporary override
GH_PAGER="batless --plain --max-lines=10" gh pr view 42
```

#### **2. Colors in JSON Mode**

```bash
# Force plain output for AI processing
batless --color=never --mode=json file.rs

# Or use explicit plain mode
batless --plain file.rs
```

#### **3. Memory Issues with Large Files**

```bash
# Limit memory usage
batless --max-bytes=1048576 --streaming-json huge-file.log

# Quick peek without full processing
batless --max-lines=20 --mode=plain large-dataset.csv
```

#### **4. Performance Issues**

```bash
# Debug mode for performance analysis
batless --debug --profile=claude problematic-file.py

# Minimal processing for speed
batless --mode=plain --max-lines=50 file.rs
```

### **Debugging Commands**

```bash
# Check batless version and capabilities
batless --version
batless --help

# Test profiles
batless --profile=claude --max-lines=5 ~/.zshrc
batless --profile=copilot --max-lines=5 ~/.zshrc

# Validate configuration
batless --validate-json --mode=json ~/.zshrc
```

## 📊 Best Practices Summary

### **AI Assistant Optimizations**

| **AI Tool** | **Recommended Command** | **Why** |
|-------------|------------------------|---------|
| **Claude/Claude Code** | `batless --profile=claude` | Summary mode perfect for context understanding |
| **Google Gemini** | `batless --fit-context --max-lines=8000` | Leverages 1M token context window |
| **GitHub Copilot** | `batless --profile=copilot` | JSON+tokens for code completion |
| **ChatGPT/OpenAI** | `batless --profile=chatgpt --ai-model=gpt4-turbo` | Model-specific optimization |
| **Aider** | `batless --mode=summary --color=never --max-lines=800` | Clean output for pair programming |
| **Cursor** | `batless --mode=json --include-tokens --max-lines=1500` | Structured data for composer |
| **Continue.dev** | `batless --profile=claude --fit-context` | Leverages Claude integration |
| **Cline** | `batless --profile=claude --max-lines=2500` | Optimized for Claude in VS Code |
| **Windsurf** | `batless --mode=json --summary-level=standard` | Balanced output for cascade feature |

### **General Use Cases**

| **Use Case** | **Recommended Command** | **Why** |
|--------------|------------------------|---------|
| **Quick Analysis** | `batless --mode=summary --max-lines=100` | Fast overview |
| **Large Files** | `batless --streaming-json --fit-context` | Memory efficient |
| **Error Debugging** | `batless --mode=summary --color=never` | Clean, focused output |
| **Copy-Paste Ready** | `batless --color=never --max-lines=1000` | No ANSI codes |
| **Multi-File Context** | `ai_multi_context file.rs` | Multiple AI tool formats |
| **Auto-Detection** | `ai_smart_context auto file.rs` | Environment-aware optimization |
| **Cat Replacement** | `export PAGER="batless --plain"` | Perfect drop-in replacement |
| **GitHub CLI** | `export GH_PAGER="batless --plain --max-lines=500"` | Optimized for GitHub workflows |

## 🎯 Complete Setup Template

Copy this to your `~/.zshrc` for a complete setup:

```bash
# =============================================================================
# Batless AI Assistant & Editor Integration Setup
# =============================================================================

# Basic environment variables
export PAGER="batless --plain"
export GH_PAGER="batless --plain --max-lines=500 --summary-level=standard"

# Multiple GH_PAGER configurations
export GH_PAGER_DEFAULT="batless --plain --max-lines=500"
export GH_PAGER_AI="batless --profile=claude --max-lines=300 --fit-context"
export GH_PAGER_JSON="batless --mode=json --validate-json --max-lines=200"

# Smart context functions
ai_view() {
    if [[ -t 1 ]]; then
        batless --profile=claude "$@"
    else
        batless --profile=copilot "$@"
    fi
}

ai_context() {
    batless --mode=json --include-tokens --summary-level=standard \
           --max-lines=200 --ai-model=claude35-sonnet "$@"
}

# Model-specific helpers
claude_context() {
    batless --profile=claude --fit-context --ai-model=claude35-sonnet "$@"
}

copilot_context() {
    batless --profile=copilot --include-tokens "$@"
}

gpt_context() {
    batless --profile=chatgpt --ai-model=gpt4-turbo --fit-context "$@"
}

# GitHub CLI helpers
gh_smart_pager() {
    case "${1:-default}" in
        "ai"|"claude") export GH_PAGER="$GH_PAGER_AI" ;;
        "json"|"api") export GH_PAGER="$GH_PAGER_JSON" ;;
        *) export GH_PAGER="$GH_PAGER_DEFAULT" ;;
    esac
}

# Quick aliases
alias catn="batless --plain -n"
alias catb="batless --plain -b"
alias gh-ai="gh_smart_pager ai"
alias gh-json="gh_smart_pager json"
alias gh-default="gh_smart_pager default"

# GitHub CLI with batless
alias ghpr="GH_PAGER='batless --profile=claude --max-lines=300' gh pr view"
alias ghfile="GH_PAGER='batless --plain -n --max-lines=200' gh api"
alias ghdiff="GH_PAGER='batless --plain --max-lines=400' gh pr diff"

echo "✅ Batless AI integration loaded! Try: ai_view file.rs, ghpr 42, or gh-ai"
```

---

## 🔗 Related Documentation

- **[Main README](../README.md)** - Installation and basic usage
- **[Development Guide](../CLAUDE.md)** - Developer documentation
- **[Performance Guide](PERFORMANCE_BASELINE.md)** - Performance optimization
- **[Contributing Guide](../CONTRIBUTING.md)** - How to contribute

---

*This guide transforms `batless` into a powerful AI assistant companion that's far superior to simple cat/less aliases! 🚀*
