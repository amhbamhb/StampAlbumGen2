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


