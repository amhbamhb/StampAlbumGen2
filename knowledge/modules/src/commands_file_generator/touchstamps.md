---
type: Rust Module
title: touchstamps
resource: src/commands_file_generator/touchstamps.rs#L1-L151
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-types-imagestatus
    resolved_by: tree-sitter
    confidence: exact
  - target: external/log-error
    resolved_by: tree-sitter
    confidence: exact
  - target: external/scraper-html-selector
    resolved_by: tree-sitter
    confidence: exact
  - target: external/reqwest-blocking-client
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [get_touchstamps_stamps](../../../functions/src/commands_file_generator/touchstamps/get_touchstamps_stamps.md)
- [get_touchstamps_image](../../../functions/src/commands_file_generator/touchstamps/get_touchstamps_image.md)

# Imports

- `crate::types::ImageStatus`
- `log::error`
- `scraper::{Html, Selector}`
- `reqwest::blocking::Client`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)