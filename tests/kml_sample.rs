#![feature(custom_attribute)]
use serde_xml_rs::{from_reader, to_string};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

include!("./gen/gpx.rs");

fn read_fixture(filename: &str) -> Result<GpxType, Box<dyn Error>> {
    let path = format!("./tests/fixtures/{}", filename);
    let reader = BufReader::new(File::open(path)?);
    let gpx: GpxType = from_reader(reader)?;

    Ok(gpx)
}

#[test]
fn test_read_kml_sample() {
    let gpx = read_fixture("gpx_sample.gpx").unwrap();
    eprintln!("{:#?}", gpx);
}

