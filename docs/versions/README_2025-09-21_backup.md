# Stamp Album Generator v0.9.0

<!-- Version: 0.9.0 | Last Updated: 2025-09-20 | Previous: docs/versions/README_2025-09-18_v1.md -->

A high-performance Rust application for generating stamp album pages in AlbumEasy format from SQLite databases. Features embedded reference images and series-specific scaling for precise stamp dimension control.

## 🚀 Features

### Core Functionality
- **Database Processing**: Reads stamp data from SQLite databases
- **AlbumEasy Output**: Generates compatible album page layouts
- **Multi-Country Support**: Works with various stamp collections
- **Year-based Organization**: Automatically groups stamps by year

### Advanced Sizing System
- **Embedded Reference Images**: No external file dependencies
- **Series-Specific Scaling**: Precise control stamp sizes
- **Dynamic Layout**: Optimizes stamp arrangement within page constraints
- **Aspect Ratio Control**: Configurable dimension calculation modes

### Performance & Portability
- **Release Build**: Optimized 12MB binary for production use
- **Cross-Platform**: Linux x86-64 compatible, windows


## 📦 Installation

### Option 1: Pre-built Release (Recommended)
```bash
# Copy the portable binary to your system
cp target/release/stamp_album_gen2 /usr/local/bin/stamp_album_gen2
chmod +x /usr/local/bin/stamp_album_gen2
```

### Option 2: Build from Source
```bash
# Clone the repository
git clone <repository-url>
cd stamp-album-gen2

# Build release version
cargo build --release

# The binary will be available at target/release/stamp_album_gen2
```

## 🎯 Usage

### Basic Album Generation
```bash
# Generate album for Canada stamps (1875-1876)
stamp_album_gen2 --db-path Albums/Canada/Canada.db --album-name Canada --start-year 1875 --end-year 1876

# Generate with output preview only
stamp_album_gen2 --output-only --db-path Albums/Canada/Canada.db --album-name Canada --start-year 1875 --end-year 1876
```

### Command Line Options
```
Stamp Album Generator: Scrape, filter, and output AlbumEasy files from stamp DBs.

Usage: stamp_album_gen2 [OPTIONS]

Options:
      --output-only
          Output-only mode: Generate AlbumEasy from existing DB (requires --db-path and --album-name)
      --country <COUNTRY>
          Country name (used as default for other parameters if not specified)
      --db-path <DB_PATH>
          Path to the SQLite database (required for --output-only mode)
      --album-name <ALBUM_NAME>
          Album name (required for album generation)
      --source <SOURCE>
          Source type: stampworld, touchstamps, extractfile, colnectexport [default: stampworld]
      --site-name <SITE_NAME>
          Site name for scraping (required for scraping mode, e.g. 'Great-Britain', 'Canada')
      --album-directory <ALBUM_DIRECTORY>
          Album directory (default: Albums/{album_name})
      --images-directory <IMAGES_DIRECTORY>
          Images directory (default: Albums/{album_name}/images)
      --album-file-name <ALBUM_FILE_NAME>
          Album file name (default: Albums/{album_name}/{album_name}.txt)
      --extract-file-name <EXTRACT_FILE_NAME>
          Extract file name (default: Albums/{album_name}/{album_name}_extract.txt)
      --catalog-name <CATALOG_NAME>
          Catalog name (e.g. 'SG', 'Michel') [default: SG]
      --overwrite-album
          Overwrite existing album text files
      --overwrite-images
          Overwrite existing image files
      --start-year <START_YEAR>
          Start year for filtering stamps [default: 0]
      --end-year <END_YEAR>
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
reference_width_mm = 200.0
reference_height_mm = 240.0

# Global album settings
stamp_box_height = 30.0
row_font_spacing = 2.0
header_font_spacing = 3.0
```

## 📊 Stamp Dimension System


### Embedded Reference System
- Uses compiled-in B4-s.jpg reference image
- Eliminates external file dependencies
- Automatic fallback to default ratios if embedded image fails

## 🏗️ Project Structure

```
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
# Bump patch version (0.7.0 -> 0.7.1)
./version_docs.sh patch

# Bump minor version (0.7.0 -> 0.8.0)
./version_docs.sh minor

# Bump major version (0.7.0 -> 1.0.0)
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

**"Could not load embedded reference image"**
- This is a warning, not an error
- The system will use default scaling ratios
- Check that the binary was built correctly

**"Database not found"**
- Verify the `--db-path` points to a valid SQLite database
- Ensure the database file has proper permissions

**"Image dimensions not found"**
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

**Version**: 0.9.0
**Release Date**: 2025-09-20
**Build**: Release (optimized)
**Platform**: Linux x86-64</content>
<parameter name="filePath">/media/ray/program/Rust/StampAlbumGen2/README.md
