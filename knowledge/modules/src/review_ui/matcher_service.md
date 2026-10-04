---
type: Rust Module
title: matcher_service
resource: src/review_ui/matcher_service.rs#L1-L1776
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-params-connection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [sync_processed_colnect_candidates](../../../functions/src/review_ui/matcher_service/sync_processed_colnect_candidates.md)
- [mark_colnect_candidate_processed](../../../functions/src/review_ui/matcher_service/mark_colnect_candidate_processed.md)
- [load_left_stamps](../../../functions/src/review_ui/matcher_service/load_left_stamps.md)
- [load_right_candidates](../../../functions/src/review_ui/matcher_service/load_right_candidates.md)
- [apply_matches](../../../functions/src/review_ui/matcher_service/apply_matches.md)
- [load_left_stamp_by_rowid](../../../functions/src/review_ui/matcher_service/load_left_stamp_by_rowid.md)
- [load_right_candidate_by_colnect_id](../../../functions/src/review_ui/matcher_service/load_right_candidate_by_colnect_id.md)
- [add_colnect_stamp_to_stamps](../../../functions/src/review_ui/matcher_service/add_colnect_stamp_to_stamps.md)
- [restore_colnect_candidate_to_queue](../../../functions/src/review_ui/matcher_service/restore_colnect_candidate_to_queue.md)
- [for_save](../../../functions/src/review_ui/matcher_service/LeftEditorImageInput/for_save.md)
- [for_add](../../../functions/src/review_ui/matcher_service/LeftEditorImageInput/for_add.md)
- [query_stamp_image_path](../../../functions/src/review_ui/matcher_service/query_stamp_image_path.md)
- [LeftEditorImageInput](../../../classes/src/review_ui/matcher_service/LeftEditorImageInput.md)
- [resolve_left_editor_image_path](../../../functions/src/review_ui/matcher_service/resolve_left_editor_image_path.md)
- [save_left_stamp](../../../functions/src/review_ui/matcher_service/save_left_stamp.md)
- [add_left_stamp](../../../functions/src/review_ui/matcher_service/add_left_stamp.md)
- [delete_left_stamp](../../../functions/src/review_ui/matcher_service/delete_left_stamp.md)
- [clear_left_colnect_link](../../../functions/src/review_ui/matcher_service/clear_left_colnect_link.md)
- [delete_colnect_candidate](../../../functions/src/review_ui/matcher_service/delete_colnect_candidate.md)
- [undo_colnect_delete](../../../functions/src/review_ui/matcher_service/undo_colnect_delete.md)

# Imports

- `super::*`
- `rusqlite::{params, Connection}`
- `std::collections::HashMap`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)