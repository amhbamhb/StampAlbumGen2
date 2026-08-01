#!/bin/bash

# Test script for conventional commit detection and version bumping
# This script validates thTesting GitHub Actions workflow:

if [ -f ".github/workflows/release.yml" ]; then
    echo "✅ GitHub Actions workflow file exists"

    # Check if workflow has required components
    if grep -q "feat:\|fix:\|BREAKING CHANGE" .github/workflows/release.yml; then
        echo "✅ Workflow includes conventional commit detection"
    else
        echo "❌ Workflow missing conventional commit detection"
    fi

    if grep -q "release" .github/workflows/release.yml; then
        echo "✅ Workflow includes release creation"
    else
        echo "❌ Workflow missing release creation"
    fi
else
    echo "❌ GitHub Actions workflow file missing"
fi

echo ""
echo "Testing GitLab CI/CD pipeline:"
echo ""

if [ -f ".gitlab-ci.yml" ]; then
    echo "✅ GitLab CI/CD pipeline file exists"

    # Check if pipeline has required components
    if grep -q "feat\|fix\|BREAKING CHANGE" .gitlab-ci.yml; then
        echo "✅ Pipeline includes conventional commit detection"
    else
        echo "❌ Pipeline missing conventional commit detection"
    fi

    if grep -q "releases" .gitlab-ci.yml; then
        echo "✅ Pipeline includes release creation"
    else
        echo "❌ Pipeline missing release creation"
    fi

    if grep -q "GITLAB_ACCESS_TOKEN" .gitlab-ci.yml; then
        echo "✅ Pipeline uses GitLab access token"
    else
        echo "❌ Pipeline missing GitLab access token configuration"
    fi
else
    echo "❌ GitLab CI/CD pipeline file missing"
fistem works correctly

echo "Testing Conventional Commit Detection"
echo "====================================="
echo ""

# Test cases for conventional commit detection
test_cases=(
    "feat: add new stamp filtering feature"
    "fix: resolve database connection issue"
    "docs: update README with installation instructions"
    "feat(api): add REST API endpoints"
    "fix(ui): correct button alignment in album view"
    "BREAKING CHANGE: redesign core architecture"
    "chore: update dependencies"
    "refactor: simplify image processing logic"
    "style: format code according to rustfmt"
    "test: add unit tests for stamp processing"
    "ci: update GitHub Actions workflow"
    "build: fix compilation on Windows"
    "perf: optimize database queries"
    "revert: undo previous commit"
    "invalid commit message"
    "another invalid message"
)

echo "Testing commit message parsing:"
echo ""

for commit_msg in "${test_cases[@]}"; do
    if echo "$commit_msg" | grep -qE "^(feat|fix|docs|style|refactor|perf|test|chore|ci|build|revert)(\(.+\))?: "; then
        # Extract the type from the commit message
        commit_type=$(echo "$commit_msg" | sed -n 's/^\([a-zA-Z]*\).*/\1/p')

        # Determine version bump type
        case $commit_type in
            feat)
                bump_type="minor"
                ;;
            fix)
                bump_type="patch"
                ;;
            BREAKING*)
                bump_type="major"
                ;;
            *)
                bump_type="patch"
                ;;
        esac

        echo "✅ VALID: '$commit_msg' -> Type: $commit_type, Bump: $bump_type"
    else
        echo "❌ INVALID: '$commit_msg'"
    fi
done

echo ""
echo "Testing version_docs.sh script:"
echo ""

# Test version_docs.sh if it exists
if [ -f "./version_docs.sh" ]; then
    echo "✅ version_docs.sh found"

    # Test help output
    echo "Testing help output:"
    ./version_docs.sh --help 2>/dev/null || echo "Help option not available"

    # Test dry run (if supported)
    echo "Testing dry run:"
    ./version_docs.sh --dry-run patch 2>/dev/null || echo "Dry run not available or no changes needed"
else
    echo "❌ version_docs.sh not found"
fi

echo ""
echo "Testing git hooks:"
echo ""

# Check if git hooks exist and are executable
if [ -x ".git/hooks/prepare-commit-msg" ]; then
    echo "✅ prepare-commit-msg hook is executable"
else
    echo "❌ prepare-commit-msg hook missing or not executable"
fi

if [ -x ".git/hooks/post-commit" ]; then
    echo "✅ post-commit hook is executable"
else
    echo "❌ post-commit hook missing or not executable"
fi

echo ""
echo "Testing conventional-commit.sh helper:"
echo ""

if [ -x "./conventional-commit.sh" ]; then
    echo "✅ conventional-commit.sh is executable"
else
    echo "❌ conventional-commit.sh missing or not executable"
fi

echo ""
echo "Testing GitHub Actions workflow:"
echo ""

if [ -f ".github/workflows/release.yml" ]; then
    echo "✅ GitHub Actions workflow file exists"

    # Check if workflow has required components
    if grep -q "feat:\|fix:\|BREAKING CHANGE" .github/workflows/release.yml; then
        echo "✅ Workflow includes conventional commit detection"
    else
        echo "❌ Workflow missing conventional commit detection"
    fi

    if grep -q "release" .github/workflows/release.yml; then
        echo "✅ Workflow includes release creation"
    else
        echo "❌ Workflow missing release creation"
    fi
else
    echo "❌ GitHub Actions workflow file missing"
fi

echo ""
echo "Testing GitLab CI/CD pipeline:"
echo ""

if [ -f ".gitlab-ci.yml" ]; then
    echo "✅ GitLab CI/CD pipeline file exists"

    # Check if pipeline has required components
    if grep -q "feat\|fix\|BREAKING CHANGE" .gitlab-ci.yml; then
        echo "✅ Pipeline includes conventional commit detection"
    else
        echo "❌ Pipeline missing conventional commit detection"
    fi

    if grep -q "releases" .gitlab-ci.yml; then
        echo "✅ Pipeline includes release creation"
    else
        echo "❌ Pipeline missing release creation"
    fi

    if grep -q "GITLAB_ACCESS_TOKEN" .gitlab-ci.yml; then
        echo "✅ Pipeline uses GitLab access token"
    else
        echo "❌ Pipeline missing GitLab access token configuration"
    fi
else
    echo "❌ GitLab CI/CD pipeline file missing"
fi

echo ""
echo "🎉 Test complete! Review the results above."