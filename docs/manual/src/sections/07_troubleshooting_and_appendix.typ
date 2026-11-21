// ==============================================================================
// Section 7: Diagnostics, Troubleshooting & Appendix
// ==============================================================================

#pagebreak()
= 7. Diagnostics, Troubleshooting & Appendix

== 7.1 Diagnostic Logging & PDF Debugging

When investigating page overflow, text wrapping issues, or scraper connection errors, enable detailed diagnostic logging:

```bash
# Launch application with diagnostic vector logging:
stamp_album_gen2 --pdf-dx-logging
```

Log files are automatically timestamped and written to:
#align(center)[#text(size: 11pt, font: "Consolas", fill: rgb("#1B365D"))[`Logs/stamp_album_gen2-<timestamp>.log`]]

=== Common Error Messages & Resolutions

#table(
  columns: (2fr, 3fr),
  fill: (x, y) => if y == 0 { rgb("#1B365D").lighten(85%) } else if calc.even(y) { rgb("#F8F9FA") } else { none },
  stroke: 0.5pt + rgb("#D0D5DD"),
  table.header([*Reported Error*], [*Cause & Recommended Action*]),
  [
    `PDF generation error: Page size not defined.`
  ], [
    `ALBUM_PAGES_SIZE` is missing or placed after `PAGE_START`. Move the command before any page declarations.
  ],
  [
    `Stamp heading offset/text exceeds usable page bounds`
  ], [
    A stamp heading string is too wide for its local stamp frame and crosses the printable page margin. Shorten the text, reduce font size, or assign it to a wider stamp.
  ],
  [
    `ALBUM_PAGES_MARGINSE left right top bottom required`
  ], [
    `ALBUM_PAGES_MARGINSE` requires exactly four positive numeric values in millimetres.
  ],
  [
    `NATIVE_STAMP_FOOTER_TEXT cannot be combined with STAMP_IMAGE single caption`
  ], [
    `NATIVE_STAMP_FOOTER_TEXT` provides a dedicated 3-column sub-mount text area; do not also pass a caption argument in `STAMP_IMAGE`.
  ],
)

== 7.2 Software License

*Laser Spider Stamp Collector License (LSSCL) v1.0*

Individual stamp collectors may use StampAlbumGen2 for personal, non-commercial album generation at no fee. Non-collector, business, corporate, institutional, educational, or commercial use requires a separate written paid agreement with Laser Spider LLC.

Refer to `LICENSE` and `LASER_SPIDER_STAMP_COLLECTOR_LICENSE_v1.0.md` for full terms.

== 7.3 Project Links & Resources

- *Official Project Website & Gallery:* #link("https://stampalbumgenerator2.com")[https://stampalbumgenerator2.com]
- *Source Code Repository:* #link("https://github.com/amhbamhb/StampAlbumGen2")[https://github.com/amhbamhb/StampAlbumGen2]
- *AlbumEasy Official Website:* #link("https://www.thestampweb.com/albumeasy")[https://www.thestampweb.com/albumeasy] (Bundu Technology Ltd.)
