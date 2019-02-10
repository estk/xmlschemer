use serde_xml_rs::from_reader;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use xmlschemer::schema::*;

#[test]
fn test_read_xml() {
	read_fixture("XMLSchema.xsd").unwrap();
}
#[test]
fn test_gen_xml() {
	let xmls = read_fixture("XMLSchema.xsd").unwrap();
	let ts = xmls.codegen();
	println!("{}", ts.to_string());
}
#[test]
fn test_read_gpx() {
	read_fixture("gpx.xsd").unwrap();
}
#[test]
fn test_read_kml() {
	pretty_env_logger::init();
	let s = read_fixture("kml22.xsd").unwrap();
	dbg!(s);
}
#[test]
fn test_gen_kml() {
	pretty_env_logger::init();
	let kml = read_fixture("kml23.xsd").unwrap();
	let ts = kml.codegen();
	eprintln!("{}", ts.to_string());
}
#[test]
fn test_gen_gpx() {
	let gpx = read_fixture("gpx.xsd").unwrap();
	let ts = gpx.codegen();
	eprintln!("{}", ts.to_string());
}

#[test]
fn test_gen_basic() {
	let gpx = read_fixture("XMLSchema-datatypes.xsd").unwrap();
	let ts = gpx.codegen();
	eprintln!("{}", ts.to_string());
}

#[test]
fn test_gen_xsd1() {
	let gpx = read_fixture("XMLSchema1.xsd").unwrap();
	let ts = gpx.codegen();
	eprintln!("{}", ts.to_string());
}

fn read_fixture(filename: &str) -> Result<Schema, Box<dyn Error>> {
	let path = format!("./tests/fixtures/{}", filename);
	let reader = BufReader::new(File::open(path)?);
	let schema: Schema = from_reader(reader)?;

	Ok(schema)
}
