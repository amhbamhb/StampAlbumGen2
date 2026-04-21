#!/usr/bin/env bash
# scripts/populate_example_db.sh
# Create a minimal sqlite DB that mirrors the canonical `stamps` schema and
# populate it with representative rows found in Example-Stampworldwebpage.txt
# so the generator's --db-path mode can be exercised meaningfully.

set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

DBPATH=${1:-/tmp/example_stampworld.db}
if [ -e "$DBPATH" ]; then
  echo "Removing existing DB at $DBPATH"
  rm -f "$DBPATH"
fi

echo "Creating DB at $DBPATH"

if command -v python3 >/dev/null 2>&1 && [ -f scripts/populate_example_db.py ] && [ -f scripts/populate_example_db.data.json ]; then
  echo "Using Python populator scripts/populate_example_db.py"
  python3 scripts/populate_example_db.py "$DBPATH"
else
  echo "Python populator not available; falling back to embedded sqlite3 commands"
  sqlite3 "$DBPATH" <<'SQL'
PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS stamps (
  id INTEGER PRIMARY KEY,
  country TEXT,
  series TEXT,
  year INTEGER,
  month TEXT,
  month_num INTEGER,
  day TEXT,
  sg_number TEXT,
  type TEXT,
  denomination TEXT,
  color TEXT,
  description TEXT,
  stamps_issued TEXT,
  mint_condition TEXT,
  unused TEXT,
  used TEXT,
  letter_fdc TEXT,
  currency TEXT,
  image_path TEXT,
  remote_image_url TEXT,
  original_image_url TEXT,
  watermark TEXT,
  design TEXT,
  engraving TEXT,
  perforation TEXT,
  context TEXT,
  uniq_key TEXT
);

CREATE INDEX IF NOT EXISTS idx_stamps_uniq_key ON stamps(uniq_key);

INSERT INTO stamps (country, series, year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, currency, design, engraving, perforation, context, uniq_key)
VALUES
('Switzerland','The 100th Anniversary of the International Metric Convention - International Women`s Year - Diplomatic Conference on Humanitarian Law',1975,'February',2,'13','1040','ANZ','0.29','multicoloured','', '10036000','USD','Roland Hirter','Courvoisier S.A.','11¾','', lower('Switzerland') || '|' || 'The 100th Anniversary of the International Metric Convention - International Women`s Year - Diplomatic Conference on Humanitarian Law' || '|' || '1040'),
('Switzerland','The 100th Anniversary of the International Metric Convention - International Women`s Year - Diplomatic Conference on Humanitarian Law',1975,'February',2,'13','1041','AOA','0.29','multicoloured','', '25556000','USD','Roland Hirter','Courvoisier S.A.','11¾','', lower('Switzerland') || '|' || 'The 100th Anniversary of the International Metric Convention - International Women`s Year - Diplomatic Conference on Humanitarian Law' || '|' || '1041'),
('Switzerland','The 100th Anniversary of the International Metric Convention - International Women`s Year - Diplomatic Conference on Humanitarian Law',1975,'February',2,'13','1042','AOB','0.58','multicoloured','', '11726000','USD','Roland Hirter','Courvoisier S.A.','11¾','', lower('Switzerland') || '|' || 'The 100th Anniversary of the International Metric Convention - International Women`s Year - Diplomatic Conference on Humanitarian Law' || '|' || '1042'),
('Switzerland','The 100th Anniversary of the International Metric Convention - International Women`s Year - Diplomatic Conference on Humanitarian Law',1975,'February',2,'13','1043','AOC','1.17','multicoloured','', '7617000','USD','Roland Hirter','Courvoisier S.A.','11¾','', lower('Switzerland') || '|' || 'The 100th Anniversary of the International Metric Convention - International Women`s Year - Diplomatic Conference on Humanitarian Law' || '|' || '1043');

COMMIT;
SQL

  echo "DB created and populated: $DBPATH"

  # Print a short verification summary
  sqlite3 "$DBPATH" "SELECT id, sg_number, type, denomination, year, month, day, uniq_key FROM stamps ORDER BY sg_number;"

  echo "Done."
fi
