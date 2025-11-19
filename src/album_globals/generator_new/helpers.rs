use crate::config::Config;
use std::io::Write;

// Local copy of helper functions so the refactor can be worked on inside
// `generator_new` without changing behavior. These mirror the existing
// `crate::album_globals::helpers` implementations and will be unified later.
pub fn emit_row_start_es(
    output: &mut dyn Write,
    config_opt: Option<&Config>,
) -> std::io::Result<()> {
    let (rs_font, rs_size, rs_spacing, rs_vspace) = if let Some(cfg) = config_opt {
        (
            cfg.row_start_es_font
                .clone()
                .unwrap_or_else(|| "HN".to_string()),
            cfg.row_start_es_size.unwrap_or(6.0),
            cfg.row_start_es_spacing.unwrap_or(0.1),
            cfg.row_start_es_vspace.unwrap_or(5.0),
        )
    } else {
        ("HN".to_string(), 6.0_f32, 0.1_f32, 5.0_f32)
    };
    writeln!(
        output,
        "ROW_START_ES ({} {} {:.1} {:.1})",
        rs_font, rs_size as i32, rs_spacing, rs_vspace
    )
}

pub fn build_page_text_template(cfg: &Config, kind: &str) -> String {
    match kind {
        "centre" => {
            if let Some(t) = cfg.page_text_centre_format.clone() {
                return t;
            }
            let font = cfg
                .page_text_centre_font
                .clone()
                .unwrap_or_else(|| "TN".to_string());
            let size = cfg.page_text_centre_size.unwrap_or(8.0) as i32;
            let vspace = cfg.page_text_centre_vspace.unwrap_or(1.0_f32);
            let head = format!("PAGE_TEXT_CENTRE ({} {} \"", font, size);
            let tail = format!("\" {:.1})", vspace);
            head + "{}" + &tail
        }
        "right" => {
            if let Some(t) = cfg.page_text_right_format.clone() {
                return t;
            }
            let font = cfg
                .page_text_right_font
                .clone()
                .unwrap_or_else(|| "HN".to_string());
            let size = cfg.page_text_right_size.unwrap_or(10.0) as i32;
            let vspace = cfg.page_text_right_vspace.unwrap_or(1.0_f32);
            let head = format!("PAGE_TEXT_RIGHT ({} {} \"", font, size);
            let tail = format!("\" {:.1})", vspace);
            head + "{}" + &tail
        }
        _ => {
            if let Some(t) = cfg.page_text_format.clone() {
                return t;
            }
            let font = cfg
                .page_text_font
                .clone()
                .unwrap_or_else(|| "HN".to_string());
            let size = cfg.page_text_size.unwrap_or(10.0) as i32;
            let vspace = cfg.page_text_vspace.unwrap_or(1.0_f32);
            let head = format!("PAGE_TEXT ({} {} \"", font, size);
            let tail = format!("\" {:.1})", vspace);
            head + "{}" + &tail
        }
    }
}
