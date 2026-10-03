use std::path::Path;

const CENTERING_MARKER: &str = "# PAGE_CENTERING";

/// Center stamp pages using the same measured heights used for pagination.
/// Explicit header and row spacing is preserved; only previously inserted
/// centering space is replaced.
pub fn center_tmp_pages(
    tmp_path: &Path,
    config: &crate::config::Config,
    usable_height: f64,
) -> std::io::Result<()> {
    let max_vspace = config.max_page_vspace_mm.unwrap_or(10.0).max(0.0) as f64;
    let content = std::fs::read_to_string(tmp_path)?;
    let mut output = Vec::new();
    let mut page = Vec::new();
    for line in content.lines() {
        if line.starts_with("### Page ") && !page.is_empty() {
            center_page(&page, config, usable_height, max_vspace, &mut output);
            page.clear();
        }
        page.push(line);
    }
    center_page(&page, config, usable_height, max_vspace, &mut output);
    std::fs::write(tmp_path, output.join("\n") + "\n")
}

fn center_page(
    page: &[&str],
    config: &crate::config::Config,
    usable_height: f64,
    max_vspace: f64,
    output: &mut Vec<String>,
) {
    let mut cleaned = Vec::new();
    let mut skip_centering_space = false;
    for line in page {
        if line.trim() == CENTERING_MARKER {
            skip_centering_space = true;
            continue;
        }
        if skip_centering_space {
            skip_centering_space = false;
            if line.trim().starts_with("PAGE_VSPACE") {
                continue;
            }
        }
        cleaned.push(*line);
    }
    let rows = cleaned
        .iter()
        .filter(|line| line.starts_with("# ROW_HEIGHT "))
        .count();
    let is_cover = cleaned
        .iter()
        .any(|line| line.starts_with("###") && line.contains("Flag"));
    let used = compute_page_used_height(&cleaned, config);
    let extra = (usable_height - used).max(0.0);
    // Never round up into space that pagination reserved for the footer.
    let top_gap = ((extra / 2.0).min(max_vspace) * 10.0).floor() / 10.0;
    for line in cleaned {
        output.push(line.to_string());
        if line.trim() == "PAGE_START" && rows > 0 && !is_cover && top_gap > 0.0 {
            output.push(CENTERING_MARKER.to_string());
            output.push(super::layout::format_page_vspace_mm(top_gap));
        }
    }
}

/// Sum measured header/row content and explicit page spacing.
/// The generator's row measurements already include heading text, row-start
/// spacing, image adjustments, and the configured packing safety allowance.
pub fn compute_page_used_height(page: &[&str], _config: &crate::config::Config) -> f64 {
    page.iter()
        .filter_map(|line| {
            let line = line.trim();
            if let Some(value) = line
                .strip_prefix("# ROW_HEIGHT mm=")
                .or_else(|| line.strip_prefix("# HEADER_HEIGHT mm="))
            {
                return value.parse::<f64>().ok();
            }
            line.strip_prefix("PAGE_VSPACE (")
                .and_then(|value| value.strip_suffix(')'))
                .and_then(|value| value.parse::<f64>().ok())
        })
        .sum::<f64>()
        .max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centering_preserves_explicit_spacing_and_is_idempotent() {
        let dir = tempfile::tempdir().expect("temporary output directory");
        let path = dir.path().join("album.txt");
        let original = "### Page 1 - Stamps ###\nPAGE_START\nROW_START_ES (HN 6 0.1 6)\n\
            PAGE_VSPACE (1)\n# HEADER_HEIGHT mm=12.0\nPAGE_TEXT_CENTRE (HN 8 \"Series\" 1)\n\
            PAGE_VSPACE (2)\n# ROW_HEIGHT mm=95.0\nROW_START_ES (HN 6 0.1 6)\n\
            STAMP_ADD (24 28 \"test\" \"\" \"\" \"\")\nPAGE_VSPACE (4)\n\
            # ROW_HEIGHT mm=90.0\nROW_START_ES (HN 6 0.1 6)\n";
        std::fs::write(&path, original).expect("write output");
        let config = crate::config::Config::with_defaults();
        assert_eq!(
            compute_page_used_height(&original.lines().collect::<Vec<_>>(), &config),
            204.0
        );
        center_tmp_pages(&path, &config, 220.0).expect("center pages");
        let once = std::fs::read_to_string(&path).expect("read centered output");
        assert!(once.contains("# PAGE_CENTERING\nPAGE_VSPACE (8)"));
        for spacing in ["PAGE_VSPACE (1)", "PAGE_VSPACE (2)", "PAGE_VSPACE (4)"] {
            assert!(once.contains(spacing));
        }
        center_tmp_pages(&path, &config, 220.0).expect("repeat centering");
        assert_eq!(std::fs::read_to_string(&path).expect("read output"), once);
    }

    #[test]
    fn full_pages_covers_and_disabled_centering_get_no_extra_space() {
        let config = crate::config::Config::with_defaults();
        let mut output = Vec::new();
        let full_page = [
            "### Page 1 - Stamps ###",
            "PAGE_START",
            "# ROW_HEIGHT mm=250",
        ];
        center_page(&full_page, &config, 240.0, 10.0, &mut output);
        assert!(!output.iter().any(|line| line == CENTERING_MARKER));
        output.clear();
        center_page(&full_page, &config, 300.0, 0.0, &mut output);
        assert!(!output.iter().any(|line| line == CENTERING_MARKER));
        output.clear();
        let cover = ["### Spain Flag ###", "PAGE_START", "PAGE_VSPACE (20)"];
        center_page(&cover, &config, 240.0, 10.0, &mut output);
        assert_eq!(output, cover);
    }
}
