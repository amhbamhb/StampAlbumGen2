# Album Author and Title Metadata Example (album-author-v1)

This fixture demonstrates and validates the AlbumEasy 4.5.0 commands `ALBUM_AUTHOR` and `ALBUM_TITLE`, which configure document-level metadata (`/Author` and `/Title`) in the generated PDF.

## Fixture Overview

| Input | Reference PDF (AlbumEasy 4.5.0) | Pass 2 Native Album | Native Direct TXT/Raw |
|---|---|---|---|
| [basic.txt](basic.txt) | [basic.pdf](basic.pdf) | [basic.stampalbum](basic.stampalbum) | [basic-raw.stampalbum](basic-raw.stampalbum) |
| [album.txt](album.txt) | [album.pdf](album.pdf) | [album.stampalbum](album.stampalbum) | [album-raw.stampalbum](album-raw.stampalbum) |

### Tested Commands

- `ALBUM_AUTHOR ("<author-string>")`: Sets the document `/Author` metadata property.
- `ALBUM_TITLE ("<title-string>")`: Sets the document `/Title` metadata property.

Both commands are scoped to initial page settings before `PAGE_START`. In AlbumEasy and our native loader, command names are case-insensitive (`Album_Author`, `album_title`, `ALBUM_AUTHOR`, `ALBUM_TITLE`).

## Reference Files & Provenance

The reference PDFs were rendered with official AlbumEasy 4.5.0.

| File | SHA-256 | Description |
|---|---|---|
| `basic.txt` | `39ad55247b07e389df4866ac402cf1ab3e7d60d9bd4a97808eec0066bf3464fd` | Direct page start layout with author & title |
| `basic.pdf` | `e64f398b7fec0a8939f5ef33e3ef39216dacce23f0702a3ef48c0ecdefd1b01d` | AlbumEasy 4.5.0 rendered reference |
| `album.txt` | `5f95b50f2961d2c4ee326f40bbdfdc8a2eec5603edfcb47f6739af653103402c` | Template group page start with author & title |
| `album.pdf` | `684854d82b006d39ced9c376e1ce4cb9df8f7ef5debecad059e1b027220bb135` | AlbumEasy 4.5.0 rendered reference |

### Stamp Asset Provenance

Images are sourced from the real-stamp pagination fixture (`../stamp-pagination-v1/`):
- `us-150.png`: DB ID 150, 1959 SG 1125a, Colnect ID 700029, physical size 52 x 29 mm.
- `us-151.png`: DB ID 151, 1959 SG 1137a, Colnect ID 700273, physical size 26 x 29 mm.

## Pass 2 Compatibility Files

1. **`basic.stampalbum` / `album.stampalbum`**: Native `STAMPALBUM 1` documents specifying `ALBUM_AUTHOR` and `ALBUM_TITLE` within the `settings` block.
2. **`basic-raw.stampalbum` / `album-raw.stampalbum`**: Raw AlbumEasy command streams verifying direct native loader compatibility.

## Document Metadata Inspection

In AlbumEasy 4.5.0 output, metadata strings are encoded as UTF-16BE hex strings in the PDF trailer Information dictionary:
- `/Author <FEFF004A006F0068006E00200044006F0065>` -> `"John Doe"`
- `/Title <FEFF004C006900620065007200740079002000490073007300750065>` -> `"Liberty Issue"`

In native PDF output (`oxidize_pdf`), metadata strings are written as standard PDF string literals or encoded strings:
- `/Author (John Doe)`
- `/Title (Liberty Issue)`

Both representations are verified by automated tests.

## Automated Verification

Run the metadata test suite:

```sh
cargo test --test native_pdf_album_metadata
```

This verifies:
1. Reference PDFs contain expected `/Author` and `/Title`.
2. Native rendering of `.txt` and `-raw.stampalbum` documents correctly sets metadata fields.
3. Native parsing and rendering of `.stampalbum` documents correctly sets metadata fields.
4. Case-insensitivity (`ALBUM_AUTHOR`, `Album_Author`, `album_author`).
5. Validation errors on missing, unquoted, or duplicate arguments.
