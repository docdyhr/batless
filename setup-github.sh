#!/bin/bash
# GitHub Repository Setup Script for batless
# This script helps you create the GitHub repository and push the initial commit

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -f "README.md" ]; then
    print_error "This script must be run from the batless project root directory"
    exit 1
fi

print_status "🦇 Setting up batless GitHub repository..."
echo

# Check if git is initialized
if [ ! -d ".git" ]; then
    print_status "Initializing Git repository..."
    git init
    print_success "Git repository initialized"
fi

# Check if files are already committed
if ! git rev-parse --verify HEAD >/dev/null 2>&1; then
    print_status "Staging all files..."
    git add .

    print_status "Creating initial commit..."
    git commit -m "🦇 Initial release: batless v0.1.0

- Complete syntax highlighting for 100+ languages
- Multiple output modes (plain, highlight, JSON)
- AI-friendly non-blocking design
- Smart file limiting (lines and bytes)
- Memory-efficient streaming
- Comprehensive test suite (26 tests)
- GitHub best practices implementation
- CI/CD pipeline with multi-platform builds
- Ready for crates.io publication

Perfect for AI assistants, CI/CD pipelines, and modern CLI workflows."

    print_success "Initial commit created"
else
    print_warning "Repository already has commits, skipping initial commit"
fi

# Rename branch to main
print_status "Setting default branch to 'main'..."
git branch -M main
print_success "Default branch set to 'main'"

# Check GitHub CLI availability
if command -v gh &>/dev/null; then
    print_status "GitHub CLI found, attempting to create repository..."

    # Try to create the repository
    if gh repo create batless --public --description "🦇 A minimal, blazing-fast syntax viewer for AI code assistants and modern CLI workflows" --clone=false; then
        print_success "GitHub repository created successfully!"

        # Add remote and push
        print_status "Adding remote origin..."
        if ! git remote get-url origin &>/dev/null; then
            git remote add origin https://github.com/$(gh api user --jq .login)/batless.git
        fi

        print_status "Pushing to GitHub..."
        git push -u origin main
        print_success "Repository pushed to GitHub!"

        # Set up repository topics
        print_status "Setting up repository topics..."
        gh repo edit --add-topic rust,cli,syntax-highlighting,ai,automation,code-viewer,syntax-highlighter,bat-alternative
        print_success "Repository topics added"

        echo
        print_success "✨ Repository successfully created and pushed!"
        echo -e "🔗 Repository URL: ${GREEN}https://github.com/$(gh api user --jq .login)/batless${NC}"

    else
        print_error "Failed to create repository with GitHub CLI"
        print_manual_instructions
    fi
else
    print_warning "GitHub CLI not found, providing manual instructions"
    print_manual_instructions
fi

print_manual_instructions() {
    echo
    print_status "📋 Manual Setup Instructions:"
    echo
    echo "1. Create a new repository on GitHub:"
    echo "   • Go to: https://github.com/new"
    echo "   • Repository name: batless"
    echo "   • Description: 🦇 A minimal, blazing-fast syntax viewer for AI code assistants and modern CLI workflows"
    echo "   • Visibility: Public"
    echo "   • Initialize: ❌ Don't check any initialization options"
    echo "   • Click 'Create repository'"
    echo
    echo "2. Add remote and push (replace YOUR_USERNAME with your GitHub username):"
    echo "   git remote add origin https://github.com/YOUR_USERNAME/batless.git"
    echo "   git push -u origin main"
    echo
    echo "3. Optional: Add repository topics in GitHub web interface:"
    echo "   rust, cli, syntax-highlighting, ai, automation, code-viewer, syntax-highlighter, bat-alternative"
    echo
    echo "4. GitHub Actions will automatically run on your first push!"
    echo
}

# Final checks and recommendations
echo
print_status "🔧 Post-setup recommendations:"
echo
echo "• GitHub Actions CI/CD pipeline is configured and ready"
echo "• Consider enabling GitHub Discussions for community engagement"
echo "• Add a GitHub personal access token with 'repo' scope for automated releases"
echo "• Update CONTRIBUTING.md with your preferred contact methods"
echo "• Update SECURITY.md with your security contact information"
echo
print_success "Setup complete! Your batless project is ready for the world! 🎉"
