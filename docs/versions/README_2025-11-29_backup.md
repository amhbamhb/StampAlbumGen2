# Stamp Album Generator v1.3.0



A Rust tool to generate AlbumEasy pages from stamp databases.

## Features


 The generator writes lines like:
 
 ```text
 STAMP_HEADING(HN 10 "SG : 1234")
 ```
cd stamp-album-gen2
cargo build --release
```

## Quick Usage

```bash
# Generate album from an existing DB (output-only behavior when --db-path is present)
stamp_album_gen2 --db-path Albums/Canada/Canada.db --album-name Canada
```

## Support / Donations

If you find this project useful, please consider donating. See the `LICENSE` file for guidance or add a sponsorship link to the repository.

## Development

```bash
# Run tests
cargo test
```

For detailed documentation, see the `docs/` directory (automation setup, page-fit tuning, and advanced configuration).
# Stamp Album Generator v1.3.0

Version: 1.3.0 | Last Updated: 2025-11-29

A high-performance Rust application for generating stamp album pages in AlbumEasy format from SQLite databases. Features embedded reference images and series-specific scaling for precise stamp dimension control. Now fully cross-platform compatible for Linux, Windows, and macOS.

## 🚀 Features

### Core Functionality

- **Database Processing**: Reads stamp data from SQLite databases
- **AlbumEasy Output**: Generates compatible album page layouts
- **Multi-Country Support**: Works with various stamp collections
- **Year-based Organization**: Automatically groups stamps by year

### Advanced Sizing System

- **Embedded Reference Images**: No external file dependencies
- **Series-Specific Scaling**: Precise control of stamp sizes
- **Dynamic Layout**: Optimizes stamp arrangement within page constraints
- **Aspect Ratio Control**: Configurable dimension calculation modes

### Performance & Portability

- **Release Build**: Optimized 12MB binary for production use
- **Cross-Platform**: Linux x86-64 compatible

## 📦 Installation

# Generate from an existing DB (output-only behavior is triggered by passing --db-path)
# Stamp Album Generator v1.3.0

**Version**: 1.3.0
**Release Date**: 2025-11-15

A compact, up-to-date README for development and contributors. This file
focuses on practical usage, configuration pointers, and best practices for
commits and releases.

## Features

- Generate AlbumEasy pages from SQLite DBs
- Conservative page-fit logic to avoid footer overlap
- Embedded reference images and series scaling
- Configurable via `config.toml` and `config_full_example.toml`

## Quick start

Build and generate an album from an existing DB:

```bash
git clone REPOSITORY_URL
cd stamp-album-gen2
cargo build --release
stamp_album_gen2 --db-path Albums/Canada/Canada.db --album-name Canada
```

Filter by years:

```bash
stamp_album_gen2 --db-path Albums/Canada/Canada.db --album-name Canada --start-year 1875 --end-year 1876
```

## Configuration

- Primary file: `config.toml` (local, not committed)
- Example config: `config_full_example.toml` (repository root)

Key areas:
- Page size, margins, borders
- Row and header spacing
- Image sizing and aspect-ratio handling
- Header toggles (design, watermark, perforation, catalog numbers, series, country, date)
- Flag cover rendering hints (keys prefixed with `flag_cover_`)
- Series ordering: `series_order` (options: "first_date", "first_sg", "alpha"; append `_desc` for descending)
- StampWorld grouped-range handling: `expand_grouped_ranges` (options: "placeholders", "detail_fetch")

For a minimal starting point use `config_template.toml`. When experimenting
with more advanced options prefer `config_full_example.toml` which contains
the superset of keys used by the project and example values.

### Placeholder overwrite behavior

- `placeholder_always_overwrite` (bool, default: true) — Controls how the
    generator treats placeholder images when exact mm -> px mappings are
    available. When true, placeholders generated with explicit mm/mm_per_px
    mappings are recreated/overwritten so their pixel dimensions exactly match
    the emitted `STAMP_ADD_IMG` box sizes. When false, the generator will
    preserve existing placeholder files if their pixel dimensions already match
    the desired size. See `config_full_example.toml` for an example entry.

New configuration groups (added in recent releases):

- album_pages_*: tune AlbumEasy header/footer output. Example keys:
    - `album_pages_title_font`, `album_pages_title_size`, `album_pages_header_position`,
        `album_pages_footer_number_enabled`.

- row_start_es_*: small layout tuning for the optional ROW_START_ES lines
    (font, size, spacing, vertical spacing). Keys: `row_start_es_enabled`,
    `row_start_es_font`, `row_start_es_size`, `row_start_es_spacing`,
    `row_start_es_vspace`.

- flag_cover_*: per-flag cover rendering hints used when `print_flag_cover`
    is enabled. These are intentionally conservative and optional; the
    generator will fall back to previous defaults when a key is missing. See
    `config_template.toml` for example values such as `flag_cover_img_w_mm` and
    `flag_cover_image_title_size`.

For page-generation marker customization you can set the following keys in
`config.toml` to control the emitted diagnostic markers used during album
generation. These are primarily used by tests and downstream post-processors
to locate headers and rows in the emitted AlbumEasy text output:

- `page_vspace_marker` (string) — emitted as the vspace marker around headers, default reproduces historic `(6)` marker.
-- `row_start_es_marker` (string) — emitted inside `ROW_START_ES` parentheses, default reproduces `HN 6 0.1 5.0`.

New: stamp heading formatting

- `stamp_heading_font` (string) — AlbumEasy font code used for the per-stamp heading (default: "HN").
- `stamp_heading_size` (integer/float) — Numeric size used in the `STAMP_HEADING` token (default: 10).
- `stamp_heading_label` (string) — Short label shown before the SG number (default: "SG").

These control the small heading emitted immediately after each `STAMP_ADD` or `STAMP_ADD_IMG` line. The generator writes lines like:

```
STAMP_HEADING(HN 10 "SG : 1234")
```

You can customize the font/size/label in your `config.toml` to match your AlbumEasy templates or parser expectations. Example values are present in `config_full_example.toml`.

Example:

```toml
page_vspace_marker = "(6)"
row_start_es_marker = "TN 6 0.1 5.0"
```

### PAGE_TEXT templates

You can customize how the generator emits AlbumEasy `PAGE_TEXT` commands by
providing template strings in `config.toml`. Each template must include a
single `{}` placeholder which will be replaced with the text you want to
output. When omitted the generator falls back to sensible historical defaults.

Available keys:

- `page_text_format` — template used for left-aligned `PAGE_TEXT` lines.
- `page_text_centre_format` — template used for centred `PAGE_TEXT_CENTRE` lines.
- `page_text_right_format` — template used for right-aligned `PAGE_TEXT_RIGHT` lines.

Per-field tuning: instead of supplying a full template string you can set
the individual font/size/vspace values and the generator will synthesize the
template for you. These keys are optional and have sensible defaults:

- `page_text_font`, `page_text_size`, `page_text_vspace`
- `page_text_centre_font`, `page_text_centre_size`, `page_text_centre_vspace`
- `page_text_right_font`, `page_text_right_size`, `page_text_right_vspace`

If both a full-format template and per-field values are present, the full
template (`page_text_*_format`) takes precedence.

Example usage in `config.toml`:

```toml
page_text_format = 'PAGE_TEXT (HN 10 "{}" 1.0)'
page_text_centre_format = 'PAGE_TEXT_CENTRE (TN 8 "{}" 1.0)'
page_text_right_format = 'PAGE_TEXT_RIGHT (HN 10 "{}" 1.0)'
```

Notes:

- Provide exactly one `{}` placeholder in each template — it will be replaced
    with the escaped text content.
- You can include an optional vspace value in the template if you want to
    control vertical spacing after the text, for example:

```toml
page_text_format = 'PAGE_TEXT (HN 10 "{}" 0.5)'
```

This is useful when composing multi-line headers where lines use different
fonts or spacing and you want precise control over the resulting paragraph
spacing.


### Flags lookup and overrides

- The repository includes a comprehensive `flags/flag_codes.json` mapping
    (country name -> alpha-2) that is now embedded in the binary. This means
    the program can resolve country names to flag codes even when run from a
    minimal working directory.
- When `flags/flag_codes.json` is not present on disk the binary will use the
    embedded mapping and will attempt to write a copy to a `flags/flag_codes.json`
    file in a sensible location (current working dir `flags/` preferred). This
    makes subsequent runs or other tools pick up the same mapping file.
- To override where flags are read/written, set the environment variable
    `STAMP_FLAGS_DIR` to a directory that contains (or should contain) the
    `flag_codes.json` and any pre-downloaded flag images. Example:

```bash
# Use custom flags directory (absolute or relative)
export STAMP_FLAGS_DIR=/path/to/my/flags
stamp_album_gen2 --source stampworld --site-name Switzerland --album-name Switzerland
```

If you prefer the embedded mapping to be used only in-memory (no write-back),
let me know and I can add an opt-in config or environment flag to disable the
automatic write-back behavior.

## Development & Best Practices

### Running tests

```bash
cargo test --all
```

### Commit and release guidance

- Use Conventional Commits:
    - `feat:` → minor bump
    - `fix:` → patch bump
    - Use `BREAKING CHANGE:` for major bumps
- Use `version_docs.sh` to keep `Cargo.toml`, `README.md`, `.github/instructions`, and `RELEASE_NOTES.md` in sync.
    - CI-friendly: `./version_docs.sh auto-patch --git --push -y` will run `cargo build` and bump patch on success, create a commit and tag, and push.

### Local developer notes

- Use `config_full_example.toml` when testing feature toggles.
- For repeatable scraping, use pre-populated DBs in `Albums/`.

## Project layout (high level)

```
stamp-album-gen2/
├── src/                # Rust sources
├── Albums/             # Album DBs and images
├── docs/               # Documentation
├── tests/              # Tests
├── config_full_example.toml
├── config.toml         # local config (usually ignored in VCS)
├── README.md
├── RELEASE_NOTES.md
└── Cargo.toml
```

## Contributing
For the authoritative and up-to-date CLI option list, see `docs/CLI_USAGE.md` which is generated from the program's clap help output.

To refresh the CLI docs locally:

```bash
cargo run -- --help > docs/CLI_USAGE.md
```

If you'd like, I can also consolidate the README to include only a short usage snippet and link to `docs/CLI_USAGE.md` for the full option list.
├── Albums/                        # Album databases and images
├── Archive/                       # Archived source files
├── docs/                          # Documentation files
├── examples/                      # Example stamp collections
├── images/                        # Image assets
├── tests/                         # Test files
├── target/                        # Build artifacts
├── API_DOCUMENTATION.md           # API documentation
├── CONTRIBUTING.md                # Contributing guidelines
├── RELEASE_NOTES.md               # Release notes
├── Cargo.toml                     # Rust dependencies
├── Cargo.lock                     # Dependency lock file
├── config.toml                    # Configuration file
├── config_template.toml           # Configuration template
└── *.log                          # Log files
```

