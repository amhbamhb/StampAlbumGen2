# Stamp Album Generation Process Instructions & Analysis
## Overview
This document summarizes the analysis performed on the configuration files, CLI usage guide, and manual to establish a reproducible workflow for testing album generation parameters. The goal is to systematically test how configuration settings affect the generated output, focusing on page sizing and data compatibility with AlbumEasy standards.

## Source Documents Used
- `config.toml`: Runtime configuration settings.
- `config_template.toml`: Baseline template structure.
- `/media/ray/program/Rust/StampAlbumGen2/docs/AlbumEasy_4.2.1.pdf`: The authoritative guide for AlbumEasy PDF output format and layout expectations.
- `docs/CLI_USAGE.md`: Reference for all available command-line options.

## Step-by-Step Execution & Findings

### 1. Test Focus Identification (Initial Request)
We analyzed the configuration files to identify tests related to `config.toml`. The primary focus areas identified are:
*   **Layout Parameters (`[page]` and `[layout]` sections):** These control physical dimensions, margins, spacing, and row packing, which directly impact PDF layout.
*   **Colnect Field Selection (`[colnect]` section):** This controls the level of metadata included in the final output.

### 2. Testing Parameter Variation (Core Request)
The requirement was to test parameter changes (e.g., `album_page_width_mm`) by observing command-line output and comparing generated `.txt` files, specifically for page size variations.

**Example Test Case: Page Size Variation**
*   **Parameter Tested:** `[page].album_page_width_mm`
    *   **Test 1 (Base):** Use value from `config.toml`.
    *   **Test 2 (Smaller):** Attempt to run the generator with a smaller width (e.g., testing against A4 or US Letter constraints).
    *   **Test 3 (Bigger):** Attempt to run the generator with a larger width.
*   **Validation:** The results must be compared against the layout rules defined in `AlbumEasy_4.2.1.pdf` to ensure physical dimensions and spacing are correctly translated into the PDF structure.

### 3. Specific Run Example (Addressing Step 8)
The specific test requested was:
*   **Target Album:** `/media/ray/program/Rust/StampAlbumGen2/Albums/Great-Britain`
*   **Date Test:** Use **1840** for the stamp year.

### 4. Output Compatibility Check (Addressing Step 3)
Before finalizing any output, it is critical to confirm data compatibility:
*   The content of the generated `.txt` file must be assessed against the structure defined in `/media/ray/program/Rust/StampAlbumGen2/docs/AlbumEasy_4.2.1.pdf`. This ensures that the text data emitted by the generator maps correctly to AlbumEasy's expectations for layout elements, headings, and footer information.

## Remaining Ambiguities & Clarifications
The core ambiguity remaining is **the exact command structure** required to execute these tests reliably. While `docs/CLI_USAGE.md` provides options, I need confirmation on the specific combination of flags needed when using `--source colnectexport` or when specifying output paths for testing.

**I require clarification on the execution flow before proceeding with the actual file manipulation and command execution.**
