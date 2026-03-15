#!/usr/bin/env python3
import re
from liberty_series_data import liberty_series_data

# Read the current file
with open('usa-1941-1969-new.txt', 'r', encoding='utf-8') as f:
    lines = f.readlines()

# Find Liberty Series section
liberty_start = liberty_end = None
for i, line in enumerate(lines):
    if 'Liberty Series' in line:
        liberty_start = i
    if liberty_start and '1955 Commemoratives' in line:
        liberty_end = i
        break

print(f"Found Liberty Series: lines {liberty_start} to {liberty_end}")

# Keep everything before Liberty Series section
new_content = lines[:liberty_start+1]  # Include the "Liberty Series" title
new_content.append("\n")

# Generate new Liberty Series with all variations
row_count = 0
stamps_per_row = 5

# Process each base catalog number
for base_cat in sorted(liberty_series_data.keys()):
    data = liberty_series_data[base_cat]
    variations = data['variations']
    
    # For each variation, add a stamp entry
    for var_idx, var in enumerate(variations):
        # Start new row every 5 stamps
        if row_count % stamps_per_row == 0:
            new_content.append("ROW_START_FS (TN 6 0.1 3.0)\n")
        
        # Get catalog number for image
        cat_num = var['cat']
        if cat_num == "—":
            cat_num = base_cat
        
        # Use base catalog number for image (strip letter suffixes like 'a', 'b', 'c', etc.)
        # All variations use the same base image
        img_name = base_cat
        
        # Determine stamp dimensions
        if base_cat in ["1030", "1031", "1033", "1034", "1035", "1036", "1038", "1039", "1040", "1041", "1042", "1045", "1046", "1048", "1049", "1050", "1051", "1052", "1053"]:
            dims = "28.0 31.0"
        else:
            dims = "31.0 28.0"
        
        # Add stamp image
        new_content.append(f'STAMP_ADD_IMG ({dims} "images/{img_name}.jpg" "" "{cat_num}" "")\n')
        
        # Create heading with variation details
        heading_text = f"{data['description']}"
        if len(variations) > 1:
            details = []
            if var['cat'] != "—":
                details.append(f"Cat# {var['cat']}")
            if var['fdc'] and var['fdc'] != "—":
                details.append(f"FDC: {var['fdc']}")
            details.append(var['type'])
            details.append(f"Perf: {var['perf']}")
            heading_text += "\\n" + "\\n".join(details)
        
        new_content.append(f'STAMP_HEADING_VA (TN 6 "{heading_text}")\n')
        row_count += 1

# Add spacing and continue with rest of file
new_content.append("\n\n")
new_content.extend(lines[liberty_end:])

# Write the updated file
with open('usa-1941-1969-new.txt', 'w', encoding='utf-8') as f:
    f.writelines(new_content)

print(f"✓ Updated Liberty Series section")
print(f"✓ Added {row_count} stamp variations total")
print(f"✓ Original had ~24 stamps, now has {row_count} variations")
