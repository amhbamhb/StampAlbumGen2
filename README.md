# Stamp Album Generator

Rust CLI that generates AlbumEasy pages from stamp data sources and SQLite databases.

## Start Here

Build and run the main program:

```bash
cargo build --release
cargo run -- --db-path Albums/Canada/Canada.db --album-name Canada
```

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

- User and project docs index: `docs/README.md`
- Development workflow and contributor guide: `DEVELOPMENT.md`
- Release history: `RELEASE_NOTES.md`
- Contribution policy: `CONTRIBUTING.md`

## License

This project is licensed under the **Laser Spider Stamp Collector License (LSSCL) v1.0**.

- Canonical license file: `LICENSE`
- Full named copy: `LASER_SPIDER_STAMP_COLLECTOR_LICENSE_v1.0.md`

Summary:

- Individual stamp collectors may use the software for personal use at no fee.
- Non-collector, business, corporate, institutional, or commercial use requires a separate written paid agreement with Laser Spider LLC.
