// Album global state and constants
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
	pub static ref FILE_SEPARATOR: String = std::path::MAIN_SEPARATOR.to_string();
	pub static ref DATA_SOURCE: Mutex<String> = Mutex::new(String::new());
	pub static ref FILE_SOURCE: Mutex<String> = Mutex::new(String::new());
	pub static ref ALBUM_DIRECTORY: Mutex<String> = Mutex::new(String::new());
	pub static ref IMAGES_DIRECTORY: Mutex<String> = Mutex::new(String::new());
	pub static ref ALBUM_NAME: Mutex<String> = Mutex::new(String::new());
	pub static ref ALBUM_FILE_NAME: Mutex<String> = Mutex::new(String::new());
	pub static ref EXTRACT_FILE_NAME: Mutex<String> = Mutex::new(String::new());
	pub static ref EXPORT_FILE_NAME: Mutex<String> = Mutex::new(String::new());
	pub static ref SITE_NAME: Mutex<String> = Mutex::new(String::new());
	pub static ref CATALOG_NAME: Mutex<String> = Mutex::new(String::new());
	pub static ref NO_IMAGES: Mutex<bool> = Mutex::new(false);
	pub static ref FIX_SIZES: Mutex<bool> = Mutex::new(false);
	pub static ref NO_SSL_CHECK: Mutex<bool> = Mutex::new(false);
	pub static ref SEPARATOR: &'static str = "@";
	pub static ref ROWS_WRITTEN: Mutex<i32> = Mutex::new(0);
	pub static ref STAMPS_WRITTEN: Mutex<i32> = Mutex::new(0);
	pub static ref USED_WIDTH: Mutex<i32> = Mutex::new(0);
	pub static ref USED_HEIGHT: Mutex<i32> = Mutex::new(0);
	pub static ref MAX_ROW_HEIGHT: Mutex<i32> = Mutex::new(0);
	pub static ref SEQUENCER: Mutex<i32> = Mutex::new(0);
	pub static ref START_YEAR: Mutex<i32> = Mutex::new(0);
	pub static ref END_YEAR: Mutex<i32> = Mutex::new(9999);
	pub static ref FORMATS: Mutex<Vec<String>> = Mutex::new(Vec::new());
	pub static ref EMISSIONS: Mutex<Vec<String>> = Mutex::new(Vec::new());
	pub static ref CURRENCIES: Mutex<Vec<String>> = Mutex::new(Vec::new());
	pub static ref MONTH_NAMES: [&'static str; 13] = ["?????", "January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
	pub static ref VALID_FORMATS: [&'static str; 12] = ["Booklet", "Booklet Pane", "Full Pane", "Gutter Pairs", "Mini Sheet", "Se-tenant", "Souvenir Sheet", "Stamp", "Stamp with Attached Label", "Stamp with Collectible Margin", "Tete-beche", "Unknown"];
	pub static ref VALID_EMISSIONS: [&'static str; 44] = [
		"Agency Issue", "Air Post", "Air Post Official", "Air Post Semi-Postal", "ATM Labels", "Cinderella", "Commemorative", "Cryptostamp", "Definitive", "Erinnophilia", "Forgery", "Franchise Stamps", "Hunting Permit", "Illegal", "Insured Letter", "Military", "Newspaper", "Official", "Official Reprint", "Other", "Parcel Post", "Personal Delivery", "Personalized - Official", "Personalized - Private", "Postage Due", "Postal Fiscal", "Postal Tax", "Precancelled", "Private", "Proof", "Regional", "Registration", "Revenue", "Savings Stamps", "Semi-Postal", "Special Delivery", "Specimen", "Telegraph & Telephone", "Used Abroad", "War Tax", "Charity", "Express", "Local", "Occupation"
	];
}

/// Struct representing a year, month, and day
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YearMonthDay {
	pub year: String,
	pub month: String,
	pub day: String,
}

impl YearMonthDay {
	pub fn new(year: &str, month: &str, day: &str) -> Self {
		Self {
			year: year.to_string(),
			month: month.to_string(),
			day: day.to_string(),
		}
	}
}

/// Operating system type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OSType {
	Windows,
	MacOS,
	Linux,
	Other,
	Unknown,
}

/// Struct representing series information
#[derive(Debug, Clone, PartialEq)]
pub struct SeriesInfo {
	pub count: i32,
	pub multi_perf: bool,
	pub width: String,
	pub height: String,
}

impl SeriesInfo {
	pub fn new(count: i32, multi_perf: bool, width: &str, height: &str) -> Self {
		Self {
			count,
			multi_perf,
			width: width.to_string(),
			height: height.to_string(),
		}
	}
}

