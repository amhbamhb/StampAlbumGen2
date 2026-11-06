---
type: Rust Function
title: parse_template_keys
resource: tests/config_template_key_usage_audit.rs#L13-L56
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/config_template_key_usage_audit/config_template_key_usage_audit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_template_keys(template: &str) -> Vec<TemplateKey>`

# Calls

- [contains](../../../functions/src/native_pdf/stamp_image/contains.md)

# Called by

- [config_template_key_usage_audit](../../../functions/tests/config_template_key_usage_audit/config_template_key_usage_audit.md)