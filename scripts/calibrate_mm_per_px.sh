#!/usr/bin/env bash
# Simple calibration helper
# Reads Albums/Canada/test_measurements/measurements.csv with columns:
# filename,width_mm,height_mm,notes
# and computes mm-per-pixel for width and height using the image pixel sizes.

set -euo pipefail
ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
TEST_DIR="$ROOT_DIR/Albums/Canada/test_measurements"
CSV="$TEST_DIR/measurements.csv"
if [ ! -f "$CSV" ]; then
  echo "No measurements.csv found in $TEST_DIR"
  echo "Create $CSV with rows: filename,width_mm,height_mm,notes"
  exit 1
fi

printf "Reading measurements from %s\n\n" "$CSV"

echo "filename,width_mm,height_mm,width_px,height_px,mm_per_px_w,mm_per_px_h" > /tmp/calib_results.csv

sum_wpp=0
sum_hpp=0
count=0

while IFS=, read -r filename width_mm height_mm notes; do
  # skip header or empty lines
  if [ "$filename" = "filename" ] || [ -z "$filename" ]; then
    continue
  fi
  img_path="$ROOT_DIR/Albums/Canada/$filename"
  if [ ! -f "$img_path" ]; then
    img_path="$TEST_DIR/$filename"
    if [ ! -f "$img_path" ]; then
      echo "WARN: image not found: $filename (looked in Albums/Canada and $TEST_DIR)" >&2
      continue
    fi
  fi
  # get pixel dims (capture output safely; identify may print warnings)
  dims=$(identify -format "%w %h" "$img_path" 2>/dev/null || true)
  read wpx hpx <<< "$dims"
  if [ -z "${wpx:-}" ] || [ -z "${hpx:-}" ]; then
    echo "WARN: failed to read pixels for $img_path (identify output: '$dims')" >&2
    continue
  fi
  # compute mm/px
  mppw=$(python3 - <<PY
wmm=float($width_mm)
wpx=float($wpx)
print(wmm/wpx if wpx>0 else 0)
PY
)
  mpph=$(python3 - <<PY
hmm=float($height_mm)
hpx=float($hpx)
print(hmm/hpx if hpx>0 else 0)
PY
)
  echo "$filename,$width_mm,$height_mm,$wpx,$hpx,$mppw,$mpph" >> /tmp/calib_results.csv
  sum_wpp=$(python3 - <<PY
sum=float($sum_wpp)
val=float($mppw)
print(sum+val)
PY
)
  sum_hpp=$(python3 - <<PY
sum=float($sum_hpp)
val=float($mpph)
print(sum+val)
PY
)
  count=$((count+1))

done < "$CSV"

if [ $count -eq 0 ]; then
  echo "No valid samples processed"
  exit 1
fi

avg_wpp=$(python3 - <<PY
s=float($sum_wpp)
count=int($count)
print(s/count)
PY
)
avg_hpp=$(python3 - <<PY
s=float($sum_hpp)
count=int($count)
print(s/count)
PY
)

printf "Processed %d samples\n" "$count"
printf "Average mm-per-px (width): %.6f mm/px\n" "$avg_wpp"
printf "Average mm-per-px (height): %.6f mm/px\n" "$avg_hpp"

# SuggestREF based on embedded reference pixel dims (from src/B4-s.jpg)
ref_px_w=$(identify -format "%w" src/B4-s.jpg 2>/dev/null || true)
ref_px_h=$(identify -format "%h" src/B4-s.jpg 2>/dev/null || true)
if [ -n "$ref_px_w" ] && [ -n "$ref_px_h" ]; then
  suggested_ref_w_mm=$(python3 - <<PY
mpp=float($avg_wpp)
ref_px=float($ref_px_w)
print(round(mpp*ref_px,2))
PY
)
  suggested_ref_h_mm=$(python3 - <<PY
mpp=float($avg_hpp)
ref_px=float($ref_px_h)
print(round(mpp*ref_px,2))
PY
)
  printf "\nBased on embedded reference pixels (%sx%s px), suggested REF_W_MM=%.2f, REF_H_MM=%.2f\n" "$ref_px_w" "$ref_px_h" "$suggested_ref_w_mm" "$suggested_ref_h_mm"
fi

printf "\nResults saved to /tmp/calib_results.csv\n"

exit 0
