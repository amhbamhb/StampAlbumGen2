#!/usr/bin/env python3
# scripts/populate_example_db.py
# Read scripts/populate_example_db.data.json and insert rows into an sqlite DB

import argparse
import json
import sqlite3
from pathlib import Path

parser = argparse.ArgumentParser(description='Populate example stamp DB from JSON data file')
parser.add_argument('dbpath', nargs='?', default='/tmp/example_stampworld.db')
parser.add_argument('--data', default='scripts/populate_example_db.data.json')
args = parser.parse_args()

db = Path(args.dbpath)
if db.exists():
    db.unlink()

data_file = Path(args.data)
if not data_file.exists():
    raise SystemExit(f"Data file not found: {data_file}")

with data_file.open() as f:
    rows = json.load(f)

conn = sqlite3.connect(str(db))
cur = conn.cursor()

cur.execute('PRAGMA foreign_keys=OFF;')
cur.execute('BEGIN TRANSACTION;')
cur.execute('''
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
''')

for r in rows:
    uniq = f"{r.get('country','').lower()}|{r.get('series','')}|{r.get('sg_number','')}"
    cur.execute('''
    INSERT INTO stamps (country, series, year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, currency, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, context, uniq_key)
    VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
    ''', (
        r.get('country'),
        r.get('series'),
        r.get('year'),
        r.get('month'),
        r.get('month_num'),
        r.get('day'),
        r.get('sg_number'),
        r.get('type'),
        r.get('denomination'),
        r.get('color'),
        r.get('description'),
        r.get('stamps_issued'),
        r.get('currency'),
        r.get('image_path'),
        r.get('remote_image_url'),
        r.get('original_image_url'),
        r.get('watermark'),
        r.get('design'),
        r.get('engraving'),
        r.get('perforation'),
        r.get('context'),
        uniq
    ))

conn.commit()
conn.close()
print(f"Populated DB: {db}")
