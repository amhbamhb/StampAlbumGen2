# Stamp Album Generator

Rust CLI that generates AlbumEasy pages from stamp data sources and SQLite databases.

## Start Here

Build release binary and run it:

```bash
cargo build --release
./target/release/stamp_album_gen2 --db-path Albums/Canada/Canada.db --album-name Canada
```

Show all available switches:

```bash
./stamp_album_gen2 --help
```

Important: when running the binary directly, do not put an extra `--` after `./stamp_album_gen2`.

Correct:

```bash
./stamp_album_gen2 --source colnectexport --colnect-inbox /path/to/colnect_inbox
```

Incorrect:

```bash
./stamp_album_gen2 -- --source colnectexport
```

## Common Usage Examples

Generate album output from an existing database:

```bash
./stamp_album_gen2 --db-path Albums/Canada/Canada.db --album-name Canada
```

Scrape StampWorld for a country/year range:

```bash
./stamp_album_gen2 \
 --source stampworld \
 --site-name Great-Britain \
 --album-name "Great Britain" \
 --start-year 1900 \
 --end-year 1950
```

Scrape US1847 mode:

```bash
./stamp_album_gen2 \
 --source us1847 \
 --site-name United-States \
 --album-name "United States" \
 --start-year 1847 \
 --end-year 2026
```

Ingest one Colnect CSV (source must be `colnectexport`):

```bash
./stamp_album_gen2 \
 --source colnectexport \
 --colnect-csv /path/to/en_stamps_csv_list_country_206-Switzerland_year_1972.csv \
 --db-path Albums/Switzerland/Switzerland.db \
 --album-name Switzerland
```

Process a Colnect inbox recursively, auto-route files by country, and create missing country DBs:

```bash
./stamp_album_gen2 \
 --source colnectexport \
 --colnect-inbox /path/to/colnect_inbox \
 --colnect-auto-route \
 --colnect-create-country-db
```

If `--colnect-inbox` is omitted for `--source colnectexport`, the program uses
`colnect_inbox_path` from `config.toml` and creates that directory on startup if
it does not exist.

Process only one country from an inbox:

```bash
./stamp_album_gen2 \
 --source colnectexport \
 --colnect-inbox /path/to/colnect_inbox \
 --colnect-country Switzerland
```

Force reimport even if CSV checksum was already imported:

```bash
./stamp_album_gen2 \
 --source colnectexport \
 --colnect-inbox /path/to/colnect_inbox \
 --colnect-force-reimport
```

Initialize local config from template:

```bash
./stamp_album_gen2 --init-config
```

Force overwrite existing config (creates backup first):

```bash
./stamp_album_gen2 --init-config --force-init-config
```

## CLI Switch Reference

### Core

- `--source <SOURCE>`
 Allowed values: `stampworld`, `touchstamps`, `extractfile`, `colnectexport`, `us1847`.
- `--site-name <SITE_NAME>`
 Site/country slug for scraping sources.
- `--album-name <ALBUM_NAME>`
 Album name used for output generation.
- `--db-path <DB_PATH>`
 SQLite DB path for output mode or explicit Colnect target DB.
- `--base-url <BASE_URL>`
 Override default StampWorld URL construction.

### Year Range and Scrape Control

- `--start-year <START_YEAR>` (default: `1840`)
- `--end-year <END_YEAR>` (default: current year at runtime)
- `--max-pages <MAX_PAGES>`
 Maximum pages to fetch when scraping (`0` means no explicit cap).
- `--stop-after-no-hits <N>` (default: `3`)
 Stops after `N` consecutive pages without matching stamps.

### Overwrite and Confirmation

- `--overwrite-album`
 Overwrite generated album output files.
- `--overwrite-images`
 Overwrite downloaded/generated image files.
- `--yes` / `-y`
 Auto-accept top source correction when source appears misspelled.

### Config Bootstrapping

- `--init-config`
 Create `config.toml` from template/defaults and exit.
- `--force-init-config`
 Use with `--init-config` to overwrite existing `config.toml` after timestamped backup.

### Colnect Ingestion

- `--colnect-csv <PATH>`
 Ingest a single Colnect CSV file.
