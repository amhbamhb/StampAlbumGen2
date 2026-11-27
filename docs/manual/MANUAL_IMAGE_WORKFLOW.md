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
  image_path: "../images/filename.png" // Omit or set to none for placeholder box
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
   - Explain *why* a feature helps (e.g. baseline configuration, mount breathing room, safe offline operation).
   - Use UI terminology matching what appears on screen (e.g. "Country dropdown" instead of "Quick Search").
3. **Collapsible State Memory:**
   - Explicitly note that collapsible panels (such as *About this app & setup hints* and *Albums Root*) remember whether they were left expanded or collapsed across user sessions.

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

### Step 5: Visual Assessment
Inspect `docs/manual/StampAlbumGenerator2HelpManual.pdf`:
- Verify page breaks: Does the figure and its explanatory text stay together naturally?
- Check image clarity: Are text labels, buttons, and badges easily readable at 100% zoom?
- Check spacing: Is vertical rhythm balanced between sections?

---

## 4. Master Figure Inventory & Status

### Section 2: The Visual Review Workspace

| Figure | Image File | Description | Status |
| :--- | :--- | :--- | :---: |
| **Figure 2.1** | `fig2_1_dashboard.png` | **The Start Review Dashboard & Main Controls:** Country dropdown, Open Review, Refresh, Site Accounts (legacy), Edit Config (universal baseline defaults), Generate PDF Now, Import Colnect, Help Manual, Update notification badge, Logout, Count badge, and + Create New Country button. | ✅ Completed |
| **Figure 2.2** | `fig2_2_country_status.png` | **Albums Root Directory & Country Status Card:** Collapsible master albums directory path, active country database path (`Norway.db`), Colnect files count, and candidate pool count. | ✅ Completed |
| **Figure 2.3** | `fig2_3_review_panels.png` | **Side-by-Side Stamp Review & Matching:** Left panel (album stamps in chronological order with values, mount box sizes) vs Right panel (candidate matches from online catalogs with photos, measurements, catalog codes). | ⏳ Placeholder |
| **Figure 2.4** | `fig2_4_zoom_inspector.png` | **High-Resolution Artwork Zoom:** Side-by-side magnified view inspecting fine engraving details, secret marks, color shades (e.g. carmine vs scarlet), and perforation counts. | ⏳ Placeholder |
| **Figure 2.5** | `fig2_5_mount_editor.png` | **Stamp Dimensions & Mount Clearances:** Editing millimeter dimensions (width, height), mount clearances (e.g. +3 mm), and custom printed captions. | ⏳ Placeholder |
| **Figure 2.6** | `fig2_6_add_colnect.png` | **Add Colnect As Stamp Dialog:** Adding an unlisted variety, souvenir sheet, or block directly into the album with one click. | ⏳ Placeholder |
| **Figure 2.7** | `fig2_7_generate_pdf.png` | **PDF Generation Toolbar & Filters:** Filtering by emission (Commemoratives, Definitives) and format (Souvenir Sheets, Blocks) with the one-click "Generate PDF Now" button. | ⏳ Placeholder |
| **Figure 2.8** | `fig2_8_settings_editor.png` | **In-Browser Settings Editor:** Plain-English sliders and switches for album titles, default mount margins, colors, and layout rules. | ⏳ Placeholder |
| **Figure 2.9** | `fig2_9_upgrade_modal.png` | **One-Click Update Notification & Modal:** Header update badge previewing release highlights, SHA256 safety verification, and automatic executable replacement. | ⏳ Placeholder |

---

## 5. Change Log

- **2026-11-27:**
  - Upgraded `ui_box` helper in `02_setup_and_config.typ` to support optional `image_path`.
  - Added Figure 2.1 (`fig2_1_dashboard.png`) documenting all 11 dashboard controls, buttons, update badge, and baseline config behavior.
  - Added Figure 2.2 (`fig2_2_country_status.png`) documenting Albums Root master path and country database health statistics.
  - Converted all system paths to cross-platform neutral formats (`Documents/StampAlbumGen2/Albums`, `Albums/Norway/Norway.db`).
  - Successfully recompiled manual into `docs/manual/StampAlbumGenerator2HelpManual.pdf` and `./StampAlbumGenerator2HelpManual.pdf`.
