use super::{
    colour::NamedColour,
    document::{PageLayout, PageSettings},
    pagination::{paginate_stamp_sections, StampItemPage, StampPageFragment, StampRowSection},
    proof::GeometryProofSettings,
    stamp_box::StampBoxGeometry,
    stamp_image::{
        NativePageBackgroundImage, PreparedStampImage, StampImageProofSettings, StampImageRowItem,
        StampImageSettings,
    },
    stamp_rows::fit_known_size_rows_with_alignment,
    stamp_text::{
        fit_text_rows_with_alignment, PositionedStampItem, StampTextItem, StampTextPolicy,
    },
    text::PositionedTextLine,
    units::mm_to_points,
    Result,
};
use oxidize_pdf::{Color, Document, Image, Page};
use std::path::Path;

/// Isolated geometry proof. Stamp-image content is available through
/// `write_stamp_image_proof`; production row layout is not wired up.
pub fn write_page_geometry_proof(
    settings: &PageSettings,
    proof: &GeometryProofSettings,
    output: &Path,
) -> Result<()> {
    let generation_date = settings.capture_generation_date();
    write_page_geometry_proof_with_date(settings, proof, output, generation_date)
}

/// Explicit context entry point for deterministic generation-date testing and
/// callers that also report the resolved layout. Fixed TOML dates take precedence.
pub fn write_page_geometry_proof_with_date(
    settings: &PageSettings,
    proof: &GeometryProofSettings,
    output: &Path,
    generation_date: Option<chrono::NaiveDate>,
) -> Result<()> {
    write_proof(settings, proof, output, generation_date, &[], &[], &[])
}

/// Writes a one-page diagnostic proof with one explicitly sized stamp image.
pub fn write_stamp_image_proof(
    settings: &PageSettings,
    proof: &GeometryProofSettings,
    stamp: &StampImageProofSettings,
    output: &Path,
) -> Result<()> {
    if proof.page_count != 1 {
        return Err("native_pdf.stamp_image proof requires exactly one page".into());
    }
    write_proof(
        settings,
        proof,
        output,
        settings.capture_generation_date(),
        std::slice::from_ref(stamp),
        &[],
        &[],
    )
}

/// Diagnostic frame/physical outlines for a known-size stamp.
/// These guides are not production stamp-frame styling.
pub fn write_stamp_box_geometry_proof(
    settings: &PageSettings,
    proof: &GeometryProofSettings,
    stamp: &StampImageProofSettings,
    geometry: &StampBoxGeometry,
    output: &Path,
) -> Result<()> {
    if proof.page_count != 1 {
        return Err("native_pdf.stamp_box proof requires exactly one page".into());
    }
    let stamp = stamp.clone().with_stamp_box(geometry);
    write_proof(
        settings,
        proof,
        output,
        settings.capture_generation_date(),
        std::slice::from_ref(&stamp),
        std::slice::from_ref(geometry),
        &[],
    )
}

/// Single-page frame-only diagnostic. Text and illustration footprints are excluded.
pub fn write_stamp_rows_geometry_proof(
    settings: &PageSettings,
    proof: &GeometryProofSettings,
    stamps: &[StampBoxGeometry],
    output: &Path,
) -> Result<()> {
    if proof.page_count != 1 {
        return Err("native_pdf.stamp_rows proof requires exactly one page".into());
    }
    let generation_date = settings.capture_generation_date();
    let layout = settings.resolve_with_context(1, 1, generation_date)?;
    let rows = fit_known_size_rows_with_alignment(
        layout.content_bounds,
        stamps,
        settings.spacing(),
        settings.row_alignment(),
    )?;
    let geometries: Vec<_> = rows.into_iter().flatten().collect();
    write_proof(
        settings,
        proof,
        output,
        generation_date,
        &[],
        &geometries,
        &[],
    )
}

