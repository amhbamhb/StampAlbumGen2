# Liberty Issue Collector Rewrite - Summary

## Problem Statement

The original `parse_liberty_issue_stamp()` function had critical data extraction issues:

1. **Only extracted 34 stamps** instead of ~91 total on the page
2. **All stamps marked with same denomination** (1/2¢) regardless of actual value
3. **All stamps linked to same image** (Benjamin Franklin - OneHalfCent.jpg)
4. **Image URLs constructed** instead of extracted from page HTML
5. **Most variation rows skipped** due to overly restrictive column count filtering

## Root Causes

### Issue 1: Single Denomination Detection

The original code found the first `colspan="8"` denomination row but then parsed ALL subsequent rows as if they belonged to that same denomination. It never detected the other ~24 denomination sections on the page.

**Fix**: Changed detection logic to check ALL td elements in a row for `colspan="8"`, not just rows with exactly 1 td element. This allows detecting denomination headers that are structured differently in the HTML.

### Issue 2: Non-incrementing Loop Index  

After breaking from the variations loop when detecting a new denomination, the code would then increment `i` again at the end of the main loop, causing it to skip the newly detected denomination row.

**Fix**: Introduced `found_new_denomination` flag to track whether we exited early. Only increment `i` at the end of main loop if we didn't find a new denomination. If we did find one, keep `i` pointing to that row for next iteration.

### Issue 3: Column Count Filtering

The code only processed rows with `data_tds.len() >= 7`, filtering out rows with fewer columns. However, the page has variation rows with 3, 5, and 6 columns containing valid Scott numbers.

**Fix**: Removed the column count requirement. Changed to process any row where the first column contains a Scott number (starts with digit).

### Issue 4: Image URL Extraction vs. Construction

Instead of using the actual image URLs embedded in the page's `<img src="...">` tags, the code was constructing URLs like `https://1847us.com/1954/1954Series{DenominationName}.jpg` which often didn't exist or were incorrect.

**Fix**: Extract image URLs directly from the denomination header row's `<img>` tag using the scraper API:

```rust
if let Some(img_elem) = next_tds[0].select(&img_selector).next() {
    if let Some(src) = img_elem.value().attr("src") {
        image_url = ...format_if_needed...;
    }
}
```

## Results

### Before Fix

```text
Total stamps parsed: 34
All 34 stamps:
  - denomination: "1/2¢"
  - image_url: "https://1847us.com/1954/1954Series{Something}.jpg" (mostly 404s)
```

### After Fix  

```text
Total stamps parsed: 91
Stamps extracted across 25 denominations:
  - 1/2¢, 1¢, 1.25¢, 1.5¢, 2¢, 2.5¢, 3¢, 4¢, 4.5¢, 5¢
  - 6¢, 7¢, 8¢, 9¢, 10¢, 11¢, 12¢, 15¢, 20¢, 25¢
  - 30¢, 40¢, 50¢, $1, $5
  
Image URLs: Extracted directly from page HTML
  - https://1847us.com/1954/1954SeriesOneHalfCent.jpg ✅
  - https://1847us.com/1954/1954Series1c.jpg ✅
  - https://1847us.com/1954/1954Series5c.jpg ✅
  - etc. (all valid and accessible)
```

## Technical Details

### Key Changes to `parse_liberty_issue_stamp()`

1. **Denomination Detection** (lines 902-920)
   - Correctly parses denomination from colspan="8" row text
   - Splits on '·' to extract denomination, color, designer, etc.

2. **Image URL Extraction** (lines 961-987)
   - Finds image row by looking for `<img>` tags
   - Extracts `src` attribute and handles relative URLs
   - Resets image_url for each denomination

3. **State Management** (lines 989-1077)
   - Maintains `found_new_denomination` flag
   - Only increments `i` if not breaking early to next denomination
   - Processes all stamp rows regardless of column count

4. **Data Mapping** (lines 1024-1057)
   - Maps extracted data to StampInfo struct
   - Uses extracted image_url from page
   - Preserves correct denomination per stamp

## Page Structure Understanding

The Liberty Issue page contains a table with this repeating pattern:

```html
<!-- Denomination Header -->
<tr>
  <td colspan="8">
    1/2¢ Benjamin Franklin · Red Orange · Designer: McCloskey · Engraver: C. A. Brooks
  </td>
</tr>

<!-- Image Row -->
<tr>
  <td>
    <img src="/1954/1954SeriesOneHalfCent.jpg" />
  </td>
  ...
</tr>

<!-- Data Header Row -->
<tr>
  <td>Cat. #</td>
  <td>FDC</td>
  <td>Type</td>
  <td>Perf</td>
  <td>Tag</td>
  <td>Var</td>
  <td>Use</td>
</tr>

<!-- Data Rows (variable column count) -->
<tr>
  <td>1030</td>
  <td>...</td>
  ... (varies from 3 to 8 columns)
</tr>
```

Each denomination section repeats this pattern ~27 times, with varying numbers of stamp variation rows.

## Testing

Tested with:

```bash
./stamp_album_gen2 --source us1847 --site-name "1954-liberty-issue" \
  --album-name test_liberty --base-url "https://1847us.com/1954-liberty-issue/"
```

Verified:

- ✅ All 91 stamps extracted
- ✅ Correct denominations applied
- ✅ Image URLs match page sources
- ✅ No 404 errors on image downloads
- ✅ Database integrity maintained
- ✅ Album generation successful

## Future Improvements

This rewrite establishes a solid foundation for:

1. Generic template-based page parsers (series vs. year-based layouts)
2. Multi-format page support detection
3. Robust variation data extraction
4. Better error handling for malformed pages
