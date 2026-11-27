# Page Breaks & Group Templates (`page-start-v1`)

This fixture family tests and showcases explicit page breaks, group header templates, and intentional blank pages in StampAlbumGen2 across both the native PDF engine and raw AlbumEasy format.

---

## 1. Stamp Collector Guide (User-Friendly Overview)

### Why Explicit Page Starts Matter in Stamp Albums

When organizing a philatelic collection, you rarely want stamps to spill across page boundaries randomly simply because space ran out:

- **Thematic & Series Grouping**: Each issue or commemorative set (e.g. the 1954–1968 US Liberty Issue) belongs on its own distinct page or sequence of pages, even if unused vertical space remains at the bottom of the preceding sheet.
- **Controlled Row Flushing**: An explicit page break command (`PAGE_START` or `NATIVE_PAGE_BREAK`) guarantees that the preceding row of stamps is properly completed and flushed onto the current page before the renderer advances to the next clean page.
- **Consistent Page Layouts**: Album borders, page headers, titles, and page numbering automatically repeat on every newly created page without manual re-declaration.

### Reusable Page Templates (Group Substitutions)

Collectors often want recurring headers with small variations (e.g. Country name, Issue Year, Series Title). Rather than copying and pasting lengthy text commands on every page, AlbumEasy and StampAlbumGen2 support **Page Start Groups**:

- Define a template once before your pages begin using `PAGE_START_GROUP_BEGIN` and `PAGE_START_GROUP_END`.
- Placeholders such as `$$1`, `$$2`, `$$3` act as variables within text commands.
- When calling `PAGE_START ("United States" 1959 "Definitive Series")`, the placeholders are instantly replaced with that page's specific details.

### Intentional Blank Pages

Traditional stamp albums and exhibition layouts frequently require intentional blank pages:

- **Glassine Interleaving & Facing Pages**: Inserting a blank page allows interleaving protection or provides room on the facing (verso) page for mounting postal history covers, first-day covers, or detailed research write-ups.
- **Section Dividers**: Blank pages can serve as quiet dividers between postal eras, reigns, or specialized definitive issues.
- **Graceful Rendering**: Consecutive page breaks (`NATIVE_PAGE_BREAK ()` or `PAGE_START`) cleanly produce bordered, numbered blank pages without layout collapse or software errors.

---

## 2. Technical Reference & Command Grammar

### Grammar & Argument Variations

```text
PAGE_START
PAGE_START ( arg1 arg2 ... arg9 )
NATIVE_PAGE_START
NATIVE_PAGE_START ( [horizontal_spacing vertical_spacing] )
NATIVE_PAGE_BREAK
NATIVE_PAGE_BREAK ( [horizontal_spacing vertical_spacing] )
PAGE_START_VAR ( horizontal_spacing vertical_spacing )
```

| Command / Argument    | Type              | Default   | Description                                                                               |
| :-------------------- | :---------------- | :-------- | :---------------------------------------------------------------------------------------- |
| `PAGE_START` (0 args) | Command           | None      | Ends the current page, flushes any active stamp row, and begins a new page.               |
| `PAGE_START` (args)   | Strings / Numbers | None      | Invokes the pre-defined `PAGE_START_GROUP` template, mapping arguments to `$$1` .. `$$9`. |
| `NATIVE_PAGE_START`   | Command           | None      | Native alias for explicit page starts with optional spacing overrides.                    |
| `NATIVE_PAGE_BREAK`   | Command           | None      | Native alias for explicit page starts with optional `(h_spacing v_spacing)` parameters.   |
| `PAGE_START_VAR`      | Floats (mm)       | Inherited | AlbumEasy command setting per-page horizontal and vertical row spacing.                   |

### Group Template Syntax

```text
PAGE_START_GROUP_BEGIN
PAGE_TEXT_CENTRE (TB 11 $$1 5)
PAGE_TEXT_CENTRE (HN 9 "Year: $$2" 5)
PAGE_START_GROUP_END
```

- Only one page start group may be defined per document, positioned prior to the first page start.
- Placeholders `$$1` through `$$9` are substituted with arguments passed in each subsequent `PAGE_START (...)` invocation.
- Quoted multiword arguments retain their quotes and handle whitespace seamlessly.

---

## 3. Fixture Variations Inventory

| Fixture Name             | Variation / Purpose                                                                  | Native Format                                                  | Raw AlbumEasy Format                                                   | Native PDF                                                     |
| :----------------------- | :----------------------------------------------------------------------------------- | :------------------------------------------------------------- | :--------------------------------------------------------------------- | :------------------------------------------------------------- |
| `basic`                  | Two explicit zero-argument page starts flushing preceding rows                       | [basic.stampalbum](basic.stampalbum)                           | [basic-raw.stampalbum](basic-raw.stampalbum)                           | [basic-native.pdf](basic-native.pdf)                           |
| `album`                  | Page-start group template with placeholder expansions across 2 pages                 | [album.stampalbum](album.stampalbum)                           | [album-raw.stampalbum](album-raw.stampalbum)                           | [album-native.pdf](album-native.pdf)                           |
| `blank-pages`            | Leading, consecutive, and trailing breaks creating intentional blank pages (5 pages) | [blank-pages.stampalbum](blank-pages.stampalbum)               | [blank-pages-raw.stampalbum](blank-pages-raw.stampalbum)               | [blank-pages-native.pdf](blank-pages-native.pdf)               |
| **`all-in-one-compare`** | **Comprehensive 9-page showcase covering ALL variations in a single document**       | [all-in-one-compare.stampalbum](all-in-one-compare.stampalbum) | [all-in-one-compare-raw.stampalbum](all-in-one-compare-raw.stampalbum) | [all-in-one-compare-native.pdf](all-in-one-compare-native.pdf) |

---

## 4. Physical Sizing & Stamp Provenance

Stamps utilized in these fixtures are sourced from the US 1954–1968 Liberty Issue records in `tests/fixtures/albumeasy/stamp-pagination-v1/source-records.json`:

| DB ID | SG Catalog | Physical Size (W × H) |  Frame Bounds  | Image File                          |
| :---: | :--------: | :-------------------: | :------------: | :---------------------------------- |
|  150  |   1125a    |    52.0 × 29.0 mm     | 58.0 × 35.0 mm | `../stamp-pagination-v1/us-150.png` |
|  151  |   1137a    |    26.0 × 29.0 mm     | 32.0 × 35.0 mm | `../stamp-pagination-v1/us-151.png` |
|  152  |   1137b    |    26.0 × 29.0 mm     | 32.0 × 35.0 mm | `../stamp-pagination-v1/us-152.png` |
|  156  |   1148a    |    52.0 × 29.0 mm     | 58.0 × 35.0 mm | `../stamp-pagination-v1/us-156.png` |
