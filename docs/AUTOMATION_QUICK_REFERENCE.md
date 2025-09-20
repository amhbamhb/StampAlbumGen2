# Automated Version Management - Quick Reference

## 🚀 Quick Start

### For New Developers
```bash
# 1. Use the interactive commit helper
./conventional-commit.sh

# 2. Or commit directly with conventional format
git commit -m "feat: add new feature"
git push  # Triggers automated release
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

### Automatic Guidance
- **Before commit**: Suggests commit types based on changes
- **After commit**: Shows version bump impact and next steps

### Manual Testing
```bash
# Test hooks
chmod +x .git/hooks/prepare-commit-msg
chmod +x .git/hooks/post-commit
```

## ⚙️ Platform Setup

### GitHub
- ✅ Automatic (no setup required)
- ✅ Uses `.github/workflows/release.yml`

### GitLab
- 🔑 Requires `GITLAB_ACCESS_TOKEN`
- 📁 Uses `.gitlab-ci.yml`
- 🏷️ Add token to CI/CD variables

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

```
Developer Commit → CI/CD Trigger → Version Analysis → File Updates → Release Creation
     ↓                ↓              ↓                ↓              ↓
  "feat: ..."     GitHub/GitLab   Detect "feat"    Update files    Create v1.1.0
  (conventional)  Actions/CI      → minor bump    (Cargo.toml,    (with notes)
                                                  README.md)      (with tag)
```

## 🎯 Key Benefits

- ✅ **Zero manual versioning** - happens automatically
- ✅ **Consistent releases** - every conventional commit
- ✅ **Cross-platform** - works on GitHub and GitLab
- ✅ **Local guidance** - git hooks help developers
- ✅ **Flexible control** - manual override available

## 🚨 Troubleshooting

### Pipeline Not Running?
```bash
# Check commit format
git log --oneline -1

# Verify branch
git branch --show-current

# Check platform CI status
```

### Version Not Updating?
```bash
# Check current version
grep '^version = ' Cargo.toml

# Test manually
./version_docs.sh --dry-run patch
```

### Hooks Not Working?
```bash
# Check permissions
ls -la .git/hooks/

# Make executable
chmod +x .git/hooks/*
```

## 📚 Full Documentation

See `docs/AUTOMATION_SYSTEM_DOCUMENTATION.md` for complete details.

---

**Quick Reference v0.7.0** | **Last Updated: 2025-09-20**</content>
<parameter name="filePath">/media/ray/program/Rust/StampAlbumGen2/AUTOMATION_QUICK_REFERENCE.md