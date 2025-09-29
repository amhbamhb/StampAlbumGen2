# Automated Version Management System Documentation

## Overview

This document describes the automated version management and related automation for the Stamp Album Generator project. The system provides conventional-commit based versioning, local git hooks, CI/CD pipelines, helper scripts, and editor integrations. Current primary hosting and CI is GitLab; GitHub support is planned (see Appendix B).

## Quick summary of recent code updates

- album layout and metadata extraction (src/album_globals.rs)
  - image paths are read from the DB and composed as Albums/{album-name}/{image_path}.
  - image physical sizes are estimated with estimate_real_dimensions(...) and scaled to fit stamp_box_height; emitted box sizes include stamp_box_adjust margins.
  - left column: denomination (sg number/denomination) is cleaned and used.
  - centre column: now emits color, description, and stamps_issued (counts shown when present) joined with newline separators.
  - right column: mint/unused/used handling improved. If mint_condition is blank the code now substitutes the literal "Unsed" as the first-line fallback (per preference). A get_meaningful(...) helper filters placeholders to determine meaningful values. used remains the second right-hand line; stamps_issued stays in the centre field when present.
  - other safety cleanups: sg and image strings sanitized for quotes/newlines.

- Commit-message generation (editor integration)
  - A repository-level Copilot instructions file (.copilot-commit-message-instructions.md) is present and referenced from VS Code settings to make generators emit both a SIMPLE and a DETAILED message by default. Edit that file to match team conventions.

- CI/CD
  - Primary pipeline: GitLab CI (.gitlab-ci.yml) performs conventional-commit detection, version calculation, version file updates, tag creation, and GitLab release creation. The logic mirrors the conventional mapping described below.
  - GitHub Actions workflow remains in the docs as a future option (Appendix B).

## Conventional Commit types (current mapping)

- feat: minor
- fix: patch
- docs/style/refactor/perf/test/chore/ci/build/revert: patch
- BREAKING CHANGE: major

Adjust pipeline mappings if different behavior is required.

## Files of interest (recently changed or relevant)

- src/album_globals.rs — stamp layout, image sizing, and metadata formatting (mint fallback to "Unsed")
- .copilot-commit-message-instructions.md — Copilot commit message rules/examples
- .gitlab-ci.yml — GitLab CI pipeline for automated releases
- conventional-commit.sh — interactive commit helper
- version_docs.sh — manual/force version bump helper
- test_automation.sh — validation and dry-run tests
- .git/hooks/prepare-commit-msg, .git/hooks/post-commit — local guidance hooks (copy to .git/hooks and make executable)

## How to verify the recent changes

- Build and run a layout preview (project-specific steps)
  - Ensure images referenced by image_path exist under Albums/{album-name}/
  - Run the stamp rendering/preview task or unit tests that exercise album layout.

- Unit / integration checks:
  - Run ./test_automation.sh (validates hooks, scripts and dry-run of bumps)
  - For album layout specifics, run the integration that renders a single album page and inspect centre/right/left fields.

## Editor integration (VS Code + Copilot)

- Ensure VS Code setting points to the repository instruction file:
  - Key: github.copilot.chat.commitMessageGeneration.instructions
  - Example: settings references .copilot-commit-message-instructions.md

- The instruction file requests two outputs from the generator:
  - SIMPLE: one-line imperative message (<= 50 chars)
  - DETAILED: 2–5 sentence explanation of what/why/notes

Edit .copilot-commit-message-instructions.md to change prefixes, ticket rules, or examples.

## Troubleshooting

- Hooks not executing: verify executable bit
  - chmod +x .git/hooks/prepare-commit-msg .git/hooks/post-commit

- Version bump not triggered:
  - Verify commit message follows the conventional pattern
  - Check pipeline logs in GitLab for conventional-commit detection step

- Image scaling/layout oddities:
  - Confirm estimate_real_dimensions can read the image; missing images fall back to reference dimensions
  - Verify album_name and image_path produce a valid file path under Albums/

## Tests and validation

- Run:

  ```bash
  ./test_automation.sh
  ```

## Day / Month parsing (code note)

The collector now uses a precompiled regular expression `DAY_MONTH_RE` (defined in `src/stampworld/data_collector.rs`) to extract day and month tokens from block detail lines. It supports Unicode month names (for example `März`, `Décembre`) and hyphenated month tokens. This avoids per-loop regex compilation and improves robustness for internationalized month names.

If you need to accept trailing dots on month abbreviations (e.g. `Feb.`), the regex can be widened to allow a trailing `.` in the capture.
