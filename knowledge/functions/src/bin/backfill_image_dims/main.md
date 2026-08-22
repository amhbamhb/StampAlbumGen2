---
type: Rust Function
title: main
resource: src/bin/backfill_image_dims.rs#L87-L172
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/backfill_image_dims/print_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/time_utils/formatted_now
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/backfill_image_dims/process_db
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn main()`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [print_usage](../../../../functions/src/bin/backfill_image_dims/print_usage.md)
- [from](../../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [formatted_now](../../../../functions/src/time_utils/formatted_now.md)
- [process_db](../../../../functions/src/bin/backfill_image_dims/process_db.md)