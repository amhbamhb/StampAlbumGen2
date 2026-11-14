# User-defined CSV import

Enter your own stamp data in a spreadsheet and import it through the existing
Colnect-compatible importer. A Colnect account or export is not required.
These are review candidates, not automatic edits to your album stamps.

## 1. Find the template

On normal application startup, the program creates:

- `UserImport/manual_import_template.csv`: a blank CSV containing column headers.
- `UserImport/README.md`: a local copy of this guide.

The folder is relative to the application's runtime working folder, after any
startup folder alignment, not necessarily the folder from which you launched it.
Installed launches normally use the application folder; development launches use
the working folder. Look alongside the runtime `config.toml` and `Albums` folder.
The template is outside import inboxes so it is not imported automatically.

If either file is missing, the next normal launch recreates that file. Existing
files are never overwritten, even if you edited them. This is not a backup of
your filled-in data: deleted working copies cannot be recovered by the program.
`--help`, `--version`, and desktop launcher install/uninstall commands exit before
template creation. Permission or path errors produce a warning; make the runtime
folder writable and launch again.

## 2. Make a working copy

Copy the template to a new filename, for example `Switzerland-manual-1975.csv`.
Keep the original blank for future use. Open the copy in Excel, LibreOffice Calc,
or a text editor.

Use one row per individual stamp or variant and one country per file. Do not
enter a combined series/group summary as though it were an individual stamp.
`Series` groups related stamps, just as a StampWorld group title does; dimensions,
face value, color, and catalog number belong to the individual stamp.

Preserve all column headers exactly, including capitalization and spaces. Keep
`Name,Country` as the first two unquoted headers. Save as comma-separated UTF-8
CSV **without a byte-order mark (BOM)**, not XLSX or semicolon-separated CSV.
The current parser does not recognize quoted first headers or a BOM.
Import catalog numbers, dates, dimensions, and quantities into the spreadsheet
as text to avoid losing leading zeros, variant suffixes, or date formatting.

## 3. Fill in the fields

The template includes all 38 supported Colnect CSV columns. Supply these core
fields for usable review candidates:

| Header | What to enter | Example |
|---|---|---|
| `Name` | Individual stamp name | `International Women's Year` |
| `Country` | Country matching your album/database | `Switzerland` |
| `Series` | Series/group title | `1975 Commemoratives` |
| `Catalog Codes` | Catalog entries separated by commas; include a stable identity | `Sg:CH 1041, Col:manual-ch-0001` |
| `Issued on` | Issue date as `YYYY-MM-DD` | `1975-02-13` |
| `Link` | Real source webpage if known; otherwise leave the cell blank | blank |

`Link` must remain a column even when its cells are empty. With a `Col:` entry,
no link is needed to identify the row. The importer does not validate all these
values: checking them before import is important.

### Stable IDs and catalog numbers

For a stamp not taken from Colnect, give it a unique `Col:manual-...` identifier,
such as `Col:manual-ch-0001`. This is an internal candidate key, not a real Colnect
listing. Never invent a numeric Colnect listing or fake website link. Reuse the
same ID when editing the same stamp; use a different ID for each variant. IDs
must be unique within a country database.

If correcting a real Colnect candidate, retain its original `Col:` ID instead.
The importer recognizes the exact, case-sensitive prefix `Col:`, not `Colnect:`.
Without it, the `Link` value becomes the identity; multiple rows with blank links
and no `Col:` ID would replace one another.

Use `Sg:`, `Sn:`, `Mi:`, or `Yt:` for actual catalog numbers. CLI import uses the
configured default catalog to choose the preferred staged number; the current
Review UI import prefers SG. The review step lets you select the catalog number
to apply. Keep the country prefix if known (for example `Sg:CH 1041`). The catalog
number is not the candidate ID. Do not change the ID merely because a catalog
number needs correcting.

### Optional columns

Leave unknown values blank; do not type `NULL`.

