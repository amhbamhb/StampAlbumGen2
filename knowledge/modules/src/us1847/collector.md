---
type: Rust Module
title: collector
resource: src/us1847/collector.rs#L1-L1632
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/log-error-info
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/reqwest-blocking-client
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-connection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/scraper-elementref-html-selector
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-error-error
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-write
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [StampInfo](../../../classes/src/us1847/collector/StampInfo.md)
- [Us1847PageOptions](../../../classes/src/us1847/collector/Us1847PageOptions.md)
- [process_1847us_page](../../../functions/src/us1847/collector/process_1847us_page.md)
- [parse_html_tables](../../../functions/src/us1847/collector/parse_html_tables.md)
- [collect_text](../../../functions/src/us1847/collector/collect_text.md)
- [parse_stamp_lists](../../../functions/src/us1847/collector/parse_stamp_lists.md)
- [parse_printing_section](../../../functions/src/us1847/collector/parse_printing_section.md)
- [parse_single_image_stamp](../../../functions/src/us1847/collector/parse_single_image_stamp.md)
- [parse_table_stamps](../../../functions/src/us1847/collector/parse_table_stamps.md)
- [parse_stamp_from_title_and_description](../../../functions/src/us1847/collector/parse_stamp_from_title_and_description.md)
- [parse_ordinary_stamp_line](../../../functions/src/us1847/collector/parse_ordinary_stamp_line.md)
- [extract_design_size](../../../functions/src/us1847/collector/extract_design_size.md)
- [parse_liberty_issue_stamp](../../../functions/src/us1847/collector/parse_liberty_issue_stamp.md)
- [parse_commemorative_line](../../../functions/src/us1847/collector/parse_commemorative_line.md)
- [insert_stamp_if_not_exists](../../../functions/src/us1847/collector/insert_stamp_if_not_exists.md)
- [download_stamp_images](../../../functions/src/us1847/collector/download_stamp_images.md)
- [sync_us1847_to_stamps](../../../functions/src/us1847/collector/sync_us1847_to_stamps.md)

# Imports

- `log::{error, info}`
- `regex::Regex`
- `reqwest::blocking::Client`
- `rusqlite::Connection`
- `scraper::{ElementRef, Html, Selector}`
- `std::error::Error`
- `std::io::Write`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)