**Version**: 1.3.0
**Release Date**: 2025-11-15


## 🔧 Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test
```

### Automated Version Management

This project uses automated version management with conventional commits and GitHub Actions for seamless releases.

### Conventional Commits

Use conventional commit format for automatic version bumping:

```bash
# Feature commits (minor version bump)
git commit -m "feat: add new stamp filtering options"

# Bug fixes (patch version bump)
git commit -m "fix: resolve image download timeout issue"
```

## The binary will be available at target/release/stamp_album_gen2

```bash

## 🎯 Usage

### Basic Album Generation

```bash
# Generate album for Canada stamps (1875-1876)
stamp_album_gen2 --db-path Albums/Canada/Canada.db --album-name Canada --start-year 1875 --end-year 1876

# Generate from an existing DB (output-only behavior is triggered by passing --db-path)
stamp_album_gen2 --db-path Albums/Canada/Canada.db --album-name Canada --start-year 1875 --end-year 1876
```

## Command Line Options

```text
Stamp Album Generator: Scrape, filter, and output AlbumEasy files from stamp DBs.

Usage: stamp_album_gen2 [OPTIONS]

Options:
      --db-path  DB_PATH 
          Path to the SQLite database. When provided the program runs in output-only mode and will generate AlbumEasy files from the existing DB (skips scraping).
      --country  COUNTRY 
          Country name (used as default for other parameters if not specified)
      --album-name  ALBUM_NAME 
          Album name (required for album generation)
      --source  SOURCE 
          Source type: stampworld, touchstamps, extractfile, colnectexport [default: stampworld]
      --site-name  SITE_NAME 
          Site name for scraping (required for scraping mode, e.g. 'Great-Britain', 'Canada')
      --album-directory  ALBUM_DIRECTORY 
          Album directory (default: Albums/{album_name})
      --images-directory  IMAGES_DIRECTORY 
          Images directory (default: Albums/{album_name}/images)
      --album-file-name  ALBUM_FILE_NAME 
          Album file name (default: Albums/{album_name}/{album_name}.txt)
      --extract-file-name  EXTRACT_FILE_NAME 
          Extract file name (default: Albums/{album_name}/{album_name}_extract.txt)
      --catalog-name  CATALOG_NAME 
          Catalog name (e.g. 'SG', 'Michel') [default: SG]
      --overwrite-album
          When set, overwrite existing album text files in place. When omitted (the default), if the
          target album file already exists the generator will create a new file and append a datetime
          suffix before the extension (for example: `Album.created-YYYYMMDD-HHMMSS.txt`) so the
          original is preserved.
      --overwrite-images
          Overwrite existing image files
      --start-year  START_YEAR 
          Start year for filtering stamps [default: 0]
      --end-year  END_YEAR 
          End year for filtering stamps [default: 9999]
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version
```

