# Refactor Inventory

Generated: exact scan of Rust source files under `src/` and `crates/`.

Notes on methodology
- LOC: physical line count (raw lines, counted by reading the file and doing newline count + 1). This includes blank lines and comments.
- #pub items: counts occurrences of top-level `pub` declarations starting lines (counts lines starting with `pub ` that are `pub fn`, `pub struct`, `pub enum`, `pub trait`, `pub mod` and `pub(...)` variants).
- Dependents: simple inbound dependents computed by searching other source files for references to the module basename (examples matched: `use crate::<basename>`, `crate::<basename>`, `use <basename>::`, or `mod <basename>`). This is a conservative, text-based estimate of direct references.

CSV (path, loc, num_pub, dependents_paths)

"path","loc","num_pub","dependents"
"crates/stamp_album_db/src/lib.rs",958,9,""
"crates/stamp_album_db/tests/unit_tests.rs",215,0,""
"src/album_globals.rs",1629,14,"src/commands_file_generator.rs;src/data_sources/stampworld.rs;src/lib.rs;src/stampworld/mod.rs"
"src/album_utils.rs",977,5,"src/album_globals.rs;src/commands_file_generator.rs;src/lib.rs;src/stampworld/data_collector.rs;src/stampworld/mod.rs"
"src/bin/backfill_image_dims.rs",138,0,""
"src/bin/insert_test.rs",57,0,""
"src/bin/migrate_fix_block_metadata.rs",40,0,""
"src/bin/migrate_stamps.rs",79,0,""
"src/commands_file_generator.rs",985,23,"src/lib.rs"
"src/config.rs",501,4,"src/album_globals.rs;src/album_utils.rs;src/commands_file_generator.rs;src/lib.rs;src/stampworld/data_collector.rs"
"src/data_sources/mod.rs",4,0,""
"src/data_sources/stampworld.rs",375,1,"src/commands_file_generator.rs;src/data_sources/mod.rs;src/db_utils.rs;src/lib.rs"
"src/db_utils.rs",1487,14,"src/album_globals.rs;src/album_utils.rs;src/data_sources/stampworld.rs;src/image_downloader.rs;src/lib.rs;src/stampworld/data_collector.rs"
"src/file_utils.rs",79,5,"src/lib.rs"
"src/flagdb.rs",261,5,"src/album_utils.rs;src/lib.rs"
"src/image_downloader.rs",224,3,"src/lib.rs"
"src/image_utils.rs",125,3,"src/album_globals.rs;src/album_utils.rs;src/commands_file_generator.rs;src/lib.rs;src/stampworld/data_collector.rs"
"src/lib.rs",43,14,""
"src/main.rs",479,0,""
"src/placeholder_image.rs",398,7,"src/lib.rs"
"src/sample_images/mod.rs",69,1,""
"src/stampworld/data_collector.rs",1561,5,"src/stampworld/mod.rs"
"src/stampworld/mod.rs",26,2,""
"src/stampworld/state.rs",108,0,"src/stampworld/mod.rs"
"src/types.rs",116,7,"src/commands_file_generator.rs;src/file_utils.rs;src/lib.rs;src/stampworld/data_collector.rs"
"src/wikipedia.rs",35,1,"src/lib.rs"


Top-5 refactor candidates (ranked by LOC with inbound coupling considered)

1) src/album_globals.rs (LOC: 1629, pub items: 14, inbound dependents: 4)
- Rationale: Largest single file. Provides album generation logic and is referenced from multiple places (commands generator, stampworld glue, lib). High complexity and cross-cutting responsibilities. Good candidate to split into smaller modules: layout/math, rendering, connectors.

2) src/stampworld/data_collector.rs (LOC: 1561, pub items: 5, inbound dependents: 1)
- Rationale: Long web-scraping and HTML-parsing heuristics, DB insert logic, image download interactions. Mixing network, parsing, and persistence. Extract parsing and network/IO (downloader) out first, then isolate DB persistence into the `crates/stamp_album_db` API.

3) src/db_utils.rs (LOC: 1487, pub items: 14, inbound dependents: 6)
- Rationale: Large compatibility shim and DB helper code. Many modules depend on it. This is a top candidate for consolidation with `crates/stamp_album_db` or for extracting a clear API surface so other modules depend on the crate rather than the shim.

4) src/commands_file_generator.rs (LOC: 985, pub items: 23, inbound dependents: 1)
- Rationale: Orchestrates collection and image download and album file creation. Lots of I/O responsibilities (download, file writes). Can be split into downloader module, generator orchestration, and file-writing helpers.

5) src/album_utils.rs (LOC: 977, pub items: 5, inbound dependents: 5)
- Rationale: Album utility functions used heavily by album_globals and other modules. Extract coherent helper sets (image layout, positioning, commons) and add focused unit tests per extracted module.


Suggested next steps (one-file-at-a-time plan)
- Step A (recommended first): Extract DB implementation concerns from `src/db_utils.rs` into `crates/stamp_album_db` (if not already present). Make `src/db_utils.rs` a thin facade delegating to the crate; update imports across the workspace. Benefits: reduces coupling and centralizes persistence.
- Step B: Split `src/stampworld/data_collector.rs` into `stampworld/parser.rs` (parsing logic), `stampworld/scraper.rs` (HTTP + fetch logic), and `stampworld/persistence.rs` (DB insert calls). Keep the downloader closure boundary to minimize diffs.
- Step C: Break `src/album_globals.rs` into `album/layout.rs`, `album/render.rs`, and `album/output.rs` with small, tested APIs. Move types to `src/types.rs` or new `album/types.rs` as needed.
- Step D: For each extraction, run tests and the build; make small commits with descriptive messages: "extract <component> from <file> into <module>, add tests".

Quality gates
- After each file extraction: run `cargo build` and `cargo test` — ensure green before next extraction.
- Add or extend unit tests focused on the extracted module boundary (happy paths + at least one edge case each).

If you'd like, I can now:
- 1) Commit this `refactor_inventory.md` (already created), and
- 2) Start the Step A extraction by creating a small, type-preserving wrapper or moving functions (one-file refactor) and run `cargo test` to validate.

If you prefer different ranking criteria (exclude test files, exclude small bins, or count only non-blank lines / code-only LOC), tell me and I will re-run with those rules.
