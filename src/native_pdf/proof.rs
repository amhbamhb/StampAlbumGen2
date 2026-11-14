use super::{Result, colour::NamedColour, document::PageLayout, units::mm_to_points};

/// Diagnostic-only policy, separate from album command state.
/// All fields are explicit TOML inputs; there is deliberately no Default.
#[derive(Clone, Debug)]
pub struct GeometryProofSettings {
    pub page_count: usize,
    pub guide_enabled: bool,
    pub guide_width_mm: f64,
    pub guide_colour: NamedColour,
}

impl GeometryProofSettings {
    pub fn validate(&self) -> Result<()> {
        if self.page_count == 0 || u32::try_from(self.page_count).is_err() {
            return Err("native_pdf.proof.page_count must be a positive PDF page count".into());
        }
        if !self.guide_width_mm.is_finite()
            || self.guide_width_mm <= 0.0
            || !mm_to_points(self.guide_width_mm).is_finite()
        {
            return Err("native_pdf.proof.guide_width_mm must be finite and positive".into());
        }
        Ok(())
    }

    pub fn validate_layout(&self, layout: &PageLayout) -> Result<()> {
        if self.guide_enabled {
            layout
                .content_bounds
                .inset(self.guide_width_mm / 2.0, self.guide_width_mm / 2.0)
                .map_err(|error| format!("native_pdf.proof.guide_width_mm: {error}"))?;
        }
        Ok(())
    }
}
