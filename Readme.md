# XMLSchemer

The big idea here is that xml data formats like gpx kml and event xml-schema are specified in xml-schema and using that schema we can generate a model to interact with those data formats.

## Usage

```bash
cargo run -- -i [Input File] -o [Output rust file]
```

## Status

Still to do:
- [] Create type from naked <element>'s in <schema> body
- [] Load External namespaces
- [] Find an alternative to Prepending "Upcase"
- [] Serialization
- [] Allow selection of struct-vec polymorphism vs vec-enum
- [] Cleanup schema codegen
- [] Review struct visibility
- [] Generate code for simpleType restrictions with a validation lib
- [] change codegen to return a Result

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