/// Text-footprint proof only: no production export/backend is selected.
pub fn write_stamp_text_geometry_proof(
    settings: &PageSettings,
    proof: &GeometryProofSettings,
    items: &[StampTextItem],
    policy: &StampTextPolicy,
    output: &Path,
) -> Result<()> {
    if proof.page_count != 1 {
        return Err("native_pdf.stamp_text proof requires exactly one page".into());
    }
    let generation_date = settings.capture_generation_date();
    let layout = settings.resolve_with_context(1, 1, generation_date)?;
    let rows = fit_text_rows_with_alignment(
        layout.content_bounds,
        items,
        settings.spacing(),
        policy,
        settings.row_alignment(),
    )?;
    let items: Vec<_> = rows.into_iter().flatten().collect();
    let geometries: Vec<_> = items.iter().map(|item| item.geometry).collect();
    write_proof(
        settings,
        proof,
        output,
        generation_date,
        &[],
        &geometries,
        &items,
    )
}

/// Combined single-page image/text proof. All assets are validated before output writes.
pub fn write_stamp_items_geometry_proof(
    settings: &PageSettings,
    proof: &GeometryProofSettings,
    items: &[StampImageRowItem],
    policy: &StampTextPolicy,
    output: &Path,
) -> Result<()> {
    if proof.page_count != 1 {
        return Err("native_pdf.stamp_items proof requires exactly one page".into());
    }
    let generation_date = settings.capture_generation_date();
    let layout = settings.resolve_with_context(1, 1, generation_date)?;
    let stamps: Vec<_> = items.iter().map(|item| item.stamp.clone()).collect();
    let rows = fit_text_rows_with_alignment(
        layout.content_bounds,
        &stamps,
        settings.spacing(),
        policy,
        settings.row_alignment(),
    )?;
    let text_items: Vec<_> = rows.into_iter().flatten().collect();
    let geometries: Vec<_> = text_items.iter().map(|item| item.geometry).collect();
    let images: Vec<_> = items
        .iter()
        .zip(&geometries)
        .map(|(item, geometry)| {
            StampImageSettings {
                image_path: item.image_path.clone(),
                box_bounds: geometry.frame_bounds(),
                image_scale_pt_per_pixel: item.image_scale_pt_per_pixel,
                options: item.options,
                image_padding_horizontal_mm: 0.0,
                image_padding_vertical_mm: 0.0,
                text_gap_mm: 0.0,
                heading: None,
                caption: None,
            }
            .with_stamp_box(geometry)
        })
        .collect();
    write_proof(
        settings,
        proof,
        output,
        generation_date,
        &images,
        &geometries,
        &text_items,
    )
}

/// Automatic-page-count diagnostic with real images and text continuation.
/// The configured proof page_count is replaced by the actual resolved count.
/// All page layouts/assets are prepared before creating or truncating output.
pub fn write_paginated_stamp_items_geometry_proof(
    settings: &PageSettings,
    proof: &GeometryProofSettings,
    items: &[StampImageRowItem],
    policy: &StampTextPolicy,
    output: &Path,
) -> Result<Vec<StampItemPage>> {
    let sections = if items.is_empty() {
        Vec::new()
    } else {
        vec![StampRowSection {
            item_count: items.len(),
            alignment: settings.row_alignment(),
        }]
    };
    write_paginated_stamp_sections_geometry_proof(settings, proof, items, &sections, policy, output)
}

pub fn write_paginated_stamp_sections_geometry_proof(
    settings: &PageSettings,
    proof: &GeometryProofSettings,
    items: &[StampImageRowItem],
    sections: &[StampRowSection],
    policy: &StampTextPolicy,
    output: &Path,
) -> Result<Vec<StampItemPage>> {
    write_paginated_stamp_sections(
        settings,
        RenderMode {
            proof: Some(proof),
            diagnostic_overlays: true,
            native_frame: None,
            page_background: None,
        },
        items,
        sections,
        policy,
        output,
    )
}

