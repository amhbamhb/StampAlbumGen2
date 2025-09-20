# Automated Version Management System Documentation

## Overview

This document describes the comprehensive automated version management system implemented for the Stamp Album Generator project. The system provides zero-touch version management through conventional commits, supporting both GitHub and GitLab platforms.

## System Architecture

### Core Components

1. **Conventional Commit Standard** - Semantic versioning through commit message patterns
2. **Git Hooks** - Local development guidance and automation
3. **CI/CD Pipelines** - Automated version bumping and release creation
4. **Helper Scripts** - Interactive tools for easy adoption

### Supported Platforms

- **GitHub**: GitHub Actions workflow (`.github/workflows/release.yml`)
- **GitLab**: GitLab CI/CD pipeline (`.gitlab-ci.yml`)
- **Local Development**: Git hooks work on any Git repository

## Conventional Commits

### Commit Message Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Supported Types

| Type | Version Bump | Description |
|------|-------------|-------------|
| `feat:` | minor | New features |
| `fix:` | patch | Bug fixes |
| `docs:` | patch | Documentation changes |
| `style:` | patch | Code style changes |
| `refactor:` | patch | Code refactoring |
| `perf:` | patch | Performance improvements |
| `test:` | patch | Test additions/changes |
| `chore:` | patch | Maintenance tasks |
| `ci:` | patch | CI/CD changes |
| `build:` | patch | Build system changes |
| `revert:` | patch | Commit reversions |
| `BREAKING CHANGE:` | major | Breaking changes |

### Examples

```bash
# Feature commit (minor version bump)
git commit -m "feat: add image optimization feature"

# Bug fix (patch version bump)
git commit -m "fix: resolve database connection timeout"

# Breaking change (major version bump)
git commit -m "feat!: redesign core architecture

BREAKING CHANGE: API endpoints have changed"

# Scoped commit
git commit -m "feat(ui): add dark mode toggle"

# With body and footer
git commit -m "fix: handle edge case in image processing

The previous implementation failed when processing corrupted images.
This fix adds proper error handling and fallback mechanisms.

Closes #123"
```

## Git Hooks

### prepare-commit-msg Hook

**Location**: `.git/hooks/prepare-commit-msg`
**Purpose**: Provides guidance before committing

**Features**:
- Analyzes staged changes
- Suggests appropriate commit types
- Shows version bump impact
- Educates developers on conventional commit format

**Example Output**:
```
Analyzing your changes...
Detected changes in: src/image_utils.rs, tests/

Suggested commit types:
- feat: for new functionality
- fix: for bug fixes
- refactor: for code restructuring

Recommended: feat (based on new functions added)
This will trigger a MINOR version bump (0.7.0 -> 0.8.0)
```

### post-commit Hook

**Location**: `.git/hooks/post-commit`
**Purpose**: Provides feedback after committing

**Features**:
- Validates commit message format
- Shows version bump result
- Suggests next steps for automation
- Provides guidance for non-conventional commits

**Example Output**:
```
✅ Conventional commit detected: feat: add image optimization
Recommended version bump: minor
Remote repository detected. Push to trigger GitHub Actions workflow.
To push and trigger automated release:
  git push origin main
```

## CI/CD Pipelines

### GitHub Actions Workflow

**File**: `.github/workflows/release.yml`
**Trigger**: Push to main/master branches
**Platform**: GitHub

#### Workflow Steps

1. **Checkout Code**
   - Fetches repository with full history
   - Uses GitHub token for authentication

2. **Setup Environment**
   - Configures Git user
   - Sets up Rust toolchain

3. **Version Detection**
   - Parses current version from Cargo.toml
   - Analyzes recent commits for conventional commit patterns

4. **Version Calculation**
   - Determines bump type (major/minor/patch)
   - Calculates new version number

5. **File Updates**
   - Updates Cargo.toml version
   - Updates README.md version references
   - Updates other version-dependent files

6. **Release Creation**
   - Creates GitHub release
   - Generates release notes
   - Creates version tag

#### Configuration

```yaml
on:
  push:
    branches: [ main, master ]
  workflow_dispatch:
    inputs:
      version_type:
        description: 'Version bump type'
        required: true
        default: 'patch'
        type: choice
        options:
        - patch
        - minor
        - major
```

### GitLab CI/CD Pipeline

