---
type: Rust Function
title: ensure_default_files
resource: src/config.rs#L439-L484
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/config/ensure_config_and_template_files
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn ensure_default_files( config_abs: &Path, template_abs: &Path, pdf_abs: &Path, ) -> Result<(), Box<dyn std::error::Error>>`

# Called by

- [ensure_config_and_template_files](../../../functions/src/config/ensure_config_and_template_files.md)