- `--colnect-inbox <DIR>`
 Recursively discover and process Colnect CSV files country-by-country.
	If omitted in `colnectexport` mode, falls back to `colnect_inbox_path` in
	`config.toml`.
- `--colnect-auto-route`
 Move discovered files into `<inbox>/<country>/` before processing.
- `--colnect-country <COUNTRY>`
 Case-insensitive country filter for inbox processing.
- `--colnect-create-country-db`
 Create missing `Albums/<Country>/<Country>.db` during inbox ingestion.
- `--colnect-skip-duplicates`
 Skip files already imported by SHA256 checksum (default behavior).
- `--colnect-force-reimport`
 Reimport even if checksum already exists in import history.

### Built-in Utility

- `--help`
- `--version`

## Runtime Entry Points

- CLI entry: `src/main.rs`
- CLI argument model: `src/cli.rs`
- Program modes: `src/modes.rs`
- Crate manifest: `Cargo.toml`
- Default configuration template: `config_template.toml`
- Full configuration example: `config_full_example.toml`

## Repository Layout

- `src/` application code
- `crates/stamp_album_db/` shared DB crate
- `scripts/` utility scripts
- `docs/` all project documentation, reports, and test run notes
- `tests/` integration tests
- `samples/` sample/reference input files

## Documentation

- Main docs index: [docs/README.md](docs/README.md)
- Cross-platform/build index: [docs/DOCUMENTATION_INDEX.md](docs/DOCUMENTATION_INDEX.md)
- Development workflow: [DEVELOPMENT.md](DEVELOPMENT.md)
- Release history: [RELEASE_NOTES.md](RELEASE_NOTES.md)
- Contribution policy: [CONTRIBUTING.md](CONTRIBUTING.md)

### Top-Level Docs

- [docs/CLI_USAGE.md](docs/CLI_USAGE.md)
- [docs/ALBUM_STAMP_IMG_NEW_SETTING_PARAMETERS.md](docs/ALBUM_STAMP_IMG_NEW_SETTING_PARAMETERS.md)
- [docs/AUTOMATION_SETUP.md](docs/AUTOMATION_SETUP.md)
- [docs/AUTOMATION_QUICK_REFERENCE.md](docs/AUTOMATION_QUICK_REFERENCE.md)
- [docs/AUTOMATION_SYSTEM_DOCUMENTATION.md](docs/AUTOMATION_SYSTEM_DOCUMENTATION.md)
- [docs/CONFIG_REINTEGRATION_REPORT.md](docs/CONFIG_REINTEGRATION_REPORT.md)
- [docs/COPILOT_AGENT_INSTRUCTIONS.md](docs/COPILOT_AGENT_INSTRUCTIONS.md)
- [docs/REEXPORTS.md](docs/REEXPORTS.md)
- [docs/RELEASE_FLOW.md](docs/RELEASE_FLOW.md)
- [docs/migration_phase1_plan.md](docs/migration_phase1_plan.md)

### Guides

- [docs/guides/API_DOCUMENTATION.md](docs/guides/API_DOCUMENTATION.md)
- [docs/guides/ALBUMEASY_COMPATIBILITY.md](docs/guides/ALBUMEASY_COMPATIBILITY.md)
- [docs/guides/CLI_OPTIONS_INVENTORY.md](docs/guides/CLI_OPTIONS_INVENTORY.md)
- [docs/guides/COLNECT_DIMENSION_ADJUSTMENT.md](docs/guides/COLNECT_DIMENSION_ADJUSTMENT.md)
- [docs/guides/COLNECT_SEPARATE_ADJUSTMENTS.md](docs/guides/COLNECT_SEPARATE_ADJUSTMENTS.md)
- [docs/guides/COMMEMORATIVE_FORMAT_INTEGRATION.md](docs/guides/COMMEMORATIVE_FORMAT_INTEGRATION.md)
- [docs/guides/CONFIG_FORMAT_BOTH_TYPES.md](docs/guides/CONFIG_FORMAT_BOTH_TYPES.md)
- [docs/guides/CONFIG_PARAMETERS_INVENTORY.md](docs/guides/CONFIG_PARAMETERS_INVENTORY.md)
- [docs/guides/CROSS_PLATFORM_BUILD.md](docs/guides/CROSS_PLATFORM_BUILD.md)
- [docs/guides/FORMATTING.md](docs/guides/FORMATTING.md)
- [docs/guides/PROGRESS_INDICATOR_IMPLEMENTATION.md](docs/guides/PROGRESS_INDICATOR_IMPLEMENTATION.md)
- [docs/guides/QUICK_BUILD_COMMANDS.md](docs/guides/QUICK_BUILD_COMMANDS.md)
- [docs/guides/TASKS_COLNECT_INTEGRATION.md](docs/guides/TASKS_COLNECT_INTEGRATION.md)
- [docs/guides/WINDOWS_BUILD_CLARIFICATION.md](docs/guides/WINDOWS_BUILD_CLARIFICATION.md)

