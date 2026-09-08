-- idempotent migration to normalize NULL text columns to safe defaults
-- and deduplicate stamps by (country, series, sg_number)
-- Usage: sqlite3 "path/to/Album.db" ".read scripts/fix_null_types.sql"

BEGIN;

-- Normalize common text columns to non-NULL defaults so consumers that
-- expect text values won't panic when reading.
UPDATE stamps SET
    type = COALESCE(type, 'PLACEHOLDER'),
    denomination = COALESCE(denomination, ''),
    color = COALESCE(color, ''),
    description = COALESCE(description, ''),
    stamps_issued = COALESCE(stamps_issued, ''),
    mint_condition = COALESCE(mint_condition, ''),
    unused = COALESCE(unused, ''),
    used = COALESCE(used, ''),
    letter_fdc = COALESCE(letter_fdc, ''),
    currency = COALESCE(currency, ''),
    image_path = COALESCE(image_path, ''),
    remote_image_url = COALESCE(remote_image_url, ''),
    original_image_url = COALESCE(original_image_url, ''),
    watermark = COALESCE(watermark, ''),
    design = COALESCE(design, ''),
    engraving = COALESCE(engraving, ''),
    perforation = COALESCE(perforation, ''),
    context = COALESCE(context, '')
WHERE 1=1;

-- Deduplicate rows by keeping the row with smallest id for each key
-- (country, series, sg_number). This is safe to run repeatedly.
DELETE FROM stamps
WHERE id NOT IN (
    SELECT MIN(id) FROM stamps GROUP BY lower(coalesce(country,'')), coalesce(series,''), coalesce(sg_number,'')
);

COMMIT;
-- End of script
