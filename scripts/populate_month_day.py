#!/usr/bin/env python3
"""
Populate missing month/day in stamps table by extracting from description or context.
Usage:
  ./scripts/populate_month_day.py --db Albums/Switzerland/Switzerland.db [--dry-run]

The script searches for patterns like '27. November' or '3. März' in the description then context fields.
It updates rows where month or day are NULL/empty. In dry-run mode it prints proposed updates without applying.
"""
import argparse
import sqlite3
import re
import sys

# Pattern: day dot space monthName (Latin letters + diacritics)
PATTERN = re.compile(r"(?i)\b(\d{1,2})\.\s*([A-Za-zÀ-ÖØ-öø-ÿ]+)\b")


def find_day_month(text):
    if not text:
        return None
    m = PATTERN.search(text)
    if not m:
        return None
    day = m.group(1).lstrip('0')
    month = m.group(2).strip()
    # Normalize month: capitalize first letter, keep rest as-is
    month_norm = month[0].upper() + month[1:] if len(month) > 0 else month
    return day, month_norm


def process_db(db_path: str, dry_run: bool = True):
    conn = sqlite3.connect(db_path)
    conn.row_factory = sqlite3.Row
    cur = conn.cursor()

    # Select candidate rows where month or day missing
    cur.execute("SELECT id, description, context, month, day FROM stamps WHERE (month IS NULL OR month = '') OR (day IS NULL OR day = '')")
    rows = cur.fetchall()
    updates = []
    for r in rows:
        id = r['id']
        desc = r['description'] or ''
        context = r['context'] or ''
        existing_month = r['month'] or ''
        existing_day = r['day'] or ''
        found = None
        # prefer description first
        for src in (desc, context):
            found = find_day_month(src)
            if found:
                day, month = found
                # Only set missing values
                new_day = existing_day if existing_day.strip() else day
                new_month = existing_month if existing_month.strip() else month
                if (new_day != existing_day) or (new_month != existing_month):
                    updates.append((id, existing_month, existing_day, new_month, new_day))
                break
    # Report
    if not updates:
        print(f"{db_path}: No month/day updates found.")
        return 0
    # Print proposed updates
    for (id, old_m, old_d, new_m, new_d) in updates:
        print(f"{db_path}: id={id} old_month='{old_m}' old_day='{old_d}' -> new_month='{new_m}' new_day='{new_d}'")
    # Apply if not dry run
    if not dry_run:
        for (id, old_m, old_d, new_m, new_d) in updates:
            try:
                cur.execute("UPDATE stamps SET month = ?, day = ? WHERE id = ?", (new_m, new_d, id))
            except Exception as e:
                print(f"Failed to update id={id}: {e}", file=sys.stderr)
        conn.commit()
        print(f"{db_path}: Applied {len(updates)} month/day updates.")
    else:
        print(f"{db_path}: Dry-run: {len(updates)} rows would be updated.")
    return len(updates)


if __name__ == '__main__':
    p = argparse.ArgumentParser(description='Populate month/day from description/context')
    p.add_argument('--db', required=True, help='Path to SQLite DB')
    p.add_argument('--dry-run', action='store_true', default=False)
    args = p.parse_args()
    try:
        rc = process_db(args.db, dry_run=args.dry_run)
        sys.exit(0)
    except Exception as e:
        print(f"Error processing {args.db}: {e}", file=sys.stderr)
        sys.exit(2)
