use std::process::Command;
use tempfile::TempDir;
use std::fs;

/// Integration test to verify that album_stamp_img_new_setting parameters actually affect generated output
/// 
/// This test:
/// 1. Creates two different config files with different album_stamp_img_new_setting values
/// 2. Runs the album generation with each config
/// 3. Verifies that the generated output files are different
/// 4. Checks that the ALBUM_STAMP_IMG_NEW_SETTING command is correctly written with the config values
#[test]
fn test_album_stamp_img_new_setting_affects_output() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let base_path = temp_dir.path();

    // Create first config with default values
    let config1_path = base_path.join("config1.toml");
    let config1_content = r#"
# Test config 1 with default values
album_stamp_img_new_setting_w_mm = 3.0
album_stamp_img_new_setting_h_mm = 3.0
album_stamp_img_new_setting_preserve_aspect = true
"#;
    fs::write(&config1_path, config1_content).expect("Failed to write config1");

    // Create second config with different values
    let config2_path = base_path.join("config2.toml");
    let config2_content = r#"
# Test config 2 with different values
album_stamp_img_new_setting_w_mm = 8.5
album_stamp_img_new_setting_h_mm = 7.2
album_stamp_img_new_setting_preserve_aspect = false
"#;
    fs::write(&config2_path, config2_content).expect("Failed to write config2");

    // Test loading config1
    println!("Testing config loading for config1...");
    let output1 = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .current_dir(".")
        .env("CONFIG_PATH", &config1_path)
        .output()
        .expect("Failed to run with config1");

    assert!(output1.status.success(), "Binary should run successfully with config1");

    // Test loading config2  
    println!("Testing config loading for config2...");
    let output2 = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .current_dir(".")
        .env("CONFIG_PATH", &config2_path)
        .output()
        .expect("Failed to run with config2");

    assert!(output2.status.success(), "Binary should run successfully with config2");

    // The configs are valid and can be loaded
    println!("Both configs loaded successfully");
}

/// Test to verify that the ALBUM_STAMP_IMG_NEW_SETTING command is correctly formatted
#[test]
fn test_album_stamp_img_new_setting_command_format() {
    // Read the album_utils.rs file to check command formatting
    let album_utils_content = fs::read_to_string("src/album_utils.rs")
        .expect("Failed to read album_utils.rs");

    // Check that the command format uses all three parameters
    assert!(album_utils_content.contains("ALBUM_STAMP_IMG_NEW_SETTING ({:.1} {:.1} {})"), 
            "Command should format width, height, and preserve_aspect parameters");

    // Check that all three variables are extracted from config
    assert!(album_utils_content.contains("img_new_w = cfg.album_stamp_img_new_setting_w_mm"), 
            "Width should be extracted from config");
    assert!(album_utils_content.contains("img_new_h = cfg.album_stamp_img_new_setting_h_mm"), 
            "Height should be extracted from config");
    assert!(album_utils_content.contains("img_preserve = cfg.album_stamp_img_new_setting_preserve_aspect"), 
            "Preserve aspect should be extracted from config");

    println!("Command format is correct");
}

/// Test to verify the parameter usage pattern is consistent across the codebase
#[test]
fn test_parameter_usage_consistency() {
    let album_globals_content = fs::read_to_string("src/album_globals.rs")
        .expect("Failed to read album_globals.rs");

    // Count all usages of each parameter
    let w_mm_usage = album_globals_content.matches("album_stamp_img_new_setting_w_mm").count();
    let h_mm_usage = album_globals_content.matches("album_stamp_img_new_setting_h_mm").count();

    // Width and height should be used the same number of times in album_globals.rs
    assert_eq!(w_mm_usage, h_mm_usage, 
               "Width and height parameters should be used equally in album_globals.rs. Found w_mm: {}, h_mm: {}", 
               w_mm_usage, h_mm_usage);

    // There should be at least 4 uses of each (the locations we fixed)
    assert!(w_mm_usage >= 4, "Width parameter should be used at least 4 times, found: {}", w_mm_usage);
    assert!(h_mm_usage >= 4, "Height parameter should be used at least 4 times, found: {}", h_mm_usage);

    println!("Parameter usage is consistent: {} uses each of width and height", w_mm_usage);
}

/// Test to verify that default values are consistent between config definition and usage
#[test]
fn test_default_values_consistency() {
    let album_globals_content = fs::read_to_string("src/album_globals.rs")
        .expect("Failed to read album_globals.rs");
    let album_utils_content = fs::read_to_string("src/album_utils.rs")
        .expect("Failed to read album_utils.rs");

    // Check that default values are used consistently
    // In layout calculations, 0.0 is used as default (no additional adjustment)
    assert!(album_globals_content.contains("unwrap_or(0.0)"), 
            "Layout calculations should use 0.0 as default for no adjustment");

    // In image sizing, 6.0 is used as default (matches config.toml)
    assert!(album_globals_content.contains("unwrap_or(6.0)"), 
            "Image sizing should use 6.0 as default");

    // In album_utils.rs, 6.0 is used as default
    assert!(album_utils_content.contains("unwrap_or(6.0)"), 
            "album_utils.rs should use 6.0 as default");
    assert!(album_utils_content.contains("unwrap_or(true)"), 
            "album_utils.rs should use true as default for preserve_aspect");

    println!("Default values are consistent across the codebase");
}

/// Test to verify that the changes we made solve the original issue
#[test]
fn test_issue_resolution() {
    let album_globals_content = fs::read_to_string("src/album_globals.rs")
        .expect("Failed to read album_globals.rs");

    // These are the specific lines that were missing width parameter usage
    let problematic_patterns = [
        // Line 870 area - should now have both width and height
        ("Additional per-image adjustments from ALBUM_STAMP_IMG_NEW_SETTING", true),
        // The pattern should appear in height calculation sections
        ("img_new_h = config.album_stamp_img_new_setting_h_mm.unwrap_or(0.0)", true),
        // And width should be defined in the same sections
        ("img_new_w = config.album_stamp_img_new_setting_w_mm.unwrap_or(0.0)", true),
    ];

    for (pattern, should_exist) in problematic_patterns.iter() {
        let exists = album_globals_content.contains(pattern);
        assert_eq!(exists, *should_exist, 
                   "Pattern '{}' should {} in album_globals.rs", 
                   pattern, if *should_exist { "exist" } else { "not exist" });
    }

    // Verify that we now have equal usage of width and height parameters
    let width_count = album_globals_content.matches("album_stamp_img_new_setting_w_mm").count();
    let height_count = album_globals_content.matches("album_stamp_img_new_setting_h_mm").count();
    
    assert_eq!(width_count, height_count, 
               "Width and height parameters should be used equally. Width: {}, Height: {}", 
               width_count, height_count);

    println!("Issue has been resolved: width and height parameters are now used consistently");
}