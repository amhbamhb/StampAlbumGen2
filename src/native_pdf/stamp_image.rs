//! Proof-only placement for one stamp image with measured heading and caption.

use super::{
    colour::NamedColour,
    document::{Bounds, PageLayout},
    stamp_box::StampBoxGeometry,
    text::{MeasuredText, PositionedTextLine},
    units::{mm_to_points, points_to_mm},
    Result,
};
use oxidize_pdf::Image;
use std::path::PathBuf;

pub(super) const STAMP_IMAGE_LABEL_GAP_MM: f64 = 1.0;

/// Image input for a complete text/frame row item; physical size comes from the stamp.
#[derive(Clone, Debug)]
pub struct StampImageRowItem {
    pub stamp: super::stamp_text::StampTextItem,
    pub image_path: PathBuf,
    pub image_scale_pt_per_pixel: f64,
    pub options: StampImageOptions,
    pub frame_style: Option<StampFrameStyle>,
    pub labels: Option<StampImageLabels>,
}

#[derive(Clone, Copy, Debug)]
pub struct StampFrameStyle {
    pub outline: bool,
    pub background: Option<NamedColour>,
    pub inner_border: Option<super::stamp_inner_border::StampInnerBorder>,
    pub outer_style: Option<super::stamp_inner_border::BorderStyleState>,
    pub inner_style: Option<super::stamp_inner_border::BorderStyleState>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImagePaddingUnit {
    Millimetres,
    Percent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageAspectRatio {
    Preserve,
    Fit,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StampImageOptions {
    pub padding_unit: ImagePaddingUnit,
    pub padding_horizontal: f64,
    pub padding_vertical: f64,
    pub stretch_small: bool,
    pub aspect_ratio: ImageAspectRatio,
    pub grayscale: bool,
    pub visible: bool,
}

#[derive(Clone, Debug, Default)]
pub struct StampImageLabels {
    pub above: Option<StampTextSettings>,
    pub below: Option<StampTextSettings>,
    pub left: Option<StampSideLabelSettings>,
    pub right: Option<StampSideLabelSettings>,
    pub wrap_width_mm: Option<f64>,
    pub max_lines: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct NativePageBackgroundImage {
    pub image_path: PathBuf,
    pub bounds: Bounds,
}

impl Default for StampImageOptions {
    fn default() -> Self {
        Self {
            padding_unit: ImagePaddingUnit::Millimetres,
            padding_horizontal: 0.0,
            padding_vertical: 0.0,
            stretch_small: false,
            aspect_ratio: ImageAspectRatio::Preserve,
            grayscale: false,
            visible: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StampTextSettings {
    pub text: String,
    pub font: String,
    pub size_pt: f64,
    pub colour: NamedColour,
}

#[derive(Clone, Debug)]
pub struct StampSideLabelSettings {
    pub text: String,
    pub font: String,
    pub size_pt: Option<f64>,
    pub colour: NamedColour,
}

impl StampSideLabelSettings {
    pub fn new(
        text: impl Into<String>,
        font: impl Into<String>,
        size_pt: Option<f64>,
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

impl StampTextSettings {
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

/// Backwards-compatible name for proof callers.
pub type StampTextProofSettings = StampTextSettings;

/// Explicit geometry and image policy for rendering one native stamp image.
///
/// The source is measured at `image_scale_pt_per_pixel`; image policy controls
/// padding, enlargement, aspect ratio, grayscale, and visibility.
#[derive(Clone, Debug)]
pub struct StampImageSettings {
    pub image_path: PathBuf,
    pub box_bounds: Bounds,
    pub image_scale_pt_per_pixel: f64,
    pub options: StampImageOptions,
    pub image_padding_horizontal_mm: f64,
    pub image_padding_vertical_mm: f64,
    pub text_gap_mm: f64,
    pub heading: Option<StampTextSettings>,
    pub caption: Option<StampTextSettings>,
    pub labels: Option<StampImageLabels>,
}

impl StampImageSettings {
    /// Uses the mounting frame for text anchors while fitting the illustration
    /// inside the physical stamp area. The source image scale is unchanged.
    pub fn with_stamp_box(mut self, geometry: &StampBoxGeometry) -> Self {
        self.box_bounds = geometry.frame_bounds();
        self.image_padding_horizontal_mm = geometry.horizontal_clearance_mm();
        self.image_padding_vertical_mm = geometry.vertical_clearance_mm();
        self
    }

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
        for (key, value) in [
            (
                "native_pdf.stamp_image.options.padding_horizontal",
                self.options.padding_horizontal,
            ),
            (
                "native_pdf.stamp_image.options.padding_vertical",
                self.options.padding_vertical,
            ),
        ] {
            if !value.is_finite() || value < 0.0 {
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
            return Err(format!(
                "native_pdf.stamp_image.box_bounds ({:.3}, {:.3})-({:.3}, {:.3}) must fit within page content bounds ({:.3}, {:.3})-({:.3}, {:.3})",
                self.box_bounds.left,
                self.box_bounds.bottom,
                self.box_bounds.right,
                self.box_bounds.top,
                page.content_bounds.left,
                page.content_bounds.bottom,
                page.content_bounds.right,
                page.content_bounds.top,
            ));
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
        let image = if self.options.grayscale {
            let decoded = image::open(&self.image_path).map_err(|error| {
                format!(
                    "Cannot load native stamp image {}: {error}",
                    self.image_path.display()
                )
            })?;
            let grayscale = decoded.to_luma8();
            Image::from_gray_data(grayscale.into_raw(), decoded.width(), decoded.height()).map_err(
                |error| {
                    format!(
                        "Cannot prepare grayscale native stamp image {}: {error}",
                        self.image_path.display()
                    )
                },
            )?
        } else if self
            .image_path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("png"))
        {
            // Decode palette/low-bit-depth PNGs before handing pixels to the PDF backend.
            let decoded = image::open(&self.image_path).map_err(|error| {
                format!(
                    "Cannot load native stamp image {}: {error}",
                    self.image_path.display()
                )
            })?;
            Image::from_rgba_data(
                decoded.to_rgba8().into_raw(),
                decoded.width(),
                decoded.height(),
            )
            .map_err(|error| {
                format!(
                    "Cannot prepare native stamp image {}: {error}",
                    self.image_path.display()
                )
            })?
        } else {
            Image::from_file(&self.image_path).map_err(|error| {
                format!(
                    "Cannot load native stamp image {}: {error}",
                    self.image_path.display()
                )
            })?
        };

        let box_width = self.box_bounds.right - self.box_bounds.left;
        let box_height = self.box_bounds.top - self.box_bounds.bottom;
        let (option_padding_horizontal_mm, option_padding_vertical_mm) =
            match self.options.padding_unit {
                ImagePaddingUnit::Millimetres => (
                    self.options.padding_horizontal,
                    self.options.padding_vertical,
                ),
                ImagePaddingUnit::Percent => (
                    box_width * self.options.padding_horizontal / 100.0,
                    box_height * self.options.padding_vertical / 100.0,
                ),
            };
        let image_area = self
            .box_bounds
            .inset(
                self.image_padding_horizontal_mm + option_padding_horizontal_mm,
                self.image_padding_vertical_mm + option_padding_vertical_mm,
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
        let available_width_mm = image_area.right - image_area.left;
        let available_height_mm = image_area.top - image_area.bottom;
        let image_is_small =
            natural_width_mm <= available_width_mm && natural_height_mm <= available_height_mm;
        let (image_width_mm, image_height_mm) = match self.options.aspect_ratio {
            ImageAspectRatio::Preserve => {
                let fit_scale = (available_width_mm / natural_width_mm)
                    .min(available_height_mm / natural_height_mm);
                let scale = if image_is_small && !self.options.stretch_small {
                    1.0
                } else {
                    fit_scale
                };
                (natural_width_mm * scale, natural_height_mm * scale)
            }
            ImageAspectRatio::Fit if image_is_small && !self.options.stretch_small => {
                (natural_width_mm, natural_height_mm)
            }
            ImageAspectRatio::Fit => (available_width_mm, available_height_mm),
        };
        if !image_width_mm.is_finite()
            || !image_height_mm.is_finite()
            || image_width_mm <= 0.0
            || image_height_mm <= 0.0
            || image_width_mm > available_width_mm + 1e-9
            || image_height_mm > available_height_mm + 1e-9
        {
            return Err("native_pdf.stamp_image cannot fit the image into its box".into());
        }
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
        let labels = self
            .labels
            .as_ref()
            .map(|labels| {
                let above = labels
                    .above
                    .as_ref()
                    .map(|text| {
                        position_image_label(
                            text,
                            Bounds {
                                left: image_area.left,
                                bottom: image_bounds.top + STAMP_IMAGE_LABEL_GAP_MM,
                                right: image_area.right,
                                top: image_area.top,
                            },
                            page,
                        )
                    })
                    .transpose()?;
                let below = labels
                    .below
                    .as_ref()
                    .map(|text| {
                        position_image_label(
                            text,
                            Bounds {
                                left: image_area.left,
                                bottom: image_area.bottom,
                                right: image_area.right,
                                top: image_bounds.bottom - STAMP_IMAGE_LABEL_GAP_MM,
                            },
                            page,
                        )
                    })
                    .transpose()?;
                let left = labels
                    .left
                    .as_ref()
                    .map(|text| position_side_image_label(text, self.box_bounds, page, true))
                    .transpose()?;
                let right = labels
                    .right
                    .as_ref()
                    .map(|text| position_side_image_label(text, self.box_bounds, page, false))
                    .transpose()?;
                Ok::<_, String>([above, below, left, right].into_iter().flatten().collect())
            })
            .transpose()?
            .unwrap_or_default();

        Ok(PreparedStampImage {
            image: self.options.visible.then_some(image),
            image_bounds,
            heading,
            caption,
            labels,
        })
    }
}

/// Backwards-compatible name for existing geometry-proof callers.
pub type StampImageProofSettings = StampImageSettings;

pub(crate) struct PreparedStampImage {
    pub image: Option<Image>,
    pub image_bounds: Bounds,
    pub heading: Option<PreparedStampText>,
    pub caption: Option<PreparedStampText>,
    pub labels: Vec<PreparedStampText>,
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
    settings: &StampTextSettings,
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

fn position_image_label(
    settings: &StampTextSettings,
    slot: Bounds,
    page: &PageLayout,
) -> Result<PreparedStampText> {
    let text = MeasuredText::new(&settings.font, settings.size_pt, &settings.text)
        .map_err(|error| format!("native_pdf.stamp_image label: {error}"))?;
    let width_mm = text
        .lines
        .iter()
        .map(|line| line.width_mm)
        .fold(0.0, f64::max);
    let height_mm =
        text.ascent_mm + text.descent_mm + (text.lines.len() - 1) as f64 * text.line_advance_mm;
    let slot_width = slot.right - slot.left;
    let slot_height = slot.top - slot.bottom;
    if width_mm > slot_width || height_mm > slot_height {
        return Err(
            "native_pdf.stamp_image label does not fit in the whitespace beside the image".into(),
        );
    }

    let center = (slot.left + slot.right) / 2.0;
    let top = slot.top - (slot_height - height_mm) / 2.0;
    let lines = text
        .lines
        .iter()
        .enumerate()
        .map(|(index, line)| PositionedTextLine {
            text: line.text.clone(),
            left_mm: center - line.width_mm / 2.0,
            baseline_mm: top - text.ascent_mm - index as f64 * text.line_advance_mm,
            width_mm: line.width_mm,
        })
        .collect();
    let bounds = Bounds {
        left: center - width_mm / 2.0,
        right: center + width_mm / 2.0,
        top,
        bottom: top - height_mm,
    };
    if !contains(slot, bounds) || !contains(page.content_bounds, bounds) {
        return Err("native_pdf.stamp_image label exceeds its available frame area".into());
    }
    Ok(PreparedStampText {
        font: text.font,
        size_pt: text.size_pt,
        colour: settings.colour,
        lines,
    })
}

fn position_side_image_label(
    settings: &StampSideLabelSettings,
    frame: Bounds,
    page: &PageLayout,
    left_side: bool,
) -> Result<PreparedStampText> {
    let size_pt = settings
        .size_pt
        .ok_or("native_pdf.stamp_image side-label font size was not resolved from TOML")?;
    let text = MeasuredText::new(&settings.font, size_pt, &settings.text)
        .map_err(|error| format!("native_pdf.stamp_image side label: {error}"))?;
    let width_mm = text
        .lines
        .iter()
        .map(|line| line.width_mm)
        .fold(0.0, f64::max);
    let height_mm =
        text.ascent_mm + text.descent_mm + (text.lines.len() - 1) as f64 * text.line_advance_mm;
    let frame_height = frame.top - frame.bottom;
    let anchor = if left_side {
        frame.left - STAMP_IMAGE_LABEL_GAP_MM
    } else {
        frame.right + STAMP_IMAGE_LABEL_GAP_MM
    };
    let top = frame.top - (frame_height - height_mm) / 2.0;
    let lines: Vec<_> = text
        .lines
        .iter()
        .enumerate()
        .map(|(index, line)| PositionedTextLine {
            text: line.text.clone(),
            left_mm: if left_side {
                anchor - line.width_mm
            } else {
                anchor
            },
            baseline_mm: top - text.ascent_mm - index as f64 * text.line_advance_mm,
            width_mm: line.width_mm,
        })
        .collect();
    let bounds = Bounds {
        left: if left_side { anchor - width_mm } else { anchor },
        right: if left_side { anchor } else { anchor + width_mm },
        top,
        bottom: top - height_mm,
    };
    if !contains(page.content_bounds, bounds) {
        return Err("native_pdf.stamp_image side label exceeds page content bounds".into());
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
            options: StampImageOptions::default(),
            image_padding_horizontal_mm: 2.0,
            image_padding_vertical_mm: 2.0,
            text_gap_mm: 2.0,
            heading: Some(StampTextSettings::new(
                "HEADING",
                "HB",
                8.0,
                NamedColour::parse("black")?,
            )),
            caption: Some(StampTextSettings::new(
                "CAPTION",
                "HN",
                6.0,
                NamedColour::parse("black")?,
            )),
            labels: None,
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
                options: StampImageOptions::default(),
                image_padding_horizontal_mm: 3.0,
                image_padding_vertical_mm: 3.0,
                text_gap_mm: 2.0,
                heading: None,
                caption: None,
                labels: None,
            };

            let prepared = settings.prepare(&page_layout()?)?;
            let width_mm = prepared.image_bounds.right - prepared.image_bounds.left;
            let height_mm = prepared.image_bounds.top - prepared.image_bounds.bottom;
            assert!((width_mm - expected_width_mm).abs() < 0.01);
            assert!((height_mm - 21.0).abs() < 0.01);
        }
        Ok(())
    }

    #[test]
    fn image_options_control_padding_fit_grayscale_and_visibility() -> Result<()> {
        let temporary = tempfile::tempdir().map_err(|error| error.to_string())?;
        let path = temporary.path().join("stamp.png");
        image::RgbImage::from_pixel(100, 50, image::Rgb([40, 80, 120]))
            .save(&path)
            .map_err(|error| error.to_string())?;
        let make_settings = |options| StampImageSettings {
            image_path: path.clone(),
            box_bounds: Bounds {
                left: 80.0,
                bottom: 100.0,
                right: 130.0,
                top: 140.0,
            },
            image_scale_pt_per_pixel: mm_to_points(0.1),
            options,
            image_padding_horizontal_mm: 0.0,
            image_padding_vertical_mm: 0.0,
            text_gap_mm: 0.0,
            heading: None,
            caption: None,
            labels: None,
        };
        let mut options = StampImageOptions {
            padding_unit: ImagePaddingUnit::Percent,
            padding_horizontal: 10.0,
            padding_vertical: 10.0,
            ..StampImageOptions::default()
        };

        let natural = make_settings(options).prepare(&page_layout()?)?;
        assert!((natural.image_bounds.right - natural.image_bounds.left - 10.0).abs() < 0.001);
        assert!((natural.image_bounds.top - natural.image_bounds.bottom - 5.0).abs() < 0.001);
        assert_eq!(
            natural
                .image
                .as_ref()
                .ok_or("visible source image missing")?
                .data()
                .len(),
            15_000
        );

        options.stretch_small = true;
        let stretched = make_settings(options).prepare(&page_layout()?)?;
        assert!((stretched.image_bounds.right - stretched.image_bounds.left - 40.0).abs() < 0.001);
        assert!((stretched.image_bounds.top - stretched.image_bounds.bottom - 20.0).abs() < 0.001);

        options.aspect_ratio = ImageAspectRatio::Fit;
        let fitted = make_settings(options).prepare(&page_layout()?)?;
        assert!((fitted.image_bounds.right - fitted.image_bounds.left - 40.0).abs() < 0.001);
        assert!((fitted.image_bounds.top - fitted.image_bounds.bottom - 32.0).abs() < 0.001);

        options.padding_unit = ImagePaddingUnit::Millimetres;
        options.padding_horizontal = 2.0;
        options.padding_vertical = 3.0;
        let padded = make_settings(options).prepare(&page_layout()?)?;
        assert!((padded.image_bounds.right - padded.image_bounds.left - 46.0).abs() < 0.001);
        assert!((padded.image_bounds.top - padded.image_bounds.bottom - 34.0).abs() < 0.001);

        options.grayscale = true;
        let grayscale = make_settings(options).prepare(&page_layout()?)?;
        assert_eq!(
            grayscale
                .image
                .as_ref()
                .ok_or("visible grayscale image missing")?
                .data()
                .len(),
            5_000
        );

        options.visible = false;
        let hidden = make_settings(options).prepare(&page_layout()?)?;
        assert!(hidden.image.is_none());
        Ok(())
    }

    #[test]
    fn image_labels_are_centered_in_whitespace_without_overlapping_the_image() -> Result<()> {
        let temporary = tempfile::tempdir().map_err(|error| error.to_string())?;
        let path = temporary.path().join("stamp.png");
        image::RgbImage::from_pixel(100, 50, image::Rgb([40, 80, 120]))
            .save(&path)
            .map_err(|error| error.to_string())?;
        let settings = StampImageSettings {
            image_path: path,
            box_bounds: Bounds {
                left: 80.0,
                bottom: 100.0,
                right: 130.0,
                top: 140.0,
            },
            image_scale_pt_per_pixel: mm_to_points(0.1),
            options: StampImageOptions::default(),
            image_padding_horizontal_mm: 0.0,
            image_padding_vertical_mm: 0.0,
            text_gap_mm: 0.0,
            heading: None,
            caption: None,
            labels: Some(StampImageLabels {
                above: Some(StampTextSettings::new(
                    "SG 1137b",
                    "HN",
                    4.0,
                    NamedColour::parse("black")?,
                )),
                below: Some(StampTextSettings::new(
                    "SG 1137b",
                    "HN",
                    4.0,
                    NamedColour::parse("black")?,
                )),
                left: None,
                right: None,
                ..StampImageLabels::default()
            }),
        };

        let prepared = settings.prepare(&page_layout()?)?;
        assert_eq!(prepared.labels.len(), 2);
        let label_metrics = MeasuredText::new("HN", 4.0, "SG 1137b")?;
        assert!(
            prepared.labels[0].lines[0].baseline_mm
                - label_metrics.descent_mm
                - prepared.image_bounds.top
                >= STAMP_IMAGE_LABEL_GAP_MM
        );
        assert!(
            prepared.image_bounds.bottom
                - prepared.labels[1].lines[0].baseline_mm
                - label_metrics.ascent_mm
                >= STAMP_IMAGE_LABEL_GAP_MM
        );
        assert!(prepared.labels.iter().all(|label| {
            label.lines.iter().all(|line| {
                line.left_mm >= settings.box_bounds.left
                    && line.left_mm + line.width_mm <= settings.box_bounds.right
            })
        }));
        Ok(())
    }

    #[test]
    fn image_labels_fail_when_the_image_leaves_no_room_inside_the_frame() -> Result<()> {
        let temporary = tempfile::tempdir().map_err(|error| error.to_string())?;
        let path = temporary.path().join("stamp.png");
        image::RgbImage::from_pixel(100, 50, image::Rgb([40, 80, 120]))
            .save(&path)
            .map_err(|error| error.to_string())?;
        let settings = StampImageSettings {
            image_path: path,
            box_bounds: Bounds {
                left: 80.0,
                bottom: 100.0,
                right: 130.0,
                top: 140.0,
            },
            image_scale_pt_per_pixel: mm_to_points(0.1),
            options: StampImageOptions {
                stretch_small: true,
                aspect_ratio: ImageAspectRatio::Fit,
                ..StampImageOptions::default()
            },
            image_padding_horizontal_mm: 0.0,
            image_padding_vertical_mm: 0.0,
            text_gap_mm: 0.0,
            heading: None,
            caption: None,
            labels: Some(StampImageLabels {
                above: Some(StampTextSettings::new(
                    "SG 1137b",
                    "HN",
                    4.0,
                    NamedColour::parse("black")?,
                )),
                below: None,
                left: None,
                right: None,
                ..StampImageLabels::default()
            }),
        };

        let error = settings
            .prepare(&page_layout()?)
            .err()
            .ok_or("label overlapped a full-frame image")?;
        assert!(error.contains("does not fit in the whitespace beside the image"));
        Ok(())
    }

    #[test]
    fn side_image_labels_are_centered_outside_the_frame() -> Result<()> {
        let temporary = tempfile::tempdir().map_err(|error| error.to_string())?;
        let path = temporary.path().join("stamp.png");
        image::RgbImage::from_pixel(100, 50, image::Rgb([40, 80, 120]))
            .save(&path)
            .map_err(|error| error.to_string())?;
        let frame = Bounds {
            left: 80.0,
            bottom: 100.0,
            right: 130.0,
            top: 140.0,
        };
        let settings = StampImageSettings {
            image_path: path,
            box_bounds: frame,
            image_scale_pt_per_pixel: mm_to_points(0.1),
            options: StampImageOptions::default(),
            image_padding_horizontal_mm: 0.0,
            image_padding_vertical_mm: 0.0,
            text_gap_mm: 0.0,
            heading: None,
            caption: None,
            labels: Some(StampImageLabels {
                above: None,
                below: None,
                left: Some(StampSideLabelSettings::new(
                    "SG 1137b",
                    "HN",
                    Some(4.0),
                    NamedColour::parse("black")?,
                )),
                right: Some(StampSideLabelSettings::new(
                    "SG 1137b",
                    "HN",
                    Some(4.0),
                    NamedColour::parse("black")?,
                )),
                ..StampImageLabels::default()
            }),
        };

        let prepared = settings.prepare(&page_layout()?)?;
        assert_eq!(prepared.labels.len(), 2);
        let text = MeasuredText::new("HN", 4.0, "SG 1137b")?;
        let left = &prepared.labels[0].lines[0];
        let right = &prepared.labels[1].lines[0];
        assert!(
            (left.left_mm + left.width_mm + STAMP_IMAGE_LABEL_GAP_MM - frame.left).abs() < 0.001
        );
        assert!((right.left_mm - frame.right - STAMP_IMAGE_LABEL_GAP_MM).abs() < 0.001);
        for label in &prepared.labels {
            let baseline = label.lines[0].baseline_mm;
            assert!(
                (baseline
                    - text.descent_mm
                    - frame.bottom
                    - (frame.top - frame.bottom - text.ascent_mm - text.descent_mm) / 2.0)
                    .abs()
                    < 0.001
            );
        }
        Ok(())
    }

    #[test]
    fn side_image_label_fails_when_it_exceeds_page_content() -> Result<()> {
        let temporary = tempfile::tempdir().map_err(|error| error.to_string())?;
        let path = temporary.path().join("stamp.png");
        image::RgbImage::from_pixel(100, 50, image::Rgb([40, 80, 120]))
            .save(&path)
            .map_err(|error| error.to_string())?;
        let settings = StampImageSettings {
            image_path: path,
            box_bounds: Bounds {
                left: 20.0,
                bottom: 100.0,
                right: 70.0,
                top: 140.0,
            },
            image_scale_pt_per_pixel: mm_to_points(0.1),
            options: StampImageOptions::default(),
            image_padding_horizontal_mm: 0.0,
            image_padding_vertical_mm: 0.0,
            text_gap_mm: 0.0,
            heading: None,
            caption: None,
            labels: Some(StampImageLabels {
                left: Some(StampSideLabelSettings::new(
                    "SG 1137b",
                    "HN",
                    Some(4.0),
                    NamedColour::parse("black")?,
                )),
                ..StampImageLabels::default()
            }),
        };

        let error = settings
            .prepare(&page_layout()?)
            .err()
            .ok_or("side label outside the page content was accepted")?;
        assert!(error.contains("side label exceeds page content bounds"));
        Ok(())
    }
}
