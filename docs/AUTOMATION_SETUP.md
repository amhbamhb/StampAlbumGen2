# Automated Version Management Setup Complete! 🎉

## Overview

Your Stamp Album Generator project includes an automated version management system that works locally and with GitHub Actions. Version bumps happen automatically based on conventional commit messages; you can also use helper scripts and editor integrations to generate and customize commit messages.

## What's Been Set Up

### 1. GitHub Actions Workflow (`.github/workflows/release.yml`)

- Triggers: push to main/master branches and manual dispatch
- Features:
  - Conventional commit detection
  - Version calculation (major/minor/patch)
  - Updates version references in project files (Cargo.toml, docs, etc.)
  - Creates GitHub releases and release notes
  - Supports automated and manual version bumps

### 2. Git Hooks (local)

- `.git/hooks/prepare-commit-msg`: provides guidance for conventional commit types before committing
- `.git/hooks/post-commit`: detects conventional commits and prints next-step guidance

> Note: Hooks are installed into the local repo's `.git/hooks` directory. If your repo was cloned before hooks were added, re-run the installation script (if present) or copy the hook files into `.git/hooks` manually and make them executable.

### 3. Helper Scripts

- `conventional-commit.sh` — interactive helper for creating conventional commits
- `version_docs.sh` — manual version bump helper (can force major/minor/patch)
- `test_automation.sh` — validation script to exercise the automation components

### 4. Editor Integration: GitHub Copilot / VS Code

- A Copilot instruction file is provided to customize generated commit messages:
  - `.copilot-commit-message-instructions.md` at the repository root
  - VS Code user/workspace setting used: `github.copilot.chat.commitMessageGeneration.instructions` (example in this environment: `~/.config/Code/User/settings.json`)
- The instruction file tells Copilot to emit both a short "SIMPLE" commit message and a fuller "DETAILED" message. Edit that file to match your team's conventions (prefixes, ticket formats, examples).

### 5. Documentation

- This document (AUTOMATION_SETUP.md) and README.md contain usage notes and examples.
- Example Copilot instructions file (repository root) contains examples for SIMPLE and DETAILED messages.

## How It Works

### Automatic Flow

1. Make a conventional commit:

   ```bash
   git commit -m "feat: add new stamp filtering feature"
   ```

2. Local hooks (if installed) will provide guidance at commit time.
3. Push triggers GitHub Actions:
   - Workflow detects commit type (`feat` = minor, `fix` = patch, `BREAKING CHANGE` = major)
   - Calculates new version
   - Updates project files and publishes a GitHub release

### Editor-assisted Flow (optional)

- Use the VS Code "Generate Commit Message" command (GitHub Copilot or GitLens AI) to create commit messages.
- With `.copilot-commit-message-instructions.md` present and VS Code configured, the generator will produce both:
  - SIMPLE: single-line, imperative, short message
  - DETAILED: multi-sentence explanation (what, why, notes)
- Customize the instruction file to change wording, ticket inclusion rules, or to add examples.

## Conventional Commit Types (default mapping)

- feat: minor
- fix: patch
- docs/style/refactor/perf/test/chore/ci/build/revert: patch
- BREAKING CHANGE: major

Adjust mappings in your workflow if your project requires different behavior.

## Usage Examples

Quick start:

```bash
# Interactive helper
./conventional-commit.sh

# Or commit directly
git commit -m "fix: correct image scaling in stamp layout"
git push
```

Manual bump:

```bash
./version_docs.sh patch   # force a patch bump
./version_docs.sh minor   # force a minor bump
```

Validate automation:

```bash
./test_automation.sh
```

## Files Created / Modified

New or important files you may see in the repo:

- `.github/workflows/release.yml` — GitHub Actions workflow
- `.git/hooks/prepare-commit-msg` — local commit guidance hook (copy to `.git/hooks`)
- `.git/hooks/post-commit` — local post-commit helper (copy to `.git/hooks`)
- `conventional-commit.sh` — interactive conventional commit helper
- `version_docs.sh` — manual version bump script
- `test_automation.sh` — automation validation script
- `.copilot-commit-message-instructions.md` — Copilot commit message instructions (repository root)

Modified files:

- `README.md` — automation section added/updated
- (Optional) VS Code user/workspace settings:
  - Example path: `~/.config/Code/User/settings.json`
  - Key: `github.copilot.chat.commitMessageGeneration.instructions` referencing `.copilot-commit-message-instructions.md`

## Customization

- Edit `.copilot-commit-message-instructions.md` to change commit generation rules, add examples, or require ticket IDs.
- Update the workflow (`.github/workflows/release.yml`) if you want different commit-type → version bump mappings.
- Ensure hooks are executable (chmod +x) and present in `.git/hooks` for local guidance.

## Testing

Run:

```bash
./test_automation.sh
```

The script runs validations for hooks, workflow config sanity, script presence, and a dry-run of the version bump logic.

## Benefits

- ✅ **Zero manual intervention** for version management
- ✅ **Consistent versioning** across all project files
- ✅ **Automatic releases** with proper semantic versioning
- ✅ **Local guidance** through git hooks
- ✅ **Flexible manual control** when needed
- ✅ **Comprehensive testing** and validation

Your automated version management system is now ready! Every conventional commit will automatically trigger the appropriate version bump and release creation. 🚀

---

## Appendix A — Current host: GitLab (important)

- This repository currently uses GitLab for CI/CD and release automation. Where this document refers to GitHub Actions or `.github/workflows/release.yml`, the equivalent in this repository is the GitLab CI pipeline (see `.gitlab-ci.yml`) and GitLab Releases.
- Pipeline triggers, job names, and release steps are implemented with GitLab CI syntax and pipeline settings (pushes, merge requests, scheduled or manual pipelines). Local hooks and helper scripts remain the same and work with GitLab workflows.
- If you need help mapping any GitHub Actions examples in this doc to GitLab CI jobs, open an issue or request a quick conversion.

## Appendix B — Future: GitHub (placeholder)

- Future releases may add GitHub as an alternative hosting/CI option. If/when GitHub support is added, place GitHub-specific instructions and workflow files in this section (for example `.github/workflows/release.yml`, Actions secrets, and any GitHub-specific release steps).
- Suggested contents for that future section:
  - Example Action workflow and triggers
  - Required repository secrets and permissions
  - Differences in release notes generation or tag creation between GitLab and GitHub
  - Migration notes for maintainers switching CI providers
