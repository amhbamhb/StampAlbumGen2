---
type: Rust Module
title: generator
resource: src/commands_file_generator/generator.rs#L1-L310
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-album-globals-stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-commands-file-generator-album-creation-save-stamp-data
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-commands-file-generator-utils-is-selected-emission-is-selected-format-month-number-snooze
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config-config
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-types-seriesinfo
    resolved_by: tree-sitter
    confidence: exact
  - target: external/log-info
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-commands-file-generator-data-collection-collect-stampworld-data
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-types-imagestatus
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [CollectStampworldCallParams](../../../classes/src/commands_file_generator/generator/CollectStampworldCallParams.md)
- [CommandsFileGenerator](../../../classes/src/commands_file_generator/generator/CommandsFileGenerator.md)
- [new](../../../functions/src/commands_file_generator/generator/CommandsFileGenerator/new.md)
- [collect_stampworld_data](../../../functions/src/commands_file_generator/generator/CommandsFileGenerator/collect_stampworld_data.md)
- [get_touchstamps_stamps](../../../functions/src/commands_file_generator/generator/CommandsFileGenerator/get_touchstamps_stamps.md)
- [create_album](../../../functions/src/commands_file_generator/generator/CommandsFileGenerator/create_album.md)
- [save_stamp_data](../../../functions/src/commands_file_generator/generator/CommandsFileGenerator/save_stamp_data.md)
- [process_stamps](../../../functions/src/commands_file_generator/generator/CommandsFileGenerator/process_stamps.md)
- [validate_config](../../../functions/src/commands_file_generator/generator/CommandsFileGenerator/validate_config.md)
- [month_number](../../../functions/src/commands_file_generator/generator/CommandsFileGenerator/month_number.md)
- [is_selected_emission](../../../functions/src/commands_file_generator/generator/CommandsFileGenerator/is_selected_emission.md)
- [is_selected_format](../../../functions/src/commands_file_generator/generator/CommandsFileGenerator/is_selected_format.md)
- [snooze](../../../functions/src/commands_file_generator/generator/CommandsFileGenerator/snooze.md)
- [default](../../../functions/src/commands_file_generator/generator/CommandsFileGenerator/default/default.md)

# Imports

- `crate::album_globals::Stamp`
- `crate::commands_file_generator::album_creation::save_stamp_data`
- `crate::commands_file_generator::utils::{
    is_selected_emission, is_selected_format, month_number, snooze,
}`
- `crate::config::Config`
- `crate::types::SeriesInfo`
- `log::info`
- `std::collections::HashMap`
- `crate::commands_file_generator::data_collection::collect_stampworld_data`
- `crate::types::ImageStatus`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)