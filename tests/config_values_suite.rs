use stamp_album_gen2::album_globals::generate_album_output_conn;
use stamp_album_gen2::db_utils;
use stamp_album_gen2::file_utils;
use std::env;
use std::fs;
use tempfile::tempdir;

fn write_config(dir: &std::path::Path, contents: &str) {
    let p = dir.join("config.toml");
    std::fs::write(&p, contents).expect("write config");
}

fn make_test_db_multi(db_path: &str) {
    let conn = db_utils::init_db(db_path).expect("init db");
    // Insert stamps with fake image paths to avoid placeholder generation hang in test environment
    let cases = vec![
        ("Switzerland", "Series S", 1974, "1", "fake_s1.png"),
        ("Switzerland", "Series S", 1975, "2", "fake_s2.png"),
        ("Canada", "Series C", 1974, "1", "fake_c1.png"),
        ("Canada", "Series C", 1975, "2", "fake_c2.png"),
        ("TestAlbum", "Test Series", 1974, "1", "fake_t1.png"),
        ("TestAlbum", "Test Series", 1975, "2", "fake_t2.png"),
    ];
    for (country, series, year, sg, img) in cases {
        // Use insert_stamp with fake image paths to prevent placeholder generation
        let result = db_utils::insert_stamp(
            &conn,
            country,
            series,
            year,
            "1",
            "1",
            "1",
            sg,
            "stamp",
            "1p",
            "red",
            "description",
            "100",
            "mint",
            "1",
            "1",
            "fdc",
            "£",
            img,
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        );
        if result.is_err() {
            // Fallback to placeholder if insert_stamp fails
            let _ = db_utils::insert_stamp_placeholder(&conn, country, series, year, sg);
        }
    }
}