### Configuration

Edit `config.toml` to customize behavior:

```toml
# Stamp dimension settings
reference_width_mm = 25.0
reference_height_mm = 30.0

# Global album settings
stamp_box_height = 30.0
row_font_spacing = 2.0
header_font_spacing = 3.0
etc...
```

Note: There is a short guide about page-fit tuning (the `page_fit_conservative_buffer_mm` option and how to avoid footer overlap) in `docs/AUTOMATION_QUICK_REFERENCE.md`.

### Flag cover and integration tests

The generator can optionally emit a large cover page that includes a country title and a flag image.
Control this behavior via `config.toml` using the `print_flag_cover` option (defaults to true when omitted):

```toml
# print_flag_cover = true  # set to false to skip the flag/title cover
```

There is also an optional network integration test that verifies downloading a small flag from FlagCDN.
To avoid running network tests in CI or offline environments this test is ignored by default and
also requires an explicit environment variable to run.

Run the integration test locally with:

FLAGDB_INTEGRATION=1 cargo test -- --ignored

This will execute just the ignored integration tests and will only perform network access when
`FLAGDB_INTEGRATION=1` is set.

## 📊 Stamp Dimension System

### Embedded Reference System

- Uses compiled-in B4-s.jpg reference image
- Eliminates external file dependencies
- Automatic fallback to default ratios if embedded image fails