**File**: `.gitlab-ci.yml`
**Trigger**: Push to main/master branches (conventional commits only)
**Platform**: GitLab

#### Pipeline Features

1. **Conditional Execution**
   - Only runs on conventional commits
   - Skips non-conventional commits

2. **Version Management**
   - Same logic as GitHub Actions
   - Updates all version files
   - Creates commits for version changes

3. **GitLab Integration**
   - Uses GitLab Releases API
   - Creates GitLab releases
   - Pushes version tags

#### Required Setup

**Personal Access Token**:
- Scopes: `api`, `write_repository`
- CI/CD Variable: `GITLAB_ACCESS_TOKEN`

**Project Settings**:
- Default branch: main or master
- Protected branches configured

## Helper Scripts

### conventional-commit.sh

**Purpose**: Interactive conventional commit creation
**Usage**: `./conventional-commit.sh`

**Features**:
- Guided commit type selection
- Scope input (optional)
- Description input
- Automatic commit execution
- Validation of inputs

**Example Session**:
```
Conventional Commit Helper
==========================

Available commit types:
  feat:     A new feature
  fix:      A bug fix
  ...

Enter commit type: feat
Enter scope (optional, press Enter to skip): ui
Enter description: add dark mode toggle

Commit message: feat(ui): add dark mode toggle

Proceed with this commit? (y/N): y
Commit created successfully!
```

### version_docs.sh

**Purpose**: Manual version management
**Usage**: `./version_docs.sh [major|minor|patch|version]`

**Features**:
- Manual version bumping
- Specific version setting
- Updates all version files
- Dry-run mode available

**Examples**:
```bash
# Minor version bump (default)
./version_docs.sh

# Specific version bump
./version_docs.sh major
./version_docs.sh minor
./version_docs.sh patch

# Set specific version
./version_docs.sh 1.2.3
```

### test_automation.sh

**Purpose**: System validation and testing
**Usage**: `./test_automation.sh`

**Features**:
- Validates all components
- Tests conventional commit detection
- Checks file existence and permissions
- Verifies CI/CD configurations

**Test Coverage**:
- Git hooks functionality
- Helper scripts
- GitHub Actions workflow
- GitLab CI/CD pipeline
- Version management scripts

## Setup Instructions

### For GitHub

1. **Repository Setup**
   ```bash
   # Ensure default branch is main or master
   git branch -m master main  # if needed
   git push -u origin main
   ```

2. **File Structure**
   ```
   .github/
     workflows/
       release.yml
   .git/
     hooks/
       prepare-commit-msg
       post-commit
   conventional-commit.sh
   version_docs.sh
   ```

3. **Permissions**
   - GitHub Actions has write permissions by default
   - No additional setup required

### For GitLab

1. **Personal Access Token**
   ```bash
   # GitLab → User Settings → Access Tokens
   # Create token with scopes: api, write_repository
   ```

2. **CI/CD Variables**
   ```
   # Project → Settings → CI/CD → Variables
   GITLAB_ACCESS_TOKEN = your_token_here
   # Mark as Protected and Masked
   ```

3. **Repository Setup**
   ```bash
   # Ensure default branch is main or master
   git branch -m master main  # if needed
   git push -u origin main
   ```

4. **File Structure**
   ```
   .gitlab-ci.yml
   .git/
     hooks/
       prepare-commit-msg
       post-commit
   conventional-commit.sh
   version_docs.sh
   ```

## Usage Workflow

### Development Process

1. **Make Changes**
   ```bash
   # Edit files
   vim src/image_utils.rs
   ```

2. **Stage Changes**
   ```bash
   git add src/image_utils.rs
   ```

3. **Create Conventional Commit**
   ```bash
   # Option 1: Interactive helper
   ./conventional-commit.sh

   # Option 2: Direct commit
   git commit -m "feat: add image optimization"
   ```

4. **Push Changes**
   ```bash
   git push origin main
   ```

5. **Automated Release**
   - CI/CD pipeline triggers
   - Version automatically calculated
   - Files updated
   - Release created
   - Tag pushed

### Manual Override

For manual version control:
```bash
# Force specific version bump
./version_docs.sh major

# Set exact version
./version_docs.sh 2.0.0
```

## Version File Updates

The system automatically updates version references in:

