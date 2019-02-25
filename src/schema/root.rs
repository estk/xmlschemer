use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use serde_derive::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;
use syn::{self, Ident};

use super::resolution;
use super::*;
use resolution::{Context, Genable};

const XML_SCHEMA_NS: &str = "http://www.w3.org/2001/XMLSchema";

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
impl Genable for SchemaBody {
	fn gen(&self, ctx: &Context) -> TokenStream {
		match self {
			Self::Include => TokenStream::new(),
			Self::Import(_) => TokenStream::new(),
			Self::Redefine => TokenStream::new(),
			Self::Override => TokenStream::new(),
			Self::Annotation(_) => TokenStream::new(),
			Self::DefaultOpenContent(_) => TokenStream::new(),
			Self::SimpleType(i) => i.gen(ctx),
			Self::ComplexType(i) => i.gen(ctx),
			Self::Group(_) => TokenStream::new(),
			Self::AttributeGroup(_) => TokenStream::new(),
			Self::Element(i) => i.gen(ctx),
			Self::Attribute => TokenStream::new(),
			Self::Notation(_) => TokenStream::new(),
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Schema {
	// Collect all the namespaces
	#[serde(rename = "$xmlns")]
	pub xmlns: HashMap<String, String>,
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
	// The namespace for the declarations herein
	pub target_namespace: Option<URI>,
	// TODO
	version: Option<String>,
	// TODO
	min_version: Option<String>,
	// TODO
	#[serde(rename = "lang")]
	xml_lang: Option<String>,

	#[serde(rename = "$value")]
	body: Option<Vec<SchemaBody>>,
}

impl Schema {
	pub fn gen(&self) -> TokenStream {
		Genable::gen(self, &self.gen_ctx())
	}
	fn gen_ctx(&self) -> Context {
		let target_ns = self.target_ns();
		let schema_ns = self.schema_ns();

		Context::new(target_ns, schema_ns, self.xmlns.clone())
	}
	fn target_ns(&self) -> Option<String> {
		let target = &self
			.target_namespace
			.as_ref()
			.expect("Target namespace unspecified")
			.0;
		let ns_str = self
			.xmlns
			.iter()
			.find(|&(_, ns)| ns == target)
			.expect("Target namespace not found in namespaces")
			.0
			.clone();

		if ns_str.is_empty() {
			None
		} else {
			Some(ns_str)
		}
	}
	fn schema_ns(&self) -> Option<String> {
		let schema_str = self
			.xmlns
			.iter()
			.find(|&(_, ns)| ns == XML_SCHEMA_NS)
			.unwrap()
			.0;

		if schema_str.is_empty() {
			None
		} else {
			Some(schema_str.to_string())
		}
	}
	fn parse_pass(&self, ctx: &Context) -> ParseResult {
		let mut pr = ParseResult::new();
		let body = self.body.as_ref().expect("Schema has no body");

		for item in body {
			match item {
				SchemaBody::Element(x) => pr.add_element(x.ident(ctx), &*x),
				SchemaBody::ComplexType(x) => pr.add_complex_type(x.ident(ctx), x),
				SchemaBody::SimpleType(x) => pr.add_simple_type(x.ident(ctx), x),
				_ => (),
			}
		}
		pr
	}
	fn gen_pass(&self, ctx: &Context, pr: ParseResult) -> TokenStream {
		let mut defs = TokenStream::new();
		for (_, v) in pr.elements.iter() {
			defs.append_all(v.gen(ctx))
		}
		for (_, v) in pr.complex_types.iter() {
			defs.append_all(v.gen(ctx))
		}
		for (_, v) in pr.simple_types.iter() {
			defs.append_all(v.gen(ctx))
		}

		quote!(
			use serde_derive::{Deserialize, Serialize};
			use std::collections::HashMap;
			use chrono::{Duration, DateTime, FixedOffset};

			#defs
		)
	}
}
pub struct ParseResult<'s> {
	elements: HashMap<Ident, &'s Element>,
	complex_types: HashMap<Ident, &'s ComplexType>,
	simple_types: HashMap<Ident, &'s SimpleType>,
}

impl<'a> ParseResult<'a> {
	pub fn new() -> Self {
		ParseResult {
			elements: HashMap::new(),
			complex_types: HashMap::new(),
			simple_types: HashMap::new(),
		}
	}
	pub fn add_element(&mut self, ident: syn::Ident, x: &'a Element) {
		self.elements.insert(ident, x);
	}
	pub fn add_complex_type(&mut self, ident: syn::Ident, x: &'a ComplexType) {
		self.complex_types.insert(ident, x);
	}
	pub fn add_simple_type(&mut self, ident: syn::Ident, x: &'a SimpleType) {
		self.simple_types.insert(ident, x);
	}
}

impl Genable for Schema {
	fn gen(&self, ctx: &Context) -> TokenStream {
		let parsed = self.parse_pass(ctx);
		self.gen_pass(ctx, parsed)
	}
}
