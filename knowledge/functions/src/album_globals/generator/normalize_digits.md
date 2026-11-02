---
type: Rust Function
title: normalize_digits
resource: src/album_globals/generator.rs#L37-L39
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/stamp_date_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/stamp_dedupe_fingerprints
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn normalize_digits(v: &str) -> String`

# Called by

- [stamp_date_key](../../../../functions/src/album_globals/generator/stamp_date_key.md)
- [stamp_dedupe_fingerprints](../../../../functions/src/album_globals/generator/stamp_dedupe_fingerprints.md)