- **Cargo.toml**: `version = "1.2.3"`
- **README.md**:
  - Title: `# Stamp Album Generator v1.2.3`
  - Version fields: `**Version**: 1.2.3`
  - Release date: `**Release Date**: 2025-09-20`

## Release Notes Generation

Automated release notes include:

- **Version Information**
  - Bump type (major/minor/patch)
  - Previous and new versions

- **Changes**
  - List of commit messages since last release

- **Metadata**
  - Automation notice
  - Generation timestamp

**Example Release Notes**:
```markdown
## Release 1.2.3

### Changes
- feat: add image optimization feature
- fix: resolve database connection timeout
- docs: update installation instructions

### Version Bump
- **Type**: minor
- **Previous**: 1.1.2
- **New**: 1.2.3

### Automated Release
This release was automatically created by the CI/CD pipeline based on conventional commits.
```

## Troubleshooting

### Common Issues

#### Git Hooks Not Working
```bash
# Check permissions
ls -la .git/hooks/

# Make executable
chmod +x .git/hooks/prepare-commit-msg
chmod +x .git/hooks/post-commit
```

#### CI/CD Pipeline Not Triggering
```bash
# Check commit message format
git log --oneline -1

# Verify branch
git branch --show-current

# Check CI/CD status in platform
```

#### Version Not Updating
```bash
# Check current version
grep '^version = ' Cargo.toml

# Test version script
./version_docs.sh --dry-run patch
```

### Debug Mode

Enable debug logging:
```bash
# Git hooks debug
GIT_TRACE=1 git commit -m "test: debug commit"

# CI/CD debug
# Add to pipeline: echo "Debug: $VARIABLE"
```

## Testing

### Automated Testing
```bash
# Run full test suite
./test_automation.sh

# Test GitLab CI specifically
./test_gitlab.sh
```

### Manual Testing
```bash
# Test conventional commit detection
echo "feat: test feature" | grep -qE "^(feat|fix|docs|style|refactor|perf|test|chore|ci|build|revert)(\(.+\))?: "

# Test version calculation
CURRENT="1.2.3"
MAJOR=$(echo $CURRENT | cut -d. -f1)
echo "Major: $MAJOR"
```

## Platform Comparison

| Feature | GitHub | GitLab |
|---------|--------|--------|
| **Setup Complexity** | Low | Medium |
| **Authentication** | Automatic | Manual PAT |
| **Release API** | GitHub Releases | GitLab Releases |
| **Workflow Syntax** | YAML | YAML |
| **Cost** | Free tier available | Free tier available |
| **Integration** | Native | Native |

## Maintenance

### Regular Tasks

1. **Update Dependencies**
   ```bash
   # Check for outdated actions/packages
   # Update Rust version in CI
   # Update Node.js versions if used
   ```

2. **Review Permissions**
   ```bash
   # Audit access tokens
   # Check CI/CD variable security
   # Review branch protection rules
   ```

3. **Monitor Performance**
   ```bash
   # Check pipeline execution times
   # Monitor for failed releases
   # Review automation success rate
   ```

### Version History

- **v0.1.0**: Initial implementation
- **v0.2.0**: Added GitLab support
- **v0.3.0**: Enhanced conventional commit detection
- **v0.4.0**: Improved error handling
- **v0.5.0**: Added manual release triggers
- **v0.6.0**: Comprehensive testing suite
- **v0.7.0**: Documentation and examples

## Contributing

### Adding New Features

1. **Test Locally**
   ```bash
   # Test git hooks
   # Test helper scripts
   # Test CI/CD pipeline locally if possible
   ```

2. **Update Documentation**
   ```bash
   # Update this document
   # Update README.md
   # Add examples and troubleshooting
   ```

3. **Platform Compatibility**
   ```bash
   # Test on both GitHub and GitLab
   # Ensure cross-platform compatibility
   # Update platform-specific documentation
   ```

### Code Standards

- Use conventional commits for all changes
- Update version files appropriately
- Test automation thoroughly
- Document new features comprehensively

---

**Last Updated**: September 20, 2025
**Version**: 0.7.0
**Platforms**: GitHub, GitLab
**Status**: Production Ready</content>
<parameter name="filePath">/media/ray/program/Rust/StampAlbumGen2/AUTOMATION_SYSTEM_DOCUMENTATION.md