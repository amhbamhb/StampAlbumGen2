use lazy_static::lazy_static;
use std::fs;
use std::io::Read;
use tempfile::tempdir;
use tokio::sync::Mutex;

// Integration test: create a tiny config and DB so a row will overflow the usable
// height and force the generator to emit a new page. Verify the emitted AlbumEasy
// text contains the expected PAGE_START followed by ROW_START_FS for the forced
// page break (Page 2).
#[tokio::test]
async fn overflowing_row_moves_to_next_page_and_emits_row_start() {
    lazy_static! {
        static ref TEST_CWD_MUTEX: Mutex<()> = Mutex::new(());
    }
    let _lock = TEST_CWD_MUTEX.lock().await;
    // Prepare temporary workspace
    let td = tempdir().expect("tempdir");
    let td_path = td.path();

    // Write a small config.toml in the tempdir that makes usable height tiny
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
"#;
    fs::write(td_path.join("config.toml"), cfg).expect("write config");

    // Create a temporary DB and insert a single series with multiple stamps
    let db_path = td_path.join("test.db");
    let db_path_str = db_path.to_string_lossy().to_string();
    // init_db exists in crate::db_utils
    let conn = stamp_album_gen2::db_utils::init_db(&db_path_str).expect("init db");

    // Insert a single series and two stamps that will form a row
    let series_name = "OverflowSeries";
    // Provide values for all columns that get_stamps_by_series selects so
    // rusqlite maps NULL -> String conversion does not fail.
    let insert_sql =
        "INSERT INTO stamps (country, series, year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, currency, context, mint_condition, unused, used, letter_fdc, uniq_key) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26)";
    let base_params = rusqlite::params![
        "TestCountry", // country
        series_name,   // series
        1900i32,       // year
        "",            // month
        0i32,          // month_num
        "",            // day
        "1",           // sg_number
        "",            // type
        "",            // denomination
        "Black",       // color
        "desc",        // description
        "",            // stamps_issued
        "",            // image_path
        "",            // remote_image_url
        "",            // original_image_url
        "",            // watermark
        "",            // design
        "",            // engraving
        "",            // perforation
        "",            // currency
        "",            // context
        "",            // mint_condition
        "",            // unused
        "",            // used
        "",            // letter_fdc
        "testcountry|OverflowSeries|1"
    ];
    conn.execute(insert_sql, base_params)
        .expect("insert stamp1");
    // second stamp: same params except sg_number
    let insert_sql2 =
        "INSERT INTO stamps (country, series, year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, currency, context, mint_condition, unused, used, letter_fdc, uniq_key) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26)";
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
        "",
        "testcountry|OverflowSeries|2"
    ];
    conn.execute(insert_sql2, params2).expect("insert stamp2");

    // Change current_dir to tempdir so load_config finds our config.toml
    let orig_cwd = std::env::current_dir().expect("cwd");
    std::env::set_current_dir(td_path).expect("chdir");

    // Run generation: this will write Albums/TestAlbum/TestAlbum.txt inside td
    stamp_album_gen2::album_globals::generate_album_output(
        &db_path_str,
        "TestAlbum",
        None::<fn(usize)>,
        None,
        None,
    )
    .await;

    // Restore cwd
    std::env::set_current_dir(orig_cwd).expect("restore cwd");

    // Read the generated file (filename may include inferred years) and verify it contains a forced page 2 header with PAGE_START followed by ROW_START_FS
    let album_dir = td_path.join("Albums").join("TestAlbum");
    let mut found_content = None;
    if let Ok(entries) = std::fs::read_dir(&album_dir) {
        for ent in entries.filter_map(|e| e.ok()) {
            let fname = ent.file_name().to_string_lossy().to_string();
            if fname.ends_with(".txt") {
                let mut s = String::new();
                if fs::File::open(ent.path())
                    .and_then(|mut f| f.read_to_string(&mut s))
                    .is_ok()
                {
                    found_content = Some(s);
                    break;
                }
            }
        }
    }
    let s = found_content.expect("no generated .txt file found in album dir");

    // Look for the Page 2 marker and ensure the sequence exists. The page
    // header may include a suffix like " - Stamps", so locate the "### Page 2"
    // marker and then assert the PAGE_START + ROW_START_FS sequence appears
    // after that location.
    let page2_pos = s
        .find("### Page 2")
        .expect("expected a Page 2 marker in generated output");
    let after_page2 = &s[page2_pos..];
    assert!(
        after_page2.contains("PAGE_START\nROW_START_FS"),
        "expected forced page 2 with PAGE_START + ROW_START_FS after Page 2 marker, got:\n{}",
        after_page2
    );
}
