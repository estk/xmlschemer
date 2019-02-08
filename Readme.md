# XMLSchemer

The big idea here is that xml data formats like gpx kml and event xml-schema are specified in xml-schema and using that schema we can generate a model to interact with those data formats.

## Usage

```bash
cargo run -- -i [Input File] -o [Output rust file]
```

## Status
We can deserialize gpx!

Still to do:
- [] Split codegen return value into (defs: TokenStream, type: TokenStream)
- [] Deserialize kml
    - [x] Generate code for <xs:extension>
    - [] Unsure if its a requirement but parse namespaces?
- [] Generate code for simpleType restrictions with a validation lib
- [] Serialization
- [] Allow selection of struct-vec polymorphism vs vec-enum