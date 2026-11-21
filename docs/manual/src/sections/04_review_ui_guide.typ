// ==============================================================================
// Section 4: How Stamp Information Is Gathered & Custom Imports
// ==============================================================================

#pagebreak()
= 4. How Stamp Information Is Gathered & Custom Imports

To save you from typing hundreds of dates and measurements, StampAlbumGen2 connects with reliable public philatelic catalog databases while keeping all your collection data safely on your own computer.

== 4.1 Built-In Catalog Sources

- *StampWorld:* Provides complete, chronological lists of stamp issues by country and year, including series titles, face values, perforation counts, and specimen artwork.
- *Colnect:* Contributes physical stamp measurements (exact width and height in millimetres) along with cross-catalog numbers (Scott, Stanley Gibbons, Michel, and Yvert & Tellier).
- *US1847:* A specialized historical catalog for United States collectors, detailing classic plate varieties, grilled papers, and secret marks.

All retrieved data is saved into your local database file (`stamp_album.db`). Once downloaded, you can generate album pages anytime, even when completely offline without an internet connection!

== 4.2 Adding Custom Stamps with a Simple Spreadsheet (`UserImport/`)

Have local stamps, revenue stamps, family covers, or specialized varieties not found in online catalogs? You can add them in minutes using a simple CSV spreadsheet!

Inside the `UserImport/` folder, you will find:
#align(center)[#text(size: 11pt, font: "Consolas", fill: rgb("#1B365D"))[`UserImport/manual_import_template.csv`]]

You can open this template in Excel, LibreOffice Calc, or Google Sheets.

#block(
  fill: rgb("#F8F9FA"),
  inset: 12pt,
  radius: 4pt,
  stroke: 0.5pt + rgb("#D0D5DD"),
  [
    *Simple Rules for Custom Spreadsheets:* \
    1. *Save as UTF-8 CSV:* When saving your spreadsheet, choose \"CSV (Comma Delimited)\". Do not save as an `.xlsx` workbook. \
    2. *Fill in the Core Columns:* Name, Country, Series, Catalog Codes, and Issue Date. \
    3. *Give Each Stamp a Unique ID:* In the `Catalog Codes` column, create a stable ID starting with `Col:manual-` (for example: `Col:manual-us-1960-flag`). \
    4. *Drop the File in UserImport:* Next time you open the Review UI, your custom stamps will appear in the candidate list ready to match!
  ]
)

== 4.3 Commemorative Title Cover Pages & National Flags

When creating a new country binder, a handsome title cover page sets the tone for the entire volume.

When enabled, StampAlbumGen2 generates a formal introductory cover sheet featuring:
- The country's official name and the years covered by the album.
- A high-resolution full-color reproduction of the national flag or historical coat of arms.
- Your name as collector, copyright notice, and date of compilation.
- The application automatically handles historical territories, protectorates, and former nations (such as Austrian Levant, Saar, or Netherlands Antilles) using its built-in database of 246 postal authorities.
