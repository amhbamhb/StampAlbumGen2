---
type: Rust Function
title: test_ensure_placeholder_for_stamp_and_update_db
resource: tests/placeholder_helper_test.rs#L27-L107
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/placeholder_helper_test/CwdGuard/chdir_to
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/ensure_placeholder_for_stamp_and_update_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_ensure_placeholder_for_stamp_and_update_db()`

# Calls

- [chdir_to](../../../functions/tests/placeholder_helper_test/CwdGuard/chdir_to.md)
- [ensure_placeholder_for_stamp_and_update_db](../../../functions/src/album_globals/generator_new/helpers/ensure_placeholder_for_stamp_and_update_db.md)
- [open](../../../functions/src/external_album/open.md)
- [prepare](../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)