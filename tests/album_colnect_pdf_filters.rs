use stamp_album_gen2::album_globals::{
    generate_album_output_conn_with_filters, generate_album_output_conn_with_pdf_override,
    selection::ColnectPdfFilters,
};
use stamp_album_gen2::{config::Config, db_utils::init_db};
use std::{fs, path::PathBuf};

struct RestoreWorkingDirectory(PathBuf);

impl Drop for RestoreWorkingDirectory {
    fn drop(&mut self) {
        std::env::set_current_dir(&self.0).expect("restore test working directory");
    }
}

#[tokio::test]
#[serial_test::serial]
async fn filtered_output_contains_only_selected_stamps_and_empty_selection_preserves_output() {
    verify_filtered_output(false).await;
}

#[tokio::test]
#[serial_test::serial]
#[ignore = "Requires a locally configured AlbumEasy CLI"]
async fn filtered_output_generates_a_real_pdf() {
    verify_filtered_output(true).await;
}

async fn verify_filtered_output(generate_pdf: bool) {
    let original = std::env::current_dir().expect("working directory");
    let cli_path = if generate_pdf {
        stamp_album_gen2::config::load_effective_config("config.toml")
            .effective_albumeasy_cli_path()
            .map(str::to_owned)
    } else {
        None
    };
    let temp = tempfile::tempdir().expect("isolated generation directory");
    let _restore = RestoreWorkingDirectory(original);
    std::env::set_current_dir(temp.path()).expect("test working directory");
    let mut config = Config::with_defaults();
    config.include_perforation = Some(true);
    config.albumeasy_cli_path = cli_path;
    fs::write("config.toml", toml::to_string(&config).expect("config")).expect("test config");
    fs::create_dir_all("Albums/Testland/images").expect("test images");
    image::RgbaImage::from_pixel(20, 20, image::Rgba([255, 255, 255, 255]))
        .save("Albums/Testland/images/stamp.png")
        .expect("fixture image");
    let conn = init_db(":memory:").expect("test DB");
    for (number, year, emission, format, series) in [
        ("1001", 1976, "Official", "Stamp", "Included Series"),
        ("1002", 1976, "Definitive", "Stamp", "Included Series"),
        (
            "1003",
            1976,
            "Official",
            "Miniature Sheet",
            "Excluded Series",
        ),
        ("1004", 1975, "Official", "Stamp", "Earlier Series"),
        ("1005", 1976, "", "", "Unknown Series"),
    ] {
        conn.execute(
            "INSERT INTO stamps (country, series, year, sg_number, type, denomination,
             color, description, image_path, image_width_px, image_height_px,
             perforation, colnect_emission, colnect_format, uniq_key)
             VALUES ('Testland', ?1, ?2, ?3, 'Postage', '1p', 'Red', 'Test stamp',
             'images/stamp.png', 20, 20, '11', ?4, ?5, ?3)",
            rusqlite::params![series, year, number, emission, format],
        )
        .expect("fixture stamp");
    }
    let filters = ColnectPdfFilters {
        emission: vec!["Official".into(), "Definitive".into()],
        format: vec!["Stamp".into()],
    };
    let artifacts = generate_album_output_conn_with_filters(
        &conn,
        "Testland",
        Some(1976),
        Some(1976),
        Some(generate_pdf),
        &filters,
    )
    .await
    .expect("filtered text");
    let previous_pdf = if generate_pdf {
        let path = artifacts.generated_pdf_path.as_ref().unwrap_or_else(|| {
            panic!(
                "AlbumEasy did not produce a PDF: {:?}",
                artifacts.pdf_failure_reason
            )
        });
        let bytes = fs::read(path).expect("generated PDF");
        assert!(bytes.starts_with(b"%PDF-"));
        Some(bytes)
    } else {
        None
    };
    let selected = fs::read_to_string(&artifacts.output_txt_path).expect("selected output");
    assert!(selected.contains(" : 1001 \\n Perforation"));
    assert!(selected.contains(" : 1002 \\n Perforation"));
    for excluded in ["1003", "1004", "1005"] {
        assert!(!selected.contains(&format!(" : {excluded} \\n Perforation")));
    }
    assert!(!selected.contains("Excluded Series"));
    assert!(!selected.contains("Earlier Series"));
    assert!(!selected.contains("Unknown Series"));
    let empty = ColnectPdfFilters {
        emission: vec!["Not present".into()],
        format: vec![],
    };
    let error = generate_album_output_conn_with_filters(
        &conn,
        "Testland",
        Some(1976),
        Some(1976),
        Some(true),
        &empty,
    )
    .await
    .expect_err("empty selection");
    assert!(error.contains("No stamps match"));
    assert_eq!(
        fs::read_to_string(&artifacts.output_txt_path).expect("previous output"),
        selected
    );
    let pdf_path = artifacts.output_txt_path.with_extension("pdf");
    if let Some(previous_pdf) = previous_pdf {
        assert_eq!(fs::read(pdf_path).expect("previous PDF"), previous_pdf);
    } else {
        assert!(!pdf_path.exists());
    }

    let baseline = generate_album_output_conn_with_pdf_override(
        &conn,
        "Testland",
        None::<fn(usize)>,
        Some(1976),
        Some(1976),
        Some(false),
    )
    .await;
    let baseline_text = fs::read_to_string(baseline.output_txt_path).expect("baseline");
    let all = generate_album_output_conn_with_filters(
        &conn,
        "Testland",
        Some(1976),
        Some(1976),
        Some(false),
        &ColnectPdfFilters::default(),
    )
    .await
    .expect("All");
    let without_timestamp = |text: &str| {
        text.lines()
            .filter(|line| !line.starts_with("# Generated:"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_eq!(
        without_timestamp(&fs::read_to_string(all.output_txt_path).expect("All output")),
        without_timestamp(&baseline_text)
    );
}
