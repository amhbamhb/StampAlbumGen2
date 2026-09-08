---
type: Rust Function
title: add_wikipedia_section
resource: src/album_utils/globals.rs#L690-L1120
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/globals/compute_usable_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/database/get_cached_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/layout/write_page_start
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/globals/parse_content_into_sections
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/utils/estimate_header_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/utils/estimate_text_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/globals/create_greedy_pages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/globals/create_balanced_pages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/layout/write_page_vspace
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_utils/globals/write_globals
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn add_wikipedia_section<W: Write>( writer: &mut W, album_name: &str, cfg: &crate::config::Config, mut page_counter: usize, conn_opt: Option<&rusqlite::Connection>, ) -> usize`

# Calls

- [compute_usable_height](../../../../functions/src/album_globals/globals/compute_usable_height.md)
- [get_cached_content](../../../../functions/src/wikipedia/database/get_cached_content.md)
- [write_page_start](../../../../functions/src/album_globals/generator_new/layout/write_page_start.md)
- [parse_content_into_sections](../../../../functions/src/album_utils/globals/parse_content_into_sections.md)
- [estimate_header_height](../../../../functions/src/wikipedia/utils/estimate_header_height.md)
- [estimate_text_height](../../../../functions/src/wikipedia/utils/estimate_text_height.md)
- [create_greedy_pages](../../../../functions/src/album_utils/globals/create_greedy_pages.md)
- [create_balanced_pages](../../../../functions/src/album_utils/globals/create_balanced_pages.md)
- [write_page_vspace](../../../../functions/src/album_globals/generator_new/layout/write_page_vspace.md)

# Called by

- [write_globals](../../../../functions/src/album_utils/globals/write_globals.md)