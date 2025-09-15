# Stamp Album Generator v0.5.0

A high-performance Rust application for generating stamp album pages in AlbumEasy format from SQLite databases. Features embedded reference images and series-specific scaling for precise stamp dimension control.

## 🚀 Features

### Core Functionality
- **Database Processing**: Reads stamp data from SQLite databases
- **AlbumEasy Output**: Generates compatible album page layouts
- **Multi-Country Support**: Works with various stamp collections
- **Year-based Organization**: Automatically groups stamps by year

### Advanced Sizing System
- **Embedded Reference Images**: No external file dependencies
- **Series-Specific Scaling**: Precise control for L/M/N stamp series
- **Dynamic Layout**: Optimizes stamp arrangement within page constraints
- **Aspect Ratio Control**: Configurable dimension calculation modes

### Performance & Portability
- **Release Build**: Optimized 12MB binary for production use
- **Cross-Platform**: Linux x86-64 compatible
- **Minimal Dependencies**: Only requires standard system libraries

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
USAGE:
    stamp_album_gen2 [OPTIONS] --db-path <DB_PATH> --album-name <ALBUM_NAME>

OPTIONS:
    --db-path <DB_PATH>          Path to SQLite database file
    --album-name <ALBUM_NAME>    Name of the album to generate
    --start-year <START_YEAR>    Starting year for stamp selection [default: 1840]
    --end-year <END_YEAR>        Ending year for stamp selection [default: 2025]
    --output-only                Generate output without database queries (for testing)
    --help                       Print help information
    --version                    Print version information
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

### Series-Specific Scaling
The application automatically applies precise scaling factors based on stamp series:

| Series | Target Dimensions | Scaling Factors | Use Case |
|--------|------------------|-----------------|----------|
| **L-series** | 49.0mm x 21.0mm | (1.25, 2.45) | Large registration stamps |
| **M-series** | 31.1mm x 24.1mm | (1.15, 1.45) | Medium definitive stamps |
| **N-series** | 26.1mm x 21.0mm | (1.10, 1.40) | Small definitive stamps |

### Embedded Reference System
- Uses compiled-in B4-s.jpg reference image
- Consistent `0.208x0.107` mm-per-pixel ratio across all installations
- Eliminates external file dependencies
- Automatic fallback to default ratios if embedded image fails

## 🏗️ Project Structure

```
stamp-album-gen2/
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library exports
│   ├── album_globals.rs     # Core album generation logic
│   ├── image_utils.rs       # Image processing utilities
│   ├── sample_images/       # Series-specific scaling system
│   │   ├── mod.rs          # Scaling factor definitions
│   │   ├── large_stamps/   # L-series reference images
│   │   ├── medium_stamps/  # M-series reference images
│   │   └── small_stamps/   # N-series reference images
│   ├── B4-s.jpg           # Embedded reference image
│   └── [other modules...]
├── Albums/                 # Album databases and images
├── config.toml            # Configuration file
├── Cargo.toml             # Rust dependencies
└── target/release/        # Built binaries
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

### Dependencies
- **rusqlite**: SQLite database access
- **image**: Image processing and format support
- **reqwest**: HTTP client for data scraping
- **serde**: Serialization/deserialization
- **clap**: Command-line argument parsing

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

#### `calculate_stamp_dimensions()`
Calculates precise stamp dimensions using embedded reference and series-specific scaling.

#### `generate_album_output_from_stamps()`
Generates complete AlbumEasy output from a filtered stamp collection.

#### `get_scaling_for_series()`
Returns appropriate scaling factors based on stamp series identifier.

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

**Version**: 0.5.0
**Release Date**: September 11, 2025
**Build**: Release (optimized)
**Platform**: Linux x86-64</content>
<parameter name="filePath">/media/ray/program/Rust/StampAlbumGen2/README.md
