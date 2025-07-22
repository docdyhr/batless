# Homebrew Tap Setup for batless

This directory contains the setup files for creating a Homebrew tap repository.

## Setup Instructions

### 1. Create the Tap Repository

Create a new GitHub repository named `homebrew-batless` under your GitHub account:
- Repository name: `homebrew-batless`
- Description: "Homebrew tap for batless - non-blocking code viewer"
- Make it public
- Initialize with README

### 2. Repository Structure

Your `homebrew-batless` repository should have this structure:
```
homebrew-batless/
├── README.md
├── Formula/
│   └── batless.rb
└── .github/
    └── workflows/
        └── test.yml
```

### 3. Copy Files

Copy the following files from this directory to your `homebrew-batless` repository:
- `Formula/batless.rb` → `Formula/batless.rb`
- `tap-README.md` → `README.md` (replace the existing README)
- `.github/workflows/test.yml` → `.github/workflows/test.yml`

### 4. Update URLs

In the copied files, ensure all URLs point to your actual repository:
- Replace `docdyhr/batless` with your actual GitHub username if different
- Update the download URL in the formula to point to your releases

### 5. Test the Tap

Once set up, users can install with:
```bash
# Add the tap
brew tap docdyhr/batless

# Install batless
brew install batless

# Or install directly
brew install docdyhr/batless
```

## Automation

The main `batless` repository's release workflow will automatically:
1. Calculate new formula hash and URL
2. Create a PR in the tap repository with updated formula
3. The tap's test workflow will validate the formula

## Maintenance

- The formula will be automatically updated on each release
- Monitor the tap repository for issues or user feedback
- Ensure the formula stays compatible with Homebrew guidelines