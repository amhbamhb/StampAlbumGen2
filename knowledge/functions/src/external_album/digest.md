---
type: Rust Function
title: digest
resource: src/external_album.rs#L181-L199
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/scan_inventory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/uploaded_hash
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/copy_tree
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/render_pair
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/verify_source
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/layout_hash
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/approve_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/check_hash
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_border_colour/named_colours_defaults_aliases_and_frozen_identities_match
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/frozen_reference_hashes_and_media_sizes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/frozen_reference_hashes_and_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/frozen_footer_date_reference_identities_are_unchanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_number/frozen_number_reference_identities_are_unchanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/frozen_reference_date_and_hashes_are_unchanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_number/frozen_header_number_reference_identities_are_unchanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_border/frozen_reference_identities_and_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_footer/frozen_footer_reference_identities_are_unchanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_header/frozen_header_reference_identities_are_unchanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_title/frozen_title_reference_identities_are_unchanged
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn digest(path: &Path) -> Result<String>`

# Calls

- [open](../../../functions/src/external_album/open.md)
- [err](../../../functions/src/external_album/err.md)

# Called by

- [scan_inventory](../../../functions/src/external_album/scan_inventory.md)
- [uploaded_hash](../../../functions/src/external_album/uploaded_hash.md)
- [copy_tree](../../../functions/src/external_album/copy_tree.md)
- [render_pair](../../../functions/src/external_album/render_pair.md)
- [verify_source](../../../functions/src/external_album/verify_source.md)
- [layout_hash](../../../functions/src/external_album/layout_hash.md)
- [approve_album](../../../functions/src/external_album/approve_album.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)
- [check_hash](../../../functions/tests/albumeasy_baseline/check_hash.md)
- [named_colours_defaults_aliases_and_frozen_identities_match](../../../functions/tests/native_pdf_border_colour/named_colours_defaults_aliases_and_frozen_identities_match.md)
- [frozen_reference_hashes_and_media_sizes](../../../functions/tests/native_pdf_crop_marks/frozen_reference_hashes_and_media_sizes.md)
- [frozen_reference_hashes_and_text](../../../functions/tests/native_pdf_decorative_border/frozen_reference_hashes_and_text.md)
- [frozen_footer_date_reference_identities_are_unchanged](../../../functions/tests/native_pdf_footer_date/frozen_footer_date_reference_identities_are_unchanged.md)
- [frozen_number_reference_identities_are_unchanged](../../../functions/tests/native_pdf_footer_number/frozen_number_reference_identities_are_unchanged.md)
- [frozen_reference_date_and_hashes_are_unchanged](../../../functions/tests/native_pdf_header_date/frozen_reference_date_and_hashes_are_unchanged.md)
- [frozen_header_number_reference_identities_are_unchanged](../../../functions/tests/native_pdf_header_number/frozen_header_number_reference_identities_are_unchanged.md)
- [frozen_reference_identities_and_text](../../../functions/tests/native_pdf_page_border/frozen_reference_identities_and_text.md)
- [frozen_footer_reference_identities_are_unchanged](../../../functions/tests/native_pdf_page_footer/frozen_footer_reference_identities_are_unchanged.md)
- [frozen_header_reference_identities_are_unchanged](../../../functions/tests/native_pdf_page_header/frozen_header_reference_identities_are_unchanged.md)
- [frozen_title_reference_identities_are_unchanged](../../../functions/tests/native_pdf_page_title/frozen_title_reference_identities_are_unchanged.md)