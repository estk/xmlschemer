use log::{debug, trace};
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use serde_derive::{Deserialize, Serialize};
use std::cell::Cell;
use std::collections::HashMap;
use syn;

mod resolution;
use resolution::{fmt_doc, make_struct, Context, Genable, Identifiable, Parent};
mod root;
pub use root::Schema;

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum ElementBody {
	Annotation(Annotation),
	ComplexType(ComplexType),
	Key(Key),
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

impl Identifiable for Element {
	fn ident(&self, ctx: &Context) -> syn::Ident {
		ctx.mk_type(&self.name.as_ref().unwrap())
	}
}
impl Genable for Element {
	fn gen(&self, ctx: &Context) -> TokenStream {
		let name = self.ident(ctx);
		trace!("codegen element: {:?}", self.name);
		let docs = fmt_doc(self.get_doc());
		let doc = quote!(
			#docs
			#[derive(Serialize, Deserialize, Debug)]
			#[serde(transparent)]
		);

		// element is a remote type
		if let Some(t) = self.r#type.as_ref() {
			let typ = ctx.resolve_type(&t.0);
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
		}
	}
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

impl Identifiable for SimpleType {
	fn ident(&self, ctx: &Context) -> syn::Ident {
		ctx.mk_type(&self.name.as_ref().unwrap().0)
	}
}
impl Genable for SimpleType {
	fn gen(&self, ctx: &Context) -> TokenStream {
		let name = self.ident(ctx);
		trace!("codegen simpletype: {:?}", self.name);
		let docs = fmt_doc(self.get_doc());
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
		def
	}
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
	#[serde(skip)]
	is_parent: Cell<bool>,
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
	Group(Group),
	ComplexContent(ComplexContent),
	SimpleContent(SimpleContent),
	Choice(Choice),
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
	fn resolve<'a>(
		&self,
		ctx: &Context,
		ct: &HashMap<syn::Ident, &'a ComplexType>,
		ps: &mut HashMap<syn::Ident, Parent<'a>>,
	) {
		let mut complex_content = None;
		for x in self.body.as_ref().unwrap() {
			if let ComplexBody::ComplexContent(c) = x {
				complex_content.replace(c);
			}
		}

		if let Some(cc) = complex_content {
			let base = cc.get_base(ctx);
			let parent = ps.iter_mut().find(|&(name, _)| *name == base);

			if let Some(p) = parent {
				p.1.new_child(self.ident(ctx));
			} else {
				let p = ct
					.iter()
					.find(|&(name, _)| *name == base)
					.expect("parent not found in parents or complextypes")
					.1;

				// Mark the complextype as a parent
				p.is_parent.replace(true);
				let parent =
					Parent::new((&base).to_owned(), p, vec![p.ident(ctx), self.ident(ctx)]);
				ps.insert(base, parent);
			}
		}
	}
}
impl Identifiable for ComplexType {
	fn ident(&self, ctx: &Context) -> syn::Ident {
		let mut name_str = if let Some(n) = ctx.name.as_ref() {
			&n
		} else {
			&self.name.as_ref().unwrap().0[..]
		};
		if self.is_parent.get() {
			return ctx.mk_type(&format!("parent{}", name_str));
		}
		ctx.mk_type(name_str)
	}
}
impl Genable for ComplexType {
	fn gen(&self, ctx: &Context) -> TokenStream {
		let name = self.ident(ctx);
		trace!("gen complextype: {}", name);

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
				// TODO: once gen_field generates defs, we need to output them somewhere here
				let (field, _defs) = a.gen_field(ctx);
				ts.append_all(quote!(
					#field,
				))
			}
			ts
		};
		let body_name = &format!("{}Body", name);
		let body_ctx = ctx.with_name(body_name);

		let def = if let Some(seq) = sequence {
			let (body_type, body) = (seq.ident(&body_ctx), seq.gen(&body_ctx));

			fields.append_all(quote!(
				#[serde(rename="$value")]
				body: Vec<#body_type>,
			));
			make_struct(&name, self.get_doc(), fields, body)
		} else if let Some(cc) = complex_content {
			let (body_type, body) = (cc.ident(&body_ctx), cc.gen(&body_ctx));

			fields.append_all(quote!(
				#[serde(rename="$value")]
				body: #body_type,
			));
			make_struct(&name, self.get_doc(), fields, body)
		} else {
			make_struct(&name, self.get_doc(), fields, TokenStream::new())
		};
		def
	}
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

impl ComplexContent {
	pub fn get_base(&self, ctx: &Context) -> syn::Ident {
		for x in &self.body {
			if let ComplexContentBody::Extension(ref r) = x {
				let base = &r.base.0;
				return ctx.mk_type(base);
			}
		}
		panic!("no base found for complex content")
	}
}

