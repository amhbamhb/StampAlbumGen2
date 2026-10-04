---
type: Rust Function
title: with_album_config_override
resource: src/config.rs#L2565-L2572
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/config/scoped_album_override_applies_stamp_visibility_keys_in_effective_config
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_album_config_override<R>(album_dir: &Path, f: impl FnOnce() -> R) -> R`

# Called by

- [scoped_album_override_applies_stamp_visibility_keys_in_effective_config](../../../functions/src/config/scoped_album_override_applies_stamp_visibility_keys_in_effective_config.md)