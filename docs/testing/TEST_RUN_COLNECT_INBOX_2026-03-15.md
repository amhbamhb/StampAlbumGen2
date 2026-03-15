# Colnect Inbox Test Run Report (2026-03-15)

## Command

```bash
cargo run -- --source colnectexport --colnect-inbox . --colnect-auto-route --colnect-create-country-db
```

## High-Level Outcome

- Result: **partial success**
- Final runtime summary: `imported=2, skipped_duplicates=2, skipped_missing_db=0, failed=5`
- Imported successfully:
  - Australia file (`41` rows)
  - Netherlands file (`30` rows)
- Failed:
  - Switzerland files (`2` files)
  - Canada files (`3` files)

## Filesystem Impact

### Source files moved by auto-route

- `en_stamps_csv_list_country_13-Australia_year_1975.csv` -> `Australia/en_stamps_csv_list_country_13-Australia_year_1975.csv`
- `en_stamps_csv_list_country_151-Netherlands_year_1970.csv` -> `Netherlands/en_stamps_csv_list_country_151-Netherlands_year_1970.csv`
- `en_stamps_csv_list_country_206-Switzerland_year_1972.csv` -> `Switzerland/en_stamps_csv_list_country_206-Switzerland_year_1972.csv`
- `en_stamps_csv_list_country_206-Switzerland_year_1973.csv` -> `Switzerland/en_stamps_csv_list_country_206-Switzerland_year_1973.csv`
- `en_stamps_csv_list_country_38-Canada_series_116111-Centennial_Definitives_1967-73_-_Low_Values_year_1967.csv` -> `Canada/en_stamps_csv_list_country_38-Canada_series_116111-Centennial_Definitives_1967-73_-_Low_Values_year_1967.csv`
- `en_stamps_csv_list_country_38-Canada_year_1967 (1).csv` -> `Canada/en_stamps_csv_list_country_38-Canada_year_1967 (1).csv`
- `en_stamps_csv_list_country_38-Canada_year_1967.csv` -> `Canada/en_stamps_csv_list_country_38-Canada_year_1967.csv`

### Additional moved inputs from `colnect/`

- `colnect/en_stamps_csv_list_country_206-Switzerland_year_1972.csv` -> `Switzerland/en_stamps_csv_list_country_206-Switzerland_year_1972.csv`
- `colnect/en_stamps_csv_list_country_206-Switzerland_year_1973.csv` -> `Switzerland/en_stamps_csv_list_country_206-Switzerland_year_1973.csv`

### New top-level country folders created

- `Australia/`
- `Canada/`
- `Netherlands/`
- `Switzerland/`

## Database-Level Impact

### Australia

- DB: `Albums/Australia/Australia.db`
- `colnect_stamps` schema includes `name` column.
- Import history: `success=1`
- `colnect_stamps` rows after run: `41`
- File: `./Australia/en_stamps_csv_list_country_13-Australia_year_1975.csv`

### Netherlands

- DB: `Albums/Netherlands/Netherlands.db`
- `colnect_stamps` schema includes `name` column.
- Import history: `success=1`
- `colnect_stamps` rows after run: `30`
- File: `./Netherlands/en_stamps_csv_list_country_151-Netherlands_year_1970.csv`

### Switzerland

- DB: `Albums/Switzerland/Switzerland.db`
- `colnect_stamps` schema **does not** include `name` column.
- Import history: `failed=2`
- `colnect_stamps` rows after run: `0`
- Files:
  - `./Switzerland/en_stamps_csv_list_country_206-Switzerland_year_1972.csv`
  - `./Switzerland/en_stamps_csv_list_country_206-Switzerland_year_1973.csv`

### Canada

- DB: `Albums/Canada/Canada.db`
- `colnect_stamps` schema **does not** include `name` column.
- Import history: `failed=3`
- `colnect_stamps` rows after run: `0`
- Files:
  - `./Canada/en_stamps_csv_list_country_38-Canada_series_116111-Centennial_Definitives_1967-73_-_Low_Values_year_1967.csv`
  - `./Canada/en_stamps_csv_list_country_38-Canada_year_1967 (1).csv`
  - `./Canada/en_stamps_csv_list_country_38-Canada_year_1967.csv`

## Error Inventory

- Error message seen on failed country DBs:
  - `table colnect_stamps has no column named name`
- Affected countries:
  - Switzerland
  - Canada

## Duplicate-Handling Observation

- Run summary reported `skipped_duplicates=2`.
- Cause in this test: duplicate Switzerland files existed in multiple locations (`./colnect/...` and `./...`), producing the same checksum.
- Current logic checks checksum presence only; it does not distinguish success vs failed prior import status.

## Logic-Risk Notes (Follow-up)

1. Schema migration gap:

- Older country DBs (Canada/Switzerland) have legacy `colnect_stamps` schema missing required columns (e.g., `name`).
- Need migration path for `colnect_stamps` similar to `stamps` table migration strategy.

1. Duplicate skip semantics:

- Duplicate check currently treats prior failed imports as already imported for checksum checks.
- Consider skipping duplicates only for prior `success` status, not `failed`.

1. Auto-route side effect:

- Top-level CSV test files were moved out of root to country folders.
- This is expected with `--colnect-auto-route`, but impacts existing manual test habits and git status.

## Country Summary Table

| Country | Files discovered/processed | Imported | Skipped duplicate | Failed | Top failure reason |
|---|---:|---:|---:|---:|---|
| Australia | 1 | 1 | 0 | 0 | N/A |
| Netherlands | 1 | 1 | 0 | 0 | N/A |
| Switzerland | 2 (+ duplicate discovery paths) | 0 | 2 | 2 | `colnect_stamps` missing `name` |
| Canada | 3 | 0 | 0 | 3 | `colnect_stamps` missing `name` |

---

## Run 2 (After Schema Migration Fix)

### Command

```bash
cargo run -- --source colnectexport --colnect-inbox . --colnect-auto-route --colnect-create-country-db
```

### Code Changes Applied Before Run 2

1. Added legacy schema migration in `init_db` to ensure `colnect_stamps` has all expected columns (including `name`) for older country DBs.
2. Adjusted duplicate check to consider only prior `success` checksums as duplicates, so previously failed files can be retried automatically.

### High-Level Outcome (Run 2)

- Result: **success with expected duplicate skips**
- Final runtime summary: `imported=5, skipped_duplicates=2, skipped_missing_db=0, failed=0`

### Post-Run Verification

#### Schema fix verification
- `Albums/Canada/Canada.db`: `colnect_stamps.name` column exists (`COUNT=1`)
- `Albums/Switzerland/Switzerland.db`: `colnect_stamps.name` column exists (`COUNT=1`)

#### Per-country ingest results
- Australia: `colnect_stamps_rows=41`, history `success:1`
- Canada: `colnect_stamps_rows=100`, history `success:3`
- Netherlands: `colnect_stamps_rows=30`, history `success:1`
- Switzerland: `colnect_stamps_rows=37`, history `success:2`

### Import History Snapshot (Run 2 state)

- Australia
  - `success | en_stamps_csv_list_country_13-Australia_year_1975.csv | rows=41`
- Canada
  - `success | ...Centennial_Definitives_..._year_1967.csv | rows=100`
  - `success | en_stamps_csv_list_country_38-Canada_year_1967 (1).csv | rows=100`
  - `success | en_stamps_csv_list_country_38-Canada_year_1967.csv | rows=100`
- Netherlands
  - `success | en_stamps_csv_list_country_151-Netherlands_year_1970.csv | rows=30`
- Switzerland
  - `success | en_stamps_csv_list_country_206-Switzerland_year_1972.csv | rows=26`
  - `success | en_stamps_csv_list_country_206-Switzerland_year_1973.csv | rows=37`

### Filesystem Notes (Run 2)

- Routed country files remained in place under:
  - `Australia/`
  - `Canada/`
  - `Netherlands/`
  - `Switzerland/`
- Total routed CSV files observed: `7`

### Logic-Risk Follow-up Status

1. Schema migration gap: **fixed for tested legacy DBs** (Canada/Switzerland).
2. Duplicate retry behavior after failed runs: **improved** by only treating prior `success` as duplicate.
3. Auto-route side effects on source file locations: unchanged by this fix.

---

## Run 3 (Reset Inputs To Root + Complete Rerun)

### Pre-run setup

- Moved all test files back to workspace root before test.
- Root file count before run: `7` (`en_stamps_csv_list_countr*.csv`)

### Command

```bash
cargo run -- --source colnectexport --colnect-inbox . --colnect-auto-route --colnect-create-country-db
```

### High-Level Outcome (Run 3)

- Result: **success**
- Final runtime summary: `imported=0, skipped_duplicates=7, skipped_missing_db=0, failed=0`

### Why imported=0

- All 7 files had previously successful checksum records in each target country DB.
- Duplicate protection correctly skipped them.

### Filesystem Verification

- Root file count after run: `0`
- All test files were auto-routed into album-local folders:
  - `Albums/Australia/colnect/...`
  - `Albums/Canada/colnect/...`
  - `Albums/Netherlands/colnect/...`
  - `Albums/Switzerland/colnect/...`

### Import History Verification

- Australia: `success|1`
- Canada: `success|3`
- Netherlands: `success|1`
- Switzerland: `success|2`

### Conclusion (Run 3)

- Routing behavior is now consistent with per-country album structure.
- Duplicate detection works as intended for repeat runs.
- No schema or ingestion errors were observed.