## 🏗️ Project Structure

```text
stamp-album-gen2/
├── src/
│   ├── main.rs                    # Application entry point
│   ├── lib.rs                     # Library exports
│   ├── album_globals.rs           # Core album generation logic
│   ├── image_utils.rs             # Image processing utilities
│   ├── image_downloader.rs        # Image downloading functionality
│   ├── db_utils.rs                # Database utilities
│   ├── file_utils.rs              # File handling utilities
│   ├── config.rs                  # Configuration management
│   ├── types.rs                   # Type definitions
│   ├── commands_file_generator.rs # Command file generation
│   ├── placeholder_image.rs       # Placeholder image handling
│   ├── B4-s.jpg                  # Embedded reference image
│   ├── data_sources/
│   │   └── mod.rs                # Data source modules
│   └── stampworld/
├── Albums/                        # Album databases and images
│   ├── Canada/                    # Canada stamp collection
│   ├── Great Britain/             # Great Britain stamp collection
│   └── Great-Britain/             # Alternative Great Britain collection
├── Archive/                       # Archived source files
├── docs/                          # Documentation files
├── examples/                      # Example stamp collections
├── images/                        # Image assets
├── tests/                         # Test files
├── target/                        # Build artifacts
├── API_DOCUMENTATION.md           # API documentation
├── CONTRIBUTING.md                # Contributing guidelines
├── RELEASE_NOTES.md               # Release notes
├── Cargo.toml                     # Rust dependencies
├── Cargo.lock                     # Dependency lock file
├── config.toml                    # Configuration file
├── config_template.toml           # Configuration template
└── *.log                          # Log files
```

## 🔧 Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test
```

### Automated Version Management

This project uses automated version management with conventional commits and GitHub Actions for seamless releases.

#### Conventional Commits

Use conventional commit format for automatic version bumping:

```bash
# Feature commits (minor version bump)
git commit -m "feat: add new stamp filtering options"

# Bug fixes (patch version bump)
git commit -m "fix: resolve image download timeout issue"

# Breaking changes (major version bump)
git commit -m "feat!: redesign album layout system"

