---
type: Rust Function
title: source_colour
resource: tests/native_pdf_border_colour.rs#L50-L63
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/tests/native_pdf_border_colour/named_colours_defaults_aliases_and_frozen_identities_match
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_border_colour/rendered_colours_match_reference_and_guide_stays_black
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn source_colour(source: &str) -> Result<AlbumBorderColour>`

# Called by

- [named_colours_defaults_aliases_and_frozen_identities_match](../../../functions/tests/native_pdf_border_colour/named_colours_defaults_aliases_and_frozen_identities_match.md)
- [rendered_colours_match_reference_and_guide_stays_black](../../../functions/tests/native_pdf_border_colour/rendered_colours_match_reference_and_guide_stays_black.md)