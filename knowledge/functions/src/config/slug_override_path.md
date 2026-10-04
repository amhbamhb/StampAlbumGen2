---
type: Rust Function
title: slug_override_path
resource: src/config.rs#L2781-L2823
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/try_load_slug_overrides
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/save_country_url
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn slug_override_path(path: &str) -> Result<std::path::PathBuf, String>`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)

# Called by

- [try_load_slug_overrides](../../../functions/src/config/try_load_slug_overrides.md)
- [save_country_url](../../../functions/src/stampworld/country_url/save_country_url.md)