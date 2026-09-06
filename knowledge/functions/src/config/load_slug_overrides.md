---
type: Rust Function
title: load_slug_overrides
resource: src/config.rs#L2904-L2960
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn load_slug_overrides(path: &str) -> HashMap<String, String>`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [run_scraping_mode](../../../functions/src/modes/run_scraping_mode.md)