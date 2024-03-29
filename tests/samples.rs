use serde::de::DeserializeOwned;
use serde_xml_rs::from_reader;
use std::fs::File;
use std::io::BufReader;

mod kml {
	use super::read_sample;
	include!("./gen/kml22.rs");
	mod xal {
		// TODO: use this once it is correctly generating
		// include!("./gen/xal.rs");
		use serde_derive::{Deserialize, Serialize};
		#[derive(Serialize, Deserialize, Debug)]
		pub struct UpcaseAddressDetails {}
	}
	mod atom {
		include!("./gen/atom.rs");
	}
	const KML_SAMPLE: &str = "sample.kml";

	#[test]
	fn test_read_kml_sample() {
		let s: Kml = read_sample(KML_SAMPLE).unwrap();
		eprintln!("{:#?}", s);
	}
}

mod gpx {
	use super::read_sample;
	use serde_xml_rs::to_string;

	include!("./gen/gpx.rs");
	const GPX_SAMPLE: &str = "sample.gpx";

	#[test]
	fn test_read_gpx_sample() {
		let gpx: GpxType = read_sample(GPX_SAMPLE).unwrap();
		eprintln!("{:#?}", gpx);
	}

	#[test]
	fn test_rw_gpx_sample() {
		let gpx: GpxType = read_sample(GPX_SAMPLE).unwrap();
		let str_gpx_out = to_string(&gpx).unwrap();
		eprintln!("{:#?}", str_gpx_out);
	}
}

fn read_sample<T>(filename: &str) -> Result<T, serde_xml_rs::Error>
where
	T: DeserializeOwned,
{
	let path = format!("./tests/fixtures/{}", filename);
	let reader = BufReader::new(File::open(path)?);
	from_reader(reader)
}
