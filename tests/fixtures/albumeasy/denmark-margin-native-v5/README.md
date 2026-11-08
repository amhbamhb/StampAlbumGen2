# Denmark 1989: margin-colour native proof v5

This derivative restores only original line 45 `COLOR_ALBUM_MARGIN_TXT`
from [v4](../denmark-settings-native-v4/README.md). All v1-v4 sources,
assets and PDFs remain untouched.

| Input | Direct native output |
|---|---|
| [Supported TXT](Denmark-1989-1989-supported.txt) | [Native PDF](Denmark-1989-1989-native.pdf) |
| [Identical raw `.stampalbum`](Denmark-1989-1989-raw.stampalbum) | [Raw-native PDF](Denmark-1989-1989-raw-native.pdf) |

Both outputs contain **16 pages, 41 visible images, every `Page N of 16`
footer and zero commented unsupported occurrences**. Historical deferrals:
v1=8, v2=5, v3=3, v4=1, v5=0. The prepared source preserves the original
216.0 x 279.4 mm paper dimensions, not exact 215.9-mm Letter width.

Denmark has no active margin-text definition. This restoration verifies
colour-setting acceptance without inventing visible side text; actual
vertical text is covered by the
[margin-text references and three-input proofs](../margin-text-v1/README.md).
Zero deferrals certifies this managed-asset derivative's bounded command
sequence only, not every AlbumEasy command/overload, unprepared asset paths,
whole-page parity or production integration.

## Reproduction / execution report

From repository root on Linux or Windows:

```sh
cargo test --test native_pdf_margin_text regenerate_denmark_margin_proofs -- --ignored --nocapture
```

Exit 0: 1 passed, zero failed. Each route verifies page/image counts, all
footers and zero remaining deferrals; the generator checks v4 source bytes
unchanged. Relative asset paths reuse the original
[v1 bundle](../denmark-txt-native-v1/README.md), without copying/resizing images.

```sh
pdftoppm -f 6 -l 6 -scale-to 1200 -singlefile -png tests/fixtures/albumeasy/denmark-margin-native-v5/Denmark-1989-1989-native.pdf <session-preview>
```

Page 6 was visually inspected. Existing black sheet/background, real images,
border and numbered footer remain. Dense caption overlap, historical
reference differences, compatibility 72-dpi image sizing and roughly 44.7 MB
PDFs remain known follow-up risks, not fixed in this margin-colour slice.
No whole-page pixel parity or human Actual-size print acceptance is claimed.
Windows runtime remains unverified; portable Rust/relative paths are used.

Filesystem impact: created this directory, two input copies, two native PDFs
and README, plus fixture-local PDF binary attributes. No earlier references, assets or original inputs were modified,
moved, deleted or relocated. Preview PNGs/logs remain session-only.
Database/ingest impact: none; no DB opened or written, schema checks/row
insertions/updates/skips/duplicate detection and inbox country tables are
not applicable. No errors occurred in regeneration.

The [margin report](../margin-text-v1/README.md) records accepted AlbumEasy
evidence, explicit native grammar, exact test commands/results and remaining
compatibility boundaries. Commit/push requires subsequent maintainer
instruction; production integration, PR and OKF remain deferred.
