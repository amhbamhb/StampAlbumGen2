# Colnect Data Linking Feature

## Overview

This feature allows the Stamp Album Generator to ingest a Colnect CSV file and automatically link the stamp data to the existing StampWorld data in the database. This enriches the album output with additional catalog codes (Michel, Scott, Yvert, etc.) and detailed descriptions.

## How It Works

1. **Ingestion**: The system reads the CSV file specified by the `--colnect-csv` argument.
2. **Matching Logic**:
   - **Primary Key**: Issue Date (Year, Month, Day).
   - **Secondary Key**: Face Value (e.g., "10c", "1.20 Fr").
   - **Disambiguation**: If multiple stamps share the same Date and Face Value, the system uses a **Fuzzy String Matching** algorithm (Jaro-Winkler) to compare the Series and Description fields.
3. **Linking**: When a match is found, the Colnect ID and Catalog Codes are stored in the database (`colnect_stamps` and `colnect_catalog_codes` tables).
4. **Output**: The generator retrieves these codes and prints them in the stamp header (e.g., `SG : 969  Mi: CH 975 ...`).

## Usage

Run the generator with the `--colnect-csv` flag:

```bash
cargo run -- \
    --album-name Switzerland \
    --start-year 1972 \
    --end-year 1972 \
    --colnect-csv en_stamps_csv_list_country_206-Switzerland_year_1972.csv
```

## Verification Results

* **Single Matches**: Stamps with unique Date/Value pairs are linked automatically.
* **Ambiguous Matches**:
  * *Case*: 1972-09-21 had two series ("Civil Defense" and "Portraits") with identical face values (10c, 20c, etc.).
  * *Result*: The fuzzy matcher successfully distinguished them.
    * SG 969 (Civil Defense) -> Linked to Civil Defense CSV.
    * SG 973 (Portraits) -> Linked to Famous People CSV.
* **Safety**: Stamps with insufficient data (e.g., missing description) to resolve an ambiguity are left unlinked to prevent errors.

## Database Changes

* New Table: `colnect_catalog_codes` (stores Mi, Sn, Yt, Sg, AFA, Un, Zum codes).
* Updated Table: `colnect_stamps` (added `series` column for matching).

## Dimension Sizing Update
*   **Feature**: If Colnect data is linked and contains Width/Height information, these dimensions are now used as the authoritative source for the stamp size.
*   **Benefit**: This provides more accurate sizing than estimating from image dimensions or using defaults.
*   **Verification**:
    *   SG 969 (Civil Defense): Colnect 36.5x26.5mm -> Output 37.5x27.5mm (including 0.5mm margin).
    *   SG 973 (Portraits): Colnect 26x36mm -> Output 27.0x37.0mm.
