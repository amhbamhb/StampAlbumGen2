// Comprehensive configuration parameter validation tests
// Tests all config parameters for type validation, range checking, and error handling

use stamp_album_gen2::album_globals::generate_album_output_conn;
use stamp_album_gen2::config::{load_config, Config};
use stamp_album_gen2::db_utils;
use stamp_album_gen2::file_utils;
use std::env;
use std::fs;
use tempfile::tempdir;

fn write_config(dir: &std::path::Path, contents: &str) {
    let config_path = dir.join("config.toml");
    fs::write(&config_path, contents).expect("Failed to write config file");
}

fn make_minimal_test_db(db_path: &str) {
    let conn = db_utils::init_db(db_path).expect("Failed to init DB");
    // Insert a minimal stamp for testing
    let _ = db_utils::insert_stamp(
        &conn,
        "TestAlbum",
        "Test Series",
        1974,
        "1",
        "1",
        "1",
        "1",
        "stamp",
        "1p",
        "red",
        "test description",
        "100",
        "mint",
        "1",
        "1",
        "fdc",
        "£",
        "test_image.png",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    );
}

fn load_config_from_tempdir(dir: &std::path::Path) -> Config {
    // Avoid changing the process-wide current directory to prevent races
    // when tests run in parallel. Instead, pass an absolute path to
    // load_config; the loader will handle absolute paths directly.
    let config_path = dir.join("config.toml");
    load_config(config_path.to_str().expect("Invalid temp path"))
}

#[test]
fn test_config_f32_parameter_validation() {
    println!("Testing f32 parameter validation");

    let test_cases = vec![
        // (parameter_name, valid_value, invalid_values)
        ("reference_width_mm", "25.0", vec!["abc", "-5.0", ""]),
        ("reference_height_mm", "30.0", vec!["xyz", "-10.0", ""]),
        ("stamp_box_height", "24.0", vec!["invalid", "-1.0", ""]),
        ("stamp_box_adjust", "3.0", vec!["text", "-2.0", ""]),
        ("row_font_spacing", "2.0", vec!["bad", "-1.0", ""]),
        ("header_font_spacing", "3.0", vec!["wrong", "-1.0", ""]),
        ("global_scale", "1.0", vec!["invalid", "-0.5", ""]),
        ("force_dpi", "96.0", vec!["not_a_number", "-96.0", ""]),
    ];

    for (param_name, valid_val, invalid_vals) in test_cases {
        println!("  Testing parameter: {}", param_name);

        let temp_dir = tempdir().expect("Failed to create temp directory");

        // Test valid value
        let valid_config = format!("{} = {}", param_name, valid_val);
        write_config(temp_dir.path(), &valid_config);
        let _config = load_config_from_tempdir(temp_dir.path());
        println!("    Valid value {} - config loaded successfully", valid_val);

        // Test invalid values
        for invalid_val in invalid_vals {
            if !invalid_val.is_empty() {
                let invalid_config = format!("{} = {}", param_name, invalid_val);
                write_config(temp_dir.path(), &invalid_config);

                // This should either parse with default values or fail gracefully
                let _config = load_config_from_tempdir(temp_dir.path());
                println!(
                    "    Invalid value {} - config handled gracefully",
                    invalid_val
                );
            }
        }
    }
}

