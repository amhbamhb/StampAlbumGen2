---
type: Rust Function
title: unmapped_country_can_reuse_a_valid_album_local_flag_without_network
resource: tests/flagdb.rs#L49-L71
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/external_album/save
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/copy_flag_for_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn unmapped_country_can_reuse_a_valid_album_local_flag_without_network()`

# Calls

- [save](../../../functions/src/review_ui/views/external_album/save.md)
- [copy_flag_for_album](../../../functions/src/flagdb/copy_flag_for_album.md)