use heck::{CamelCase, MixedCase};
use if_chain::if_chain;
use log::trace;
use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use std::collections::hash_map::HashMap;
use std::fmt::{self, Display};
use syn::{self, Ident};

#[derive(Debug)]
pub struct GenResult(pub proc_macro2::Ident, pub TokenStream);
impl GenResult {
	pub fn new(name: proc_macro2::Ident, def: TokenStream) -> Self {
		GenResult(name, def)
	}
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
	fn codegen(&self, ctx: &Context) -> GenResult;
}

#[derive(Clone)]
pub struct Context {
	pub name: Option<String>,
	pub ns: Option<String>,
	schema_ns: Option<String>,
	nss: HashMap<String, String>,
}
impl Context {
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
		}
	}
	pub fn with_name(&self, name: &str) -> Self {
		let mut me = self.clone();
		me.name.replace(name.to_string());
		me
	}
	pub fn add_ns(&mut self, ns: &str, v: &str) {
		self.nss.insert(ns.to_string(), v.to_string());
	}
	pub fn resolve_ident(&self, s: &str) -> syn::Ident {
		let tn = self.resolve_type_path(s).last().unwrap().clone();
		if tn == "type" {
			syn::parse_str("r#type").unwrap()
		} else {
			syn::parse_str(&tn).unwrap()
		}
	}

	pub fn resolve_type(&self, s: &str) -> syn::TypePath {
		let tp = self.resolve_type_path(s).join("::");

		syn::parse_str(&tp).unwrap()
	}

	pub fn mk_field(&self, s: &str) -> syn::Ident {
		let typ = s
			.split(':')
			.map(ToString::to_string)
			.last()
			.expect("make type called with empty string");
		let tn = &typ.to_mixed_case();
		if tn == "type" {
			syn::parse_str("r#type").unwrap()
		} else {
			syn::parse_str(&tn).unwrap()
		}
	}
	pub fn make_type(&self, s: &str) -> syn::Ident {
		let typ = s
			.split(':')
			.map(ToString::to_string)
			.last()
			.expect("make type called with empty string");
		let tn = if typ.chars().next().unwrap().is_uppercase() {
			format!("Upcase{}", typ.to_camel_case())
		} else {
			typ.to_camel_case()
		};
		if tn == "type" {
			syn::parse_str("r#type").unwrap()
		} else {
			syn::parse_str(&tn).unwrap()
		}
	}

	fn resolve_type_path(&self, s: &str) -> Vec<String> {
		trace!("resolving {}", s);
		let mut split = s.split(':').map(|e| e.to_string()).collect::<Vec<String>>();
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
				*x = if x.chars().next().unwrap().is_uppercase() {
					format!("Upcase{}", x.to_camel_case())
				} else {
					x.to_camel_case()
				};
			}
			if split.len() > 1 {
				split.remove(0);
			}
		} else {
			if let Some(x) = split.last_mut() {
				*x = if x.chars().next().unwrap().is_uppercase() {
					format!("Upcase{}", x.to_camel_case())
				} else {
					x.to_camel_case()
				};
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

impl Default for Context {
	fn default() -> Self {
		Context {
			name: None,
			ns: None,
			schema_ns: None,
			nss: HashMap::new(),
		}
	}
}

pub fn format_doc_block(doc: Option<String>) -> TokenStream {
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
