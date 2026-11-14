use super::{LayoutCommand, LayoutPage, Result, parser::number};

fn setting<'a>(globals: &'a [LayoutCommand], name: &str) -> Result<&'a LayoutCommand> {
    globals
        .iter()
        .rev()
        .find(|c| c.name == name)
        .ok_or_else(|| format!("Missing setting {name}"))
}

pub(super) fn reflow(globals: &[LayoutCommand], page: LayoutPage) -> Result<LayoutPage> {
    let size = setting(globals, "ALBUM_PAGES_SIZE")?;
    let margins = if page.index.is_multiple_of(2)
        && globals.iter().any(|c| c.name == "ALBUM_PAGES_MARGINSE")
    {
        setting(globals, "ALBUM_PAGES_MARGINSE")?
    } else {
        setting(globals, "ALBUM_PAGES_MARGINS")?
    };
    let spacing = setting(globals, "ALBUM_PAGES_SPACING")?;
    let width =
        number(size, 0)? - number(margins, 0)? - number(margins, 1)? - 2.0 * number(spacing, 0)?;
    let height = number(size, 1)? - number(margins, 2)? - number(margins, 3)?;
    if width <= 0.0 || height <= 0.0 {
        return Err("Overflow: margins leave no usable page area".into());
    }
    let adjust = globals
        .iter()
        .rev()
        .find(|c| c.name == "STAMP_BOXES_SIZE_ADJUST")
        .map(|c| number(c, 0))
        .transpose()?
        .unwrap_or(0.0);
    let vertical = number(spacing, 1)?;
    let mut used_height = globals
        .iter()
        .rev()
        .find(|c| c.name == "ALBUM_PAGES_TITLE")
        .map(|c| number(c, 1).map(|font| font * 25.4 / 72.0 * 1.2 + vertical))
        .transpose()?
        .unwrap_or(0.0);
    let mut result = LayoutPage {
        index: page.index,
        commands: vec![],
    };
    let mut row: Option<LayoutCommand> = None;
    let mut row_width = 0.0;
    let mut row_height: f64 = 0.0;
    let mut row_count = 0;
    for command in page.commands {
        if command.name != "STAMP_ADD_IMG" {
            if row_count > 0 {
                used_height += row_height + vertical;
            }
            row_count = 0;
            row_width = 0.0;
            row_height = 0.0;
            if command.name == "ROW_START_FS" {
                row = Some(command.clone());
            } else if command.name == "PAGE_TEXT_CENTRE" {
                let font_height = number(&command, 1)? * 25.4 / 72.0;
                let text = &command.args[2].value;
                // Conservative built-in-font bound. Renderer validation remains mandatory.
                let lines: usize = text
                    .split("\\n")
                    .map(|line| {
                        ((line.chars().count() as f64 * font_height / width).ceil() as usize).max(1)
                    })
                    .sum();
                used_height += lines as f64 * font_height * 1.2
                    + if command.args.len() == 4 {
                        number(&command, 3)?
                    } else {
                        vertical
                    };
                row = None;
            }
            result.commands.push(command);
            continue;
        }
        let settings = row.as_ref().ok_or("Stamp placement requires a row")?;
        let stamp_width = number(&command, 0)? + adjust;
        let stamp_height = number(&command, 1)? + adjust;
        if stamp_width <= 0.0 || stamp_height <= 0.0 || stamp_width > width {
            return Err(format!(
                "Overflow on page {}: stamp at line {} does not fit the usable width ({width:.2}mm)",
                page.index, command.source_line
            ));
        }
        let gap = if row_count == 0 {
            0.0
        } else {
            number(settings, 3)?
        };
        if row_width + gap + stamp_width > width {
            used_height += row_height + vertical;
            result.commands.push(settings.clone());
            row_width = 0.0;
            row_height = 0.0;
            row_count = 0;
        }
        row_width += stamp_width
            + if row_count == 0 {
                0.0
            } else {
                number(settings, 3)?
            };
        let caption_lines = command.args[3..6]
            .iter()
            .map(|a| {
                if a.value.is_empty() {
                    0
                } else {
                    a.value.split("\\n").count()
                }
            })
            .max()
            .unwrap_or(0);
        row_height = row_height
            .max(stamp_height + caption_lines as f64 * number(settings, 1)? * 25.4 / 72.0 * 1.2);
        row_count += 1;
        result.commands.push(command);
    }
    if row_count > 0 {
        used_height += row_height + vertical;
    }
    if used_height > height {
        return Err(format!(
            "Overflow on page {}: conservative required height {used_height:.2}mm exceeds {height:.2}mm; no new page was created",
            page.index
        ));
    }
    Ok(result)
}
