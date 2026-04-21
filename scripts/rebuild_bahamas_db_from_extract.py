#!/usr/bin/env python3
import sqlite3, re, os, shutil, sys, datetime

album_dir = 'Albums/Bahamas'
db_path = os.path.join(album_dir, 'Bahamas.db')
extract = os.path.join(album_dir, 'Bahamas-1859-1863.txt')
if not os.path.exists(db_path):
    print('DB not found:', db_path)
    sys.exit(1)
if not os.path.exists(extract):
    print('Extract file not found:', extract)
    sys.exit(1)

# Backup DB
bak = db_path + '.bak.' + datetime.datetime.now().strftime('%Y%m%d%H%M%S')
shutil.copy2(db_path, bak)
print('Backed up DB to', bak)

# Read extract and parse EMIT_STAMP lines
emit_re = re.compile(r'EMIT_STAMP\s+series="(?P<series>[^"]*)"\s+sg="(?P<sg>[^"]*)"\s+img="(?P<img>[^"]*)"')
entries = []
with open(extract, 'r', encoding='utf-8') as f:
    for line in f:
        m = emit_re.search(line)
        if m:
            series = m.group('series').strip()
            sg = m.group('sg').strip()
            img = m.group('img').strip()
            # Extract first four-digit year from series if present
            year_match = re.search(r'(18|19|20)\d{2}', series)
            year = int(year_match.group(0)) if year_match else 0
            entries.append((series, sg, img, year))
print('Parsed', len(entries), 'entries from extract')

# Insert into DB
conn = sqlite3.connect(db_path)
cur = conn.cursor()
# Ensure stamps table exists
cur.execute("SELECT name FROM sqlite_master WHERE type='table' AND name='stamps'")
if cur.fetchone() is None:
    print('stamps table missing')
    conn.close()
    sys.exit(1)

# Prepare insert with explicit columns (uniq_key column may exist)
# Detect columns
cols = [r[1] for r in cur.execute("PRAGMA table_info(stamps)").fetchall()]
print('stamps columns:', cols)
has_uniq = 'uniq_key' in cols

insert_cols = ['country','series','year','month','month_num','day','sg_number','type','denomination','color','description','stamps_issued','mint_condition','unused','used','letter_fdc','currency','image_path','remote_image_url','original_image_url','watermark','design','engraving','perforation','context']
if has_uniq:
    insert_cols.append('uniq_key')
placeholders = ','.join(['?']*len(insert_cols))
col_list = ','.join(insert_cols)
sql = f'INSERT OR IGNORE INTO stamps ({col_list}) VALUES ({placeholders})'
print('Insert SQL:', sql)

country = 'Bahamas'
count = 0
for series, sg, img, year in entries:
    image_path = img if img else ''
    remote_image_url = ''
    original_image_url = os.path.basename(img) if img else ''
    uniq_key = (country or '').lower() + '|' + series + '|' + sg if has_uniq else None
    params = [country, series, year, '', 0, '', sg, '', '', '', '', '', '', '', '', '', '', image_path, remote_image_url, original_image_url, '', '', '', '', '',]
    if has_uniq:
        params.append(uniq_key)
    try:
        cur.execute(sql, params)
        count += 1
    except Exception as e:
        print('Insert error for', series, sg, e)

conn.commit()
print('Inserted (or ignored) {} entries'.format(count))
# Show final count
cur.execute('SELECT COUNT(*) FROM stamps')
print('Total stamps now in DB:', cur.fetchone()[0])
conn.close()
print('Done')
