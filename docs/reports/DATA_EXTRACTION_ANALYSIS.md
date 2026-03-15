# US1847 Collector Data Extraction Analysis

## Current Issues

### 1. **Only 34 stamps extracted instead of ~91**
   - Missing variation rows because code assumes all data rows have exactly 7 `<td>` elements
   - Many variation rows may have different structure or number of columns
   - Need to handle rows with colspan attributes

### 2. **Incorrect denomination assignment**
   - Code is hardcoding denomination lookup based on Scott number prefixes
   - Should extract denomination from the denomination header row (colspan="8")
   - The extracted denomination persists across all following variation rows until next denomination section

### 3. **Image URL extraction failing**
   - Code attempts to construct image URLs like `https://1847us.com/1954/1954Series1AndOneHalfCent.jpg`
   - Many of these constructed URLs don't exist (404 errors)
   - **Solution**: Extract the actual `src` attribute from `<img>` tags in the denomination section row

## Page HTML Structure Analysis

### Denomination Section Pattern:
```html
<tr>
  <td colspan="8" ...>
    <strong>1/2¢ Benjamin Franklin<br/>· Red Orange</strong>
    <!-- Designer/Engraver info -->
  </td>
</tr>

<tr>
  <td rowspan="N" ...>
    <img src="https://1847us.com/1954/1954SeriesOneHalfCent.jpg" ...>
  </td>
  <td>Cat. #</td>
  <td>FDC</td>
  <td>Printing Method · Type</td>
  <td>Perforation</td>
  <td>Tagging</td>
  <td>Collectible Varieties</td>
  <td>Usage</td>
</tr>

<!-- N variation rows, each with 7 <td> elements for stamp data -->
<tr>
  <td>1031b</td>
  <td>10/20/1955</td>
  <td>sheet stamp – wet printing</td>
  <td>11 x 10.5</td>
  <td>none</td>
  <td>–</td>
  <td></td>
</tr>
```

## Data Extraction Requirements

### Per Denomination Section:
1. **Denomination Header Row** (colspan="8"):
   - Extract: denomination, color, designer, engraver
   - **Extract image URL from `<img src="...">`**

2. **Image Row** with header:
   - Contains the image and column headers
   - `rowspan` attribute indicates number of variations

3. **Variation Rows** (7 `<td>` columns each):
   - Scott number (td[0])
   - Issue date / FDC (td[1])
   - Printing method (td[2])
   - Perforation (td[3])
   - Tagging (td[4])
   - Collectible varieties (td[5])
   - Usage (td[6])

## Different Page Formats Identified

### Format 1: Series (e.g., Liberty Issue)
- https://1847us.com/1954-liberty-issue/
- Organized by denomination
- Each denomination has multiple variations

### Format 2: Year-based
- https://1847us.com/byyear/1915-year/
- Likely different data structure
- May group stamps differently

## Recommended Solution

### Phase 1: Fix Liberty Issue Collector
1. Extract and store image URL when processing denomination header
2. Don't construct URLs; use extracted ones
3. Properly count and extract all variation rows (not just those with 7 `<td>`)
4. Track rowspan to understand variation count

### Phase 2: Create Template System
Different page formats need different parsers:
- `SeriesCollector` - for series-based pages (Liberty Issue, Identifier series)
- `YearCollector` - for year-based pages
- Generic logic for common patterns

### Phase 3: Add Variation Support
- Extract variation data (color, printing, perforation, etc.)
- Link variations to base stamp via relationship table
- Store all metadata for each variation

## Data to Extract per Variation

```
- Scott Number (primary key)
- Denomination (from header)
- Color (from header)
- Designer (from header)
- Engraver (from header)
- Image URL (from <img> tag)
- Issue Date
- Printing Method
- Perforation
- Tagging
- Collectible Varieties
- Usage
```

