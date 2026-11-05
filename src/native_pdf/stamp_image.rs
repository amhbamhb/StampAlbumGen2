//! Proof-only placement for one stamp image with measured heading and caption.

use super::{
    colour::NamedColour,
    document::{Bounds, PageLayout},
    text::{MeasuredText, PositionedTextLine},
    units::{mm_to_points, points_to_mm},
    Result,
};
use oxidize_pdf::Image;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct StampTextProofSettings {
    pub text: String,
    pub font: String,
    pub size_pt: f64,
    pub colour: NamedColour,
}

impl StampTextProofSettings {
    pub fn new(
        text: impl Into<String>,
        font: impl Into<String>,
        size_pt: f64,
        colour: NamedColour,
    ) -> Self {
        Self {
            text: text.into(),
            font: font.into(),
            size_pt,
            colour,
        }
    }
}

/// Explicit geometry and image policy for the isolated native stamp proof.
///
/// The source is measured at `image_scale_pt_per_pixel`; images are never
/// enlarged and are proportionally reduced only when needed to fit the padded
/// box. This proof policy is not yet a production AlbumEasy command mapping.
#[derive(Clone, Debug)]
pub struct StampImageProofSettings {
    pub image_path: PathBuf,
    pub box_bounds: Bounds,
    pub image_scale_pt_per_pixel: f64,
    pub image_padding_horizontal_mm: f64,
    pub image_padding_vertical_mm: f64,
    pub text_gap_mm: f64,
    pub heading: Option<StampTextProofSettings>,
    pub caption: Option<StampTextProofSettings>,
}

impl StampImageProofSettings {
    fn validate_numbers(&self) -> Result<()> {
        if !self.image_scale_pt_per_pixel.is_finite()
            || self.image_scale_pt_per_pixel <= 0.0
            || !mm_to_points(self.image_scale_pt_per_pixel).is_finite()
        {
            return Err(
                "native_pdf.stamp_image.image_scale_pt_per_pixel must be finite and positive"
                    .into(),
            );
        }
        for (key, value) in [
            (
                "native_pdf.stamp_image.image_padding_horizontal_mm",
                self.image_padding_horizontal_mm,
            ),
            (
                "native_pdf.stamp_image.image_padding_vertical_mm",
                self.image_padding_vertical_mm,
            ),
            ("native_pdf.stamp_image.text_gap_mm", self.text_gap_mm),
        ] {
            if !value.is_finite() || value < 0.0 || !mm_to_points(value).is_finite() {
                return Err(format!("{key} must be finite and nonnegative"));
            }
        }
        self.box_bounds
            .inset(0.0, 0.0)
            .map_err(|error| format!("native_pdf.stamp_image.box_bounds: {error}"))?;
        Ok(())
    }

