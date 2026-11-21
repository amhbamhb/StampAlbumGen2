# StampAlbumGen2 User Manual & Command Catalog

**Document Path:** `docs/manual/`  
**Generated PDF Artifact:** `StampAlbumGenerator2HelpManual.pdf` (in repository root and `docs/manual/`)  
**Format:** US Letter (8.5 × 11 in / 215.9 × 279.4 mm)  
**Tracking Issue:** [Issue #123](https://github.com/amhbamhb/StampAlbumGen2/issues/123)

---

## 1. Overview & Architecture

The user manual is embedded directly into the compiled Rust binary:
```rust
// src/config.rs
const EMBEDDED_PDF_HELP: &[u8] = include_bytes!("../StampAlbumGenerator2HelpManual.pdf");
```
When `stamp_album_gen2` starts up, it automatically verifies and updates `StampAlbumGenerator2HelpManual.pdf` in the collector's working directory so that the latest manual is always accessible locally and through the Review UI (`/help`).

---

## 2. Dual Pipeline Architecture (Option A & Option B)

To give the project maintainers full flexibility, two authoring formats are maintained in parallel:

### Option A: Automated Markup Pipeline (Typst)
- **Source Directory:** `docs/manual/src/`
- **Master File:** [`docs/manual/src/manual.typ`](src/manual.typ)
- **Key Advantages:**
  - Written in clean, version-controlled plaintext.
  - Sub-second compilation into publication-quality vector PDF.
  - Automatically regenerates Table of Contents, page headers, footers, and section numbering.
  - Cross-platform one-click build scripts for Windows and Linux automatically compile and update the root embedded PDF.

#### Tool Installation (`typst`)
- **Windows:**
  ```powershell
  winget install --id Typst.Typst
  # or via Cargo (if Rust is installed):
  cargo install --locked typst-cli
  ```
- **Linux:**
  ```bash
  # Using Cargo (Universal for Rust developers):
  cargo install --locked typst-cli

  # Debian / Ubuntu (24.04+):
  sudo apt install typst

  # Arch Linux:
  sudo pacman -S typst

  # Snap:
  sudo snap install typst
  ```

#### Build Commands
- **Windows (PowerShell):**
  ```powershell
  # Compile manual and synchronize root binary copy:
  powershell -ExecutionPolicy Bypass -File docs/manual/build_manual.ps1

  # Compile and immediately open in default viewer:
  powershell -ExecutionPolicy Bypass -File docs/manual/build_manual.ps1 -Open
  ```
- **Linux (Bash):**
  ```bash
  # Ensure executable permission:
  chmod +x docs/manual/build_manual.sh

  # Compile manual and synchronize root binary copy:
  ./docs/manual/build_manual.sh

  # Compile and open with system viewer (xdg-open):
  ./docs/manual/build_manual.sh --open
  ```

### Option B: Desktop Publishing Layout (Scribus / InDesign)
- **Source Directory:** `docs/manual/sla/`
- **Source File:** `StampAlbumGenerator2HelpManual_v2.sla` (and `StampAlbumGenerator2HelpManual.designcraft`)
- **Key Advantages:**
  - Manual pixel-level visual placement of frames, fonts, and screenshots using GUI tools.
- **Workflow:**
  - Open `docs/manual/sla/StampAlbumGenerator2HelpManual_v2.sla` in Scribus (or InDesign).
  - Export final PDF directly to `StampAlbumGenerator2HelpManual.pdf` in the repository root.

---

## 3. Manual Content Structure

1. **Section 1: Introduction & System Architecture**
   - Philatelic mounting principles, dual-engine philosophy (AlbumEasy vs Native Vector `.stampalbum`).
2. **Section 2: Installation, Directory Layout & Configuration**
   - Startup behavior, directory inventory (`Albums/`, `Logs/`, `flags/`, `UserImport/`), `config.toml` parameters, CLI arguments.
3. **Section 3: Data Sources, Scrapers & Import Workflows**
   - StampWorld HTML scraping, Colnect CSV ingestion & catalog prefixes, US1847 scraper, manual offline CSVs.
4. **Section 4: Review UI Operator's Manual**
   - Detailed screen-by-screen walkthrough with reserved image callout frames for screenshots (Country Dashboard, Reconciliation, Zoom Inspector, Overrides, Add Colnect, Filters, Configuration Editor).
5. **Section 5: Vector Page Geometry, Coordinate Systems & Mount Standards**
   - PostScript point-to-mm conversions, MediaBox, alternating binding margins (`ALBUM_PAGES_MARGINSE`), mount tolerances.
6. **Section 6: Comprehensive Illustrated Command Catalog**
   - Complete coverage of all 43 command families from the website gallery across all 8 categories.
7. **Section 7: Diagnostics, Troubleshooting & Appendix**
   - Logging flags (`--pdf-dx-logging`), error message resolution, LSSCL v1.0 license summary.
