# User Manual Image Update Workflow & Figure Inventory

This document tracks the progressive update of screenshot placeholders in the **Stamp Album Generator 2 Help Manual** (`StampAlbumGenerator2HelpManual.pdf`), written in Typst.

---

## 1. Overview & Architecture

The user manual is maintained as a modular Typst document in `docs/manual/src/`:

- **Master Entry Point:** [`docs/manual/src/manual.typ`](src/manual.typ)
- **Section Sources:** [`docs/manual/src/sections/`](src/sections/)
- **Image Directory:** [`docs/manual/src/images/`](src/images/)
- **Build Output:** `docs/manual/StampAlbumGenerator2HelpManual.pdf` and `./StampAlbumGenerator2HelpManual.pdf` (embedded in binary)

### The `ui_box` Helper Function

In [`docs/manual/src/sections/02_setup_and_config.typ`](src/sections/02_setup_and_config.typ), screenshots and placeholders use the unified `ui_box` helper:

```typst
#ui_box(
  "Figure Title",
  "Figure Caption text",
  image_path: "../images/filename.png", // Omit or set to none for placeholder box
  height_val: 160pt // Can be set smaller (e.g. 75pt) for compact mini-placeholders
)
```

- When `image_path` is specified, the image is rendered with a clean subtle frame, white background card, and centered caption.
- When `image_path: none`, it automatically displays an inviting dashed placeholder card. This allows updating images **one at a time** while keeping the rest of the manual looking polished.

---

## 2. Cross-Platform & Style Guidelines

1. **Platform-Neutral Descriptions:**
   - Avoid host-specific bang paths (e.g. `/media/ray/...`).
   - Use neutral, user-friendly examples that work across Windows, Linux, and macOS:
     - `Documents/StampAlbumGen2/Albums` (Linux/macOS)
     - `Documents\StampAlbumGen2\Albums` (Windows)
     - `Albums/Norway/Norway.db` (database relative location)
2. **User-Driven Language:**
   - Write from the perspective of a stamp collector arranging their collection.
   - Explain _why_ a feature helps (e.g. baseline configuration, mount breathing room, safe offline operation).
   - Use UI terminology matching what appears on screen (e.g. "Country dropdown" instead of "Quick Search").
3. **Proportional Image Sizing & Balanced Grids:**
   - Dialog modals (such as the software update and config migration dialogs) should be rendered proportionally at a reduced scale (~45–50% column width) alongside descriptive text using Typst grids, keeping pages clean and readable.
4. **Collapsible State Memory:**
   - Explicitly note that collapsible panels (such as _About this app & setup hints_, _Albums Root_, and _Import StampWorld_) remember whether they were left expanded or collapsed across user sessions.
5. **Safety, Optionality & Configuration Updates:**
   - Always emphasize that in-app software updates are completely optional.
   - When new features add keys to `config.toml`, explain that safe in-memory defaults are always retained, so updating configuration files is a helpful suggestion rather than a requirement.
   - Clarify that manual download links, SHA-256 cryptographic checksums, and Sigstore release attestations are available on GitHub for collectors who prefer manual verification.

---

## 3. Step-by-Step Procedure (One Image at a Time)

For each new screenshot to add:

### Step 1: Capture the Screenshot

- Open the application locally at `http://127.0.0.1:7878`.
- Capture the relevant window, dialog, or panel cleanly (recommended ~1000–1200 px wide).
- Format: PNG (lossless, crystal-clear text and buttons).

### Step 2: Save to the Images Directory

Copy or save the file into `docs/manual/src/images/` using standard descriptive naming:

```bash
cp /path/to/screenshot.png docs/manual/src/images/figX_Y_name.png
```

### Step 3: Wire into the Typst Section

Edit the corresponding section file in `docs/manual/src/sections/`:

```typst
#ui_box(
  "Figure X.Y: ...",
  "Figure X.Y: ...",
  image_path: "../images/figX_Y_name.png"
)
```

Add or refine the accompanying bullet points explaining every button, badge, and input field using user-driven language.

### Step 4: Recompile the PDF

- **Linux (Bash):**
  ```bash
  ./docs/manual/build_manual.sh
  ```
- **Windows (PowerShell):**
  ```powershell
  powershell -ExecutionPolicy Bypass -File docs/manual/build_manual.ps1
  ```

### Step 5: Visual Assessment (Confirm with User Before Commit)

Inspect `docs/manual/StampAlbumGenerator2HelpManual.pdf`:

- Verify page breaks: Does the figure and its explanatory text stay together naturally?
- Check image clarity and sizing: Are images scaled proportionally without overpowering the text?
- Always ask the user if the PDF layout is acceptable before creating a git commit.

---

## 4. Master Figure Inventory & Status

### Section 1: Welcome & Quick Start Guide

| Asset / Figure       | Image File                                                       | Description                                                                                                                                      |    Status    |
| :------------------- | :--------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------- | :----------: |
| **Desktop Launcher** | `fig1_desktop_icon_windows.png`<br>`fig1_desktop_icon_linux.png` | **Desktop Shortcuts:** Automatically generated launcher shortcuts on Windows (Start Menu & Desktop) and Linux (Desktop & Applications launcher). | ✅ Completed |

### Section 2: The Visual Review Workspace

