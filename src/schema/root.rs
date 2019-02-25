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
		CodeGenerator::codegen(self, &self.gen_ctx()).1
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
	// TODO: unless we need the docs, this is basically useless
	// fn codegen_root(&self, root: Element, ctx: Context) -> TokenStream {
	// 	let root_name_str = ctx.ns.expect("Schema does not have a target ns");

	// 	let doc = format_doc_block(root.get_doc());
	// 	let doc_ts = quote!(
	// 		#doc
	// 		#[serde(rename = #root_name_str)]
	// 	);

	// 	if let Some(name) = root.r#type.as_ref() {
	// 		(name.0.clone(), doc_ts)
	// 	} else {
	// 		for x in r.body.as_ref().unwrap() {
	// 			if let ElementBody::ComplexType(t) = x {
	// 				debug!("building with root name: {}", root_name_str);
	// 				defs.append_all(t.codegen(&ctx.with_name(&root_name_str)).1);
	// 			}
	// 		}
	// 		(root_name_str.to_string(), doc_ts)
	// 	}
	// 	quote!(
	// 		#doc_ts
	// 		#root
	// 	)
	// }
}

impl CodeGenerator for Schema {
	fn codegen(&self, ctx: &Context) -> GenResult {
		// let mut root = None;
		let mut defs = TokenStream::new();
		if let Some(body) = self.body.as_ref() {
			for item in body {
				match item {
					SchemaBody::Element(x) => {
						// if x.name == ctx.ns {
						// 	debug!("Found root: {:?}", x.name);
						// 	root = Some(*(x.clone()));
						// } else {
						defs.append_all(x.codegen(ctx).1);
						// }
					}
					SchemaBody::ComplexType(x) => defs.append_all(x.codegen(ctx).1),
					SchemaBody::SimpleType(x) => defs.append_all(x.codegen(ctx).1),
					_ => (),
				}
			}
		}
		// let root_ts = self.codegen_root(root.expect("Unable to find root element"), ctx.clone());

		GenResult::new(
			Ident::new("schema", Span::call_site()),
			quote!(
				use serde_derive::{Deserialize, Serialize};
				use std::collections::HashMap;
				use chrono::{Duration, DateTime, FixedOffset};

				#defs
			),
		)
	}
}
