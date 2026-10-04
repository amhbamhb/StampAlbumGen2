---
type: Rust Function
title: valid_file
resource: src/flagdb/assets.rs#L49-L55
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/flagdb/assets/ensure_flag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/local_album_flag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/copy_to_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/valid_cache_skips_network_corrupt_cache_is_repaired_and_overwrite_refreshes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn valid_file(path: &Path) -> AssetResult<()>`

# Called by

- [ensure_flag](../../../../functions/src/flagdb/assets/ensure_flag.md)
- [local_album_flag](../../../../functions/src/flagdb/assets/local_album_flag.md)
- [copy_to_album](../../../../functions/src/flagdb/assets/copy_to_album.md)
- [valid_cache_skips_network_corrupt_cache_is_repaired_and_overwrite_refreshes](../../../../functions/src/flagdb/assets/valid_cache_skips_network_corrupt_cache_is_repaired_and_overwrite_refreshes.md)