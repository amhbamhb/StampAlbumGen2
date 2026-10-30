---
type: Rust Module
title: extraction
resource: src/review_ui/pdf_parser/extraction.rs#L1-L294
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super-types-pdfstamprecord
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pdf-extract-document-mediabox-outputdev-outputerror-transform
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/scraper-html-selector
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [LayoutText](../../../../classes/src/review_ui/pdf_parser/extraction/LayoutText.md)
- [begin_page](../../../../functions/src/review_ui/pdf_parser/extraction/LayoutText/outputdev/begin_page.md)
- [output_character](../../../../functions/src/review_ui/pdf_parser/extraction/LayoutText/outputdev/output_character.md)
- [end_page](../../../../functions/src/review_ui/pdf_parser/extraction/LayoutText/outputdev/end_page.md)
- [begin_word](../../../../functions/src/review_ui/pdf_parser/extraction/LayoutText/outputdev/begin_word.md)
- [end_word](../../../../functions/src/review_ui/pdf_parser/extraction/LayoutText/outputdev/end_word.md)
- [end_line](../../../../functions/src/review_ui/pdf_parser/extraction/LayoutText/outputdev/end_line.md)
- [parse_stampworld_pdf](../../../../functions/src/review_ui/pdf_parser/extraction/parse_stampworld_pdf.md)
- [extract_stampworld_pdf_text](../../../../functions/src/review_ui/pdf_parser/extraction/extract_stampworld_pdf_text.md)
- [normalized](../../../../functions/src/review_ui/pdf_parser/extraction/normalized.md)
- [metadata](../../../../functions/src/review_ui/pdf_parser/extraction/metadata.md)
- [parse_stampworld_text](../../../../functions/src/review_ui/pdf_parser/extraction/parse_stampworld_text.md)
- [parse_stampworld_html](../../../../functions/src/review_ui/pdf_parser/extraction/parse_stampworld_html.md)

# Imports

- `super::types::PdfStampRecord`
- `pdf_extract::{Document, MediaBox, OutputDev, OutputError, Transform}`
- `regex::Regex`
- `scraper::{Html, Selector}`
- `std::path::Path`

# Member of

- [stamp_album_gen2](../../../../packages/stamp_album_gen2.md)