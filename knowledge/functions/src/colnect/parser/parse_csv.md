---
type: Rust Function
title: parse_csv
resource: src/colnect/parser.rs#L118-L176
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/progress/ProgressIndicator/clear
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/parser/clean_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/parser/clean_perforation
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/modes/run_colnect_inbox_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_database_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/colnect_import_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/colnect_integration/test_colnect_integration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/ingest_switzerland/ingest_switzerland_1972
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/manual_import_template/manual_rows_parse_and_replace_only_the_same_staged_identity
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/manual_import_template/documented_example_is_accepted_by_the_existing_parser
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse_csv<P: AsRef<Path>>(path: P) -> Result<Vec<ColnectStamp>, Box<dyn Error>>`

# Calls

- [open](../../../../functions/src/external_album/open.md)
- [clear](../../../../functions/src/progress/ProgressIndicator/clear.md)
- [clean_name](../../../../functions/src/colnect/parser/clean_name.md)
- [clean_perforation](../../../../functions/src/colnect/parser/clean_perforation.md)

# Called by

- [run_colnect_inbox_mode](../../../../functions/src/modes/run_colnect_inbox_mode.md)
- [run_database_mode](../../../../functions/src/modes/run_database_mode.md)
- [run_scraping_mode](../../../../functions/src/modes/run_scraping_mode.md)
- [colnect_import_data](../../../../functions/src/review_ui/colnect_import_data.md)
- [test_colnect_integration](../../../../functions/tests/colnect_integration/test_colnect_integration.md)
- [ingest_switzerland_1972](../../../../functions/tests/ingest_switzerland/ingest_switzerland_1972.md)
- [manual_rows_parse_and_replace_only_the_same_staged_identity](../../../../functions/tests/manual_import_template/manual_rows_parse_and_replace_only_the_same_staged_identity.md)
- [documented_example_is_accepted_by_the_existing_parser](../../../../functions/tests/manual_import_template/documented_example_is_accepted_by_the_existing_parser.md)