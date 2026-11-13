use stamp_album_gen2::native_pdf::{
    colour::NamedColour, document::PageSettings, proof::GeometryProofSettings, renderer,
};
use std::path::Path;

// Frozen reference diagnostics, not production rendering defaults.
pub fn write_page_geometry_proof(
    settings: &PageSettings,
    pages: usize,
    output: &Path,
) -> stamp_album_gen2::native_pdf::Result<()> {
    let proof = GeometryProofSettings {
        page_count: pages,
        guide_enabled: true,
        guide_width_mm: 0.1,
        guide_colour: NamedColour::parse("black")?,
    };
    renderer::write_page_geometry_proof(settings, &proof, output)
}
