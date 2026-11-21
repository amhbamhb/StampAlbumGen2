// ==============================================================================
// Section 3: Data Sources, Scrapers & Import Workflows
// ==============================================================================

#pagebreak()
= 3. Data Sources, Scrapers & Import Workflows

== 3.1 StampWorld Scraper

StampWorld serves as a primary source for worldwide stamp chronological listings, group sets, dates of issue, perforations, and specimen artwork.

The scraper maps HTML structures into normalized database records:
- *Set Group Boxes:* Outer containers (`div.container-fluid.content_table` with `id="group_box_<group_id>"`).
- *Header Summaries:* Issue dates, series title, engraver, designer, and perforation specs.
- *Stamp Rows:* Individual stamp rows (`tr.stamp_tr`) containing face values, colors, SG catalog numbers, and specimen image URLs.
- *Rate Limiting:* Configurable politeness delays ensure catalog web servers are not overwhelmed during mass synchronization.

== 3.2 Colnect Ingestion & Catalog Correlation

Colnect provides comprehensive physical stamp dimensions (width × height in millimetres), paper types, watermark details, and cross-catalog identifiers (Michel, Stanley Gibbons, Scott, Yvert & Tellier).

- Colnect exports are ingested via structured CSV files placed in the application root or uploaded through the Review UI.
- StampAlbumGen2 normalizes catalog numbers:
  - `Sn:` Scott Catalog
  - `Sg:` Stanley Gibbons Catalog
  - `Mi:` Michel Catalog
  - `Yt:` Yvert & Tellier
  - `Col:` Colnect internal identifier

== 3.3 US1847 Specialized Scraper

For United States classic and modern issues, the built-in US1847 scraper provides specialized philatelic data, plate numbers, grilled paper varieties, and secret mark identifiers specific to early American postage.

== 3.4 User-Defined Manual CSV Imports (`UserImport/`)

For self-scanned collections, local varieties, or stamps not listed in online catalogs, collectors can create offline import spreadsheets:

#block(
  fill: rgb("#F8F9FA"),
  inset: 12pt,
  radius: 4pt,
  stroke: 0.5pt + rgb("#D0D5DD"),
  [
    *Template Location:* `UserImport/manual_import_template.csv` \
    *Encoding:* Standard UTF-8 without Byte Order Mark (BOM). Do not save as Microsoft Excel binary workbook (`.xlsx`). \
    *Mandatory Fields:* `Name`, `Country`, `Series`, `Catalog Codes`, `Issued on`, `Link` \
    *Unique Identification:* Every custom stamp record requires a stable unique key in the `Catalog Codes` column prefixed with `Col:manual-` (for example, `Col:manual-us-1959-flag`).
  ]
)

== 3.5 National Flags & Cover Heraldry

When `print_flag_cover = true`, StampAlbumGen2 generates a formal commemorative cover page featuring the issuing country's national flag, official name, and date of generation.
- Uses the embedded ISO 3166 database (`ISOCountryCodes.csv`) supporting 246 countries and historical administrative territories.
- High-resolution SVG and PNG flags are retrieved and cached locally in `flags/`.
- Custom historical flags or coats of arms can be overridden using `flags/flag_codes.json` or placing an image at `Albums/<Country>/images/flag_placeholder.png`.
