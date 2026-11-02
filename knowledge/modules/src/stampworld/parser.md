---
type: Rust Module
title: parser
resource: src/stampworld/parser.rs#L1-L159
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/once-cell-sync-lazy
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
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

- [normalize_stamps_issued](../../../functions/src/stampworld/parser/normalize_stamps_issued.md)
- [dedupe_details](../../../functions/src/stampworld/parser/dedupe_details.md)
- [extract_block_metadata](../../../functions/src/stampworld/parser/extract_block_metadata.md)
- [compress_context](../../../functions/src/stampworld/parser/compress_context.md)

# Imports

- `once_cell::sync::Lazy`
- `regex::Regex`
- `std::collections::HashSet`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)