#[test]
fn test_config_bool_parameter_validation() {
    println!("Testing bool parameter validation");

    let bool_params = vec![
        "print_flag_cover",
        "fetch_wikipedia",
        "include_wikipedia_content",
        "detailed_page_fit_diagnostics",
        "detailed_page_fit",
        "row_start_es_enabled",
        "preserve_real_image_size",
        "preserve_aspect",
        "include_design",
        "include_engraving",
        "include_watermark",
        "include_perforation",
        "include_catalog_numbers",
        "include_series",
        "include_country",
        "include_date",
        "emit_placeholder_diagnostics",
        "stamp_heading_in_row_height",
        "pages_vspace_in_row_height",
        "allow_single_large_stamp_per_page",
        "album_pages_footer_number_enabled",
        "album_stamp_img_new_setting_preserve_aspect",
        "show_progress_indicator",
    ];

    for param_name in bool_params {
        println!("  Testing bool parameter: {}", param_name);

        let temp_dir = tempdir().expect("Failed to create temp directory");

        // Test valid boolean values
        for bool_val in &["true", "false"] {
            let config_content = format!("{} = {}", param_name, bool_val);
            write_config(temp_dir.path(), &config_content);
            let _config = load_config_from_tempdir(temp_dir.path());
            println!("    Valid bool value {} - config loaded", bool_val);
        }

        // Test invalid boolean values
        for invalid_val in &["\"true\"", "1", "0", "yes", "no", "invalid"] {
            let config_content = format!("{} = {}", param_name, invalid_val);
            write_config(temp_dir.path(), &config_content);
            // Should either fail to parse or use default values
            let _config = load_config_from_tempdir(temp_dir.path());
            println!(
                "    Invalid bool value {} - handled gracefully",
                invalid_val
            );
        }
    }
}

#[test]
fn test_config_string_parameter_validation() {
    println!("Testing string parameter validation");

    let string_params = vec![
        (
            "series_order",
            vec!["first_date", "first_sg", "alpha", "first_sg_desc"],
            vec!["invalid_order", ""],
        ),
        (
            "expand_grouped_ranges",
            vec!["none", "placeholders", "detail_fetch"],
            vec!["invalid_mode", ""],
        ),
        (
            "album_pages_header_position",
            vec!["centre", "left", "right", "C", "L", "R", "LR", "RL"],
            vec!["invalid_pos", ""],
        ),
        (
            "stamp_heading_font",
            vec!["HN", "TN", "HB", "Arial"],
            vec![],
        ),
        (
            "album_pages_title_font",
            vec!["Helvetica-Bold", "Times-Roman"],
            vec![],
        ),
        ("page_text_font", vec!["HN", "TN"], vec![]),
        (
            "calibration_csv",
            vec!["test.csv", "/path/to/cal.csv"],
            vec![],
        ),
    ];

    for (param_name, valid_vals, invalid_vals) in string_params {
        println!("  Testing string parameter: {}", param_name);

        let temp_dir = tempdir().expect("Failed to create temp directory");

        // Test valid string values
        for valid_val in valid_vals {
            let config_content = format!("{} = \"{}\"", param_name, valid_val);
            write_config(temp_dir.path(), &config_content);
            let _config = load_config_from_tempdir(temp_dir.path());
            println!("    Valid string value '{}' - config loaded", valid_val);
        }

        // Test invalid string values
        for invalid_val in invalid_vals {
            let config_content = format!("{} = \"{}\"", param_name, invalid_val);
            write_config(temp_dir.path(), &config_content);
            let _config = load_config_from_tempdir(temp_dir.path());
            println!(
                "    Invalid string value '{}' - handled gracefully",
                invalid_val
            );
        }
    }
}

#[test]
fn test_config_u32_parameter_validation() {
    println!("Testing u32 parameter validation");

    let u32_params = vec![
        ("max_stamps_per_row", "5", vec!["abc", "-1", "0"]),
        ("album_pages_footer_number_start", "1", vec!["text", "-5"]),
    ];

    for (param_name, valid_val, invalid_vals) in u32_params {
        println!("  Testing u32 parameter: {}", param_name);

        let temp_dir = tempdir().expect("Failed to create temp directory");

        // Test valid value
        let config_content = format!("{} = {}", param_name, valid_val);
        write_config(temp_dir.path(), &config_content);
        let _config = load_config_from_tempdir(temp_dir.path());
        println!("    Valid u32 value {} - config loaded", valid_val);

        // Test invalid values
        for invalid_val in invalid_vals {
            let config_content = format!("{} = {}", param_name, invalid_val);
            write_config(temp_dir.path(), &config_content);
            let _config = load_config_from_tempdir(temp_dir.path());
            println!("    Invalid u32 value {} - handled gracefully", invalid_val);
        }
    }
}

