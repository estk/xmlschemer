#![feature(unrestricted_attribute_tokens)]
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

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

pub fn read(reader: BufReader<File>) -> Result<(), Box<dyn Error>> {
    let schema: Schema = from_reader(reader)?;

    println!("Schema is: {:#?}", schema);
    Ok(())
}

// default unqualified
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
#[serde(deny_unknown_fields)]
enum AttributeForm {
    Qualified,
    Unqualified,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
#[serde(deny_unknown_fields)]
enum Block {
    // #all
    Extension,
    Restriction,
    Substitution,
}
#[derive(Deserialize, Debug)]
#[serde(transparent)]
struct URI(String);

#[derive(Deserialize, Debug)]
#[serde(transparent)]
struct ID(String);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
#[serde(deny_unknown_fields)]
enum Final {
    // #all
    Extension,
    Restriction,
    List,
    Union,
}
#[derive(Deserialize, Debug)]
#[serde(transparent)]
struct QName(String);

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
struct Element {
    name: String,
    r#type: QName,
    min_occurs: Option<u32>,
    #[serde(default = Some(MaxOccurs::Bounded(1)))]
    max_occurs: Option<MaxOccurs>,
    #[serde(rename = "$value")]
    content: Annotation,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum MaxOccurs {
    Bounded(u32),
    Unbounded,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct SimpleType {
    name: QName,
    #[serde(rename = "$value")]
    content: Option<Vec<SimpleContent>>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum SimpleContent {
    Annotation(Annotation),
    Restriction { base: String },
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct ComplexType {
    name: QName,
    r#type: Option<QName>,
    #[serde(rename = "$value")]
    content: Option<Vec<ComplexContent>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum ComplexContent {
    Annotation(Annotation),
    Sequence(Sequence),
    Attribute(Attribute),
}

#[derive(Deserialize, Debug)]
struct Attribute {
    name: String,
    r#type: String,
    r#use: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum SequenceContent {
    Element(Element),
    Any(Any),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Any {
    min_occurs: Option<u32>,
    #[serde(default = Some(MaxOccurs::Bounded(1)))]
    max_occurs: Option<MaxOccurs>,
}

#[derive(Deserialize, Debug)]
struct Sequence {
    #[serde(rename = "$value")]
    content: Option<Vec<SequenceContent>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum SchemaContent {
    Include,
    Import,
    Redefine,
    Override,
    Annotation(Annotation),
    DefaultOpenContent,
    SimpleType(SimpleType),
    ComplexType(ComplexType),
    Group,
    AttributeGroup,
    Element(Element),
    Attribute,
    Notation,
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
struct Documentation(String);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
#[serde(deny_unknown_fields)]
enum AnnotationContent {
    AppInfo,
    Documentation(Documentation),
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Annotation {
    #[serde(rename = "$value")]
    content: AnnotationContent,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
struct Schema {
    // default unqualified
    attribute_form_default: Option<AttributeForm>,
    // default empty
    // could be #all, which would block all
    block_default: Option<Vec<Block>>,
    // TODO
    default_attributes: Option<String>,
    // default ##local
    // ##defaultNamespace | ##targetNamespace | ##local)
    xpath_default_namespace: Option<URI>,
    // default unqualified
    element_form_default: Option<AttributeForm>,
    // default empty
    final_default: Option<Vec<Final>>,
    id: Option<ID>,
    target_namespace: Option<URI>,
    // TODO
    version: Option<String>,
    // TODO
    xml_lang: Option<String>,

    #[serde(rename = "$value")]
    content: Option<Vec<SchemaContent>>,
}

mod test {
    use crate::*;
    #[test]
    fn test_print_gpx() {
        let reader = BufReader::new(File::open("./fixtures/gpx.xsd").unwrap());
        let res = read(reader);
        println!("{:#?}", res);
    }
}
