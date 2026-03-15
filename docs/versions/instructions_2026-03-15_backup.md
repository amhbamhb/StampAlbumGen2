# Stamp Album Generator - Instructions
# Version: 1.5.0
# Last Updated: 2026-03-15
# Previous Versions: docs/versions/instructions_2025-09-18_v1.md

# Best Practices
- Follow Rust coding standards and conventions.
- Write modular and reusable code.
- Include unit tests for critical functions.
- Use version control effectively with Git.
- Always use best practices for security and performance.
- Keep your code organized and well-documented.
- Commit changes frequently with clear messages.
- Use descriptive names for files and directories.
- Review and test your code before pushing.
- When adding, removing, or renaming configuration keys, ALWAYS update all of the following:
  1) the `Config` struct (and its `with_defaults()` implementation) in `src/config.rs`
  2) the `config_template.toml` file (human-friendly template with comments)
  3) the default-creation logic in `src/config.rs` which copies `config_template.toml` when `config.toml` is absent
- Use GitLab Issues for project-related questions.

# Git Repository Structure
- Source code is in `/src`.
- Configuration files are in `/config.toml` and `/config_template.toml`.
- Documentation is in `/docs`:
  - User/project index: `/docs/README.md`
  - Guides: `/docs/guides/`
  - Reports: `/docs/reports/`
  - Testing artifacts: `/docs/testing/`
- Root contributor/project docs: `/README.md`, `/DEVELOPMENT.md`, `/CONTRIBUTING.md`, `/RELEASE_NOTES.md`.
- License: `/LICENSE` (Laser Spider Stamp Collector License (LSSCL) v1.0).
- Tests are in `/tests`.
- Example data is in `/examples`.
- Stamp collections are in `/Albums/{country name}/`.
- Archived source files are in `/Archive`.
- Image assets are in `/images`.
- Build artifacts are in `/target`.

# SQLite Database
- Databases are stored in `/Albums/{country name}/{country name}.db`.
- Database backups are stored as `{country name}.db.backup`.

# Postage Stamps
- Store stamp images in `Albums/{country name}/images/`.

# StampEasy Output
- Output files are generated in `/Albums/{country name}`.
- Album text files are named `{country name}.txt`.
- Extract files are named `{country name}_extract.txt`.
- Review output logs in the project root (timestamped `.log` files).

# Developing with Automated Version Management
- Refer to `docs/AUTOMATION_SETUP.md` for setup instructions.
- Use `docs/AUTOMATION_QUICK_REFERENCE.md` for quick usage.
- Consult `docs/AUTOMATION_SYSTEM_DOCUMENTATION.md` for full technical details.

# Development Testing
- Run tests with `cargo test --all`.
- Use a single year for deterministic test scenarios when validating scrape behavior.
- Use best practices for testing (separate test files, focused unit tests, and clear assertions).

# License and Usage Policy
- Personal no-fee use is limited to individual stamp collectors per `/LICENSE`.
- Any non-collector, business, institutional, or commercial use requires a separate written paid agreement with Laser Spider LLC.
