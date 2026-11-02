---
type: Rust Module
title: windows_picker
resource: src/review_ui/windows_picker.rs#L1-L146
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/std-process-command
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-os-windows-process-commandext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [PickerKind](../../../classes/src/review_ui/windows_picker/PickerKind.md)
- [picker_command](../../../functions/src/review_ui/windows_picker/picker_command.md)
- [decode_selection](../../../functions/src/review_ui/windows_picker/decode_selection.md)
- [pick](../../../functions/src/review_ui/windows_picker/pick.md)
- [windows_picker_uses_sta_and_passes_inputs_as_literal_environment_values](../../../functions/src/review_ui/windows_picker/windows_picker_uses_sta_and_passes_inputs_as_literal_environment_values.md)
- [windows_picker_preserves_unicode_spaces_and_unc_paths](../../../functions/src/review_ui/windows_picker/windows_picker_preserves_unicode_spaces_and_unc_paths.md)
- [windows_picker_distinguishes_cancellation_from_failure](../../../functions/src/review_ui/windows_picker/windows_picker_distinguishes_cancellation_from_failure.md)
- [windows_picker_rejects_invalid_success_output](../../../functions/src/review_ui/windows_picker/windows_picker_rejects_invalid_success_output.md)

# Imports

- `std::process::Command`
- `std::os::windows::process::CommandExt`
- `super::*`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)