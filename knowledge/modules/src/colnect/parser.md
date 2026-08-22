---
type: Rust Module
title: parser
resource: src/colnect/parser.rs#L1-L174
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-error-error
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs-file
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-bufread-bufreader
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

- [clean_name](../../../functions/src/colnect/parser/clean_name.md)
- [clean_perforation](../../../functions/src/colnect/parser/clean_perforation.md)
- [ColnectStamp](../../../classes/src/colnect/parser/ColnectStamp.md)
- [parse_csv](../../../functions/src/colnect/parser/parse_csv.md)

# Imports

- `regex::Regex`
- `serde::Deserialize`
- `std::error::Error`
- `std::fs::File`
- `std::io::{BufRead, BufReader}`
- `std::path::Path`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)