### Reports

- [docs/reports/AUDIT_COMPLETE.md](docs/reports/AUDIT_COMPLETE.md)
- [docs/reports/AUDIT_RESULTS_2025-11-29.md](docs/reports/AUDIT_RESULTS_2025-11-29.md)
- [docs/reports/BUILD_SUMMARY.md](docs/reports/BUILD_SUMMARY.md)
- [docs/reports/COLLECTOR_FIX_SUMMARY.md](docs/reports/COLLECTOR_FIX_SUMMARY.md)
- [docs/reports/CROSS_PLATFORM_STATUS.md](docs/reports/CROSS_PLATFORM_STATUS.md)
- [docs/reports/DATA_EXTRACTION_ANALYSIS.md](docs/reports/DATA_EXTRACTION_ANALYSIS.md)
- [docs/reports/LINKING_REPORT.md](docs/reports/LINKING_REPORT.md)
- [docs/reports/PLATFORM_FIXES_DETAILED.md](docs/reports/PLATFORM_FIXES_DETAILED.md)
- [docs/reports/PLATFORM_SPECIFIC_CODE_FIXES.md](docs/reports/PLATFORM_SPECIFIC_CODE_FIXES.md)
- [docs/reports/SESSION_SUMMARY.md](docs/reports/SESSION_SUMMARY.md)
- [docs/reports/TESTING_AND_DOCUMENTATION_PLAN.md](docs/reports/TESTING_AND_DOCUMENTATION_PLAN.md)
- [docs/reports/refactor_inventory.md](docs/reports/refactor_inventory.md)

### Testing Docs

- [docs/testing/CLI_TESTING_MATRIX.md](docs/testing/CLI_TESTING_MATRIX.md)
- [docs/testing/CONFIG_TESTING_MATRIX.md](docs/testing/CONFIG_TESTING_MATRIX.md)
- [docs/testing/1955_TEST_RESULTS.md](docs/testing/1955_TEST_RESULTS.md)
- [docs/testing/TEST_RUN_COLNECT_INBOX_2026-03-15.md](docs/testing/TEST_RUN_COLNECT_INBOX_2026-03-15.md)
- [docs/testing/TEST_SUMMARY.txt](docs/testing/TEST_SUMMARY.txt)
- [docs/testing/1955_COMMEMORATIVE_TEST_RESULTS.txt](docs/testing/1955_COMMEMORATIVE_TEST_RESULTS.txt)

### Versioned Backups and Reference Assets

- Versioned docs archive: [docs/versions/README.md](docs/versions/README.md)
- Full backup folder: [docs/versions/](docs/versions/)
- AlbumEasy manual (PDF): [docs/AlbumEasy_4.2.1.pdf](docs/AlbumEasy_4.2.1.pdf)
- US1847 catalog reference (PDF): [docs/U.S. Stamps Catalog and Identifier of classic and modern stamps 1847usa.pdf](docs/U.S.%20Stamps%20Catalog%20and%20Identifier%20of%20classic%20and%20modern%20stamps%201847usa.pdf)
- AlbumEasy reference files: [docs/easy/](docs/easy/)
- Command breakdown references: [docs/breakdown/](docs/breakdown/)

## License

This project is licensed under the **Laser Spider Stamp Collector License (LSSCL) v1.0**.

- Canonical license file: `LICENSE`
- Full named copy: `LASER_SPIDER_STAMP_COLLECTOR_LICENSE_v1.0.md`

Summary:

- Individual stamp collectors may use the software for personal use at no fee.
- Non-collector, business, corporate, institutional, or commercial use requires a separate written paid agreement with Laser Spider LLC.