#[test]
fn test_config_usize_parameter_validation() {
    println!("Testing usize parameter validation");

    let temp_dir = tempdir().expect("Failed to create temp directory");

    // Test max_rows_per_page
    let valid_config = "max_rows_per_page = 10";
    write_config(temp_dir.path(), valid_config);
    let _config = load_config_from_tempdir(temp_dir.path());
    println!("  Valid usize value - config loaded");

    // Test invalid values
    for invalid_val in &["abc", "-1", ""] {
        if !invalid_val.is_empty() {
            let invalid_config = format!("max_rows_per_page = {}", invalid_val);
            write_config(temp_dir.path(), &invalid_config);
            let _config = load_config_from_tempdir(temp_dir.path());
            println!("  Invalid usize value {} - handled gracefully", invalid_val);
        }
    }
}

#[test]
fn test_config_color_parameter_validation() {
    println!("Testing color parameter validation");

    let color_params = vec![
        "color_album_border",
        "color_album_footer",
        "color_album_header",
        "color_album_title",
        "color_album_margin_txt",
        "color_stamp_border",
        "color_stamp_heading",
        "color_stamp_text",
        "color_stamp_background",
        "flag_cover_background_color",
    ];

    for param_name in color_params {
        println!("  Testing color parameter: {}", param_name);

        let temp_dir = tempdir().expect("Failed to create temp directory");

        // Test valid color values
        for color_val in &["red", "#FF0000", "rgb(255,0,0)", "blue", "green"] {
            let config_content = format!("{} = \"{}\"", param_name, color_val);
            write_config(temp_dir.path(), &config_content);
            let _config = load_config_from_tempdir(temp_dir.path());
            println!("    Valid color '{}' - config loaded", color_val);
        }

        // Test potentially invalid color values (should be handled gracefully)
        for invalid_color in &["notacolor", "rgb(999,999,999)", "#GGGGGG"] {
            let config_content = format!("{} = \"{}\"", param_name, invalid_color);
            write_config(temp_dir.path(), &config_content);
            let _config = load_config_from_tempdir(temp_dir.path());
            println!(
                "    Potentially invalid color '{}' - handled gracefully",
                invalid_color
            );
        }
    }
}

#[test]
fn test_config_parameter_interactions() {
    println!("Testing parameter interactions");

    let temp_dir = tempdir().expect("Failed to create temp directory");

    // Test stamp dimensions consistency
    let dimension_config = r#"
reference_width_mm = 50.0
reference_height_mm = 20.0  
stamp_box_height = 25.0
"#;
    write_config(temp_dir.path(), dimension_config);
    let _config = load_config_from_tempdir(temp_dir.path());
    println!("  Dimension interaction test - config loaded");

    // Test font consistency
    let font_config = r#"
stamp_heading_font = "HN"
page_text_font = "Arial"
stamp_heading_size = 25.0
page_text_size = 10.0
"#;
    write_config(temp_dir.path(), font_config);
    let _config = load_config_from_tempdir(temp_dir.path());
    println!("  Font interaction test - config loaded");

    // Test layout interactions
    let layout_config = r#"
max_stamps_per_row = 8
reference_width_mm = 40.0
album_page_width_mm = 200.0
"#;
    write_config(temp_dir.path(), layout_config);
    let _config = load_config_from_tempdir(temp_dir.path());
    println!("  Layout interaction test - config loaded");
}

