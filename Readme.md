# XMLSchemer

The big idea here is that xml data formats like gpx kml and event xml-schema are specified in xml-schema and using that schema we can generate a model to interact with those data formats.

## Usage

```bash
cargo run -- -i [Input File] -o [Output rust file]
```

## Status
We can deserialize gpx!

Still to do:
- [] Gen for atom.xsd
	```
	RUST_LOG=serde-xml-rs=trace RUST_BACKTRACE=1 cargo run -- -i tests/fixtures/atom-oasis.xsd -o ./tests/gen/atom.rs
	```
- [] Create type from naked <element>'s in <schema> body
- [] Load External namespaces
- [] Find an alternative to Prepending "Upcase"
- [] Serialization
- [] Generate code for simpleType restrictions with a validation lib
- [] Allow selection of struct-vec polymorphism vs vec-enum
- [] Cleanup schema codegen
- [] Move resolution stuff into its own module

Done:
- [x] Fix double upcase
- [x] Handle xsd:ID
- [x] Handle refs correctly: 
- [x] Split codegen return value into (defs: Ident, type: TokenStream)
- [x] Generate code for <xs:extension>
- [x] Unsure if its a requirement but parse namespaces?
- [x] Lookup namespaces and resolve types properly