| Headers | Meaning / recommended format |
|---|---|
| `Width`, `Height` | Stamp dimensions in millimeters, e.g. `24.5`, `30`; numbers only |
| `Watermark`, `Format`, `Emission` | Watermark, physical format (e.g. `Stamp`), issue type (e.g. `Commemorative`) |
| `Perforation` | Text such as `11.75` or `14 x 15` |
| `Printing`, `Paper`, `Gum` | Production details |
| `Ord Mi`, `Ord Sn`, `Ord Yt`, `Ord Sg` | Optional catalog sorting values; usually leave blank |
| `Currency`, `FaceValue` | Currency and denomination, e.g. `CHF`, `0.40` |
| `Print run` | Issued quantity, e.g. `1000000` |
| `Colors`, `Designers`, `Printers`, `Themes` | Descriptive metadata |
| `Description` | Detailed description or subject |
| `Expiry date` | If known, `YYYY-MM-DD` |
| `Parent Item`, `Variant`, `Overprint Variant` | Relationships/variant details; these do not automatically create links |
| `Score`, `Accuracy` | Optional Colnect-style metadata; usually leave blank |
| `List`, `Quantity`, `Condition` | Collection details, e.g. `Collection`, `1`, `Used` |
| `Public Note`, `Private Note` | Free text; both are stored locally and neither should be treated as encrypted/private storage |

Not every stored column is available as an independently selectable Review UI
update, and collection fields do not imply inventory/accounting functionality.
There is no image-file column; this template does not import local images.
Leave `Link` blank for manual-only entries to avoid treating a non-Colnect site
as a Colnect image source. Album output may use placeholders without images.

### Small example

This reduced example uses only core headers (the parser also accepts optional
columns being omitted). When filling the full template, keep all its columns.

```csv
Name,Country,Series,Catalog Codes,Issued on,Link
International Women's Year,Switzerland,1975 Commemoratives,"Sg:CH 1041, Col:manual-ch-0001",1975-02-13,
Humanitarian Law,Switzerland,1975 Commemoratives,"Sg:CH 1042, Col:manual-ch-0002",1975-02-13,
```

Quote cells containing commas, including multi-catalog `Catalog Codes` cells.
Inside a quoted cell, escape a quote by doubling it, for example
`"Design titled ""Peace"", blue"`. Prefer single-line descriptions; avoid blank
lines inside multiline cells because the current parser strips blank lines.
Do not add instructional text or demonstration rows to your real import file.

## 4. Import and review

Back up the country database before importing or applying edits. Never work on
a copy while the application is writing it; close the application for backups.

### Recommended: Review UI

Launch the Review UI for your country database:

```bash
./stamp_album_gen2 --review-ui \
  --db-path Albums/Switzerland/Switzerland.db --album-name Switzerland
```

In the active country's Colnect import section, use the CSV file picker to copy
your completed working CSV into the country's import folder, then run the import.
Alternatively, copy it into `Albums/Switzerland/colnect` yourself before importing.
The UI uses the configured `colnect_inbox_path` first if that folder contains CSV
files, otherwise the active album's `colnect` folder. An arbitrary `.csv` filename
works in this UI flow. Keep this folder country-specific: the UI imports into
the active database, not a database selected separately for every row.

Review the candidate on the right against the existing stamp on the left.
Assign it to the appropriate stamp, choose the supported fields to replace, and
apply the reviewed changes. Use the UI's add-stamp action for a genuinely new
stamp rather than assigning it to an unrelated existing stamp.

### CLI: one file

```bash
./stamp_album_gen2 --source colnectexport \
  --colnect-csv UserImport/Switzerland-manual-1975.csv \
  --db-path Albums/Switzerland/Switzerland.db --album-name Switzerland
```

This imports into the staging tables and then runs album generation using the
existing main stamp records. It does **not** apply the candidates to those records.
Open the Review UI afterward to approve additions or supported updates.