/// Writes a native album PDF directly from resolved native layout inputs.
/// This path does not draw diagnostic guides or require proof settings.
pub fn write_native_album_pdf(
    settings: &PageSettings,
    items: &[StampImageRowItem],
    policy: &StampTextPolicy,
    frame_width_mm: f64,
    frame_colour: NamedColour,
    page_background: Option<&NativePageBackgroundImage>,
    output: &Path,
) -> Result<Vec<StampItemPage>> {
    let sections = if items.is_empty() {
        Vec::new()
    } else {
        vec![StampRowSection {
            item_count: items.len(),
            alignment: settings.row_alignment(),
        }]
    };
    write_native_album_pdf_with_sections(
        settings,
        items,
        &sections,
        policy,
        frame_width_mm,
        frame_colour,
        page_background,
        output,
    )
}

pub fn write_native_album_pdf_with_sections(
    settings: &PageSettings,
    items: &[StampImageRowItem],
    sections: &[StampRowSection],
    policy: &StampTextPolicy,
    frame_width_mm: f64,
    frame_colour: NamedColour,
    page_background: Option<&NativePageBackgroundImage>,
    output: &Path,
) -> Result<Vec<StampItemPage>> {
    if !frame_width_mm.is_finite()
        || frame_width_mm <= 0.0
        || !mm_to_points(frame_width_mm).is_finite()
    {
        return Err("Native stamp frame width must be finite and positive".into());
    }
    write_paginated_stamp_sections(
        settings,
        RenderMode {
            proof: None,
            diagnostic_overlays: false,
            native_frame: Some((frame_width_mm, frame_colour)),
            page_background,
        },
        items,
        sections,
        policy,
        output,
    )
}

fn prepare_page_background(
    background: &NativePageBackgroundImage,
    layout: &PageLayout,
) -> Result<(Image, super::document::Bounds)> {
    let bounds = background.bounds;
    bounds
        .inset(0.0, 0.0)
        .map_err(|error| format!("Native page background bounds: {error}"))?;
    if bounds.left < 0.0
        || bounds.bottom < 0.0
        || bounds.right > layout.size.width_mm()
        || bounds.top > layout.size.height_mm()
    {
        return Err("Native page background image must fit within the physical page".into());
    }
    let image = Image::from_file(&background.image_path).map_err(|error| {
        format!(
            "Cannot load native page background image {}: {error}",
            background.image_path.display()
        )
    })?;
    Ok((image, bounds))
}

