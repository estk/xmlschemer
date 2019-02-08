use heck::CamelCase;
use if_chain::if_chain;
use log::{debug, trace};
use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use serde_derive::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;
use std::fmt::{self, Display};
use syn::{self, Ident};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum AttributeForm {
	Qualified,
	Unqualified,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum Block {
	#[serde(rename = "#all")]
	All,
	Extension,
	Restriction,
	Substitution,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct URI(String);

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
pub struct ID(String);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum Final {
	#[serde(rename = "#all")]
	All,
	Extension,
	Restriction,
	List,
	Union,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
pub struct QName(String);

/// This corresponds to a field of a struct
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Element {
	id: Option<String>,
	name: Option<String>,
	default: Option<String>,
	r#final: Option<Final>,
	r#abstract: Option<bool>,
	r#type: Option<QName>,
	substitution_group: Option<QName>,
	min_occurs: Option<u32>,
	#[serde(default = Some(MaxOccurs::Bounded(1)))]
	max_occurs: Option<MaxOccurs>,
	r#ref: Option<String>,

	#[serde(rename = "$value")]
	body: Option<Vec<ElementBody>>,
}

impl Element {
	fn get_doc(&self) -> Option<String> {
		if let Some(body) = self.body.as_ref() {
			for e in body {
				if let ElementBody::Annotation(a) = e {
					return a.get_doc();
				}
			}
		}
		None
	}
}

impl CodeGenerator for Element {
	fn codegen(&self, _ctx: &mut Context) -> GenResult {
		let name = resolve_type_str(&self.name.as_ref().unwrap());
		let docs = format_doc_block(self.get_doc());
		let doc = quote!(
			#docs
			#[derive(Serialize, Deserialize, Debug)]
			#[serde(transparent)]
		);

		// element is of a remote type
		let def = if let Some(t) = self.r#type.as_ref() {
			let typ = resolve_type(&t.0);
			quote!(
				#doc
				pub struct #name(#typ);
			)
		// element's type is self defined
		} else {
			quote!(
				#doc
				pub struct #name {
					#[serde(flatten)]
					other: HashMap<String, String>,
				}
			)
		};
		GenResult::new(name, def)
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum ElementBody {
	Annotation(Annotation),
	ComplexType(ComplexType),
	Key(Key),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Key {
	id: Option<ID>,
	name: Option<String>,
	#[serde(rename = "$value")]
	body: Option<Vec<KeyBody>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum KeyBody {
	Selector { xpath: String },
	Field { xpath: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaxOccurs {
	Bounded(u32),
	Unbounded(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SimpleType {
	id: Option<ID>,
	name: Option<QName>,
	#[serde(rename = "$value")]
	body: Option<Vec<SimpleBody>>,
}

impl SimpleType {
	fn get_doc(&self) -> Option<String> {
		if let Some(body) = self.body.as_ref() {
			for e in body {
				if let SimpleBody::Annotation(a) = e {
					return a.get_doc();
				}
			}
		}
		None
	}
}

impl CodeGenerator for SimpleType {
	fn codegen(&self, _ctx: &mut Context) -> GenResult {
		let name = resolve_type_str(&self.name.as_ref().unwrap().0);
		let docs = format_doc_block(self.get_doc());
		let doc = quote!(
			#docs
			#[derive(Serialize, Deserialize, Debug)]
			#[serde(transparent)]
		);

		let def = if name == "String" {
			quote!(
				#doc
				pub struct #name(std::string::String);
			)
		} else {
			quote!(
				#doc
				pub struct #name(String);
			)
		};
		GenResult::new(name, def)
	}
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum SimpleBody {
	Annotation(Annotation),
	Documentation(Documentation),
	Restriction(Restriction),
	Extension(Extension),
	Union(Union),
	List(List),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct List {
	item_type: Option<QName>,
	#[serde(rename = "$value")]
	body: Option<Vec<SimpleType>>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Union {
	member_types: Option<String>,
	#[serde(rename = "$value")]
	body: Option<Vec<SimpleType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ComplexType {
	name: Option<QName>,
	mixed: Option<bool>,
	r#final: Option<Final>,
	r#abstract: Option<bool>,
	r#type: Option<QName>,
	#[serde(rename = "$value")]
	body: Option<Vec<ComplexBody>>,
}

impl ComplexType {
	fn get_doc(&self) -> Option<String> {
		if let Some(body) = self.body.as_ref() {
			for e in body {
				if let ComplexBody::Annotation(a) = e {
					return a.get_doc();
				}
			}
		}
		None
	}
}
impl CodeGenerator for ComplexType {
	fn codegen(&self, ctx: &mut Context) -> GenResult {
		let name_str = if let Some(n) = ctx.name.as_ref() {
			&n[..]
		} else {
			&self.name.as_ref().unwrap().0[..]
		};
		debug!("Building complex type: {}", name_str);
		let name = resolve_type_str(&name_str);

		let mut attrs = vec![];
		let mut sequence = None;
		let mut complex_content = None;

		for x in self.body.as_ref().unwrap() {
			match x {
				ComplexBody::Attribute(a) => attrs.push(a),
				ComplexBody::Sequence(s) => {
					sequence.replace(s);
				}
				ComplexBody::ComplexContent(c) => {
					complex_content.replace(c);
				}
				_ => (),
			}
		}

		let mut fields = {
			let mut ts = TokenStream::new();
			for a in attrs {
				let GenResult(_, field) = a.codegen(ctx);
				ts.append_all(quote!(
					#field,
				))
			}
			ts
		};
		let body_name = &format!("{}Body", name);

		let def = if let Some(seq) = sequence {
			let mut body_ctx = ctx.with_name(body_name);
			let GenResult(body_type, body) = seq.codegen(&mut body_ctx);

			fields.append_all(quote!(
				#[serde(rename="$value")]
				body: #body_type,
			));
			make_struct(&name, self.get_doc(), fields, body)
		} else if let Some(cc) = complex_content {
			let mut body_ctx = ctx.with_name(body_name);
			let GenResult(body_type, body) = cc.codegen(&mut body_ctx);

			fields.append_all(quote!(
				#[serde(rename="$value")]
				body: #body_type,
			));
			make_struct(&name, self.get_doc(), fields, body)
		} else {
			make_struct(&name, self.get_doc(), fields, TokenStream::new())
		};
		GenResult::new(name, def)
	}
}
fn make_struct(
	name: &proc_macro2::Ident,
	docs: Option<String>,
	fields: TokenStream,
	defs: TokenStream,
) -> TokenStream {
	let doc = format_doc_block(docs);
	quote!(
		#doc
		#[derive(Serialize, Deserialize, Debug)]
		#[serde(rename_all = "camelCase")]
		pub struct #name {
			#fields
		}
		#defs
	)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum ComplexBody {
	All(All),
	Assert(Assert),
	Annotation(Annotation),
	Sequence(Sequence),
	Attribute(Attribute),
	AttributeGroup(AttributeGroup),
	AnyAttribute(AnyAttribute),
	ComplexContent(ComplexContent),
	SimpleContent(SimpleContent),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SimpleContent {
	id: Option<ID>,
	#[serde(rename = "$value")]
	body: Option<Vec<SimpleContentBody>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum SimpleContentBody {
	Restriction(Restriction),
	Extension(Extension),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ComplexContent {
	#[serde(rename = "$value")]
	body: Vec<ComplexContentBody>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum ComplexContentBody {
	Annotation(Annotation),
	Restriction(Restriction),
	Extension(Extension),
}

impl ComplexContent {}
impl CodeGenerator for ComplexContent {
	fn codegen(&self, ctx: &mut Context) -> GenResult {
		let name_str = ctx.name.as_ref().unwrap();
		let name = Ident::new(name_str, Span::call_site());
		let mut doc = quote!(
			#[derive(Serialize, Deserialize, Debug)]
		);
		let mut defs = TokenStream::new();
		for x in &self.body {
			match x {
				ComplexContentBody::Annotation(a) => {
					doc.append_all(a.get_doc());
				}
				ComplexContentBody::Extension(ref r) => {
					let base = &r.base.0;
					let base_ty = resolve_type(base);
					let mut seq = None;
					let mut attrs = TokenStream::new();
					for x in &r.body {
						match x {
							ExtensionBody::Sequence(s) => {
								seq.replace(s);
							}
							ExtensionBody::Attribute(a) => {
								let mut cc = ctx.clone();
								let GenResult(_, attr) = a.codegen(&mut cc);
								attrs.append_all(quote!(
									#attr,
								));
							}
							_ => panic!("unhandled extension body element {:?}", x),
						}
					}
					let body_name = &format!("{}Extension", name);
					let mut body_ctx = ctx.with_name(body_name);
					let GenResult(body_type, body) = seq.unwrap().codegen(&mut body_ctx);

					debug!("made seq body: {}", body);
					debug!("made seq defs: {}", defs);
					defs.append_all(quote!(
						#doc
						pub struct #name {
							base: #base_ty,
							body: #body_type,
							#attrs
						}
						#body
					));
				}
				ComplexContentBody::Restriction(_) => panic!("unhandled extension body element"),
			}
		}
		GenResult::new(name, defs)
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Extension {
	base: QName,
	#[serde(rename = "$value")]
	body: Vec<ExtensionBody>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum ExtensionBody {
	All(All),
	Assert(Assert),
	Group(Group),
	Attribute(Attribute),
	AnyAttribute(AnyAttribute),
	AttributeGroup(AttributeGroup),
	Sequence(Sequence),
	Choice(Choice),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Restriction {
	base: Option<String>,
	#[serde(rename = "$value")]
	body: Option<Vec<RestrictionBody>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum RestrictionBody {
	Pattern(Pattern),
	Length(Pattern),
	Annotation(Annotation),
	WhiteSpace(WhiteSpace),
	SimpleType(SimpleType),
	AnyAttribute(AnyAttribute),
	MinInclusive {
		value: f32,
	},
	MaxInclusive {
		value: f32,
	},
	MinExclusive {
		value: f32,
	},
	MaxExclusive {
		value: f32,
	},
	MinLength {
		value: f32,
	},
	MaxLength {
		value: f32,
	},
	FractionDigits {
		id: Option<ID>,
		value: u32,
		fixed: Option<bool>,
	},
	Enumeration(Enumeration),
	Sequence(Sequence),
	Attribute(Attribute),
	Group(Group),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Length {
	id: Option<ID>,
	value: String,

	#[serde(rename = "$value")]
	body: Option<Annotation>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct WhiteSpace {
	id: Option<ID>,
	fixed: Option<bool>,
	value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Pattern {
	id: Option<ID>,
	value: String,

	#[serde(rename = "$value")]
	body: Option<Annotation>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Enumeration {
	value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AnyAttribute {
	namespace: Option<String>,
	process_contents: Option<String>,

	#[serde(rename = "$value")]
	body: Option<Vec<Element>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Attribute {
	name: Option<String>,
	r#type: Option<QName>,
	r#use: Option<String>,
	r#ref: Option<String>,
	default: Option<String>,
	fixed: Option<String>,

	#[serde(rename = "$value")]
	body: Option<Vec<AttributeBody>>,
}
impl Attribute {
	fn get_doc(&self) -> Option<String> {
		if let Some(es) = self.body.as_ref() {
			for e in es {
				if let AttributeBody::Annotation(a) = e {
					return a.get_doc();
				}
			}
		}
		None
	}
}
impl CodeGenerator for Attribute {
	fn codegen(&self, _ctx: &mut Context) -> GenResult {
		let name = self.name.as_ref().unwrap().clone();
		let name_id = if name == "type" {
			syn::parse_str("r#type").unwrap()
		} else {
			Ident::new(&name, Span::call_site())
		};
		let ty = resolve_type(&self.r#type.as_ref().unwrap().0);
		let doc_ts = format_doc_block(self.get_doc());
		let def = quote!(
			#doc_ts
			#name_id: #ty
		);
		GenResult::new(name_id, def)
	}
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum AttributeBody {
	Annotation(Annotation),
	SimpleType(SimpleType),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Any {
	namespace: Option<String>,
	process_contents: Option<String>,
	min_occurs: Option<u32>,
	#[serde(default = Some(MaxOccurs::Bounded(1)))]
	max_occurs: Option<MaxOccurs>,

	#[serde(rename = "$value")]
	body: Option<Vec<Annotation>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Sequence {
	min_occurs: Option<u32>,
	#[serde(default = Some(MaxOccurs::Bounded(1)))]
	max_occurs: Option<MaxOccurs>,

	#[serde(rename = "$value")]
	body: Option<Vec<SequenceBody>>,
}

impl CodeGenerator for Sequence {
	fn codegen(&self, ctx: &mut Context) -> GenResult {
		let name = resolve_type_str(&ctx.name.as_ref().unwrap());

		// TODO: move into element codegen?
		let variants: Vec<TokenStream> = self
			.body
			.as_ref()
			.unwrap()
			.iter()
			.filter_map(|e| match e {
				SequenceBody::Element(e) => Some(e),
				_ => None,
			})
			.map(|e| {
				let (name, ty): (String, String) = if let Some(rf) = e.r#ref.as_ref() {
					(rf.clone(), rf.clone())
				} else if let Some(nm) = e.name.as_ref() {
					let t = e.r#type.as_ref().unwrap().0.clone();
					(nm.clone(), t)
				} else if let Some(ty) = e.substitution_group.as_ref() {
					(e.name.as_ref().unwrap().clone(), ty.0.clone())
				} else {
					panic!("element should have either a name, type pair or a ref")
				};
				let name = resolve_type_str(&name);
				let ty_name = resolve_type(&ty);
				quote!(
					#name(#ty_name)
				)
			})
			.collect();

		let mut variant_stream = TokenStream::new();
		for v in variants {
			variant_stream.append_all(quote!(
				#v,
			));
		}
		let doc = quote!(
			#[derive(Serialize, Deserialize, Debug)]
			#[serde(rename_all = "camelCase")]
		);

		let companion_type = quote!(
			#doc
			pub enum #name {
				#variant_stream
			}
		);
		GenResult::new(name, companion_type)
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum SequenceBody {
	Any(Any),
	Annotation(Annotation),
	Element(Element),
	Group(Group),
	Sequence(Sequence),
	Choice(Choice),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Import {
	namespace: URI,
	schema_location: URI,
	#[serde(rename = "$value")]
	body: Option<Vec<Annotation>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Notation {
	name: String,
	public: String,
	system: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AttributeGroup {
	name: Option<String>,
	r#ref: Option<String>,
	#[serde(rename = "$value")]
	body: Option<Vec<AttributeGroupBody>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum AttributeGroupBody {
	Annotation(Annotation),
	Attribute(Attribute),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Group {
	name: Option<String>,
	r#ref: Option<String>,
	min_occurs: Option<u32>,
	#[serde(default = Some(MaxOccurs::Bounded(1)))]
	max_occurs: Option<MaxOccurs>,
	#[serde(rename = "$value")]
	body: Option<Vec<GroupBody>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum GroupBody {
	All(All),
	Annotation(Annotation),
	Assert(Assert),
	Choice(Choice),
	Sequence(Sequence),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Assert {
	id: Option<ID>,
	test: Option<String>,
	#[serde(rename = "$value")]
	body: Option<Annotation>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct All {
	id: Option<ID>,
	max_occurs: Option<u32>,
	#[serde(rename = "$value")]
	body: Option<Vec<AllBody>>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum AllBody {
	Element(Element),
	Any(Any),
	Group(Group),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Choice {
	min_occurs: Option<u32>,
	#[serde(default = Some(MaxOccurs::Bounded(1)))]
	max_occurs: Option<MaxOccurs>,
	#[serde(rename = "$value")]
	body: Option<Vec<ChoiceBody>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum ChoiceBody {
	Any(Any),
	Annotation(Annotation),
	Element(Element),
	Group(Group),
	Sequence(Sequence),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Documentation {
	source: Option<String>,
	#[serde(rename = "$value")]
	body: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AnnotationBody {
	#[serde(rename = "appinfo")]
	AppInfo(String),
	Documentation(Documentation),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
	namespace: Option<String>,
	#[serde(rename = "$value")]
	body: Option<Vec<AnnotationBody>>,
}
impl Annotation {
	fn get_doc(&self) -> Option<String> {
		if let Some(es) = self.body.as_ref() {
			for e in es {
				if let AnnotationBody::Documentation(doc) = e {
					return doc.body.clone();
				}
			}
		}
		None
	}
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DefaultOpenContentt {
	mode: Option<OpenContentMode>,
	id: Option<ID>,
	applies_to_empty: Option<bool>,
	#[serde(rename = "$value")]
	body: Option<Vec<Any>>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum OpenContentMode {
	Interleave,
	Suffix,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum SchemaBody {
	Include,
	Import(Import),
	Redefine,
	Override,
	Annotation(Annotation),
	DefaultOpenContent(DefaultOpenContentt),
	SimpleType(SimpleType),
	ComplexType(ComplexType),
	Group(Group),
	AttributeGroup(AttributeGroup),
	// boxed to reduce enum size
	Element(Box<Element>),
	Attribute,
	Notation(Notation),
}
impl CodeGenerator for SchemaBody {
	fn codegen(&self, ctx: &mut Context) -> GenResult {
		match self {
			Self::Include => GenResult::default(),
			Self::Import(_) => GenResult::default(),
			Self::Redefine => GenResult::default(),
			Self::Override => GenResult::default(),
			Self::Annotation(_) => GenResult::default(),
			Self::DefaultOpenContent(_) => GenResult::default(),
			Self::SimpleType(i) => i.codegen(ctx),
			Self::ComplexType(i) => i.codegen(ctx),
			Self::Group(_) => GenResult::default(),
			Self::AttributeGroup(_) => GenResult::default(),
			Self::Element(i) => i.codegen(ctx),
			Self::Attribute => GenResult::default(),
			Self::Notation(_) => GenResult::default(),
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Schema {
	// default unqualified
	attribute_form_default: Option<AttributeForm>,
	// default empty
	// could be #all, which would block all
	block_default: Option<Block>,
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
	min_version: Option<String>,
	// TODO
	#[serde(rename = "lang")]
	xml_lang: Option<String>,

	// #[serde(flatten)]
	// other: HashMap<String, String>,
	#[serde(rename = "$value")]
	body: Option<Vec<SchemaBody>>,
}
#[derive(Clone)]
pub struct Context {
	name: Option<String>,
	defs: TokenStream,
	nss: HashMap<String, String>,
}
impl Context {
	pub fn with_name(&self, name: &str) -> Self {
		let mut me = self.clone();
		me.name.replace(name.to_string());
		me
	}
	pub fn add_ns(&mut self, ns: &str, v: &str) {
		self.nss.insert(ns.to_string(), v.to_string());
	}
}

impl Default for Context {
	fn default() -> Self {
		Context {
			name: None,
			defs: TokenStream::new(),
			nss: HashMap::new(),
		}
	}
}

impl CodeGenerator for Schema {
	fn codegen(&self, ctx: &mut Context) -> GenResult {
		let mut root = None;
		let mut elements = vec![];
		let mut simple_types = vec![];
		let mut complex_types = vec![];
		if let Some(body) = self.body.as_ref() {
			for item in body {
				match item {
					SchemaBody::Element(x) => {
						if root.is_some() {
							elements.push(x);
						} else {
							root = Some(x);
						}
					}
					SchemaBody::ComplexType(x) => complex_types.push(x),
					SchemaBody::SimpleType(x) => simple_types.push(x),
					_ => (),
				}
			}
		}
		let mut root_ts = TokenStream::new();
		let mut defs = TokenStream::new();

		let root_name_str = root
			.map(|r| r.name.clone().unwrap())
			.unwrap_or(String::from("unnamed"));

		let (root_tn, root_doc) = if let Some(r) = root {
			let mut doc_ts = TokenStream::new();
			let doc = format_doc_block(r.get_doc());

			doc_ts.append_all(quote!(
				#doc
				#[serde(rename = #root_name_str)]
			));

			if let Some(name) = r.r#type.as_ref() {
				(name.0.clone(), doc_ts)
			} else {
				for x in r.body.as_ref().unwrap() {
					if let ElementBody::ComplexType(t) = x {
						debug!("building with root name: {}", root_name_str);
						defs.append_all(t.codegen(&mut ctx.with_name(&root_name_str)).1);
					}
				}
				(root_name_str.to_string(), doc_ts)
			}
		} else {
			("".to_string(), TokenStream::new())
		};

		for t in simple_types {
			defs.append_all(t.codegen(ctx).1);
		}
		for e in elements {
			defs.append_all(e.codegen(ctx).1)
		}
		for t in complex_types {
			if_chain! {
				if let Some(n) = t.name.as_ref();
				if n.0 == root_tn;
				then {
					root_ts.append_all(root_doc.clone());
					root_ts.append_all(t.codegen(ctx).1);
				} else {
					debug!("genning for {:?}", t);
					defs.append_all(t.codegen(ctx).1);
				}
			};
		}
		GenResult::new(
			Ident::new("schema", Span::call_site()),
			quote!(
				use serde_derive::{Deserialize, Serialize};
				use std::collections::HashMap;
				use chrono::{Duration, DateTime, FixedOffset};

				#root_ts
				#defs
			),
		)
	}
}

#[derive(Debug)]
pub struct GenResult(pub proc_macro2::Ident, pub TokenStream);
impl GenResult {
	pub fn new(name: proc_macro2::Ident, def: TokenStream) -> Self {
		GenResult(name, def)
	}
	// pub fn append_all(&mut self, defs: TokenStream) {
	// 	self.1.append_all(defs);
	// }
	pub fn append_all(&mut self, defs: GenResult) {
		self.1.append_all(defs.1);
	}
}
impl Default for GenResult {
	fn default() -> Self {
		let name = Ident::new("", Span::call_site());
		let def = TokenStream::new();
		GenResult::new(name, def)
	}
}
impl Display for GenResult {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "name: {}, definition: {}", self.0, self.1)
	}
}

pub trait CodeGenerator {
	fn codegen(&self, ctx: &mut Context) -> GenResult;
}

fn format_doc_block(doc: Option<String>) -> TokenStream {
	if let Some(bod) = doc {
		let ts = syn::LitStr::new(&bod, Span::call_site());
		quote!(
			#[doc = #ts]
		)
	} else {
		TokenStream::new()
	}
}

fn resolve_typ_inner(s: &str) -> Vec<String> {
	trace!("resolving {}", s);
	let mut split = s.split(':').map(|e| e.to_string()).collect::<Vec<String>>();
	if let Some(s) = split.first() {
		if split.len() > 1 && s == "kml" {
			split.remove(0);
		}
	}
	if let Some(f) = split.first().as_ref() {
		if &f[..] == "xsd" {
			let last = split.last_mut().unwrap();
			let swap = match &last[..] {
				"anySimpleType" => Some("String"),
				"dateTime" => Some("DateTime<FixedOffset>"),
				"gYear" => Some("i32"),
				"gMonth" => Some("u32"),
				"gDay" => Some("u32"),
				"duration" => Some("Duration"),
				"string" => Some("String"),
				"decimal" => Some("String"),
				"double" => Some("f64"),
				"float" => Some("f32"),
				"boolean" => Some("bool"),
				"int" => Some("i32"),
				"long" => Some("i64"),
				"unsignedInt" => Some("u32"),
				"unsignedLong" => Some("u64"),
				"nonNegativeInteger" => Some("String"),
				"anyURI" => Some("String"),
				_ => None,
			};
			if let Some(s) = swap {
				*last = s.to_string();
				split.remove(0);
				return split;
			}
		}
	}

	if let Some(x) = split.last_mut() {
		*x = if x.chars().next().unwrap().is_uppercase() {
			format!("Upcase{}", x.to_camel_case())
		} else {
			x.to_camel_case()
		};
	}
	split
}

fn resolve_type_str(s: &str) -> syn::Ident {
	let tn = resolve_typ_inner(s).last().unwrap().clone();

	syn::parse_str(&tn).unwrap()
}

fn resolve_type(s: &str) -> syn::TypePath {
	let tp = resolve_typ_inner(s).join("::");

	syn::parse_str(&tp).unwrap()
}
