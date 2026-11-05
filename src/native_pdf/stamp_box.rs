//! Known physical stamp bounds and independent mounting-frame clearance.

use super::{document::Bounds, units::mm_to_points, Result};

#[derive(Clone, Copy, Debug)]
pub struct StampBoxGeometry {
    physical_bounds: Bounds,
    frame_bounds: Bounds,
    horizontal_clearance_mm: f64,
    vertical_clearance_mm: f64,
}

impl StampBoxGeometry {
    /// Bounds include the outer perforation tips. Existing width/height extras
    /// and the shared adjustment are total additions, not per-edge padding.
    /// Signed components are accepted only when their net clearance is nonnegative.
    pub fn from_total_additions(
        physical_bounds: Bounds,
        extra_width_mm: f64,
        extra_height_mm: f64,
        stamp_box_adjust_mm: f64,
    ) -> Result<Self> {
        validate_bounds(physical_bounds)?;
        for (name, value) in [
            ("extra_width_mm", extra_width_mm),
            ("extra_height_mm", extra_height_mm),
            ("stamp_box_adjust", stamp_box_adjust_mm),
        ] {
            if !value.is_finite() || !mm_to_points(value).is_finite() {
                return Err(format!(
                    "native_pdf.stamp_box.{name} must be finite in PDF units"
                ));
            }
        }
        let total_width_mm = extra_width_mm + stamp_box_adjust_mm;
        let total_height_mm = extra_height_mm + stamp_box_adjust_mm;
        if ![total_width_mm, total_height_mm]
            .iter()
            .all(|value| value.is_finite() && *value >= 0.0 && mm_to_points(*value).is_finite())
        {
            return Err(
                "native_pdf.stamp_box net additions must be finite and nonnegative; \
                 a mounting frame cannot shrink the known physical stamp"
                    .into(),
            );
        }
        let horizontal_clearance_mm = total_width_mm / 2.0;
        let vertical_clearance_mm = total_height_mm / 2.0;
        let frame_bounds = Bounds {
            left: physical_bounds.left - horizontal_clearance_mm,
            right: physical_bounds.right + horizontal_clearance_mm,
            bottom: physical_bounds.bottom - vertical_clearance_mm,
            top: physical_bounds.top + vertical_clearance_mm,
        };
        validate_bounds(frame_bounds)?;
        Ok(Self {
            physical_bounds,
            frame_bounds,
            horizontal_clearance_mm,
            vertical_clearance_mm,
        })
    }

    pub fn physical_bounds(&self) -> Bounds {
        self.physical_bounds
    }

    pub fn frame_bounds(&self) -> Bounds {
        self.frame_bounds
    }

    pub fn horizontal_clearance_mm(&self) -> f64 {
        self.horizontal_clearance_mm
    }

    pub fn vertical_clearance_mm(&self) -> f64 {
        self.vertical_clearance_mm
    }
}

fn validate_bounds(bounds: Bounds) -> Result<()> {
    bounds
        .inset(0.0, 0.0)
        .map_err(|error| format!("native_pdf.stamp_box bounds: {error}"))?;
    if ![
        bounds.left,
        bounds.right,
        bounds.bottom,
        bounds.top,
        bounds.right - bounds.left,
        bounds.top - bounds.bottom,
    ]
    .iter()
    .all(|value| mm_to_points(*value).is_finite())
    {
        return Err("native_pdf.stamp_box bounds overflow PDF coordinates".into());
    }
    Ok(())
}
