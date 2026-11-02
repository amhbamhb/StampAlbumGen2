# StampWorld PDF Import Feature Plan

## 🎯 Objective

Enable data ingestion from StampWorld (and potentially Colnect) via PDF exports, moving away from exclusive web scraping. This will increase stability against site anti-bot measures.

## 🛠 Proposed Workflow

1. **UI Addition**: Add an "Import PDF" trigger in both:
    * Bootstrap (Main) Screen (Country setup/management)
    * Country Review Screen (Data refinement)
2. **File Ingestion**: Open a file picker dialog (using existing `windows_picker` or equivalent) to select a PDF.
3. **Parsing**: Utilize a parsing strategy (to be determined) to extract stamp data mapping to `StampStub` format.
4. **Database Integration**: Import extracted records into the active country database.

---

## 🔍 Investigation Required

- [ ] **Parsing Strategy**: Determine the best way to extract structured text/tables from the sample PDFs.
  * *Candidate 1*: `pdftotext` (External tool)
  * *Candidate 2*: `lopdf` or `pdf` (Native Rust crates)
* [ ] **Data Mapping**: Verify if the PDF content aligns with the `StampStub` fields we currently scrape from StampWorld.
* [ ] **UI Integration**: Map out the required changes for `bootstrap.html`/`compare.html` and the corresponding JS handlers.

---

## ❓ Questions for Direction

- **Parsing Preference**: Native Rust (e.g., `lopdf`, `pdf` crates). Avoid external dependencies/CLI tools.
* **Mapping Priority**: Full mapping parity with the current scraper (designer, engraving, perforation, etc.).
* **Conflict Strategy**: Skip existing records. Report an informative message summarizing:
  * X records found in file.
  * Y records successfully imported.
  * Z records skipped due to collision.

1. **Parsing Preference**: Do you have a preference for using a native Rust PDF parsing crate (like `lopdf`) versus invoking a standard command-line tool (like `pdftotext`)?
2. **Mapping Priority**: Should we prioritize full mapping parity with the current scraper (e.g., designer, engraving, etc.), or start with a MVP (catalog number, year, description)?
3. **UI Flow**: Is there a specific behavior expected when an import conflicts with existing DB records? (e.g., skip, overwrite, or create duplicate)

---

## 📅 Status

* **Branch**: `feature/stampworld-pdf-import` created.
* **Samples Located**: `download-pdf/` contains three sample files.