/// Struct representing image status
#[derive(Debug, Clone, PartialEq)]
pub struct ImageStatus {
	pub link: String,
	pub exists: bool,
	pub width: i32,
	pub height: i32,
}

impl ImageStatus {
	pub fn new(link: &str, exists: bool, width: i32, height: i32) -> Self {
		Self {
			link: link.to_string(),
			exists,
			width,
			height,
		}
	}
}

/// Struct representing miscellaneous attributes
#[derive(Debug, Clone, PartialEq)]
pub struct MiscAttrs {
	pub width: String,
	pub height: String,
	pub face_value: String,
	pub currency: String,
	pub month: String,
	pub day: String,
}

impl MiscAttrs {
	pub fn new(width: &str, height: &str, face_value: &str, currency: &str, month: &str, day: &str) -> Self {
		Self {
			width: width.to_string(),
			height: height.to_string(),
			face_value: face_value.to_string(),
			currency: currency.to_string(),
			month: month.to_string(),
			day: day.to_string(),
		}
	}
}

/// Writes a new series header, handles page breaks, and updates counters
pub fn new_series<W: Write>(
	writer: &mut W,
	series_name: &str,
	year: &str,
	month: &str,
	day: &str,
	stamps_in_series: i32,
	is_perforated: bool,
	perforation: &str,
	rows_written: &mut i32,
	stamps_written: &mut i32,
	used_width: &mut i32,
	used_height: &mut i32,
	max_row_height: &mut i32,
	month_names: &[&str],
) {
	// ...function body from old album_globals.rs...
}

/// Writes a line to the output, with error handling
pub fn write_line<W: Write>(writer: &mut W, line: &str) {
	if let Err(e) = writeln!(writer, "{}", line) {
		log::error!("Error writing output file: {}", e);
		std::process::exit(1);
	}
}
/// Writes a new page header and resets counters
/// Writes a new page header and resets counters
pub fn new_page<W: std::io::Write>(writer: &mut W, rows_written: &mut i32, stamps_written: &mut i32, used_width: &mut i32, used_height: &mut i32, max_row_height: &mut i32) {
	let _ = writeln!(writer, "################################################################################");
	let _ = writeln!(writer, "PAGE_START");
	*rows_written = 0;
	*stamps_written = 0;
	*used_width = 0;
	*used_height = 14;
	*max_row_height = 0;
}

/// Writes a new year header
pub fn new_year<W: std::io::Write>(writer: &mut W, year: &str) {
	let _ = writeln!(writer, "PAGE_TEXT_CENTRE (TN 8 \"{}\" 1.0)", year);
}

/// Writes a new row, handles page breaks, and updates counters
pub fn new_row<W: std::io::Write>(writer: &mut W, year: &str, rows_written: &mut i32, stamps_written: &mut i32, used_width: &mut i32, used_height: &mut i32, max_row_height: &mut i32) {
	*used_height += *max_row_height;
	if *used_height + 43 > 245 {
		new_page(writer, rows_written, stamps_written, used_width, used_height, max_row_height);
		new_year(writer, year);
	}
	let _ = writeln!(writer, "ROW_START_FS (TN 6 0.1 5.0)");
	*rows_written += 1;
	*stamps_written = 0;
	*used_width = 0;
	*used_height += *max_row_height;
	*max_row_height = 0;
}

/// Writes a stamp entry (stub implementation)
pub fn add_stamp<W: std::io::Write>(
	writer: &mut W,
	year: &str,
	field1: &str,
	field2: &str,
	field3: &str,
	color: String,
	multi_perf: bool,
	desc: &str,
	width: &str,
	height: &str,
	catalog_num: &str,
	field4: &str,
) {
	let _ = writeln!(writer, "STAMP {} {} {} {} {} {} {} {} {} {} {}", year, field1, field2, field3, color, multi_perf, desc, width, height, catalog_num, field4);
}

// AlbumEasy Output Generation
// ==========================
// This section contains the main entry point and helpers for generating AlbumEasy output.

use std::io::Write;
use crate::db_utils::{init_db, get_all_series, get_stamps_by_series};
use crate::image_utils::{calc_mm_per_pixel, estimate_real_dimensions};
use std::fs::File;
use std::io::BufWriter;

/// Main entry point for AlbumEasy output generation.
/// Queries the database, formats output, and writes to a .txt file for AlbumEasy import.
pub fn generate_album_output<F>(db_path: &str, album_name: &str, mut page_callback: Option<F>)
where
	F: FnMut(usize),
{
	// ...existing code from album_globals.rs...
}

// ...rest of code from album_globals.rs...
// Stampworld-specific AlbumEasy output logic
// Formerly album_globals.rs

// ...existing code from album_globals.rs will be moved here...
