---
type: Rust Module
title: config
resource: src/native_pdf/config.rs#L1-L676
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super-colour-namedcolour-commands-colour-colour-album-border-albumbordercolour-commands-page-album-pages-border-pageborder-album-pages-crop-marks-albumpagescropmarks-album-pages-decorative-border-albumpagesdecorativeborder-decorativeborderpolicy-album-pages-footer-albumpagesfooter-album-pages-footer-date-albumpagesfooterdate-album-pages-footer-number-albumpagesfooternumber-album-pages-footer-pad-footerpadding-album-pages-header-albumpagesheader-headerposition-album-pages-header-date-albumpagesheaderdate-album-pages-header-number-albumpagesheadernumber-album-pages-header-pad-headerpadding-album-pages-margins-margins-album-pages-marginse-evenmargins-album-pages-size-pagesize-album-pages-spacing-spacing-album-pages-title-albumpagestitle-date-dateformat-datesource-document-pagesettings-proof-geometryproofsettings-text-textposition-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/toml-table-value
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [NativePdfConfig](../../../classes/src/native_pdf/config/NativePdfConfig.md)
- [load](../../../functions/src/native_pdf/config/NativePdfConfig/load.md)
- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [from_toml_in](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml_in.md)
- [parse](../../../functions/src/native_pdf/config/NativePdfConfig/parse.md)
- [table](../../../functions/src/native_pdf/config/table.md)
- [string](../../../functions/src/native_pdf/config/string.md)
- [boolean](../../../functions/src/native_pdf/config/boolean.md)
- [numbers](../../../functions/src/native_pdf/config/numbers.md)
- [number](../../../functions/src/native_pdf/config/number.md)
- [collect_keys](../../../functions/src/native_pdf/config/collect_keys.md)

# Imports

- `super::{
    colour::NamedColour,
    commands::colour::colour_album_border::AlbumBorderColour,
    commands::page::{
        album_pages_border::PageBorder,
        album_pages_crop_marks::AlbumPagesCropMarks,
        album_pages_decorative_border::{AlbumPagesDecorativeBorder, DecorativeBorderPolicy},
        album_pages_footer::AlbumPagesFooter,
        album_pages_footer_date::AlbumPagesFooterDate,
        album_pages_footer_number::AlbumPagesFooterNumber,
        album_pages_footer_pad::FooterPadding,
        album_pages_header::{AlbumPagesHeader, HeaderPosition},
        album_pages_header_date::AlbumPagesHeaderDate,
        album_pages_header_number::AlbumPagesHeaderNumber,
        album_pages_header_pad::HeaderPadding,
        album_pages_margins::Margins,
        album_pages_marginse::EvenMargins,
        album_pages_size::PageSize,
        album_pages_spacing::Spacing,
        album_pages_title::AlbumPagesTitle,
    },
    date::{DateFormat, DateSource},
    document::PageSettings,
    proof::GeometryProofSettings,
    text::TextPosition,
    Result,
}`
- `std::{fs, path::Path}`
- `toml::{Table, Value}`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)