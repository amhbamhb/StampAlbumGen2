# Stamp Album Generator v0.9.1 Release Notes

## Overview
This release fixes critical page numbering issues and eliminates blank pages in album generation. The embedded reference stamp sizing system from v0.5.0 is maintained with improved reliability.

## 🚀 New Features

### Page Numbering System Fixes
- **Sequential page numbering**: Fixed inconsistent page numbering (1, 3, 5, 7, 8, 10, 13...) to proper sequential order (1, 2, 3, 4, 5...)
- **Eliminated blank pages**: Removed duplicate `PAGE_START` commands that created empty pages
- **Centralized page management**: Created helper functions for consistent page header writing and page creation
- **Improved album generation reliability**: All pages now contain proper content with no blank pages

### Embedded Reference System (Maintained from v0.5.0)
- **Embedded B4-s.jpg reference image** using `include_bytes!` macro
- Eliminates dependency on external reference files
- Consistent sizing across all countries and installations
- Automatic mm-per-pixel ratio calculation (0.208x0.107)

### Series-Specific Scaling Factors
- **L-series stamps**: 49.0mm x 21.0mm target (1.25x, 2.45x scaling)
- **M-series stamps**: 31.1mm x 24.1mm (1.15x, 1.45x scaling)
- **N-series stamps**: 26.1mm x 21.0mm (1.10x, 1.40x scaling)
- Precise control over stamp dimensions for optimal album layout

### Global Settings Integration
- Seamless integration with `ALBUM_STAMP_IMG_ASPECT_RATIO` settings
- Dynamic row layout that accounts for actual stamp widths
- Maintains 180mm page width constraints

## 🔧 Technical Improvements

### Code Architecture
- New `sample_images` module for centralized scaling logic
- Enhanced `image_utils.rs` with embedded reference processing
- Updated `album_globals.rs` with series-specific dimension calculations
- Modular design for easy maintenance and extension

### Performance Optimizations
- Release build optimizations for production deployment
- Embedded assets reduce I/O operations
- Efficient scaling factor calculations

### Build System
- Production-ready release binary (12MB)
- Cross-platform compatibility (Linux x86-64)
- Minimal runtime dependencies

## 📊 Stamp Dimension Results

| Series | Target Dimensions | Actual Dimensions | Scaling Factors |
|--------|------------------|-------------------|-----------------|
| L-series | 49.0mm x 21.0mm | 49.0mm x 21.0mm ✅ | (1.25, 2.45) |
| M-series | ~31mm x ~24mm | 31.1mm x 24.1mm ✅ | (1.15, 1.45) |
| N-series | ~26mm x ~21mm | 26.1mm x 21.0mm ✅ | (1.10, 1.40) |

## 🛠️ Installation & Usage

### Production Deployment
```bash
# Copy the portable binary to target system
cp target/release/stamp_album_gen2 stamp_album_gen2_portable

# Run on any compatible Linux system
./stamp_album_gen2_portable --help
```

### Dependencies
- SQLite3 library
- OpenSSL libraries
- Standard C libraries (usually pre-installed on Linux)

## 📁 File Structure
```
src/
├── sample_images/
│   ├── mod.rs              # Series-specific scaling system
│   ├── large_stamps/       # L-series reference images
│   ├── medium_stamps/      # M-series reference images
│   └── small_stamps/       # N-series reference images
├── album_globals.rs        # Main album generation logic
├── image_utils.rs          # Image processing utilities
├── B4-s.jpg               # Embedded reference image
└── [other modules...]
```

## 🔄 Migration Notes

### From Previous Versions
- No breaking changes to command-line interface
- Existing configuration files remain compatible
- Automatic fallback to embedded reference system

### Configuration
- All existing global settings work unchanged
- New embedded system provides consistent results
- No manual reference file management required

## 🐛 Bug Fixes
- Fixed oversized stamp dimensions hitting 50mm width limit
- Resolved inconsistent sizing across stamp series
- Eliminated external file dependency issues

## 📈 Performance Improvements
- Faster album generation with embedded assets
- Reduced I/O operations
- Optimized scaling calculations

## 🔮 Future Enhancements
- Additional stamp series support
- Configurable scaling factors per country
- Enhanced image processing options

## 📞 Support
For issues or questions, please check the embedded documentation in the source code or create an issue in the project repository.

---

**Release Date**: 2025-09-21
**Version**: 0.9.1
**Build**: Release (optimized)</content>
<parameter name="filePath">RELEASE_NOTES.md