#[tokio::test]
async fn config_values_suite_iterates_and_changes_output() {
    // This suite iterates a set of config keys and verifies that toggling or
    // changing values produces a different AlbumEasy TXT output (quick sanity)
    let td = tempdir().expect("tempdir");
    let orig = env::current_dir().expect("cwd");
    env::set_current_dir(td.path()).expect("chdir");

    // Initialize logging to capture info/debug from generator during the test
    let _ = env_logger::builder().is_test(true).try_init();
    println!("[TEST] logging initialized");

    let db_path = td.path().join("Multi.db");
    let db_str = db_path.to_string_lossy().to_string();
    make_test_db_multi(&db_str);
    file_utils::set_overwrite_album_from_cli(true);

    // Define test cases: (name, album_name, cfg1, cfg2) - use real country names
    let cases = vec![
        (
            "stamp_heading_size",
            "Switzerland",
            "stamp_heading_size = 10\n",
            "stamp_heading_size = 20\n",
        ),
        (
            "stamp_heading_label",
            "Canada",
            "stamp_heading_label = \"A\"\n",
            "stamp_heading_label = \"B\"\n",
        ),
        (
            "image_spacing_mm",
            "Switzerland",
            "image_spacing_mm = 1.0\n",
            "image_spacing_mm = 5.0\n",
        ),
        (
            "include_date",
            "Canada",
            "include_date = true\n",
            "include_date = false\n",
        ),
    ];

    for (name, album, cfg1, cfg2) in cases {
        // First config
        println!(
            "[CASE START] {} album={} cfg1={}",
            name,
            album,
            cfg1.replace('\n', "\\n")
        );
        write_config(td.path(), cfg1);
        let conn = db_utils::init_db(&db_str).expect("open db");
        println!("[GENERATE] before generate album={} (cfg1)", album);

        // Print DB series and per-album stamp counts for debugging
        if let Ok(all_series) = db_utils::get_all_series(&conn, None) {
            println!("[DBG] all_series={:?}", all_series);
        }
        // Use the correct series name for each album
        let series_name = match album {
            "Switzerland" => "Series S",
            "Canada" => "Series C",
            _ => "unknown",
        };
        if let Ok(stamps_for_series) = db_utils::get_stamps_by_series(&conn, series_name, "0") {
            println!(
                "[DBG] album='{}' series='{}' has {} stamps",
                album,
                series_name,
                stamps_for_series.len()
            );
        }

        generate_album_output_conn(
            &conn,
            album,
            Option::<fn(usize)>::None,
            Some(1974),
            Some(1975),
        )
        .await;
        println!("[GENERATE] after generate album={} (cfg1)", album);
        let out1 = td
            .path()
            .join("Albums")
            .join(album)
            .join(format!("{}-1974-1975.txt", album.replace(' ', "-")));
        // fallback filename when generator uses single-album name
        let fallback1 = td
            .path()
            .join("Albums")
            .join(album)
            .join(format!("{}.txt", album.replace(' ', "-")));
        let path1 = if out1.exists() { out1 } else { fallback1 };
        assert!(
            path1.exists(),
            "output missing for case {} cfg1: {}",
            name,
            path1.display()
        );
        let s1 = fs::read_to_string(&path1).expect("read out1");

        // Second config
        write_config(td.path(), cfg2);
        let conn2 = db_utils::init_db(&db_str).expect("open db2");
        println!("[GENERATE] before generate album={} (cfg2)", album);

        if let Ok(all_series2) = db_utils::get_all_series(&conn2, None) {
            println!("[DBG] all_series(conn2)={:?}", all_series2);
        }
        let series_name2 = match album {
            "Switzerland" => "Series S",
            "Canada" => "Series C",
            _ => "unknown",
        };
        if let Ok(stamps_for_series2) = db_utils::get_stamps_by_series(&conn2, series_name2, "0") {
            println!(
                "[DBG] album(conn2)='{}' series='{}' has {} stamps",
                album,
                series_name2,
                stamps_for_series2.len()
            );
        }

        generate_album_output_conn(
            &conn2,
            album,
            Option::<fn(usize)>::None,
            Some(1974),
            Some(1975),
        )
        .await;
        println!("[GENERATE] after generate album={} (cfg2)", album);
        let out2 = td
            .path()
            .join("Albums")
            .join(album)
            .join(format!("{}-1974-1975.txt", album.replace(' ', "-")));
        let fallback2 = td
            .path()
            .join("Albums")
            .join(album)
            .join(format!("{}.txt", album.replace(' ', "-")));
        let path2 = if out2.exists() { out2 } else { fallback2 };
        assert!(
            path2.exists(),
            "output missing for case {} cfg2: {}",
            name,
            path2.display()
        );
        let s2 = fs::read_to_string(&path2).expect("read out2");

        println!("[CASE END] {} album={} - compare outputs", name, album);
        // Basic check: outputs differ
        assert_ne!(
            s1, s2,
            "Expected outputs to differ for config {} ({} vs {})",
            name, cfg1, cfg2
        );
    }

    // Optionally create a full output for a chosen album for manual AlbumEasy testing.
    // We'll generate for Switzerland and leave the file in tempdir; the test prints its path.
    let conn_full = db_utils::init_db(&db_str).expect("open db full");
    write_config(td.path(), "stamp_heading_size = 12\n");
    generate_album_output_conn(
        &conn_full,
        "Switzerland",
        Option::<fn(usize)>::None,
        Some(1974),
        Some(1975),
    )
    .await;
    let full_path = td
        .path()
        .join("Albums")
        .join("Switzerland")
        .join("Switzerland-1974-1975.txt");
    let fallback_full = td
        .path()
        .join("Albums")
        .join("Switzerland")
        .join("Switzerland.txt");
    let final_full = if full_path.exists() {
        full_path
    } else {
        fallback_full
    };
    println!(
        "Full output for manual AlbumEasy test: {}",
        final_full.display()
    );
    assert!(
        final_full.exists(),
        "Expected full output to exist: {}",
        final_full.display()
    );

    env::set_current_dir(orig).expect("restore cwd");
}
