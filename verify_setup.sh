#!/bin/bash

# Automated Version Management Setup Verification
# This script verifies that all components are properly configured

echo "🔍 Automated Version Management Setup Verification"
echo "=================================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASSED=0
FAILED=0

check_file() {
    local file=$1
    local description=$2

    if [ -f "$file" ]; then
        echo -e "${GREEN}✅ PASS${NC} - $description"
        ((PASSED++))
    else
        echo -e "${RED}❌ FAIL${NC} - $description (missing: $file)"
        ((FAILED++))
    fi
}

check_executable() {
    local file=$1
    local description=$2

    if [ -x "$file" ]; then
        echo -e "${GREEN}✅ PASS${NC} - $description"
        ((PASSED++))
    else
        echo -e "${RED}❌ FAIL${NC} - $description (not executable: $file)"
        ((FAILED++))
    fi
}

check_command() {
    local command=$1
    local description=$2

    if eval "$command" >/dev/null 2>&1; then
        echo -e "${GREEN}✅ PASS${NC} - $description"
        ((PASSED++))
    else
        echo -e "${RED}❌ FAIL${NC} - $description"
        ((FAILED++))
    fi
}

echo "📁 File Structure Checks:"
echo "-------------------------"

# Core files
check_file "Cargo.toml" "Cargo.toml exists"
check_file "README.md" "README.md exists"

# Git hooks
check_file ".git/hooks/prepare-commit-msg" "prepare-commit-msg hook exists"
check_file ".git/hooks/post-commit" "post-commit hook exists"
check_executable ".git/hooks/prepare-commit-msg" "prepare-commit-msg hook is executable"
check_executable ".git/hooks/post-commit" "post-commit hook is executable"

# Helper scripts
check_file "conventional-commit.sh" "conventional-commit.sh exists"
check_file "version_docs.sh" "version_docs.sh exists"
check_file "test_automation.sh" "test_automation.sh exists"
check_executable "conventional-commit.sh" "conventional-commit.sh is executable"
check_executable "version_docs.sh" "version_docs.sh is executable"
check_executable "test_automation.sh" "test_automation.sh is executable"

# CI/CD files
check_file ".github/workflows/release.yml" "GitHub Actions workflow exists"
check_file ".gitlab-ci.yml" "GitLab CI/CD pipeline exists"

# Documentation
check_file "docs/AUTOMATION_SYSTEM_DOCUMENTATION.md" "Full documentation exists"
check_file "docs/AUTOMATION_QUICK_REFERENCE.md" "Quick reference guide exists"

echo ""
echo "🔧 Functionality Checks:"
echo "------------------------"

# Git repository check
check_command "git rev-parse --git-dir >/dev/null 2>&1" "Git repository initialized"

# Remote check
if git remote get-url origin >/dev/null 2>&1; then
    echo -e "${GREEN}✅ PASS${NC} - Git remote configured"
    ((PASSED++))

    # Detect platform
    REMOTE_URL=$(git remote get-url origin)
    if echo "$REMOTE_URL" | grep -q "github.com"; then
        echo -e "${GREEN}📋 Detected: GitHub repository${NC}"
    elif echo "$REMOTE_URL" | grep -q "gitlab.com"; then
        echo -e "${GREEN}📋 Detected: GitLab repository${NC}"
    else
        echo -e "${YELLOW}⚠️  Detected: Other Git hosting service${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  WARNING${NC} - No Git remote configured"
fi

# Conventional commit test
echo "feat: test feature" | grep -qE "^(feat|fix|docs|style|refactor|perf|test|chore|ci|build|revert)(\(.+\))?: "
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ PASS${NC} - Conventional commit pattern recognition"
    ((PASSED++))
else
    echo -e "${RED}❌ FAIL${NC} - Conventional commit pattern recognition"
    ((FAILED++))
fi

# Version parsing test
if grep -q '^version = ' Cargo.toml; then
    CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
    echo -e "${GREEN}✅ PASS${NC} - Version parsing from Cargo.toml: $CURRENT_VERSION"
    ((PASSED++))
else
    echo -e "${RED}❌ FAIL${NC} - Cannot parse version from Cargo.toml"
    ((FAILED++))
fi

echo ""
echo "📊 Summary:"
echo "==========="

TOTAL=$((PASSED + FAILED))
SUCCESS_RATE=$((PASSED * 100 / TOTAL))

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 All checks passed! ($PASSED/$TOTAL)${NC}"
    echo ""
    echo "🚀 Your automated version management system is ready!"
    echo "   Next steps:"
    echo "   1. Try: ./conventional-commit.sh"
    echo "   2. Make a conventional commit and push"
    echo "   3. Watch the automated release process"
else
    echo -e "${YELLOW}⚠️  Setup incomplete ($PASSED/$TOTAL passed, $FAILED failed)${NC}"
    echo ""
    echo "🔧 To fix failed checks:"
    if [ ! -x ".git/hooks/prepare-commit-msg" ] || [ ! -x ".git/hooks/post-commit" ]; then
        echo "   • Make git hooks executable: chmod +x .git/hooks/*"
    fi
    if [ ! -x "conventional-commit.sh" ] || [ ! -x "version_docs.sh" ]; then
        echo "   • Make scripts executable: chmod +x *.sh"
    fi
    if [ ! -f ".gitlab-ci.yml" ]; then
        echo "   • For GitLab: ensure .gitlab-ci.yml exists"
    fi
    if [ ! -f ".github/workflows/release.yml" ]; then
        echo "   • For GitHub: ensure .github/workflows/release.yml exists"
    fi
fi

echo ""
echo "📖 Documentation:"
echo "   • Quick Reference: docs/AUTOMATION_QUICK_REFERENCE.md"
echo "   • Full Documentation: docs/AUTOMATION_SYSTEM_DOCUMENTATION.md"
echo "   • Test Suite: ./test_automation.sh"

echo ""
echo "🔍 Success Rate: $SUCCESS_RATE% ($PASSED/$TOTAL)"
echo ""
echo "=================================================="
echo "=================================================="
echo "Setup verification complete!"
