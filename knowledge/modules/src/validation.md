---
type: Rust Module
title: validation
resource: src/validation.rs#L1-L215
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/strsim-jaro-winkler
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-isterminal
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-self-write
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ValidationError](../../classes/src/validation/ValidationError.md)
- [fmt](../../functions/src/validation/ValidationError/std-fmt-display/fmt.md)
- [validate_user_input](../../functions/src/validation/validate_user_input.md)
- [validate_year_range](../../functions/src/validation/validate_year_range.md)
- [validate_source](../../functions/src/validation/validate_source.md)
- [print_validation_error](../../functions/src/validation/print_validation_error.md)

# Imports

- `strsim::jaro_winkler`
- `std::io::IsTerminal`
- `std::io::{self, Write}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)