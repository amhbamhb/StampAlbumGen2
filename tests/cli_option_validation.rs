// Comprehensive CLI option validation tests
// Tests all CLI options for parsing, validation, error handling, and combinations

use std::fs;
use std::process::Command;
use tempfile::tempdir;

// Helper function to run the CLI with given arguments and capture output
fn run_cli_with_args(args: &[&str]) -> (i32, String, String) {
    let output = Command::new("target/debug/stamp_album_gen2")
        .args(args)
        .output()
        .expect("Failed to execute CLI");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    (exit_code, stdout, stderr)
}

// Helper function to create a test database
fn create_test_db(path: &str) {
    let conn = stamp_album_gen2::db_utils::init_db(path).expect("Failed to create test DB");
    // Insert minimal test data
    let _ = stamp_album_gen2::db_utils::insert_stamp(
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

#[test]
fn test_cli_help_and_version() {
    println!("Testing basic CLI help and version");

    // Test --help
    let (exit_code, stdout, _stderr) = run_cli_with_args(&["--help"]);
    assert_eq!(exit_code, 0, "Help should exit with code 0");
    assert!(
        stdout.contains("Usage:"),
        "Help should contain usage information"
    );
    assert!(
        stdout.contains("--source"),
        "Help should list --source option"
    );
    assert!(
        stdout.contains("--site-name"),
        "Help should list --site-name option"
    );
    println!("  ✓ --help shows complete help text");

    // Test --version
    let (exit_code, stdout, _stderr) = run_cli_with_args(&["--version"]);
    assert_eq!(exit_code, 0, "Version should exit with code 0");
    assert!(
        stdout.contains("stamp_album_gen2"),
        "Version should contain program name"
    );
    println!("  ✓ --version shows version information");
}

#[test]
fn test_cli_no_arguments() {
    println!("Testing CLI with no arguments");

    let (exit_code, _stdout, stderr) = run_cli_with_args(&[]);
    assert_ne!(exit_code, 0, "No arguments should exit with non-zero code");
    assert!(
        stderr.contains("didn't provide any command options"),
        "Should provide guidance when no options given"
    );
    assert!(
        stderr.contains("--db-path"),
        "Should mention db-path option"
    );
    assert!(
        stderr.contains("--site-name"),
        "Should mention site-name option"
    );
    println!("  ✓ No arguments shows helpful guidance");
}

#[test]
fn test_cli_source_validation() {
    println!("Testing --source option validation");

    // Valid sources
    let valid_sources = ["stampworld", "touchstamps", "extractfile", "colnectexport"];
    for source in valid_sources.iter() {
        // Just test parsing, not full execution
        let (exit_code, _stdout, stderr) =
            run_cli_with_args(&["--source", source, "--site-name", "TestSite"]);
        // Exit code may be non-zero due to missing data, but should not be a source validation error
        assert!(
            !stderr.contains("Unrecognized source"),
            "Valid source '{}' should not show unrecognized error",
            source
        );
        println!("  ✓ Valid source '{}' accepted", source);
    }

    // Invalid source
    let (exit_code, _stdout, stderr) = run_cli_with_args(&["--source", "invalid_source"]);
    assert_ne!(exit_code, 0, "Invalid source should exit with error");
    assert!(
        stderr.contains("Unrecognized source"),
        "Should show unrecognized source error"
    );
    assert!(
        stderr.contains("stampworld"),
        "Should suggest valid sources"
    );
    println!("  ✓ Invalid source shows error and suggestions");

    // Typo in source (should suggest correction)
    let (exit_code, _stdout, stderr) = run_cli_with_args(&["--source", "stamworld"]);
    assert_ne!(exit_code, 0, "Typo source should exit with error");
    assert!(
        stderr.contains("Unrecognized source"),
        "Should show unrecognized source error"
    );
    assert!(
        stderr.contains("stampworld"),
        "Should suggest correct spelling"
    );
    println!("  ✓ Source typo suggests correct spelling");
}

#[test]
fn test_cli_year_range_validation() {
    println!("Testing year range validation");

    // Valid year range
    let (exit_code, _stdout, stderr) = run_cli_with_args(&[
        "--source",
        "stampworld",
        "--site-name",
        "TestSite",
        "--start-year",
        "1900",
        "--end-year",
        "2000",
    ]);
    assert!(
        !stderr.contains("Invalid year range"),
        "Valid year range should not show error"
    );
    println!("  ✓ Valid year range accepted");

    // Invalid year range (start > end)
    let (exit_code, _stdout, stderr) = run_cli_with_args(&[
        "--source",
        "stampworld",
        "--site-name",
        "TestSite",
        "--start-year",
        "2000",
        "--end-year",
        "1900",
    ]);
    assert_ne!(exit_code, 0, "Invalid year range should exit with error");
    assert!(
        stderr.contains("Invalid year range"),
        "Should show year range error"
    );
    assert!(
        stderr.contains("start-year"),
        "Should mention start-year in error"
    );
    assert!(
        stderr.contains("end-year"),
        "Should mention end-year in error"
    );
    println!("  ✓ Invalid year range (start > end) shows error");

    // Edge case: same year
    let (exit_code, _stdout, stderr) = run_cli_with_args(&[
        "--source",
        "stampworld",
        "--site-name",
        "TestSite",
        "--start-year",
        "2000",
        "--end-year",
        "2000",
    ]);
    assert!(
        !stderr.contains("Invalid year range"),
        "Same start/end year should be valid"
    );
    println!("  ✓ Same start and end year accepted");
}

#[test]
fn test_cli_database_path_handling() {
    println!("Testing database path handling");

    let temp_dir = tempdir().expect("Failed to create temp directory");

    // Valid database path
    let db_path = temp_dir.path().join("test.db");
    create_test_db(&db_path.to_string_lossy());

    let (exit_code, _stdout, stderr) = run_cli_with_args(&[
        "--db-path",
        &db_path.to_string_lossy(),
        "--album-name",
        "TestAlbum",
    ]);
    assert_eq!(exit_code, 0, "Valid DB path should succeed");
    println!("  ✓ Valid database path handled correctly");

    // Non-existent database file (should be created)
    let new_db_path = temp_dir.path().join("new.db");
    let (exit_code, _stdout, stderr) = run_cli_with_args(&[
        "--db-path",
        &new_db_path.to_string_lossy(),
        "--album-name",
        "TestAlbum",
    ]);
    // Should attempt to create the file (may fail due to no data, but not due to path issues)
    assert!(
        !stderr.contains("Failed to create DB file"),
        "Should be able to create new DB file in valid directory"
    );
    println!("  ✓ Non-existent DB file creation attempted");

    // Invalid directory path
    let invalid_path = "/definitely/does/not/exist/test.db";
    let (exit_code, _stdout, stderr) =
        run_cli_with_args(&["--db-path", invalid_path, "--album-name", "TestAlbum"]);
    assert_ne!(exit_code, 0, "Invalid directory path should fail");
    assert!(
        stderr.contains("Failed to create parent directory")
            || stderr.contains("Failed to create DB file"),
        "Should show directory/file creation error"
    );
    println!("  ✓ Invalid directory path shows appropriate error");
}

#[test]
fn test_cli_boolean_flags() {
    println!("Testing boolean flag handling");

    // Test individual flags
    let boolean_flags = ["--overwrite-album", "--overwrite-images", "--yes", "-y"];

    for flag in boolean_flags.iter() {
        let (exit_code, _stdout, stderr) =
            run_cli_with_args(&["--source", "stampworld", "--site-name", "TestSite", flag]);
        // Flag should be parsed correctly (exit code may be non-zero due to other issues)
        assert!(
            !stderr.contains("unexpected argument"),
            "Boolean flag '{}' should be recognized",
            flag
        );
        println!("  ✓ Boolean flag '{}' recognized", flag);
    }

    // Test flag combinations
    let (exit_code, _stdout, stderr) = run_cli_with_args(&[
        "--source",
        "stampworld",
        "--site-name",
        "TestSite",
        "--overwrite-album",
        "--overwrite-images",
        "--yes",
    ]);
    assert!(
        !stderr.contains("unexpected argument"),
        "Multiple boolean flags should be accepted"
    );
    println!("  ✓ Multiple boolean flags work together");
}

#[test]
fn test_cli_required_combinations() {
    println!("Testing required option combinations");

    // Test scraping mode requires site-name
    let (exit_code, _stdout, stderr) = run_cli_with_args(&["--source", "stampworld"]);
    assert_ne!(exit_code, 0, "Scraping without site-name should fail");
    assert!(
        stderr.contains("site name") || stderr.contains("site-name"),
        "Should mention missing site name"
    );
    println!("  ✓ Scraping mode requires site-name");

    // Test db-path mode works without site-name
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test.db");
    create_test_db(&db_path.to_string_lossy());

    let (exit_code, _stdout, stderr) = run_cli_with_args(&[
        "--db-path",
        &db_path.to_string_lossy(),
        "--album-name",
        "TestAlbum",
    ]);
    assert_eq!(exit_code, 0, "DB-only mode should work without site-name");
    println!("  ✓ DB-only mode works without site-name");
}

#[test]
fn test_cli_error_message_quality() {
    println!("Testing error message quality");

    // Test unrecognized source error message
    let (exit_code, _stdout, stderr) = run_cli_with_args(&["--source", "unknown"]);
    assert!(
        stderr.contains("Unrecognized source"),
        "Should identify the problem"
    );
    assert!(
        stderr.contains("stampworld"),
        "Should suggest valid alternatives"
    );
    assert!(stderr.contains("examples"), "Should provide examples");
    println!("  ✓ Unrecognized source error provides good guidance");

    // Test missing site-name error message
    let (exit_code, _stdout, stderr) =
        run_cli_with_args(&["--source", "stampworld", "--start-year", "1900"]);
    assert!(
        stderr.contains("site name"),
        "Should identify missing site name"
    );
    assert!(
        stderr.contains("--site-name"),
        "Should mention the specific option"
    );
    assert!(stderr.contains("example"), "Should provide example usage");
    println!("  ✓ Missing site-name error provides clear guidance");

    // Test invalid year range error message
    let (exit_code, _stdout, stderr) = run_cli_with_args(&[
        "--source",
        "stampworld",
        "--site-name",
        "TestSite",
        "--start-year",
        "2000",
        "--end-year",
        "1900",
    ]);
    assert!(
        stderr.contains("Invalid year range"),
        "Should identify the problem"
    );
    assert!(stderr.contains("start-year"), "Should mention start-year");
    assert!(stderr.contains("end-year"), "Should mention end-year");
    assert!(stderr.contains("2000"), "Should show actual values");
    assert!(stderr.contains("1900"), "Should show actual values");
    println!("  ✓ Invalid year range error shows specific values");
}

#[test]
fn test_cli_special_characters_and_edge_cases() {
    println!("Testing special characters and edge cases");

    // Test site name with special characters
    let (exit_code, _stdout, stderr) = run_cli_with_args(&[
        "--source",
        "stampworld",
        "--site-name",
        "British-Columbia!Vancouver-Island",
    ]);
    assert!(
        !stderr.contains("unexpected argument"),
        "Site names with special characters should be accepted"
    );
    println!("  ✓ Site names with special characters handled");

    // Test empty string values
    let (exit_code, _stdout, stderr) =
        run_cli_with_args(&["--source", "", "--site-name", "TestSite"]);
    assert_ne!(exit_code, 0, "Empty source should be rejected");
    println!("  ✓ Empty string values properly rejected");

    // Test very long site name
    let long_name = "Very-Long-Site-Name-With-Many-Hyphens-And-More-Text";
    let (exit_code, _stdout, stderr) =
        run_cli_with_args(&["--source", "stampworld", "--site-name", long_name]);
    assert!(
        !stderr.contains("unexpected argument"),
        "Long site names should be handled"
    );
    println!("  ✓ Long site names handled correctly");
}

#[test]
fn test_cli_numeric_parameter_validation() {
    println!("Testing numeric parameter validation");

    // Test max-pages with valid values
    let (exit_code, _stdout, stderr) = run_cli_with_args(&[
        "--source",
        "stampworld",
        "--site-name",
        "TestSite",
        "--max-pages",
        "5",
    ]);
    assert!(
        !stderr.contains("invalid value"),
        "Valid max-pages should be accepted"
    );
    println!("  ✓ Valid max-pages value accepted");

    // Test max-pages with invalid values
    let (exit_code, _stdout, stderr) = run_cli_with_args(&[
        "--source",
        "stampworld",
        "--site-name",
        "TestSite",
        "--max-pages",
        "not-a-number",
    ]);
    assert_ne!(exit_code, 0, "Invalid max-pages should be rejected");
    assert!(
        stderr.contains("invalid value") || stderr.contains("invalid digit"),
        "Should show parsing error for non-numeric max-pages"
    );
    println!("  ✓ Invalid max-pages value rejected");

    // Test stop-after-no-hits
    let (exit_code, _stdout, stderr) = run_cli_with_args(&[
        "--source",
        "stampworld",
        "--site-name",
        "TestSite",
        "--stop-after-no-hits",
        "1",
    ]);
    assert!(
        !stderr.contains("invalid value"),
        "Valid stop-after-no-hits should be accepted"
    );
    println!("  ✓ Valid stop-after-no-hits value accepted");
}

#[test]
fn test_cli_auto_correction_with_yes_flag() {
    println!("Testing auto-correction with --yes flag");

    // Test --yes flag with a close misspelling (should auto-correct)
    let (exit_code, _stdout, stderr) =
        run_cli_with_args(&["--source", "stamworld", "--yes", "--site-name", "TestSite"]);
    // With --yes flag, it might auto-correct and continue (implementation dependent)
    // At minimum, it shouldn't crash
    println!("  ✓ --yes flag handled with source typo (auto-correction behavior)");

    // Test --yes flag with valid source (should work normally)
    let (exit_code, _stdout, stderr) =
        run_cli_with_args(&["--source", "stampworld", "--yes", "--site-name", "TestSite"]);
    assert!(
        !stderr.contains("Unrecognized source"),
        "Valid source with --yes should work normally"
    );
    println!("  ✓ --yes flag works with valid source");
}

// Test helper to ensure the binary is built before running tests
#[test]
fn test_ensure_binary_built() {
    println!("Ensuring binary is built for CLI tests");

    // Build the binary if it doesn't exist
    if !std::path::Path::new("target/debug/stamp_album_gen2").exists() {
        let output = Command::new("cargo")
            .args(&["build", "--bin", "stamp_album_gen2"])
            .output()
            .expect("Failed to build binary");

        if !output.status.success() {
            panic!(
                "Failed to build binary: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    println!("  ✓ Binary available for testing");
}
