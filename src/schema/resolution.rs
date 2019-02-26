use super::*;
use heck::{CamelCase, MixedCase};
use if_chain::if_chain;
use log::trace;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::hash_map::HashMap;
use syn::{self, Ident};

pub trait Identifiable {
	fn ident(&self, ctx: &Context) -> syn::Ident;
}

pub trait Genable {
	fn gen(&self, ctx: &Context) -> TokenStream;
}

#[derive(Debug)]
pub struct Parent<'a> {
	ident: syn::Ident,
	base: &'a ComplexType,
	children: Vec<Ident>,
}

impl<'a> Parent<'a> {
	pub fn new_child(&mut self, id: Ident) {
		self.children.push(id);
	}
	pub fn new(ident: Ident, base: &'a ComplexType, children: Vec<Ident>) -> Parent {
		Parent {
			ident,
			base,
			children,
		}
	}
}
impl<'a> Identifiable for Parent<'a> {
	fn ident(&self, _ctx: &Context) -> syn::Ident {
		self.ident.clone()
	}
}
impl<'a> Genable for Parent<'a> {
	fn gen(&self, ctx: &Context) -> TokenStream {
		debug!("Genning parent: {:?}", &self);
		let ident = self.ident(ctx);
		let variants: TokenStream = self.children.iter().map(|n| quote!( #n(#n), )).collect();
		quote!(
			enum #ident{
				#variants
			}

		)
	}
}

pub struct ParseResult<'s> {
	pub elements: HashMap<Ident, &'s Element>,
	pub complex_types: HashMap<Ident, &'s ComplexType>,
	pub simple_types: HashMap<Ident, &'s SimpleType>,
	pub parents: HashMap<Ident, Parent<'s>>,
}

impl<'a> ParseResult<'a> {
	pub fn new() -> Self {
		ParseResult {
			elements: HashMap::new(),
			complex_types: HashMap::new(),
			simple_types: HashMap::new(),
			parents: HashMap::new(),
		}
	}
}

#[derive(Clone)]
pub struct Context<'a> {
	pub name: Option<String>,
	pub ns: Option<String>,
	schema_ns: Option<String>,
	nss: HashMap<String, String>,
	parse_result: Option<&'a ParseResult<'a>>,
}
impl<'a> Context<'a> {
	pub fn new(
		ns: Option<String>,
		schema_ns: Option<String>,
		nss: HashMap<String, String>,
	) -> Self {
		Context {
			name: None,
			ns,
			schema_ns,
			nss,
			parse_result: None,
		}
	}
	pub fn set_pr(&mut self, pr: &'a ParseResult<'a>) {
		self.parse_result.replace(pr);
	}
	pub fn with_name(&self, name: &str) -> Self {
		let mut me = self.clone();
		me.name.replace(name.to_string());
		me
	}
	pub fn add_ns(&mut self, ns: &str, v: &str) {
		self.nss.insert(ns.to_string(), v.to_string());
	}

	fn resolve_ident_str(&self, s: &str) -> String {
		let tn = if s.chars().next().unwrap().is_uppercase() && !s.starts_with("Upcase") {
			format!("Upcase{}", s.to_camel_case())
		} else {
			s.to_camel_case()
		};
		if tn == "type" {
			"r#type".to_string()
		} else {
			tn
		}
	}
	pub fn resolve_ident(&self, s: &str) -> syn::Ident {
		syn::parse_str(&self.resolve_ident_str(s)).unwrap()
	}
	pub fn mk_field(&self, s: &str) -> syn::Ident {
		let tn = &s.to_mixed_case();
		if tn == "type" {
			syn::parse_str("r#type").unwrap()
		} else {
			syn::parse_str(&tn).unwrap()
		}
	}
	pub fn mk_type(&self, s: &str) -> syn::Ident {
		let typ = s
			.split(':')
			.map(ToString::to_string)
			.last()
			.expect("make type called with empty string");
		self.resolve_ident(&typ)
	}

	pub fn resolve_type(&self, s: &str) -> syn::TypePath {
		let tp = self.resolve_type_path(s).join("::");

		syn::parse_str(&tp).unwrap()
	}

	fn resolve_type_path(&self, s: &str) -> Vec<String> {
		trace!("resolving type {}", s);
		let mut split = s
			.split(':')
			.map(ToString::to_string)
			.collect::<Vec<String>>();

		assert_ne!(split.len(), 0);
		let this_ns = if split.len() == 1 {
			None
		} else {
			Some(split.first().as_ref().unwrap().to_string())
		};

		if self.in_xml_schema(&this_ns) {
			if !translate_xsd(&mut split) {
				let estr = format!("Unable to translate xsd type: {}", s);
				panic!(estr);
			}
		} else if self.in_local_schema(&this_ns) {
			// If its just a rando type, in our namespace, camel case it
			if let Some(x) = split.last_mut() {
				*x = self.resolve_ident_str(x)
			}
			if split.len() > 1 {
				split.remove(0);
			}
		} else {
			if let Some(x) = split.last_mut() {
				*x = self.resolve_ident_str(x)
			}
		}

		split
	}

	fn in_xml_schema(&self, ns: &Option<String>) -> bool {
		if_chain! {
			if let Some(sns) = &self.schema_ns;
			if let Some(this_ns) = ns;
			then {
				sns == this_ns
			} else {
				self.schema_ns.is_none() && ns.is_none()
			}
		}
	}
	fn in_local_schema(&self, ns: &Option<String>) -> bool {
		if_chain! {
			if let Some(lns) = &self.ns;
			if let Some(this_ns) = ns;
			then {
				lns == this_ns
			} else {
				self.ns.is_none() && ns.is_none()
			}
		}
	}
}

impl<'a> Default for Context<'a> {
	fn default() -> Self {
		Context {
			name: None,
			ns: None,
			schema_ns: None,
			nss: HashMap::new(),
			parse_result: None,
		}
	}
}

pub fn fmt_doc(doc: Option<String>) -> TokenStream {
	if let Some(bod) = doc {
		let ts = syn::LitStr::new(&bod, Span::call_site());
		quote!(
			#[doc = #ts]
		)
	} else {
		TokenStream::new()
	}
}

pub fn make_struct(
	name: &proc_macro2::Ident,
	docs: Option<String>,
	fields: TokenStream,
	defs: TokenStream,
) -> TokenStream {
	trace!("making struct {:?}", name);
	let doc = fmt_doc(docs);
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

// For xsd elements translate them into primitives.
// Returns true if we were able to translate
fn translate_xsd(split: &mut Vec<String>) -> bool {
	trace!("translating {:?}", split);
	let last = split.last_mut().unwrap();

	let swap = match &last[..] {
		"anySimpleType" => Some("String"),
		"dateTime" => Some("DateTime<FixedOffset>"),
		"gYear" => Some("i32"),
		"gMonth" => Some("u32"),
		"gDay" => Some("u32"),
		"duration" => Some("Duration"),
		"ID" => Some("String"),
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
		if split.len() > 1 {
			split.remove(0);
		}
		trace!("translated to {:?}", &split);
		true
	} else {
		trace!("translation failed");
		false
	}
}
