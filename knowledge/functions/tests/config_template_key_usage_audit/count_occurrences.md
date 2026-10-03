---
type: Rust Function
title: count_occurrences
resource: tests/config_template_key_usage_audit.rs#L76-L78
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/selection/count
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/config_template_key_usage_audit/config_template_key_usage_audit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn count_occurrences(haystack: &str, needle: &str) -> usize`

# Calls

- [count](../../../functions/src/album_globals/selection/count.md)

# Called by

- [config_template_key_usage_audit](../../../functions/tests/config_template_key_usage_audit/config_template_key_usage_audit.md)