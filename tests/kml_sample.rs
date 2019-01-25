#![feature(custom_attribute)]
use serde_xml_rs::{from_reader, to_string};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

include!("./gen/kml22.rs");

#[test]
fn test_read_kml_sample() {
    let gpx = read_fixture("kml_sample.gpx").unwrap();
    eprintln!("{:#?}", gpx);
}

fn read_fixture(filename: &str) -> Result<KmlType, Box<dyn Error>> {
    let path = format!("./tests/fixtures/{}", filename);
    let reader = BufReader::new(File::open(path)?);
    let gpx: KmlType = from_reader(reader)?;

    Ok(gpx)
}
