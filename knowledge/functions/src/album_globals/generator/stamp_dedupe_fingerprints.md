---
type: Rust Function
title: stamp_dedupe_fingerprints
resource: src/album_globals/generator.rs#L75-L108
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
  - target: functions/src/album_globals/generator/stamp_date_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/normalize_denom
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn stamp_dedupe_fingerprints(stamp: &Stamp) -> Vec<String>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [normalize_digits](../../../../functions/src/album_globals/generator/normalize_digits.md)
- [stamp_date_key](../../../../functions/src/album_globals/generator/stamp_date_key.md)
- [normalize_denom](../../../../functions/src/album_globals/generator/normalize_denom.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [generate_album_output_conn](../../../../functions/src/album_globals/generator/generate_album_output_conn.md)