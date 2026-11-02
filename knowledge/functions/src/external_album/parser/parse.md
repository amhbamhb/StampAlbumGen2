---
type: Rust Function
title: parse
resource: src/external_album/parser.rs#L194-L236
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/parser/Lexer/skip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/Lexer/command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/Lexer/peek
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/Lexer/bump
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/validate_document
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/crates/stamp_album_db/src/insert_stamp_from_struct
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/touchstamps/get_touchstamps_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/data_sources/stampworld/collect_stampworld_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/inspect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/import_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/rasterize
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/probe_port
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/db_query/process_pdf_records
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/route
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/extraction/parse_stampworld_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/readSiteAccountResponse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/restorePdfUiState
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/readSiteAccountResponse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/initReviewStampWorldPanel
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/initReviewPdfYearInputs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/init
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/slug_from_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/process_1847us_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_html_tables
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_table_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/extract_design_size
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_liberty_issue_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/parsers/parse_grokipedia_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/parsers/parse_wikipedia_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/review_ui_external_album/test/harness
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/stampworld_pdf_import_ui/fixture
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/test_stampworld_parsing/description_extraction_prefers_td6_with_fallback
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn parse(text: &str) -> (Vec<LayoutCommand>, Vec<LayoutPage>, Vec<String>)`

# Calls

- [skip](../../../../functions/src/external_album/parser/Lexer/skip.md)
- [command](../../../../functions/src/external_album/parser/Lexer/command.md)
- [peek](../../../../functions/src/external_album/parser/Lexer/peek.md)
- [bump](../../../../functions/src/external_album/parser/Lexer/bump.md)
- [validate_document](../../../../functions/src/external_album/parser/validate_document.md)

# Called by

- [insert_stamp_from_struct](../../../../functions/crates/stamp_album_db/src/insert_stamp_from_struct.md)
- [get_touchstamps_stamps](../../../../functions/src/commands_file_generator/touchstamps/get_touchstamps_stamps.md)
- [collect_stampworld_data](../../../../functions/src/data_sources/stampworld/collect_stampworld_data.md)
- [inspect](../../../../functions/src/external_album/inspect.md)
- [import_album](../../../../functions/src/external_album/import_album.md)
- [rasterize](../../../../functions/src/flagdb/assets/rasterize.md)
- [main](../../../../functions/src/main.md)
- [probe_port](../../../../functions/src/review_ui/probe_port.md)
- [process_pdf_records](../../../../functions/src/review_ui/db_query/process_pdf_records.md)
- [route](../../../../functions/src/review_ui/external_album_api/route.md)
- [parse_stampworld_html](../../../../functions/src/review_ui/pdf_parser/extraction/parse_stampworld_html.md)
- [readSiteAccountResponse](../../../../functions/src/review_ui/views/bootstrap/readSiteAccountResponse.md)
- [restorePdfUiState](../../../../functions/src/review_ui/views/compare/restorePdfUiState.md)
- [readSiteAccountResponse](../../../../functions/src/review_ui/views/compare/readSiteAccountResponse.md)
- [initReviewStampWorldPanel](../../../../functions/src/review_ui/views/compare/initReviewStampWorldPanel.md)
- [initReviewPdfYearInputs](../../../../functions/src/review_ui/views/compare/initReviewPdfYearInputs.md)
- [init](../../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/init.md)
- [slug_from_url](../../../../functions/src/stampworld/country_url/slug_from_url.md)
- [collect_stampworld_data_with_fetcher](../../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)
- [process_1847us_page](../../../../functions/src/us1847/collector/process_1847us_page.md)
- [parse_html_tables](../../../../functions/src/us1847/collector/parse_html_tables.md)
- [parse_table_stamps](../../../../functions/src/us1847/collector/parse_table_stamps.md)
- [extract_design_size](../../../../functions/src/us1847/collector/extract_design_size.md)
- [parse_liberty_issue_stamp](../../../../functions/src/us1847/collector/parse_liberty_issue_stamp.md)
- [parse_grokipedia_html](../../../../functions/src/wikipedia/parsers/parse_grokipedia_html.md)
- [parse_wikipedia_html](../../../../functions/src/wikipedia/parsers/parse_wikipedia_html.md)
- [harness](../../../../functions/tests/review_ui_external_album/test/harness.md)
- [fixture](../../../../functions/tests/stampworld_pdf_import_ui/fixture.md)
- [description_extraction_prefers_td6_with_fallback](../../../../functions/tests/test_stampworld_parsing/description_extraction_prefers_td6_with_fallback.md)