    pub(crate) fn prepare(&self, page: &PageLayout) -> Result<PreparedStampImage> {
        self.validate_numbers()?;
        if !contains(page.content_bounds, self.box_bounds) {
            return Err(
                "native_pdf.stamp_image.box_bounds must fit within the page content area".into(),
            );
        }

        match self
            .image_path
            .extension()
            .and_then(|extension| extension.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref()
        {
            Some("png" | "jpg" | "jpeg") => {}
            _ => {
                return Err(
                    "native_pdf.stamp_image.image_path must reference a PNG or JPEG image".into(),
                )
            }
        }
        let image = Image::from_file(&self.image_path).map_err(|error| {
            format!(
                "Cannot load native stamp image {}: {error}",
                self.image_path.display()
            )
        })?;

        let image_area = self
            .box_bounds
            .inset(
                self.image_padding_horizontal_mm,
                self.image_padding_vertical_mm,
            )
            .map_err(|error| format!("native_pdf.stamp_image padding: {error}"))?;
        let natural_width_mm =
            points_to_mm(f64::from(image.width()) * self.image_scale_pt_per_pixel);
        let natural_height_mm =
            points_to_mm(f64::from(image.height()) * self.image_scale_pt_per_pixel);
        if ![natural_width_mm, natural_height_mm]
            .iter()
            .all(|value| value.is_finite() && *value > 0.0 && mm_to_points(*value).is_finite())
        {
            return Err("native_pdf.stamp_image source dimensions overflow PDF coordinates".into());
        }
        let scale = ((image_area.right - image_area.left) / natural_width_mm)
            .min((image_area.top - image_area.bottom) / natural_height_mm)
            .min(1.0);
        if !scale.is_finite() || scale <= 0.0 {
            return Err("native_pdf.stamp_image cannot fit the image into its box".into());
        }
        let image_width_mm = natural_width_mm * scale;
        let image_height_mm = natural_height_mm * scale;
        let image_bounds = Bounds {
            left: (image_area.left + image_area.right - image_width_mm) / 2.0,
            right: (image_area.left + image_area.right + image_width_mm) / 2.0,
            bottom: (image_area.bottom + image_area.top - image_height_mm) / 2.0,
            top: (image_area.bottom + image_area.top + image_height_mm) / 2.0,
        };
        let heading = self
            .heading
            .as_ref()
            .map(|text| position_text(text, self.box_bounds, self.text_gap_mm, true, page))
            .transpose()?;
        let caption = self
            .caption
            .as_ref()
            .map(|text| position_text(text, self.box_bounds, self.text_gap_mm, false, page))
            .transpose()?;

        Ok(PreparedStampImage {
            image,
            image_bounds,
            heading,
            caption,
        })
    }
}

pub(crate) struct PreparedStampImage {
    pub image: Image,
    pub image_bounds: Bounds,
    pub heading: Option<PreparedStampText>,
    pub caption: Option<PreparedStampText>,
}

pub(crate) struct PreparedStampText {
    pub font: oxidize_pdf::text::Font,
    pub size_pt: f64,
    pub colour: NamedColour,
    pub lines: Vec<PositionedTextLine>,
}

fn contains(outer: Bounds, inner: Bounds) -> bool {
    inner.left >= outer.left
        && inner.bottom >= outer.bottom
        && inner.right <= outer.right
        && inner.top <= outer.top
}

fn position_text(
    settings: &StampTextProofSettings,
    bounds: Bounds,
    gap_mm: f64,
    above: bool,
    page: &PageLayout,
) -> Result<PreparedStampText> {
    let text = MeasuredText::new(&settings.font, settings.size_pt, &settings.text)
        .map_err(|error| format!("native_pdf.stamp_image text: {error}"))?;
    let available_width = bounds.right - bounds.left;
    if text
        .lines
        .iter()
        .any(|line| line.width_mm > available_width)
    {
        return Err("native_pdf.stamp_image heading/caption exceeds the stamp box width".into());
    }
    let mut lines = Vec::with_capacity(text.lines.len());
    for (index, line) in text.lines.iter().enumerate() {
        let baseline_mm = if above {
            bounds.top
                + gap_mm
                + text.descent_mm
                + (text.lines.len() - index - 1) as f64 * text.line_advance_mm
        } else {
            bounds.bottom - gap_mm - text.ascent_mm - index as f64 * text.line_advance_mm
        };
        lines.push(PositionedTextLine {
            text: line.text.clone(),
            left_mm: (bounds.left + bounds.right - line.width_mm) / 2.0,
            baseline_mm,
            width_mm: line.width_mm,
        });
    }
    let first = lines
        .first()
        .ok_or("native_pdf.stamp_image text has no lines")?;
    let last = lines
        .last()
        .ok_or("native_pdf.stamp_image text has no lines")?;
    let text_bounds = Bounds {
        left: lines
            .iter()
            .map(|line| line.left_mm)
            .fold(f64::INFINITY, f64::min),
        right: lines
            .iter()
            .map(|line| line.left_mm + line.width_mm)
            .fold(f64::NEG_INFINITY, f64::max),
        bottom: last.baseline_mm - text.descent_mm,
        top: first.baseline_mm + text.ascent_mm,
    };
    if !contains(page.content_bounds, text_bounds) {
        return Err(
            "native_pdf.stamp_image heading/caption must fit within the page content area".into(),
        );
    }
    Ok(PreparedStampText {
        font: text.font,
        size_pt: text.size_pt,
        colour: settings.colour,
        lines,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_pdf::{
        commands::{
            colour::colour_album_border::AlbumBorderColour,
            page::{
                album_pages_border::PageBorder, album_pages_margins::Margins,
                album_pages_size::PageSize, album_pages_spacing::Spacing,
            },
        },
        document::PageSettings,
    };

    fn page_layout() -> Result<PageLayout> {
        let settings = PageSettings::new(
            PageSize::parse(&[210.0, 297.0])?,
            Margins::parse(&[12.0; 4])?,
            Spacing::parse(&[3.0, 4.0])?,
            PageBorder::parse(&[0.5, 0.0, 0.0, 0.0])?,
            AlbumBorderColour::parse(&["black"])?,
        );
        settings.resolve(1)
    }

    #[test]
    fn natural_size_image_is_centered_in_padded_box_without_upscaling() -> Result<()> {
        let temporary = tempfile::tempdir().map_err(|error| error.to_string())?;
        let path = temporary.path().join("stamp.png");
        image::RgbImage::from_pixel(72, 100, image::Rgb([40, 80, 120]))
            .save(&path)
            .map_err(|error| error.to_string())?;
        let settings = StampImageProofSettings {
            image_path: path,
            box_bounds: Bounds {
                left: 80.0,
                bottom: 116.0,
                right: 130.0,
                top: 176.0,
            },
            image_scale_pt_per_pixel: 1.0,
            image_padding_horizontal_mm: 2.0,
            image_padding_vertical_mm: 2.0,
            text_gap_mm: 2.0,
            heading: Some(StampTextProofSettings::new(
                "HEADING",
                "HB",
                8.0,
                NamedColour::parse("black")?,
            )),
            caption: Some(StampTextProofSettings::new(
                "CAPTION",
                "HN",
                6.0,
                NamedColour::parse("black")?,
            )),
        };

        let prepared = settings.prepare(&page_layout()?)?;

        let width_mm = prepared.image_bounds.right - prepared.image_bounds.left;
        let height_mm = prepared.image_bounds.top - prepared.image_bounds.bottom;
        assert!((width_mm - points_to_mm(72.0)).abs() < 0.000001);
        assert!((height_mm - points_to_mm(100.0)).abs() < 0.000001);
        assert!(
            (prepared.image_bounds.left + prepared.image_bounds.right - 210.0).abs() < 0.000001
        );
        assert!(
            (prepared.image_bounds.bottom + prepared.image_bounds.top - 292.0).abs() < 0.000001
        );
        let heading = prepared
            .heading
            .as_ref()
            .and_then(|heading| heading.lines.first())
            .ok_or("heading line")?;
        let heading_metrics = MeasuredText::new("HB", 8.0, "HEADING")?;
        assert!(
            (heading.baseline_mm
                - settings.box_bounds.top
                - settings.text_gap_mm
                - heading_metrics.descent_mm)
                .abs()
                < 0.000001
        );
        let caption = prepared
            .caption
            .as_ref()
            .and_then(|caption| caption.lines.first())
            .ok_or("caption line")?;
        let caption_metrics = MeasuredText::new("HN", 6.0, "CAPTION")?;
        assert!(
            (settings.box_bounds.bottom
                - caption.baseline_mm
                - caption_metrics.ascent_mm
                - settings.text_gap_mm)
                .abs()
                < 0.000001
        );
        Ok(())
    }

    #[test]
    fn embedded_reference_scale_matches_great_britain_albumeasy_footprints() -> Result<()> {
        let temporary = tempfile::tempdir().map_err(|error| error.to_string())?;
        let (_, mm_per_pixel_h) = crate::image_utils::calc_mm_per_pixel_embedded(25.0, 30.0)
            .ok_or("embedded reference image dimensions")?;
        let image_scale_pt_per_pixel = f64::from(mm_per_pixel_h) * 72.0 / 25.4;
        let box_bounds = Bounds {
            left: 80.0,
            bottom: 116.0,
            right: 104.0,
            top: 143.0,
        };

        for (filename, width, height, expected_width_mm) in [
            ("high-resolution.png", 398, 473, 17.6702),
            ("low-resolution.jpg", 117, 140, 17.55),
        ] {
            let path = temporary.path().join(filename);
            image::RgbImage::from_pixel(width, height, image::Rgb([40, 80, 120]))
                .save(&path)
                .map_err(|error| error.to_string())?;
            let settings = StampImageProofSettings {
                image_path: path,
                box_bounds,
                image_scale_pt_per_pixel,
                image_padding_horizontal_mm: 3.0,
                image_padding_vertical_mm: 3.0,
                text_gap_mm: 2.0,
                heading: None,
                caption: None,
            };

            let prepared = settings.prepare(&page_layout()?)?;
            let width_mm = prepared.image_bounds.right - prepared.image_bounds.left;
            let height_mm = prepared.image_bounds.top - prepared.image_bounds.bottom;
            assert!((width_mm - expected_width_mm).abs() < 0.01);
            assert!((height_mm - 21.0).abs() < 0.01);
        }
        Ok(())
    }
}
