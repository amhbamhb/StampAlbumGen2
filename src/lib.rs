// Main library file for StampAlbumGen2
// External crates
use csv::{Reader, Writer};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::vec::Vec;
use std::env;

// Image and font handling
use image::{DynamicImage, ImageBuffer, Rgba, io::Reader as ImageReader};
use rusttype::{Font, Scale};

// HTML parsing and HTTP requests
use scraper::{Html, Selector};
use reqwest::{blocking::Client, StatusCode};

// Add more modules as needed

pub mod csv_utils;
pub mod image_utils;
pub mod html_utils;
pub mod commands_file_generator;
pub mod album_globals;

// ...existing code...
