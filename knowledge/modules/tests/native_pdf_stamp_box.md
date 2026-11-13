---
type: Rust Module
title: native_pdf_stamp_box
resource: tests/native_pdf_stamp_box.rs#L1-L137
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/stamp-album-gen2-native-pdf-document-bounds-stamp-box-stampboxgeometry
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [physical_bounds](../../functions/tests/native_pdf_stamp_box/physical_bounds.md)
- [assert_close](../../functions/tests/native_pdf_stamp_box/assert_close.md)
- [total_additions_are_split_per_edge_not_applied_twice](../../functions/tests/native_pdf_stamp_box/total_additions_are_split_per_edge_not_applied_twice.md)
- [physical_dimensions_are_not_capped_or_rotated_for_any_item_format](../../functions/tests/native_pdf_stamp_box/physical_dimensions_are_not_capped_or_rotated_for_any_item_format.md)
- [zero_clearance_and_signed_adjustments_preserve_total_addition_semantics](../../functions/tests/native_pdf_stamp_box/zero_clearance_and_signed_adjustments_preserve_total_addition_semantics.md)
- [invalid_sizes_and_frames_that_would_shrink_known_stamps_fail_explicitly](../../functions/tests/native_pdf_stamp_box/invalid_sizes_and_frames_that_would_shrink_known_stamps_fail_explicitly.md)
- [coordinate_and_extent_overflow_are_rejected_before_rendering](../../functions/tests/native_pdf_stamp_box/coordinate_and_extent_overflow_are_rejected_before_rendering.md)

# Imports

- `stamp_album_gen2::native_pdf::{document::Bounds, stamp_box::StampBoxGeometry}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)