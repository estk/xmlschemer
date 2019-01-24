use std::error::Error;
use crate::schema::Schema;
const SCHEMAS_DIR: &str = "./schemas";
const PRIMITIVE_SCHEMA_PATH: &str = "./schemas/primitives.xsd";

fn main() -> Result<(), Box<dyn Error>> {
    let primitives = gen_primitives();
    Ok(())
}

fn gen_primitives() -> Result<(), Box<dyn Error>> {
    let prim_f = File::open(PRIMITIVE_SCHEMA_PATH)?;
    let reader = BufReader::new(prim_f);
    let schema: Schema = from_reader(reader)?;
    let source = schema.codegen(&mut Context::default());
    let out_file = File::create(out_filename).expect("Unable to open out file");
    let mut out_writer = BufWriter::new(out_file);
    out_writer.write_all(&source_string.to_strin().as_bytes())?;
    out_writer.flush()
}
