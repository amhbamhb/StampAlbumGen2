---
type: Rust Module
title: globals
resource: src/album_utils/globals.rs#L1-L1363
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-album-globals-generator-new-layout
    resolved_by: tree-sitter
    confidence: exact
  - target: external/log-info
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-write
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-utc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-datelike
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashset
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [write_globals](../../../functions/src/album_utils/globals/write_globals.md)
- [add_wikipedia_section](../../../functions/src/album_utils/globals/add_wikipedia_section.md)
- [ContentSection](../../../classes/src/album_utils/globals/ContentSection.md)
- [parse_content_into_sections](../../../functions/src/album_utils/globals/parse_content_into_sections.md)
- [create_balanced_pages](../../../functions/src/album_utils/globals/create_balanced_pages.md)
- [create_greedy_pages](../../../functions/src/album_utils/globals/create_greedy_pages.md)
- [balance_pages](../../../functions/src/album_utils/globals/balance_pages.md)

# Imports

- `crate::album_globals::generator_new::layout`
- `log::info`
- `std::io::Write`
- `std::path::Path`
- `chrono::Utc`
- `chrono::Datelike`
- `std::collections::HashSet`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)