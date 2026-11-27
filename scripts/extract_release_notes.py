#!/usr/bin/env python3
"""
Extract release notes for a given tag from RELEASE_NOTES.md.

Usage:
    python3 scripts/extract_release_notes.py <tag> [output_file]

If output_file is omitted, writes to release_notes_body.md.
Exits 0 if notes found and written, 1 otherwise.
"""
import re
import sys
from pathlib import Path


def main():
    if len(sys.argv) < 2:
        print("Usage: extract_release_notes.py <tag> [output_file]", file=sys.stderr)
        sys.exit(1)

    tag = sys.argv[1].lstrip("v")
    out_path = Path(sys.argv[2]) if len(sys.argv) > 2 else Path("release_notes_body.md")
    notes_path = Path("RELEASE_NOTES.md")

    if not notes_path.exists():
        sys.exit(1)

    try:
        text = notes_path.read_text(encoding="utf-8")
        pattern = rf"(#+\s+Stamp Album Generator\s+v?{re.escape(tag)}\s+Release Notes.*?(?=\n#+\s+Stamp Album Generator|\Z))"
        m = re.search(pattern, text, re.DOTALL | re.IGNORECASE)
        if m:
            content = m.group(1).strip()
            if len(content) > 50:
                out_path.write_text(content + "\n", encoding="utf-8")
                print(f"Extracted release notes for {tag} to {out_path}")
                sys.exit(0)
    except Exception as exc:
        print(f"Error extracting release notes: {exc}", file=sys.stderr)

    sys.exit(1)


if __name__ == "__main__":
    main()
