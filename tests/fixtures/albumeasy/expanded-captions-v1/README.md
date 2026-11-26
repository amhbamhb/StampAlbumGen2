# Expanded Caption & Side Label Specification (expanded-captions-v1)

This fixture demonstrates how the native-pdf engine allows stamp caption bounding boxes and side text labels to expand wider than the stamp specimen mount.

---

## 1. Caption Expansion Modes

### 1.1 Bottom Caption Footprint Expansion (`expand_footprint`)
In traditional AlbumEasy generation, stamp captions are strictly wrapped and clipped within the horizontal bounding box of the stamp mount frame. For narrow definitive stamps (e.g. 22–26 mm wide), long catalog descriptions or commemorative titles either get forced into tiny fonts or broken across 5–6 fragmented lines.

With the native command:
```text
NATIVE_STAMP_TEXT_POLICY (expand_footprint percent 150 2.0 1.0 2.5)
```
- **`percent 150`**: Allows the text footprint under the stamp to expand up to 150% of the mount width (e.g., a 26 mm stamp gets a 39 mm wide caption block).
- **Automatic Clearance**: The engine reserves extra horizontal space in the row packing so expanded text blocks never overlap adjacent stamps.
- **`2.5 mm text gap`**: Enforces minimum clearance between the bottom stroke of the stamp frame and the top line of text.

### 1.2 Side Text Labels (`NATIVE_STAMP_IMAGE_SIDE_LABEL`)
For detailed exhibition narrative (watermarks, perforations, print methods, designer credits), text can be placed directly to the right or left of the mount:
```text
NATIVE_STAMP_IMAGE_SIDE_LABEL (HN 6.5 darkblue 38.0 8 "" "Issue Date: 1903\nPerforation: 11.5 x 12\nWatermark: Crown\nPrint: Typography")
```
- **Wrap Width (`38.0 mm`)**: Formats the narrative into a neat, column-like side block beside the stamp.
- **Font & Colour**: Independently styled using philatelic typography (Helvetica 6.5 pt, `darkblue`).

---

## 2. Fixture Variations & Rendered Outputs

| File | Type | Description |
|---|---|---|
| [demo.stampalbum](demo.stampalbum) | Native Source | Complete native vector source demonstrating both styles |
| [demo-native.pdf](demo-native.pdf) | Native Vector PDF | Rendered single-page US Letter showcase |
