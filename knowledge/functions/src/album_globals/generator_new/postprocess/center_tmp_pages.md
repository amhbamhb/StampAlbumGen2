---
type: Rust Function
title: center_tmp_pages
resource: src/album_globals/generator_new/postprocess.rs#L8-L26
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/postprocess/center_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/progress/ProgressIndicator/clear
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn center_tmp_pages( tmp_path: &Path, config: &crate::config::Config, usable_height: f64, ) -> std::io::Result<()>`

# Calls

- [center_page](../../../../../functions/src/album_globals/generator_new/postprocess/center_page.md)
- [clear](../../../../../functions/src/progress/ProgressIndicator/clear.md)