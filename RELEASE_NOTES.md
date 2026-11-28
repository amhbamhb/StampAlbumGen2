# Stamp Album Generator v1.15.12 Release Notes

## 🌟 What's New for Stamp Collectors

This release introduces comprehensive page layout, binding margin, and custom dimension enhancements for creating stamp album pages:

### 📐 Printer Crop Marks & Cutting Guides (`crop-marks-v1`)
- **Precise Trimming Guides**: Easily add crosshair crop marks to all four corners of your pages, making it simple to trim printed pages to exact album dimensions with a paper trimmer or guillotine cutter.
- **Custom Color Accents**: Set crop mark colors using hex codes (`#1E4A7A`), functional RGB, or named colors (`forestgreen`, `darkred`, etc.) to match page borders or stand out against cream/textured archival paper.
- **Flexible Styling**: All dimensions (hairline thickness, padding, arm lengths) are fully customizable or can be safely omitted for standard defaults.

### 📖 Binding Margins & Facing Pages (`margins-v1`)
- **Binder Clearance**: Set custom left, right, top, and bottom margins to provide ample clearance for 3-ring, 4-ring, spring-back, or screw-post binders so stamps and borders never get clipped.
- **Book-Binding / Facing Pages (`ALBUM_PAGES_MARGINSE` / `NATIVE_PAGES_MARGINSE`)**: Configure independent odd and even page margins. Right-hand (recto) pages expand the left binding clearance, while left-hand (verso) pages expand the right clearance automatically across page breaks.

### 📄 Page Dimensions & Orientations (`page-size-v1`)
- **Standard & International Formats**: Full support for North American **US Letter** (8.5 × 11 in), international ISO **A4** (210 × 297 mm), and custom millimeter dimensions for European specialty albums (Lindner, Schaubek, Davo).
- **Automatic Orientation**: Pages automatically render in Portrait (upright) or Landscape (horizontal) based on width and height without manual flags.
- **Mixed Orientations in a Single Album**: Seamlessly include both portrait and landscape pages within the same album—ideal for mounting wide souvenir sheets or postal history covers on dedicated landscape pages!

---

# Stamp Album Generator v1.15.12 Release Notes

## Overview

This release fixes critical page numbering issues and eliminates blank pages in album generation. The embedded reference stamp sizing system from v0.5.0 is maintained with improved reliability.

## 🚀 New Features

### Auto-Update Application System (Issue #124)

- **Review UI Update Notification Badge**: Background check against GitHub Releases (`amhbamhb/StampAlbumGen2`) displays a pulsing notification in the top navigation header whenever a newer version is available.
- **One-Click In-Browser Upgrade**: Clicking the notification opens an update modal showing version comparison (`v1.15.0 -> v1.X.X`), release highlights, and a one-click **"Update & Restart Now"** button.
- **Cryptographic Verification**: Automatically downloads the release binary, matches its SHA256 checksum against `.sha256` asset, and performs atomic self-replacement across Windows and Linux.
- **CLI Commands**: Added `--check-update` and `--update` flags for headless/terminal management.
- **Configurable**: Configured via `[updates].check_on_startup` (default: `true`).

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

**Release Date**: 2026-11-28
**Version**: 1.15.12
**Build**: Release (optimized)</content>

---

## Overview 1

This patch fixes a header aggregation bug and improves placeholder image sizing so that generated AlbumEasy output uses a representative real-image size when available (falls back to 22.0 x 25.0 mm otherwise). It also adds tests to prevent regressions and minor cleanup.

## Bug fixes & improvements

- Aggregate header fields (Design, Engraving, Watermark, Perforation) across all stamps in a year group instead of taking values only from the first stamp. This fixes missing header entries when present on later stamps in the group.
- Placeholder sizing: if a real image exists in the same series/year, placeholders now inherit that image's measured mm dimensions so page layout remains consistent. If no representative image is available, placeholders default to 22.0 × 25.0 mm.
- Added unit tests covering header aggregation and direct emission checks for `write_page_header` to ensure fields are written into AlbumEasy output.

- Page-fit robustness: added a new configuration key `page_fit_conservative_buffer_mm` (default 11.0 mm) to reserve extra vertical space at the bottom of each page to avoid footer overlap. Also added a regression test (`tests/page_fit_regression.rs`) to prevent regressions to page-fit behavior.

## Notes

- A small lint/cleanup pass remains (non-functional dead_code warnings) and can be addressed in a follow-up.

**Release Date**: 2026-11-28
**Version**: 1.15.12

<parameter name="filePath">RELEASE_NOTES.md
