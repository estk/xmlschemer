# XMLSchemer

The big idea here is that xml data formats like gpx kml and event xml-schema are specified in xml-schema and using that schema we can generate a model to interact with those data formats.

## Usage

Basic usage:
```bash
cargo run -- -i [Input File] -o [Output rust file]
```

To generate code for the fixtures:
```bash
cargo run -- -i tests/fixtures/gpx.xsd -o ./tests/gen/gpx.rs
```

### Debug logging

This project uses the pretty_env_logger crate, logging could be done for example by:
```bash
RUST_LOG=xmlschemer=trace cargo run -- -i tests/fixtures/gpx.xsd -o ./tests/gen/gpx.rs
```

## Status

Currently the big limitations of this project are that serialization is waiting on [serde-xml-rs](https://github.com/RReverser/serde-xml-rs/pull/36), and that types defined using extensions of abstract types do not work. The failing tests
reflect that these two features are not working yet.

The plan is to work on type inheritance then to proceed to serialization. Additionally this project uses a fork
of the serde-xml-rs project that allows namespaces to be parsed out of xml, I'm working on getting that merged
upstream [here](https://github.com/RReverser/serde-xml-rs/pull/95).

Still to do:
- [ ] Type hirearchy
- [ ] Handle Abstract types
- [ ] Load External namespaces
- [ ] Find an alternative to Prepending "Upcase"
- [ ] Serialization
- [ ] Allow selection of struct-vec polymorphism vs vec-enum
- [ ] Cleanup schema codegen
- [ ] Review struct visibility
- [ ] Generate code for simpleType restrictions with a validation lib
- [ ] change codegen to return a Result

Done:
- [x] Move resolution stuff into its own module
- [x] Review logging
- [x] Gen for atom.xsd
- [x] Fix double upcase
- [x] Handle xsd:ID
- [x] Handle refs correctly: 
- [x] Split codegen return value into (defs: Ident, type: TokenStream)
- [x] Generate code for <xs:extension>
- [x] Unsure if its a requirement but parse namespaces?
- [x] Lookup namespaces and resolve types properly