use if_chain::if_chain;
use log::debug;
use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use serde_derive::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;
use syn::{self, Ident};

use super::resolution;
use super::*;
use resolution::{format_doc_block, CodeGenerator, Context, GenResult};

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
	fn codegen(&self, ctx: &Context) -> GenResult {
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
	pub fn codegen(&self) -> TokenStream {
		let ctx = self.gen_ctx();
		CodeGenerator::codegen(self, &ctx).1
	}
	fn gen_ctx(&self) -> Context {
		let target = &self.target_namespace.as_ref().unwrap().0;
		// Get the name of the current namespace
		let ns_str = self.xmlns.iter().find(|&(_, ns)| ns == target).unwrap().0;
		let ns = if ns_str.is_empty() {
			None
		} else {
			Some(ns_str.to_string())
		};
		let schema_str = self
			.xmlns
			.iter()
			.find(|&(_, ns)| ns == "http://www.w3.org/2001/XMLSchema")
			.unwrap()
			.0;
		let schema_ns = if schema_str.is_empty() {
			None
		} else {
			Some(schema_str.to_string())
		};

		Context::new(ns, schema_ns, self.xmlns.clone())
	}
}

impl CodeGenerator for Schema {
	fn codegen(&self, ctx: &Context) -> GenResult {
		let mut root = None;
		let mut elements = vec![];
		let mut simple_types = vec![];
		let mut complex_types = vec![];
		if let Some(body) = self.body.as_ref() {
			for item in body {
				match item {
					SchemaBody::Element(x) => {
						if x.name == ctx.ns {
							root = Some(x);
						} else {
							elements.push(x);
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
						defs.append_all(t.codegen(&ctx.with_name(&root_name_str)).1);
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
			println!("codegen for {:?}", e);
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
