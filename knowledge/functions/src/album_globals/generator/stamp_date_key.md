---
type: Rust Function
title: stamp_date_key
resource: src/album_globals/generator.rs#L47-L73
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/normalize_digits
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/stamp_dedupe_fingerprints
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn stamp_date_key(stamp: &Stamp) -> String`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [normalize_digits](../../../../functions/src/album_globals/generator/normalize_digits.md)

# Called by

- [stamp_dedupe_fingerprints](../../../../functions/src/album_globals/generator/stamp_dedupe_fingerprints.md)
- [generate_album_output_conn](../../../../functions/src/album_globals/generator/generate_album_output_conn.md)