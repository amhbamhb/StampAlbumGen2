---
type: Rust Function
title: sample_record
resource: tests/pdf_import_tool.rs#L11-L30
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/tests/pdf_import_tool/duplicates_are_country_scoped_and_do_not_overwrite_existing_metadata
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn sample_record(number: &str) -> PdfStampRecord`

# Called by

- [duplicates_are_country_scoped_and_do_not_overwrite_existing_metadata](../../../functions/tests/pdf_import_tool/duplicates_are_country_scoped_and_do_not_overwrite_existing_metadata.md)