// CSV utilities for StampAlbumGen2
use csv::{Reader, Writer};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

pub fn read_csv<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = Reader::from_reader(BufReader::new(file));
    let mut records = Vec::new();
    for result in rdr.records() {
        let record = result?;
        records.push(record.iter().map(|s| s.to_string()).collect());
    }
    Ok(records)
}

pub fn write_csv<P: AsRef<Path>>(path: P, data: &[Vec<String>]) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    let mut wtr = Writer::from_writer(BufWriter::new(file));
    for row in data {
        wtr.write_record(row)?;
    }
    wtr.flush()?;
    Ok(())
}
