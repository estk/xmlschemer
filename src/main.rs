#![feature(unrestricted_attribute_tokens)]
#![feature(type_alias_enum_variants)]
#![feature(custom_attribute)]
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate quote;

use clap::{app_from_crate, Arg};
use log::debug;
use serde_xml_rs::from_reader;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("in")
                .short("i")
                .long("in")
                .help("Location of the input file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .help("Location of the output file")
                .takes_value(true),
        )
        .get_matches();

    let in_filename = matches.value_of("in").expect("Input file is required");
    debug!("opening input file: {}", in_filename);
    let in_file = File::open(in_filename).expect("Unable to open in file");

    let out_filename = matches.value_of("out").expect("Output file is required");
    debug!("opening output file: {}", out_filename);
    let out_file = File::open(out_filename).expect("Unable to open out file");

    // let in_reader = BufReader::new(in_file);
    // let data = read(in_reader)?;
    //
    // let out_writer = BufWriter::new(out_file);
    // out_writer.write_all(data.as_rust());
}