fn write_paginated_stamp_sections(
    settings: &PageSettings,
    mode: RenderMode<'_>,
    items: &[StampImageRowItem],
    sections: &[StampRowSection],
    policy: &StampTextPolicy,
    output: &Path,
) -> Result<Vec<StampItemPage>> {
    let stamps: Vec<_> = items.iter().map(|i| i.stamp.clone()).collect();
    let pages = paginate_stamp_sections(
        settings,
        &stamps,
        sections,
        policy,
        settings.capture_generation_date(),
    )?;
    let proof = mode
        .proof
        .cloned()
        .map(|mut proof| {
            proof.page_count = pages.len();
            proof.validate()?;
            Ok::<GeometryProofSettings, String>(proof)
        })
        .transpose()?;
    let prepared = pages
        .iter()
        .map(|page| {
            if let Some(proof) = &proof {
                proof.validate_layout(&page.layout)?;
            }
            let mut images = Vec::new();
            let mut geometries = Vec::new();
            let mut frame_styles = Vec::new();
            let mut frame_backgrounds = Vec::new();
            let mut inner_borders = Vec::new();
            for fragment in &page.items {
                if let Some(geometry) = fragment.geometry {
                    let item = &items[fragment.source_index];
                    let image = StampImageSettings {
                        image_path: item.image_path.clone(),
                        box_bounds: geometry.frame_bounds(),
                        image_scale_pt_per_pixel: item.image_scale_pt_per_pixel,
                        options: item.options,
                        image_padding_horizontal_mm: 0.0,
                        image_padding_vertical_mm: 0.0,
                        text_gap_mm: 0.0,
                        heading: None,
                        caption: None,
                    }
                    .with_stamp_box(&geometry)
                    .prepare(&page.layout)
                    .map_err(|e| {
                        format!("Native PDF stamp image {}: {e}", fragment.source_index + 1)
                    })?;
                    images.push(image);
                    geometries.push(geometry);
                    if let Some((width, colour)) = mode.native_frame {
                        let bounds = geometry.frame_bounds();
                        if let Some(mut inner) =
                            item.frame_style.and_then(|style| style.inner_border)
                        {
                            if let Some(state) =
                                item.frame_style.and_then(|style| style.inner_style)
                            {
                                inner.state = state;
                            }
                            if let Some(paths) = inner.layout(bounds, width).map_err(|error| {
                                format!(
                                    "Native PDF stamp inner border {}: {error}",
                                    fragment.source_index + 1
                                )
                            })? {
                                inner_borders.push(paths);
                            }
                        }
                        if item.frame_style.is_none_or(|style| style.outline) {
                            if let Some(state) = item
                                .frame_style
                                .and_then(|style| style.outer_style)
                                .filter(|state| !state.uses_legacy_rectangle())
                            {
                                inner_borders.push(state.outer_paths(bounds, width, colour)?);
                            } else {
                                frame_styles.push((bounds, width, colour));
                            }
                        }
                        if let Some(background) =
                            item.frame_style.and_then(|style| style.background)
                        {
                            frame_backgrounds.push((bounds, background));
                        }
                    }
                }
            }
            let background_image = mode
                .page_background
                .map(|background| prepare_page_background(background, &page.layout))
                .transpose()?;
            Ok(PreparedPdfPage {
                layout: page.layout.clone(),
                stamps: images,
                background_image,
                geometries,
                text_items: page.items.clone(),
                diagnostic_overlays: mode.diagnostic_overlays,
                additional_text: Vec::new(),
                frame_styles,
                frame_backgrounds,
                inner_borders,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    save_pdf(settings, proof.as_ref(), output, prepared)?;
    Ok(pages)
}

#[derive(Clone, Copy)]
struct RenderMode<'a> {
    proof: Option<&'a GeometryProofSettings>,
    diagnostic_overlays: bool,
    native_frame: Option<(f64, NamedColour)>,
    page_background: Option<&'a NativePageBackgroundImage>,
}

struct PreparedPdfPage {
    layout: PageLayout,
    stamps: Vec<PreparedStampImage>,
    background_image: Option<(Image, super::document::Bounds)>,
    geometries: Vec<StampBoxGeometry>,
    text_items: Vec<StampPageFragment>,
    diagnostic_overlays: bool,
    additional_text: Vec<super::stamp_text::PositionedStampText>,
    frame_styles: Vec<(super::document::Bounds, f64, NamedColour)>,
    frame_backgrounds: Vec<(super::document::Bounds, NamedColour)>,
    inner_borders: Vec<super::stamp_inner_border::InnerBorderPaths>,
}

fn write_proof(
    settings: &PageSettings,
    proof: &GeometryProofSettings,
    output: &Path,
    generation_date: Option<chrono::NaiveDate>,
    stamps: &[StampImageProofSettings],
    geometries: &[StampBoxGeometry],
    text_items: &[PositionedStampItem],
) -> Result<()> {
    proof.validate()?;
    let layouts = (1..=proof.page_count)
        .map(|number| {
            let layout = settings
                .resolve_with_context(number, proof.page_count, generation_date)
                .map_err(|error| format!("Native PDF page {number}: {error}"))?;
            proof.validate_layout(&layout)?;
            Ok(layout)
        })
        .collect::<Result<Vec<_>>>()?;
    let pages = layouts
        .into_iter()
        .map(|layout| {
            let stamps = stamps
                .iter()
                .enumerate()
                .map(|(index, stamp)| {
                    stamp
                        .prepare(&layout)
                        .map_err(|error| format!("Native PDF stamp image {}: {error}", index + 1))
                })
                .collect::<Result<Vec<_>>>()?;
            Ok(PreparedPdfPage {
                layout,
                stamps,
                background_image: None,
                geometries: geometries.to_vec(),
                text_items: text_items
                    .iter()
                    .enumerate()
                    .map(|(index, item)| StampPageFragment {
                        source_index: index,
                        footprint: item.footprint,
                        geometry: Some(item.geometry),
                        heading: item.heading.clone(),
                        caption: item.caption.clone(),
                    })
                    .collect(),
                diagnostic_overlays: true,
                additional_text: Vec::new(),
                frame_styles: Vec::new(),
                frame_backgrounds: Vec::new(),
                inner_borders: Vec::new(),
            })
        })
        .collect::<Result<Vec<_>>>()?;
    save_pdf(settings, Some(proof), output, pages)
}

fn save_pdf(
    settings: &PageSettings,
    proof: Option<&GeometryProofSettings>,
    output: &Path,
    pages: Vec<PreparedPdfPage>,
) -> Result<()> {
    let mut document = Document::new();
    for (
        index,
        PreparedPdfPage {
            layout,
            stamps,
            background_image,
            geometries,
            text_items,
            diagnostic_overlays,
            additional_text,
            frame_styles,
            frame_backgrounds,
            inner_borders,
        },
    ) in pages.into_iter().enumerate()
    {
        let mut page = Page::new(
            mm_to_points(layout.size.width_mm()),
            mm_to_points(layout.size.height_mm()),
        );
        if let Some((image, bounds)) = background_image {
            let name = format!("native_page_background_{index}");
            page.add_image(&name, image);
            page.draw_image(
                &name,
                mm_to_points(bounds.left),
                mm_to_points(bounds.bottom),
                mm_to_points(bounds.right - bounds.left),
                mm_to_points(bounds.top - bounds.bottom),
            )
            .map_err(|error| format!("Cannot draw native page background image: {error}"))?;
        }
        let mut strokes: Vec<_> = settings
            .border()
            .strokes(layout.margin_bounds)?
            .into_iter()
            .map(|stroke| {
                (
                    stroke.bounds,
                    stroke.width_mm,
                    settings.border_line_colour(stroke.line).rgb(),
                )
            })
            .collect();
        if let Some(proof) = proof.filter(|proof| proof.guide_enabled) {
            strokes.push((
                layout.content_bounds,
                proof.guide_width_mm,
                proof.guide_colour.rgb(),
            ));
        }
        for (bounds, width_mm, [red, green, blue]) in strokes {
            page.graphics()
                .set_stroke_color(Color::rgb(red, green, blue))
                .set_line_width(mm_to_points(width_mm))
                .rect(
                    mm_to_points(bounds.left),
                    mm_to_points(bounds.bottom),
                    mm_to_points(bounds.right - bounds.left),
                    mm_to_points(bounds.top - bounds.bottom),
                )
                .stroke();
        }
        if let Some(border) = layout.decorative_border {
            let footprints = border.footprints();
            if footprints.iter().any(|b| {
                b.left < 0.0
                    || b.bottom < 0.0
                    || b.right > layout.size.width_mm()
                    || b.top > layout.size.height_mm()
            }) {
                eprintln!("Warning: Native PDF page {}: ALBUM_PAGES_DECORATIVE_BORDER crosses the physical page and will be clipped; adjust IMAGE_SCALE/IMAGE_CORNER_PAD/IMAGE_EDGE_PAD/BORDER_PAD or native_pdf.decorative_border policy. Page size is unchanged.",index+1);
            }
            if footprints.iter().any(|b| {
                b.left < layout.content_bounds.right
                    && b.right > layout.content_bounds.left
                    && b.bottom < layout.content_bounds.top
                    && b.top > layout.content_bounds.bottom
            }) {
                // TODO(native-pdf-ui): Show page/definition element and offer scale,
                // padding/end-offset or content-spacing/hint corrections. Do not silently move content.
                eprintln!("Warning: Native PDF page {}: ALBUM_PAGES_DECORATIVE_BORDER footprint intersects the content area; transparent image pixels may be clear. Adjust definition scale/corner/edge padding/end offsets or content spacing/height hints. Decorative borders do not reserve content space (AlbumEasy-compatible).",index+1);
            }
            for image in border.corners.iter().chain(&border.edges) {
                page.add_image(image.resource_name, image.image.as_ref().clone());
                let [x, y, width, height] = image.rect_mm;
                page.draw_image(
                    image.resource_name,
                    mm_to_points(x),
                    mm_to_points(y),
                    mm_to_points(width),
                    mm_to_points(height),
                )
                .map_err(|error| format!("Cannot draw native decorative corner: {error}"))?;
            }
            let [red, green, blue] = border.colour.rgb();
            for line in border.lines {
                let [a, b] = line.points;
                page.graphics()
                    .set_stroke_color(Color::rgb(red, green, blue))
                    .set_line_width(mm_to_points(line.width_mm))
                    .move_to(mm_to_points(a[0]), mm_to_points(a[1]))
                    .line_to(mm_to_points(b[0]), mm_to_points(b[1]))
                    .stroke();
            }
        }
        if let Some(marks) = layout.crop_marks {
            if marks.clipped {
                // TODO(native-pdf-ui): Show the physical-page clipping warning
                // and offer padding/arms corrections without resizing silently.
                eprintln!("Warning: Native PDF page {}: ALBUM_PAGES_CROP_MARKS extend partly or wholly outside the physical page and will be clipped; adjust native_pdf.crop_marks.horizontal_padding_mm/vertical_padding_mm/arms_mm/line_width_mm. Page size and content are unchanged (AlbumEasy-compatible).", index + 1);
            }
            let [red, green, blue] = marks.colour.rgb();
            for mark in marks.marks {
                let [a, b, c, d] = mark.points;
                page.graphics()
                    .set_stroke_color(Color::rgb(red, green, blue))
                    .set_line_width(mm_to_points(marks.line_width_mm))
                    .move_to(mm_to_points(a[0]), mm_to_points(a[1]))
                    .line_to(mm_to_points(b[0]), mm_to_points(b[1]))
                    .move_to(mm_to_points(c[0]), mm_to_points(c[1]))
                    .line_to(mm_to_points(d[0]), mm_to_points(d[1]))
                    .stroke();
            }
        }
        for margin in &layout.margin_text {
            let [red, green, blue] = margin.colour.rgb();
            let up = matches!(
                margin.direction,
                super::commands::page::album_pages_margin_txt::MarginDirection::Up
            );
            for line in &margin.lines {
                page.graphics().save_state().transform(
                    0.0,
                    if up { 1.0 } else { -1.0 },
                    if up { -1.0 } else { 1.0 },
                    0.0,
                    mm_to_points(line.x_mm),
                    mm_to_points(line.y_mm),
                );
                let result = page
                    .text()
                    .set_font(margin.font.clone(), margin.size_pt)
                    .set_fill_color(Color::rgb(red, green, blue))
                    .at(0.0, 0.0)
                    .write(&line.text)
                    .map(|_| ())
                    .map_err(|error| format!("Cannot draw native margin text: {error}"));
                page.graphics().restore_state();
                result?;
            }
        }
        if let Some(number) = layout.header_number {
            draw_text_lines(
                &mut page,
                number.font,
                number.size_pt,
                number.colour,
                number.lines,
                "header number",
            )?;
        }
        if let Some(number) = layout.footer_number {
            draw_text_lines(
                &mut page,
                number.font,
                number.size_pt,
                number.colour,
                number.lines,
                "footer number",
            )?;
        }
        if let Some(date) = layout.header_date {
            draw_text_lines(
                &mut page,
                date.font,
                date.size_pt,
                date.colour,
                date.lines,
                "header date",
            )?;
        }
        if let Some(date) = layout.footer_date {
            draw_text_lines(
                &mut page,
                date.font,
                date.size_pt,
                date.colour,
                date.lines,
                "footer date",
            )?;
        }
        if let Some(header) = layout.header {
            draw_text_lines(
                &mut page,
                header.font,
                header.size_pt,
                header.colour,
                header.lines,
                "header",
            )?;
        }
        if let Some(footer) = layout.footer {
            draw_text_lines(
                &mut page,
                footer.font,
                footer.size_pt,
                footer.colour,
                footer.lines,
                "footer",
            )?;
        }
        if let Some(title) = layout.title {
            draw_text_lines(
                &mut page,
                title.font,
                title.size_pt,
                title.colour,
                title.lines,
                "title",
            )?;
        }
        for (bounds, colour) in frame_backgrounds {
            let [red, green, blue] = colour.rgb();
            page.graphics()
                .set_fill_color(Color::rgb(red, green, blue))
                .rect(
                    mm_to_points(bounds.left),
                    mm_to_points(bounds.bottom),
                    mm_to_points(bounds.right - bounds.left),
                    mm_to_points(bounds.top - bounds.bottom),
                )
                .fill();
        }
        for (image_index, stamp) in stamps.iter().enumerate() {
            let name = if stamps.len() == 1 {
                "native_stamp_image".to_string()
            } else {
                format!("native_stamp_image_{image_index}")
            };
            if let Some(image) = &stamp.image {
                page.add_image(&name, image.clone());
                let bounds = stamp.image_bounds;
                page.draw_image(
                    &name,
                    mm_to_points(bounds.left),
                    mm_to_points(bounds.bottom),
                    mm_to_points(bounds.right - bounds.left),
                    mm_to_points(bounds.top - bounds.bottom),
                )
                .map_err(|error| format!("Cannot draw native stamp image: {error}"))?;
            }
            for (text, kind) in [
                (stamp.heading.as_ref(), "stamp heading"),
                (stamp.caption.as_ref(), "stamp caption"),
            ] {
                if let Some(text) = text {
                    draw_text_lines(
                        &mut page,
                        text.font.clone(),
                        text.size_pt,
                        text.colour,
                        text.lines.clone(),
                        kind,
                    )?;
                }
            }
        }
        for item in text_items {
            let bounds = item.footprint;
            if diagnostic_overlays {
                page.graphics()
                    .set_stroke_color(Color::rgb(0.0, 0.6, 0.2))
                    .set_line_width(mm_to_points(0.1))
                    .rect(
                        mm_to_points(bounds.left),
                        mm_to_points(bounds.bottom),
                        mm_to_points(bounds.right - bounds.left),
                        mm_to_points(bounds.top - bounds.bottom),
                    )
                    .stroke();
            }
            for text in [item.heading.as_ref(), item.caption.as_ref()]
                .into_iter()
                .flatten()
            {
                draw_text_lines(
                    &mut page,
                    text.measured.font.clone(),
                    text.measured.size_pt,
                    text.colour,
                    text.lines.clone(),
                    "stamp text footprint",
                )?;
            }
        }
        for text in additional_text {
            draw_text_lines(
                &mut page,
                text.measured.font.clone(),
                text.measured.size_pt,
                text.colour,
                text.lines,
                "imported AlbumEasy text",
            )?;
        }
        for (bounds, width_mm, colour) in frame_styles {
            let [red, green, blue] = colour.rgb();
            page.graphics()
                .set_stroke_color(Color::rgb(red, green, blue))
                .set_line_width(mm_to_points(width_mm))
                .rect(
                    mm_to_points(bounds.left),
                    mm_to_points(bounds.bottom),
                    mm_to_points(bounds.right - bounds.left),
                    mm_to_points(bounds.top - bounds.bottom),
                )
                .stroke();
        }
        for inner in inner_borders {
            let [red, green, blue] = inner.colour.rgb();
            let graphics = page.graphics();
            graphics
                .save_state()
                .set_stroke_color(Color::rgb(red, green, blue))
                .set_line_width(mm_to_points(inner.thickness_mm))
                .set_line_dash_pattern(oxidize_pdf::graphics::LineDashPattern::new(
                    inner.style.dash_points(),
                    0.0,
                ));
            for path in inner.stroke_paths() {
                if let Some(&(x, y)) = path.first() {
                    graphics.move_to(mm_to_points(x), mm_to_points(y));
                    for &(x, y) in &path[1..] {
                        graphics.line_to(mm_to_points(x), mm_to_points(y));
                    }
                    graphics.stroke();
                }
            }
            graphics.restore_state();
        }
        for geometry in geometries {
            if diagnostic_overlays {
                for (bounds, colour) in [
                    (geometry.frame_bounds(), Color::rgb(0.0, 0.0, 0.0)),
                    (geometry.physical_bounds(), Color::rgb(0.0, 0.3, 0.9)),
                ] {
                    page.graphics()
                        .set_stroke_color(colour)
                        .set_line_width(mm_to_points(0.1))
                        .rect(
                            mm_to_points(bounds.left),
                            mm_to_points(bounds.bottom),
                            mm_to_points(bounds.right - bounds.left),
                            mm_to_points(bounds.top - bounds.bottom),
                        )
                        .stroke();
                }
            }
        }

        document.add_page(page);
    }
    document
        .save(output)
        .map_err(|error| format!("Cannot write native PDF {}: {error}", output.display()))
}

fn draw_text_lines(
    page: &mut Page,
    font: oxidize_pdf::text::Font,
    size_pt: f64,
    colour: NamedColour,
    lines: Vec<PositionedTextLine>,
    kind: &str,
) -> Result<()> {
    let [red, green, blue] = colour.rgb();
    for line in lines {
        page.text()
            .set_font(font.clone(), size_pt)
            .set_fill_color(Color::rgb(red, green, blue))
            .at(mm_to_points(line.left_mm), mm_to_points(line.baseline_mm))
            .write(&line.text)
            .map_err(|error| format!("Cannot draw native page {kind}: {error}"))?;
    }

    Ok(())
}

pub(crate) struct ImportedPdfPage {
    pub layout: PageLayout,
    pub images: Vec<StampImageSettings>,
    pub text: Vec<super::stamp_text::PositionedStampText>,
    pub frames: Vec<(super::document::Bounds, f64, NamedColour)>,
    pub backgrounds: Vec<(super::document::Bounds, NamedColour)>,
    pub inner_borders: Vec<super::stamp_inner_border::InnerBorderPaths>,
}

pub(crate) fn write_imported_album_pdf(
    settings: &PageSettings,
    pages: Vec<ImportedPdfPage>,
    output: &Path,
) -> Result<()> {
    let prepared = pages
        .into_iter()
        .map(|page| {
            let stamps = page
                .images
                .iter()
                .map(|image| {
                    image.prepare(&page.layout).map_err(|error| {
                        format!("Imported image {}: {error}", image.image_path.display())
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            Ok(PreparedPdfPage {
                layout: page.layout,
                stamps,
                background_image: None,
                geometries: Vec::new(),
                text_items: Vec::new(),
                diagnostic_overlays: false,
                additional_text: page.text,
                frame_styles: page.frames,
                frame_backgrounds: page.backgrounds,
                inner_borders: page.inner_borders,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    save_pdf(settings, None, output, prepared)
}
