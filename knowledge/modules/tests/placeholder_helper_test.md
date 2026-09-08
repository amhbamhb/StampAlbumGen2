---
type: Rust Module
title: placeholder_helper_test
resource: tests/placeholder_helper_test.rs#L1-L107
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  - target: external/image-genericimageview
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serial-test-serial
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [CwdGuard](../../classes/tests/placeholder_helper_test/CwdGuard.md)
- [chdir_to](../../functions/tests/placeholder_helper_test/CwdGuard/chdir_to.md)
- [drop](../../functions/tests/placeholder_helper_test/CwdGuard/drop/drop.md)
- [test_ensure_placeholder_for_stamp_and_update_db](../../functions/tests/placeholder_helper_test/test_ensure_placeholder_for_stamp_and_update_db.md)

# Imports

- `std::path::Path`
- `tempfile::tempdir`
- `image::GenericImageView`
- `serial_test::serial`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)