**Warning:** single-file CLI import clears the existing Colnect staging tables
before loading this file. It does not clear the main `stamps` table. Use the UI
or inbox mode to retain other staged candidates.

### CLI: inbox (preserves other staged candidates)

Inbox discovery currently accepts filenames starting with
`en_stamps_csv_list_countr` and ending with lowercase `.csv`. For manual files,
use the conventional pattern, for example:
`en_stamps_csv_list_country_206-Switzerland_year_1975.csv`.
The country/year in the filename must agree with your data. The numeric country
code is a filename convention; it is not the stamp's `Col:` ID.

```bash
./stamp_album_gen2 --source colnectexport \
  --colnect-inbox UserImport/completed \
  --colnect-create-country-db
```

Create the `completed` subfolder and put only filled-in files there. Inbox mode
recursively routes imports to country databases in the configured albums root
(default `Albums`). `--colnect-create-country-db` permits new country databases;
omit it when importing only into existing ones. Import is ingestion-only by
default. `--colnect-auto-route`, if added, moves input files into country
subfolders. Back up working CSVs outside the inbox first.

On Windows, use `.\stamp_album_gen2.exe` and enter each example command on one
line (the backslash line continuations above are for Linux shells). Quote paths
or country names containing spaces on either platform.

## 5. Corrections, re-imports, and limits

- Edit the working copy and keep the same `Col:` ID. Re-importing replaces that
  staged candidate's values; blank cells do not mean "keep the old staged value."
  Keep a complete row for each stamp rather than using sparse patch rows.
- Inbox duplicate detection uses the file's SHA256 checksum per country database.
  An unchanged file is skipped; changing its contents gives it a new checksum.
  Use `--colnect-force-reimport` only when deliberately re-importing an unchanged
  file. The UI reports already-present IDs as "already in database" even when
  their staged values were replaced; inspect the candidate values as well as
  its result counts/errors.
- A candidate already linked to a main stamp is marked processed and may no longer
  appear as an available right-side candidate. Re-importing does not automatically
  refresh the main stamp. To re-review, use the UI's clear-link action on the
  linked stamp, then reload/reassign the candidate and explicitly apply changes.
  Back up first: clearing a link changes stored linkage/metadata.
- Applying review changes uses the UI's selected fields. This is not a bulk
  overwrite, arbitrary database-column editor, deletion tool, or blank-value
  clearing mechanism. Some fields retain existing values when the candidate is
  blank. Review the result before generating pages.
- Removing a CSV row does not delete the corresponding main stamp. Inbox/UI
  import does not remove unrelated staged rows; single-file CLI import does.
- Catalog-code subrecords use the existing importer behavior: replacing a row
  may retain older catalog types omitted from the edited `Catalog Codes`.
  Do not assume removing a catalog entry from the CSV deletes all its old data.
- Parser warnings identify malformed rows, but the current importer can continue
  with valid rows. Check reported counts and errors, not just command completion.
  If fewer stamps were parsed than expected, correct the CSV before approving.

## Troubleshooting

| Symptom | Check |
|---|---|
| Template missing | Run a normal launch; check the runtime folder and startup permission warnings |
| `Could not find CSV header starting with 'Name,Country'` | First two headers, UTF-8 without BOM, comma delimiter, no quoted first headers |
| CSV record parse warning | Exact headers, balanced quotes, complete cells, unchanged column count |
| Multiple manual rows collapse into one | Give every row a nonempty, unique `Col:manual-...` ID |
| No matching inbox files | Use the filename pattern above, lowercase `.csv`, and the correct inbox folder |
| UI imports another folder | Check `colnect_inbox_path`; the first candidate folder containing CSV files wins |
| Wrong country database | Check active UI country/database or inbox filename; use one country per file |
| Existing album unchanged | Import only stages candidates; approve updates/additions in the Review UI |
| Already linked candidate missing | Review the processed/linkage behavior above before clearing a link |
