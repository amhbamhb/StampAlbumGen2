# Copilot Agent — DB import instructions

Purpose

- Ensure all DB import work for StampAlbumGen2 consistently references the sample StampWorld webpage provided in the repository.

Canonical sample file

- Path: /home/ray/program/Rust/StampAlbumGen2/Example-Stampworldwebpage.txt
- This file contains the canonical HTML structure and example data to be used when parsing and mapping stamp records into the database.

Rules for DB import tasks

1. Always load and reference the canonical sample file above before creating or modifying any import logic or parsers.
2. Use the HTML structure and classes/attributes in the sample file as the authoritative mapping for extraction. Key elements to map:
   - outer container: div.container-fluid.content_table with id "group_box_<group_id>"
   - header fields inside .table_header: title link text (year + description), date, designer, engraving, perforation
   - images: spans with class stamp_img and attributes data-stamp-type and data-stamp-group-id; image src and alt provide type and image URL
   - table rows: tr.stamp_tr containing data-stamp-group-id and data-stamp-type; per-row number in &lt;a name&gt; / id; type column; D column; color column; quantity in parentheses in the hidden-xs column; price columns and currency cells
3. Preserve stamp groupings: group-level summary rows (e.g. combined group 1040-1043) must be recognized separately from individual stamp rows.
4. Handle multiple stamp types per group (ANZ, AOA, AOB, AOC in the sample) and map them to separate DB records as appropriate.
5. Use the sample file to create unit tests for parsers so that future changes to parsing logic can be validated against this canonical example.

Notes

- If additional sample pages from stampworld.com are added, include them in docs/ and update this instruction file to reference them.
- The sample file is authoritative for field names and HTML structure for import tasks unless explicitly replaced by a newer canonical sample.

Debugging outside VS Code

-------------------------

If running with RUST_LOG=debug inside VS Code causes issues (e.g. terminal crashes), use the helper script instead which runs the app in the background and writes a timestamped debug log under `Logs/`:

```
./scripts/run_debug_no_vscode.sh -- --source stampworld --site-name Switzerland --album-name Switzerland --start-year 1976 --end-year 1976
```

This will write output to `Logs/debug-run-<timestamp>.log` and run the process in the background. Use `tail -f` on the logfile to follow progress.

Contact

- If the sample file appears to diverge from live site structure, document the differences and add an updated example file under docs/ before changing parser logic.
