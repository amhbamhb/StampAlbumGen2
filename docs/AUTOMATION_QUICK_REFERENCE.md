# Automated Version Management - Quick Reference

## 🚀 Quick Start

### For New Developers

```bash
# 1. Use the interactive commit helper
./conventional-commit.sh

# 2. Or commit directly with conventional format
git commit -m "feat: add new feature"
git push  # Triggers CI/CD pipeline (GitLab primary)
```

### For Manual Control

```bash
# Version bump types
./version_docs.sh patch    # 1.0.0 → 1.0.1
./version_docs.sh minor    # 1.0.0 → 1.1.0
./version_docs.sh major    # 1.0.0 → 2.0.0

# Set specific version
./version_docs.sh 2.1.3
```

## 📝 Conventional Commit Types

| Type | Version | Example |
|------|---------|---------|
| `feat:` | minor | `feat: add user authentication` |
| `fix:` | patch | `fix: resolve login bug` |
| `docs:` | patch | `docs: update API documentation` |
| `BREAKING CHANGE:` | major | `feat!: redesign API` |

## 🔧 Git Hooks

- Before commit: suggests/validates conventional commit format
- After commit: prints next-step guidance and bump impact

```bash
# Enable hooks if missing
chmod +x .git/hooks/prepare-commit-msg
chmod +x .git/hooks/post-commit
```

## ⚙️ Platform Setup

### Primary: GitLab (current)

- 🔑 Requires `GITLAB_ACCESS_TOKEN` in CI/CD variables (scopes: api, write_repository)
- 📁 Uses `.gitlab-ci.yml` for pipelines and release creation
- Pushes / merge requests to protected branches trigger release pipeline

### Future: GitHub (planned)

- When added, GitHub-specific workflows and docs will be placed in docs/ and Appendix B of system documentation.
- Suggested artifacts: `.github/workflows/release.yml`, Actions secrets, mapping notes.

## 🧪 Testing

### Full System Test

```bash
./test_automation.sh
```

### GitLab Specific

```bash
./test_gitlab.sh
```

## 📊 Version Flow

```text
Developer Commit → CI/CD Trigger → Version Analysis → File Updates → Release Creation
```

## 🖼 Album layout / metadata — current code notes

- Image paths are read from the DB and composed as:
  Albums/{album-name}/{image_path} (album_name spaces replaced with '-').
- Physical sizes estimated using estimate_real_dimensions(...). Images are scaled so height ≤ stamp_box_height; final emitted size adds stamp_box_adjust margins.
- Left column: cleaned denomination (sg number/denomination).
- Centre column: color, description, and stamps_issued (counts shown when present) joined with newline separators.
- Right column: first line = mint_condition fallback — if blank the code substitutes the literal "Unsed"; second line = used; stamps_issued remains in centre.
- get_meaningful(...) helper filters common placeholders (e.g. "n/a", "-", "0") to determine when a value is meaningful.
- Strings (sg, image_path) are sanitized for quotes/newlines before rendering.

## 🚨 Troubleshooting

### Pipeline not running?

```bash
git log --oneline -1
git branch --show-current
# Check GitLab pipeline page for the repo
```

### Version not updating?

```bash
# Ensure commit uses conventional format
./version_docs.sh --dry-run patch
```

### Hooks not working?

```bash
ls -la .git/hooks/
chmod +x .git/hooks/*
```

### Image/layout issues?

- Confirm Images exist at Albums/{album-name}/{image_path}
- Missing/unreadable images fall back to reference dimensions in estimate_real_dimensions

## 📚 Full Documentation

See `docs/AUTOMATION_SYSTEM_DOCUMENTATION.md` for complete details and Appendix entries for GitLab/GitHub.

---

**Quick Reference v0.8.0** | **Last Updated: 2025-09-28**
