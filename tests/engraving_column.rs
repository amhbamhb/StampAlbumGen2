use rusqlite::{params, Connection};
use std::env;
use std::fs;
use tempfile::tempdir;

/// Test that the generator emits an 'Engraving:' header line when the
/// `engraving` column is populated in the DB and include_engraving is enabled.
#[test]
fn test_engraving_column_emitted() {
    let tmp = tempdir().expect("tempdir");
    let orig_cwd = env::current_dir().expect("cwd");
    env::set_current_dir(tmp.path()).expect("chdir to tmp");

    fs::create_dir_all(tmp.path().join("Albums/TestAlbum")).expect("create album dir");

    let conn = Connection::open_in_memory().expect("open in-memory DB");
    conn.execute_batch(
        "CREATE TABLE stamps (
            id INTEGER PRIMARY KEY,
            year INTEGER,
            month TEXT,
            month_num INTEGER,
            day TEXT,
            sg_number TEXT,
            type TEXT,
            denomination TEXT,
            color TEXT,
            description TEXT,
            stamps_issued TEXT,
            image_path TEXT,
            remote_image_url TEXT,
            original_image_url TEXT,
            watermark TEXT,
            design TEXT,
            engraving TEXT,
            perforation TEXT,
            series TEXT,
            currency TEXT,
            context TEXT,
            mint_condition TEXT,
            unused TEXT,
            used TEXT,
            letter_fdc TEXT,
            image_width_px INTEGER,
            image_height_px INTEGER,
            issue_location TEXT,
            quantity_printed TEXT
        );",
    )
    .expect("create table");

    conn.execute(
        "INSERT INTO stamps (year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, series, currency, context, mint_condition, unused, used, letter_fdc)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24)",
        params![
            1974i32,
            "June",
            6i32,
            "15",
            "1",
            "Definitive",
            "1d",
            "Blue",
            "Engraved stamp",
            "",
            "no_image.png",
            "",
            "",
            "",
            "Design text",
            "Recess",
            "12",
            "TestSeries",
            "CHF",
            "",
            "",
            "",
            "",
            "",
        ],
    ).expect("insert stamp");

    let db_path = tmp.path().join("Albums/TestAlbum/TestAlbum.db");
    stamp_album_gen2::file_utils::set_db_path_from_cli(db_path.to_str().unwrap());

    stamp_album_gen2::album_globals::generate_album_output_conn(
        &conn,
        "TestAlbum",
        None::<fn(usize)>,
        Some(1974),
        Some(1974),
    );

    let output_file = tmp.path().join("Albums/TestAlbum/TestAlbum-1974-1974.txt");
    assert!(
        output_file.exists(),
        "Expected output file to exist: {}",
        output_file.display()
    );

    let contents = fs::read_to_string(&output_file).expect("read output");

    assert!(
        contents.contains("Engraving: Recess"),
        "Expected Engraving line in header; got:\n{}",
        contents
    );

    env::set_current_dir(orig_cwd).expect("restore cwd");
}
