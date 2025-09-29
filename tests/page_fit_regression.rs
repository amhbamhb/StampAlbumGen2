use std::fs;
use std::io::Read;
use tempfile::tempdir;

// Regression test: ensure a header+row that would otherwise overlap the footer
// is moved to the next page. This mirrors the integration test used earlier but
// provides a lightweight guard to prevent regressions to the `page_fit` logic.
#[test]
fn page_fit_regression_for_conservative_buffer() {
    // Create tempdir and small config to make usable height small
    let td = tempdir().expect("tempdir");
    let td_path = td.path();
    let cfg = r#"
album_page_height_mm = 60.0
page_top_margin_mm = 5.0
page_bottom_margin_mm = 5.0
page_border_outer = 0.5
page_border_inner1 = 0.1
page_border_inner2 = 0.0
page_border_spacing = 3.0
usable_height_padding_mm = 20.0
header_vspace_before_mm = 8.0
header_vspace_after_mm = 12.5
text_line_height_mm = 4.0
row_font_spacing = 6.0
album_stamp_img_new_setting_h_mm = 3.0
row_start_es_enabled = true
allow_single_large_stamp_per_page = true
single_stamp_threshold_pct = 0.5
page_fit_conservative_buffer_mm = 11.0
"#;
    fs::write(td_path.join("config.toml"), cfg).expect("write config");

    // Create a temporary DB and insert a single series with two stamps (forms a row)
    let db_path = td_path.join("test.db");
    let db_path_str = db_path.to_string_lossy().to_string();
    let conn = stamp_album_gen2::db_utils::init_db(&db_path_str).expect("init db");
    let series_name = "RegressionSeries";
    let insert_sql = "INSERT INTO stamps (country, series, year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, currency, context, mint_condition, unused, used, letter_fdc) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25)";
    let params = rusqlite::params![
        "TestCountry",
        series_name,
        1900i32,
        "",
        0i32,
        "",
        "1",
        "",
        "",
        "Black",
        "desc",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        ""
    ];
    conn.execute(insert_sql, params).expect("insert stamp1");
    let params2 = rusqlite::params![
        "TestCountry",
        series_name,
        1900i32,
        "",
        0i32,
        "",
        "2",
        "",
        "",
        "Black",
        "desc",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        ""
    ];
    conn.execute(insert_sql, params2).expect("insert stamp2");

    // Run generation from tempdir so load_config picks our config
    let orig = std::env::current_dir().expect("cwd");
    std::env::set_current_dir(&td_path).expect("chdir");
    stamp_album_gen2::album_globals::generate_album_output(&db_path_str, "RegTestAlbum", None::<fn(usize)>, None, None);
    std::env::set_current_dir(orig).expect("restore cwd");

    // Verify output contains forced page 2 sequence
    let out_file = td_path.join("Albums").join("RegTestAlbum").join("RegTestAlbum.txt");
    let mut s = String::new();
    fs::File::open(&out_file).expect("open output").read_to_string(&mut s).expect("read output");
    let page2_seq = "### Page 2 ###\nPAGE_START\nROW_START_ES";
    assert!(s.contains(page2_seq), "expected forced page 2 with PAGE_START + ROW_START_ES, got:\n{}", s);
}