impl Identifiable for ComplexContent {
	fn ident(&self, ctx: &Context) -> syn::Ident {
		let name_str = ctx.name.as_ref().unwrap();
		ctx.mk_type(name_str)
	}
}
impl Genable for ComplexContent {
	fn gen(&self, ctx: &Context) -> TokenStream {
		let name = self.ident(ctx);
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
					let base_ty = ctx.resolve_type(base);
					let mut seq = None;
					let mut attrs = TokenStream::new();
					for x in &r.body {
						match x {
							ExtensionBody::Sequence(s) => {
								seq.replace(s);
							}
							ExtensionBody::Attribute(a) => {
								let cc = ctx.clone();
								// TODO: need to use defs once anon defs are output
								let (attr, _defs) = a.gen_field(&cc);
								attrs.append_all(quote!(
									#attr,
								));
							}
							_ => panic!("unhandled extension body element {:?}", x),
						}
					}
					let seq_ctx = ctx.with_name(&format!("{}Extension", name));
					let seq = seq.expect("Complex type extension declared but no seq found");
					let body_type = seq.ident(&seq_ctx);
					let body_defs = seq.gen(&seq_ctx);

					defs.append_all(quote!(
						#doc
						pub struct #name {
							base: #base_ty,
							body: #body_type,
							#attrs
						}
						#body_defs
					));
				}
				ComplexContentBody::Restriction(_) => panic!("unhandled extension body element"),
			}
		}
		defs
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
	// Since attributes are an embedded type, its not really appropriate to use
	// the codegen trait.
	pub fn gen_field(&self, ctx: &Context) -> (TokenStream, TokenStream) {
		let name = ctx.mk_field(&self.name.as_ref().unwrap());
		trace!("gen_field attribute: {:?}", name);
		let doc_ts = fmt_doc(self.get_doc());

		let typ = self.get_type(ctx);
		let defs = self.gen(ctx);
		(
			quote!(
				#doc_ts
				#name: #typ
			),
			defs,
		)
	}
	fn get_type(&self, ctx: &Context) -> syn::TypePath {
		if let Some(ty) = &self.r#type.as_ref() {
			ctx.resolve_type(&ty.0)
		} else {
			debug!("Unable to identify attribute, it'll be a string for now");
			ctx.resolve_type("String")
		}
	}
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

impl Genable for Attribute {
	fn gen(&self, ctx: &Context) -> TokenStream {
		// TODO: make an anon type if neccessary
		TokenStream::new()
		// let (ty, defs) = if let Some(ty) = &self.r#type.as_ref() {
		// 	let ty = ctx.resolve_ident(&ty.0);
		// 	(ty, TokenStream::new())
		// } else {
		// TODO: gen anon type
		// let st = self
		// 	.body
		// 	.as_ref()
		// 	.unwrap()
		// 	.iter()
		// 	.filter_map(|x| match x {
		// 		AttributeBody::SimpleType(e) => Some(e),
		// 		_ => None,
		// 	})
		// 	.next()
		// 	.expect("No type or body found for attribute");
		// let defs = st.gen(tx);
		// (defs.0, defs.1)
		// 	(ctx.resolve_ident("String"), TokenStream::new())
		// };
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

impl Identifiable for Sequence {
	fn ident(&self, ctx: &Context) -> syn::Ident {
		ctx.mk_type(&ctx.name.as_ref().expect("Name not given to sequence"))
	}
}

impl Genable for Sequence {
	fn gen(&self, ctx: &Context) -> TokenStream {
		let name = self.ident(ctx);
		trace!("codegen sequence: {:?}", name);

		// TODO: move into element codegen?
		let variants: Vec<TokenStream> = self
			.body
			.as_ref()
			.unwrap()
			.iter()
			// Lets only consider elements
			.filter_map(|e| match e {
				SequenceBody::Element(e) => Some(e),
				_ => None,
			})
			.map(|e| {
				// ref attr, so type is pasted from unnamed type in a particle
				let (name, ty): (String, String) = if let Some(rf) = e.r#ref.as_ref() {
					(rf.clone(), rf.clone())
				// has name so type is defined
				} else if let Some(nm) = e.name.as_ref() {
					let t = e.r#type.as_ref().unwrap().0.clone();
					(nm.clone(), t)
				// substitution group
				} else if let Some(ty) = e.substitution_group.as_ref() {
					(e.name.as_ref().unwrap().clone(), ty.0.clone())
				} else {
					panic!("element should have either a name, type pair or a ref")
				};
				let name = ctx.mk_type(&name);
				let ty_name = ctx.resolve_type(&ty);
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

		quote!(
			#doc
			pub enum #name {
				#variant_stream
			}
		)
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
	AnyAttribute(AnyAttribute),
	AttributeGroup(AttributeGroup),
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
	AppInfo(AppInfo),
	Documentation(Documentation),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppInfo {}

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