# Other types: docs, style, refactor, perf, test, chore, ci, build, revert
```

#### Quick Commit Helper

Use the interactive commit helper for easy conventional commits:

```bash
./conventional-commit.sh
```

## Slug overrides (stampworld)

If StampWorld uses a canonical slug that doesn't match the `--site-name` you pass, you can create a `slug_overrides.toml` in the project root to map your input to the canonical slug. The loader will search upward from the current working directory (like `config.toml`).

Example `slug_overrides.toml`:

```toml
[slug_overrides]
"British-ColumbiaIsland" = "British-Columbia!Vancouver-Island"
```

When present, this mapping is used before any automatic heuristic fallback. This is the preferred long-term approach for site names that consistently require a different slug.

## Configuration keys (full list)

Below are the configuration keys recognized in `config.toml` (top-level or some inside `[header_formatting]`):

- reference_width_mm: f32 (optional)
- reference_height_mm: f32 (optional)
- stamp_box_height: f32 (optional)
- stamp_box_adjust: f32 (optional)
- median_width_cap_3: f32 (optional)
- median_width_cap_4: f32 (optional)
- row_font_spacing: f32 (optional)
- global_scale: f32 (optional)
- header_font_spacing: f32 (optional)
- header_formatting: table (optional)
  - date_format: string ("dashes" or "commas")
  - label_style: string ("abbreviated" or "full")
  - series_style: string ("title", "subtitle" or "none")
  - include_currency: bool
  - include_design: bool
  - include_engraving: bool
  - include_watermark: bool
  - include_perforation: bool
  - include_catalog_numbers: bool
  - include_series: bool
  - include_country: bool
  - include_date: bool
  - print_flag_cover: bool
  - header_spacing: float
- calibration_csv: string (optional)
- max_rows_per_page: integer (optional)
- include_design/include_engraving/include_watermark/include_perforation/include_catalog_numbers/include_series/include_country/include_date: booleans (top-level alternative placement supported)
- print_flag_cover: bool (top-level or under header_formatting)

The loader will propagate values placed inside `[header_formatting]` up to the top-level fields for backwards compatibility.

## CLI options (complete)

The CLI supports these flags and options (more verbose listing):

- --source  stampworld|touchstamps|extractfile|colnectexport  (default: stampworld)
- --site-name  SITE_NAME  (required for scraping)
- --album-name  ALBUM_NAME 
- --db-path  DB_PATH  (required in output-only mode)
- --base-url  URL  (optional: override automatic StampWorld URL construction)
- --start-year  START_YEAR  (default 0)
- --end-year  END_YEAR  (default 9999)
- --overwrite-album (boolean)
- --overwrite-images (boolean)
- --max-pages  N  (0 = unlimited)
- --stop-after-no_hits  N  (default 3)

When scraping StampWorld, the program will by default percent-encode common problematic characters (space -  `%20`, `!` -  `%21`, `/` -  `%2F`). It also consults `slug_overrides.toml` and will attempt a small deterministic fallback (insert `!` before first or last hyphen) if the constructed listing returns a 404. This behavior is logged at info level.

#### Git Hooks

Automatic git hooks provide guidance for conventional commits:

- **prepare-commit-msg**: Suggests version bump types based on your changes
- **post-commit**: Detects conventional commits and recommends next steps

#### GitHub Actions Workflow

The `.github/workflows/release.yml` provides:

- **Automatic Releases**: Conventional commits trigger version bumps and releases
- **Version Consistency**: Updates all version references across the project
- **File Updates**: Automatically updates README.md, Cargo.toml, and other files
- **Release Notes**: Generates comprehensive release notes

#### GitLab CI/CD Pipeline (Alternative)

For GitLab repositories, use `.gitlab-ci.yml` instead:

- **Same Automation**: Conventional commits trigger automated releases
- **GitLab Releases**: Creates GitLab releases with proper versioning
- **Access Token**: Requires `GITLAB_ACCESS_TOKEN` with API and repository permissions

**GitLab Setup Requirements:**

1. Create a Personal Access Token with `api` and `write_repository` scopes
2. Add `GITLAB_ACCESS_TOKEN` as a CI/CD variable in your GitLab project
3. The pipeline will automatically trigger on conventional commits to main/master

#### Manual Version Management

For manual version control, use the version management script:

```bash
# Bump patch version (0.7.0 -  0.7.1)
./version_docs.sh patch

# Bump minor version (0.7.0 -  0.8.0)
./version_docs.sh minor