| Figure          | Image File                                                   | Description                                                                                                                                                                                                                                                                                         |     Status     |
| :-------------- | :----------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------: |
| **Figure 2.1**  | `fig2_1_dashboard.png`                                       | **The Start Review Dashboard & Main Controls:** Country dropdown, Open Review, Refresh, Site Accounts (legacy), Edit Config (universal baseline defaults), Generate PDF Now, Import Colnect, Help Manual, Update notification badge, Logout, Count badge, and + Create New Country button.          |  ✅ Completed  |
| **Figure 2.2**  | `fig2_2_country_status.png`                                  | **Albums Root Directory & Country Status Card:** Collapsible master albums directory path, active country database path (`Albums/Norway/Norway.db`), Colnect files count, and candidate pool count.                                                                                                 |  ✅ Completed  |
| **Figure 2.3**  | `fig2_update_modal.png`                                      | **Software Update Available Notification & Release Notes (Side-by-Side):** Current vs Latest version badges, scrollable release notes ("What's New for Stamp Collectors"), "Full Release Notes ↗" GitHub link, Help Manual auto-sync indicator, and Remind Me Later / Update & Restart Now buttons. |  ✅ Completed  |
| **Figure 2.4**  | `fig2_update_progress.png`                                   | **In-Place Automatic Upgrade Progress:** "Initiating update download..." progress bar tracking download, SHA256 safety verification, and automatic application restart.                                                                                                                             |  ✅ Completed  |
| **Figure 2.5**  | `fig2_config_staleness_dialog.png`                           | **Configuration File Updates & Compatibility (`config.toml`):** Dialog displaying newly introduced settings with safe in-memory fallback defaults and user action choices.                                                                                                                          |  ✅ Completed  |
| **Figure 2.6**  | `fig2_buttons_after_update.png`<br>`fig2_version_footer.png` | **Version Verification (Toolbar & Footer):** Action toolbar after update (yellow update button removed) and persistent version indicator in the application page footer.                                                                                                                            |  ✅ Completed  |
| **Figure 2.7**  | `fig2_github_releases.png`                                   | **GitHub Release Assets & Cryptographic Checksums (Side-by-Side):** Precompiled binaries (`.exe` and Linux), standalone PDF manual, SHA-256 hash files, and Sigstore attestations.                                                                                                                  |  ✅ Completed  |
| _(Review UI)_   | `fig2_3_import_stampworld.png`                               | **Import StampWorld - PDF / HTML Files:** StampWorld PDF/HTML import tool (located directly inside the Review UI for precise album matching).                                                                                                                                                       |    ⏳ Queued    |
| _(Utility)_     | `fig2_folder_picker.png`                                     | **Native Directory Selection Dialog:** Folder picker window for choosing Albums root or import targets.                                                                                                                                                                                             |   ⏳ Queued    |
| **Figure 2.9**  | `fig2_review_panels.png`                                     | **Side-by-Side Stamp Review & Matching:** Left panel (album stamps in chronological order with values, mount box sizes) vs Right panel (candidate matches from online catalogs with photos, measurements, catalog codes).                                                                           | ⏳ Placeholder |
| **Figure 2.10** | `fig2_zoom_inspector.png`                                    | **High-Resolution Artwork Zoom:** Side-by-side magnified view inspecting fine engraving details, secret marks, color shades (e.g. carmine vs scarlet), and perforation counts.                                                                                                                      | ⏳ Placeholder |
| **Figure 2.11** | `fig2_mount_editor.png`                                      | **Stamp Dimensions & Mount Clearances:** Editing millimeter dimensions (width, height), mount clearances (e.g. +3 mm), and custom printed captions.                                                                                                                                                 | ⏳ Placeholder |
| **Figure 2.12** | `fig2_add_colnect.png`                                       | **Add Colnect As Stamp Dialog:** Adding an unlisted variety, souvenir sheet, or block directly into the album with one click.                                                                                                                                                                       | ⏳ Placeholder |
| **Figure 2.13** | `fig2_generate_pdf.png`                                      | **PDF Generation Toolbar & Filters:** Filtering by emission (Commemoratives, Definitives) and format (Souvenir Sheets, Blocks) with the one-click "Generate PDF Now" button.                                                                                                                        | ⏳ Placeholder |
| **Figure 2.14** | `fig2_settings_editor.png`                                   | **In-Browser Settings Editor:** Plain-English sliders and switches for album titles, default mount margins, colors, and layout rules.                                                                                                                                                               | ⏳ Placeholder |

---

## 5. Change Log

- **2026-11-27 (Part 2 - Software Updates, Desktop Shortcuts & Config Migration):**
  - Added user-friendly notice to Cover Page and Section 1 indicating that this manual is actively updated alongside Native-PDF engine features.
  - Added Desktop Launcher documentation and images (`fig1_desktop_icon_windows.png` & `fig1_desktop_icon_linux.png`) to Section 1.1 Quick Start.
  - Refactored Figure 2.3 (`fig2_update_modal.png`) into a proportional side-by-side grid, reducing image scale to ~45% width alongside text.
  - Added bullet point documenting the "Full Release Notes ↗" link opening GitHub release notes.
  - Moved **Configuration File Updates & Compatibility (`config.toml`)** right after **In-Place Automatic Upgrade & Restart**, and wired in the real screenshot `fig2_config_staleness_dialog.png` (Figure 2.5).
  - Documented safe in-memory defaults and the 3 user choices.
  - Recompiled and synchronized manual into `docs/manual/StampAlbumGenerator2HelpManual.pdf` and `./StampAlbumGenerator2HelpManual.pdf`.
- **2026-11-27 (Part 1 - Dashboard & Status):**
  - Upgraded `ui_box` helper in `02_setup_and_config.typ` to support optional `image_path`.
  - Added Figure 2.1 (`fig2_1_dashboard.png`) documenting all 11 dashboard controls, buttons, update badge, and baseline config behavior.
  - Added Figure 2.2 (`fig2_2_country_status.png`) documenting Albums Root master path and country database health statistics.
  - Converted all system paths to cross-platform neutral formats (`Documents/StampAlbumGen2/Albums`, `Albums/Norway/Norway.db`).
  - Created initial workflow tracker and pushed branch `feature/issue-161-help-manual-screenshots` to GitHub.
