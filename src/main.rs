#![feature(unrestricted_attribute_tokens)]
#![feature(type_alias_enum_variants)]
#![feature(custom_attribute)]

#[macro_use]
extern crate clap;

use clap::{app_from_crate, Arg};
use log::debug;
use rustfmt_nightly::{Config, Input, Session};
use serde_xml_rs::from_reader;
use std::error::Error;
use std::fs::File;
use std::io::{sink, BufReader, BufWriter, Write};
use std::path::PathBuf;
use xmlschemer::schema::{CodeGenerator, Context, Schema};

fn main() -> Result<(), Box<dyn Error>> {
	pretty_env_logger::init();
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
	let out_file = File::create(out_filename).expect("Unable to open out file");

	let reader = BufReader::new(in_file);
	let schema: Schema = from_reader(reader)?;
	let res = schema.codegen(&mut Context::default());
	let source_string = res.1.to_string();

	let mut out_writer = BufWriter::new(out_file);
	out_writer.write_all(&source_string.as_bytes())?;
	out_writer.flush()?;
	drop(out_writer);

	let mut output = sink();
	let fmt_res = Session::new(Config::default(), Some(&mut output))
		.format(Input::File(PathBuf::from(out_filename)))
		.map_err(|_| "Error formatting")?;

	println!("result: {}", fmt_res);
	Ok(())
}
