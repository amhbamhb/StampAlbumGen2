---
type: Rust Method
title: resolve_with_context
resource: src/native_pdf/document.rs#L244-L260
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/document/PageSettings/resolve_page
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/native_pdf/config/NativePdfConfig/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/resolve_with_page_count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/renderer/write_page_geometry_proof_with_date
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/header_and_footer_share_one_generation_date_with_independent_fixed_overrides
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/defaults_enablement_snapshots_and_colour_precedence_are_explicit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/generation_date_requires_one_explicit_document_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/defaults_disabled_validation_snapshot_and_unapplied_keys_are_explicit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn resolve_with_context( &self, page_number: usize, total_pages: usize, generation_date: Option<chrono::NaiveDate>, ) -> Result<PageLayout>`

# Calls

- [resolve_page](../../../../../functions/src/native_pdf/document/PageSettings/resolve_page.md)

# Called by

- [parse](../../../../../functions/src/native_pdf/config/NativePdfConfig/parse.md)
- [resolve_with_page_count](../../../../../functions/src/native_pdf/document/PageSettings/resolve_with_page_count.md)
- [write_page_geometry_proof_with_date](../../../../../functions/src/native_pdf/renderer/write_page_geometry_proof_with_date.md)
- [header_and_footer_share_one_generation_date_with_independent_fixed_overrides](../../../../../functions/tests/native_pdf_footer_date/header_and_footer_share_one_generation_date_with_independent_fixed_overrides.md)
- [defaults_enablement_snapshots_and_colour_precedence_are_explicit](../../../../../functions/tests/native_pdf_footer_date/defaults_enablement_snapshots_and_colour_precedence_are_explicit.md)
- [generation_date_requires_one_explicit_document_context](../../../../../functions/tests/native_pdf_header_date/generation_date_requires_one_explicit_document_context.md)
- [defaults_disabled_validation_snapshot_and_unapplied_keys_are_explicit](../../../../../functions/tests/native_pdf_header_date/defaults_disabled_validation_snapshot_and_unapplied_keys_are_explicit.md)