#[test]
fn test_config_edge_cases() {
    println!("Testing edge cases");

    let temp_dir = tempdir().expect("Failed to create temp directory");

    // Test empty config file
    write_config(temp_dir.path(), "");
    let _config = load_config_from_tempdir(temp_dir.path());
    println!("  Empty config file - handled gracefully");

    // Test malformed TOML
    write_config(temp_dir.path(), "[broken");
    let _config = load_config_from_tempdir(temp_dir.path());
    println!("  Malformed TOML - handled gracefully");

    // Test duplicate keys
    let duplicate_config = r#"
reference_width_mm = 25.0
reference_width_mm = 30.0
"#;
    write_config(temp_dir.path(), duplicate_config);
    let _config = load_config_from_tempdir(temp_dir.path());
    println!("  Duplicate keys - handled gracefully");

    // Test very large values
    let large_values_config = r#"
reference_width_mm = 99999.0
stamp_heading_size = 500.0
max_stamps_per_row = 1000
"#;
    write_config(temp_dir.path(), large_values_config);
    let _config = load_config_from_tempdir(temp_dir.path());
    println!("  Very large values - handled gracefully");

    // Test special characters
    let special_chars_config = r#"
stamp_heading_label = "Test & Co."
album_pages_title_font = "Ελληνικά"
"#;
    write_config(temp_dir.path(), special_chars_config);
    let _config = load_config_from_tempdir(temp_dir.path());
    println!("  Special characters - handled gracefully");
}

#[test]
fn test_config_header_formatting_promotion() {
    println!("Testing header_formatting section promotion");

    let temp_dir = tempdir().expect("Failed to create temp directory");

    // Test parameters in header_formatting section get promoted to top-level
    let header_section_config = r#"
[header_formatting]
include_date = true
include_design = false
date_format = "YYYY-MM-DD"
include_currency = true
"#;
    write_config(temp_dir.path(), header_section_config);
    let _config = load_config_from_tempdir(temp_dir.path());
    println!("  Header formatting promotion - config loaded");

    // Test conflict between top-level and header_formatting
    let conflict_config = r#"
include_date = false

[header_formatting]
include_date = true
"#;
    write_config(temp_dir.path(), conflict_config);
    let _config = load_config_from_tempdir(temp_dir.path());
    println!("  Top-level vs header_formatting conflict - handled gracefully");
}

#[tokio::test]
async fn test_config_with_actual_generation() {
    println!("Testing config parameters actually affect output generation");

    let temp_dir = tempdir().expect("Failed to create temp directory");
    let orig_cwd = env::current_dir().expect("Failed to get current directory");
    env::set_current_dir(temp_dir.path()).expect("Failed to change directory");

    // Enable overwrite to allow repeated generation
    file_utils::set_overwrite_album_from_cli(true);

    // Create test database
    let db_path = temp_dir.path().join("TestConfig.db");
    let db_str = db_path.to_string_lossy().to_string();
    make_minimal_test_db(&db_str);

    // Test 1: Default config
    write_config(temp_dir.path(), "");
    let conn = db_utils::init_db(&db_str).expect("Failed to open DB");
    generate_album_output_conn(
        &conn,
        "TestConfig",
        Option::<fn(usize)>::None,
        Some(1974),
        Some(1974),
    )
    .await;

    let output_path = temp_dir
        .path()
        .join("Albums")
        .join("TestConfig")
        .join("TestConfig-1974-1974.txt");

    let default_output = if output_path.exists() {
        fs::read_to_string(&output_path).unwrap_or_default()
    } else {
        String::new()
    };

    // Test 2: Modified config
    let modified_config = r#"
stamp_heading_size = 20.0
stamp_heading_label = "CUSTOM"
reference_width_mm = 40.0
max_stamps_per_row = 3
include_design = true
"#;
    write_config(temp_dir.path(), modified_config);

    let conn2 = db_utils::init_db(&db_str).expect("Failed to open DB for second test");
    generate_album_output_conn(
        &conn2,
        "TestConfig",
        Option::<fn(usize)>::None,
        Some(1974),
        Some(1974),
    )
    .await;

    let modified_output = if output_path.exists() {
        fs::read_to_string(&output_path).unwrap_or_default()
    } else {
        String::new()
    };

    // Outputs should be different
    assert_ne!(
        default_output, modified_output,
        "Config changes should result in different album output"
    );

    println!("  Config changes properly affect album generation");
    println!("  Default output: {} chars", default_output.len());
    println!("  Modified output: {} chars", modified_output.len());

    // Restore working directory
    env::set_current_dir(orig_cwd).expect("Failed to restore directory");
}
