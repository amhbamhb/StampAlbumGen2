#!/usr/bin/env python3
"""
Update Prominent Americans Series section in usa-1941-1969-new.txt
Similar to Liberty Series, expands from base stamps to include all variations
"""

import re
from prominent_americans_data import prominent_americans_data

def find_prominent_americans_section(lines):
    """Find the start and end of the Prominent Americans Series section"""
    start_line = None
    end_line = None
    
    for i, line in enumerate(lines):
        if 'Prominent Americans Series' in line:
            # Find the ROW_START_FS before this heading
            for j in range(i-1, -1, -1):
                if 'ROW_START_FS' in lines[j]:
                    start_line = j
                    break
        
        if start_line is not None and '1966 Commemoratives' in line:
            # Find the PAGE_START_VAR before this (end of Prominent Americans)
            for j in range(i-1, start_line, -1):
                if 'PAGE_START_VAR' in lines[j] and '# Page -37-' in lines[j]:
                    end_line = j
                    break
            break
    
    return start_line, end_line

def generate_prominent_americans_section():
    """Generate the complete Prominent Americans section with all variations"""
    output = []
    output.append('ROW_START_FS (TN 6 0.1 6.0)')
    output.append('PAGE_TEXT_CENTRE_VSPACE (TN 9 "Prominent Americans Series" 3.0)')
    output.append('')
    
    # Group stamps by base catalog number in display order
    stamp_order = [
        "1278", "1279", "1280", "1281", "1282",  # Row 1: 1¢-4¢
        "1283", "1283B", "1284", "1305", "1393", "1393D", "1285",  # Row 2-3: 5¢-8¢ (Washington variations, FDR variations, Eisenhower, Franklin, Einstein)
        "1394", "1395", "1286", "1286A", "1287", "1397",  # Row 4-5: 8¢ Ike, 10¢-14¢
        "1288", "1288d", "1288B", "1398", "1399",  # Row 6-7: 15¢ Holmes (3 types), 16¢, 18¢
        "1289", "1400", "1290", "1291", "1292", "1293", "1294", "1295"  # Row 8+: 20¢-$5
    ]
    
    stamps_in_row = 0
    max_per_row = 5
    
    for base_cat in stamp_order:
        if base_cat not in prominent_americans_data:
            print(f"Warning: {base_cat} not in data")
            continue
        
        stamp_data = prominent_americans_data[base_cat]
        variations = stamp_data["variations"]
        
        # Each variation gets its own entry
        for variation in variations:
            if stamps_in_row == 0:
                output.append('ROW_START_FS (TN 6 0.1 6.0)')
            
            cat_num = variation["cat"]
            # Use base catalog number for image (strip letter suffixes)
            base_cat_for_img = re.sub(r'[a-zA-Z]+$', '', cat_num)
            img_name = base_cat_for_img
            
            # Determine dimensions (most are 28x31, some are 31x28 for horizontal)
            # Based on original: 1281, 1284, 1286A, 1291 are 31x28
            if base_cat_for_img in ["1281", "1284", "1286A", "1291"]:
                dims = "31.0 28.0"
            else:
                dims = "28.0 31.0"
            
            output.append(f'STAMP_ADD_IMG ({dims} "images/{img_name}.jpg" "" "{cat_num}" "")')
            
            # Create heading with variation details
            description = stamp_data["description"]
            fdc = variation.get("fdc", "")
            vtype = variation.get("type", "")
            perf = variation.get("perf", "")
            
            heading_parts = [description]
            if cat_num != base_cat_for_img:  # Only add details for variations
                heading_parts.append(f"Cat# {cat_num}")
                if fdc:
                    heading_parts.append(f"FDC: {fdc}")
                if vtype:
                    heading_parts.append(vtype)
                if perf:
                    heading_parts.append(f"Perf: {perf}")
            
            heading = "\\n".join(heading_parts)
            output.append(f'STAMP_HEADING_VA (TN 6 "{heading}")')
            
            stamps_in_row += 1
            
            if stamps_in_row >= max_per_row:
                output.append('')
                stamps_in_row = 0
    
    # Close any open row
    if stamps_in_row > 0:
        output.append('')
    
    return output

def main():
    # Read the file
    with open('usa-1941-1969-new.txt', 'r') as f:
        lines = f.readlines()
    
    # Remove trailing newlines for processing
    lines = [line.rstrip('\n') for line in lines]
    
    # Find the section
    start_line, end_line = find_prominent_americans_section(lines)
    
    if start_line is None or end_line is None:
        print("ERROR: Could not find Prominent Americans Series section")
        return
    
    print(f"Found Prominent Americans Series: lines {start_line} to {end_line}")
    
    # Generate new section
    new_section = generate_prominent_americans_section()
    
    # Count stamps in old vs new
    old_section = lines[start_line:end_line]
    old_stamp_count = sum(1 for line in old_section if 'STAMP_ADD_IMG' in line)
    new_stamp_count = sum(1 for line in new_section if 'STAMP_ADD_IMG' in line)
    
    # Replace the section
    new_lines = lines[:start_line] + new_section + lines[end_line:]
    
    # Write back
    with open('usa-1941-1969-new.txt', 'w') as f:
        for line in new_lines:
            f.write(line + '\n')
    
    print(f"✓ Updated Prominent Americans Series section")
    print(f"✓ Added {new_stamp_count} stamp variations total")
    print(f"✓ Original had ~{old_stamp_count} stamps, now has {new_stamp_count} variations")

if __name__ == '__main__':
    main()
