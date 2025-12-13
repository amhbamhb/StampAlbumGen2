# Colnect Data Integration Plan

This document outlines the steps to integrate Colnect CSV data into the StampAlbumGen2 application.

## 1. Database Schema Update

- [x] Create a new table `colnect_stamps` in the SQLite database.
- [x] Define columns to match the CSV structure and useful fields:
  - `id` (Primary Key)
  - `sg_number` (Foreign Key link, indexed)
  - `colnect_id` (Unique identifier from Colnect)
  - `catalog_codes` (Full string)
  - `width`, `height`
  - `printing`, `paper`, `gum`
  - `themes`
  - `colnect_link`
  - `description` (Colnect version)
  - `issued_on`
  - `print_run`
  - `face_value`
  - `currency`
  - `colors`
  - `designers`
  - `printers`

## 2. CSV Parser Implementation

- [x] Create a new module `src/colnect/parser.rs` (or similar).
- [x] Define a struct `ColnectStamp` that maps to the CSV columns using `serde`.
- [x] Implement a function to read the CSV file and return a vector of `ColnectStamp`.

## 3. Data Ingestion Logic

- [x] Create a module `src/colnect/ingest.rs`.
- [x] Implement logic to iterate through parsed `ColnectStamp` records.
- [x] Extract `sg_number` from the "Catalog Codes" column (e.g., parse "Sg:CH 824" -> "824" or "CH 824").
- [x] Insert records into `colnect_stamps` table.

## 4. Data Linking & Retrieval

- [x] Update `src/album_utils/db_loader.rs` or create a new loader.
  - *Note: Implemented in `src/colnect/db.rs` and integrated in `src/album_globals/generator_new/helpers.rs`*
- [x] Implement a JOIN query or separate fetch to retrieve Colnect data for a given stamp.
- [x] Update `StampStub` or create a `ExtendedStamp` struct to hold the combined data.
  - *Note: Integrated into `Stamp` attributes.*

## 5. UI/Output Integration

- [x] Update the album generator (`src/album_generator.rs` or similar) to use the extended data.
  - *Note: Data is now available in `Stamp.attrs` as `colnect_*` fields.*
- [ ] Display new fields (Themes, Printing, etc.) in the generated album pages (HTML/PDF).

## 6. Testing & Verification

- [x] Create unit tests for the CSV parser.
- [x] Create integration tests for the DB ingestion and retrieval.
- [x] Verify with the provided sample CSV: `en_stamps_csv_list_country_206-Switzerland_year_1972.csv`.