# Bump major version (0.7.0 -  1.0.0)
./version_docs.sh major

# Set specific version
./version_docs.sh 1.2.3
```

### Dependencies

- **rusqlite**: SQLite database access
- **image**: Image processing and format support (jpg features)
- **reqwest**: HTTP client for data scraping (blocking + JSON features)
- **serde**: Serialization/deserialization
- **clap**: Command-line argument parsing (derive features)
- **csv**: CSV file handling
- **rusttype**: Font rendering for text processing
- **scraper**: HTML scraping and parsing
- **lazy_static**: Lazy static initialization
- **regex**: Regular expression matching
- **log**: Logging framework
- **env_logger**: Environment-based logging
- **flexi_logger**: Flexible logging configuration
- **indicatif**: Progress bars and indicators
- **toml**: TOML configuration file parsing

## 📋 System Requirements

### Minimum Requirements

- **OS**: Linux x86-64
- **RAM**: 256MB
- **Storage**: 50MB (plus album data)
- **Dependencies**: SQLite3, OpenSSL, standard C libraries

### Recommended Setup

- **RAM**: 512MB or more
- **Storage**: 1GB+ for large stamp collections
- **CPU**: Multi-core processor for faster processing

## 🐛 Troubleshooting

### Common Issues

#### "Could not load embedded reference image"

- This is a warning, not an error
- The system will use default scaling ratios
- Check that the binary was built correctly

#### "Database not found"#### 

- Verify the `--db-path` points to a valid SQLite database
- Ensure the database file has proper permissions

#### "Image dimensions not found"#### 

- Check that stamp images exist in `Albums/{album}/images/`
- Verify image files are not corrupted

### Debug Mode

Run with debug logging:

```bash
RUST_LOG=debug stamp_album_gen2 [options]
```

## 📖 API Documentation

### Core Functions

#### `generate_album_output_from_stamps(stamps, album_name, page_callback)`

Generates complete AlbumEasy output from a filtered list of stamps. Takes pre-filtered stamp data and creates optimized album page layouts with proper spacing and year-based organization.

#### `calc_mm_per_pixel_embedded(real_width_mm, real_height_mm)`

Calculates precise mm-per-pixel ratios using the embedded B4-s.jpg reference image. Eliminates external file dependencies and ensures consistent dimension calculations across all installations.

#### `get_scaling_for_series(series)`

Returns appropriate scaling factors based on stamp series identifier (L/M/N/R/S/T/U series). Each series has optimized scaling factors for precise stamp dimension control in album layouts.

#### `load_stamps_from_db(db_path)`

Loads and filters stamp data from SQLite databases with support for year-based filtering and series-specific processing. Handles database connections and stamp data extraction.

#### `download_stamp_image_if_missing(album_name, image_path, base_url)`

Downloads missing stamp images automatically from configured sources. Supports multiple image formats and handles network errors gracefully with retry logic.

#### `extract_header_info(stamps)`

Extracts page header information from stamp collections including year ranges, series information, and layout metadata for AlbumEasy format generation.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Built with Rust for performance and reliability
- Uses AlbumEasy format for stamp album compatibility
- Inspired by various stamp collection management systems

---

#### Version#### : 0.10.0
  
Additional CLI flags (added recently):

  --base-url  BASE_URL 
          Optional override for the base URL used when scraping web sources that
          paginate by appending a page number. Example: "https://...&page="

  --max-pages  MAX_PAGES 
          Optional explicit limit on number of pages to fetch when scraping.
          Use 0 to disable the explicit cap and rely on internal heuristics.

  --stop-after-no-hits  N 
          Stop scraping after N consecutive pages that yield zero stamps in the
          requested year range. Useful to avoid long, multi-year listings from
      causing excessive downloads. Default is 1.

Example: controlled scrape for Bahamas (1859-1863) using an explicit base URL
and stop-after-no-hits heuristic to avoid over-downloading:

```bash
./target/release/stamp_album_gen2 \
    --source stampworld \
    --site-name Bahamas \
    --album-name Bahamas \
    --base-url "https://www.stampworld.com/en/stamps/Bahamas/Postage%20stamps/1859-2024?user=676374&page=" \
    --start-year 1859 --end-year 1863 \
    --stop-after-no-hits 3 --max-pages 0
```

**Release Date**: 2025-11-15
**Build**: Release (optimized)
**Platform**: Linux x86-64 /content 
`filePath`: README.md
