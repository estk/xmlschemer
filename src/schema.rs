use heck::*;
use if_chain::if_chain;
use log::debug;
use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use serde_derive::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;
use std::str::FromStr;
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
    // xmlns: String,
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
    fn codegen(&self, _ctx: &mut Context) -> TokenStream {
        let name = resolve_type_str(&self.name.as_ref().unwrap());
        let mut doc_ts = TokenStream::new();
        let doc = format_doc_block(self.get_doc());
        doc_ts.append_all(quote!(
            #doc
            #[derive(Serialize, Deserialize, Debug)]
            #[serde(transparent)]
        ));

        if let Some(t) = self.r#type.as_ref() {
            let typ = resolve_type(&t.0);
            quote!(
                #doc_ts
                pub struct #name(#typ);
            )
        } else {
            quote!(
                #doc_ts
                pub struct #name {
                    #[serde(flatten)]
                    other: HashMap<String, String>,
                }
            )
        }
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
    fn codegen(&self, _ctx: &mut Context) -> TokenStream {
        let name = resolve_type_str(&self.name.as_ref().unwrap().0);
        let mut doc_ts = TokenStream::new();
        let doc = format_doc_block(self.get_doc());
        doc_ts.append_all(quote!(
            #doc
            #[derive(Serialize, Deserialize, Debug)]
            #[serde(rename_all = "camelCase")]
            #[serde(transparent)]
        ));

        if name == "String" {
            quote!(
                #doc_ts
                pub struct #name(std::string::String);
            )
        } else {
            quote!(
                #doc_ts
                pub struct #name(String);
            )
        }
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
    fn codegen(&self, ctx: &mut Context) -> TokenStream {
        let name_str = if let Some(n) = ctx.name.as_ref() {
            &n
        } else {
            &self.name.as_ref().unwrap().0
        };
        debug!("Building complex type: {}", name_str);
        let name = resolve_type_str(&name_str);

        let mut doc_ts = TokenStream::new();
        let doc = format_doc_block(self.get_doc());
        doc_ts.append_all(quote!(
            #doc
            #[derive(Serialize, Deserialize, Debug)]
            #[serde(rename_all = "camelCase")]
        ));

        let mut sequence = vec![];
        let mut attrs = vec![];

        for x in self.body.as_ref().unwrap() {
            match x {
                ComplexBody::Sequence(s) => sequence.push(s),
                ComplexBody::Attribute(a) => attrs.push(a),
                _ => (),
            }
        }

        let fields = {
            let mut ts = TokenStream::new();
            for a in attrs {
                debug!("ts is: {}", ts.to_string());
                debug!("attr is: {:#?}", a);
                let field = a.codegen(ctx);
                ts.append_all(quote!(
                    #field,
                ))
            }
            ts
        };

        if sequence.is_empty() {
            quote!(
                #doc_ts
                pub struct #name {
                    #fields
                }
            )
        } else {
            let mut body_ctx = ctx.with_name(&format!("{}Body", name));
            let body = sequence.first().unwrap().codegen(&mut body_ctx);
            let defs = body_ctx.defs;

            quote!(
                #doc_ts
                pub struct #name {
                    #fields
                    #[serde(rename="$value")]
                    body: #body,
                }
                #defs
            )
        }
    }
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
    body: Option<SimpleContentBody>,
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
    restriction: Option<Vec<ComplexContentBody>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum ComplexContentBody {
    Restriction(Restriction),
    Extension(Extension),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Extension {
    base: QName,
    #[serde(rename = "$value")]
    body: Option<Vec<ExtensionBody>>,
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
    fn codegen(&self, _ctx: &mut Context) -> TokenStream {
        let name = self.name.as_ref().unwrap().clone();
        let name_id = if name == "type" {
            syn::parse_str("r#type").unwrap()
        } else {
            Ident::new(&name, Span::call_site())
        };
        let ty = resolve_type(&self.r#type.as_ref().unwrap().0);
        let doc_ts = format_doc_block(self.get_doc());
        quote!(
            #doc_ts
            #name_id: #ty
        )
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
    fn codegen(&self, ctx: &mut Context) -> TokenStream {
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
        ctx.defs.append_all(companion_type);
        quote!(
            Vec<#name>
        )
    }
}

fn resolve_typ_inner(s: &str) -> Vec<String> {
    let mut split = s.split(':').map(|e| e.to_string()).collect::<Vec<String>>();
    if let Some(s) = split.first() {
        if split.len() > 1 && s == "kml" {
            split.remove(0);
        }
    }
    if split.len() == 1 {
        let last = split.last_mut().unwrap();
        let swap = match &last[..] {
            "anySimpleType" => Some("String"),
            "string" => Some("String"),
            "double" => Some("f64"),
            "boolean" => Some("bool"),
            "int" => Some("i64"),
            _ => None,
        };
        if let Some(s) = swap {
            *last = s.to_string();
            return split;
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
    Element(Element),
    Attribute,
    Notation(Notation),
}
impl CodeGenerator for SchemaBody {
    fn codegen(&self, ctx: &mut Context) -> TokenStream {
        match self {
            Self::Include => TokenStream::new(),
            Self::Import(_) => TokenStream::new(),
            Self::Redefine => TokenStream::new(),
            Self::Override => TokenStream::new(),
            Self::Annotation(_) => TokenStream::new(),
            Self::DefaultOpenContent(_) => TokenStream::new(),
            Self::SimpleType(i) => i.codegen(ctx),
            Self::ComplexType(i) => i.codegen(ctx),
            Self::Group(_) => TokenStream::new(),
            Self::AttributeGroup(_) => TokenStream::new(),
            Self::Element(i) => i.codegen(ctx),
            Self::Attribute => TokenStream::new(),
            Self::Notation(_) => TokenStream::new(),
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
pub trait CodeGenerator {
    fn codegen(&self, ctx: &mut Context) -> TokenStream;
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
    fn codegen(&self, ctx: &mut Context) -> TokenStream {
        // for (k, v) in self.other.iter() {
        //     if k.starts_with("xmlns:") {
        //         let ns = &k[6..];
        //         ctx.add_ns(ns, v);
        //     }
        // }

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
            .unwrap_or("unnamed".to_string());

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
                        defs.append_all(t.codegen(&mut ctx.with_name(&root_name_str)));
                    }
                }
                (root_name_str.to_string(), doc_ts)
            }
        } else {
            ("".to_string(), TokenStream::new())
        };

        for t in simple_types {
            defs.append_all(t.codegen(ctx));
        }
        for e in elements {
            defs.append_all(e.codegen(ctx))
        }
        for t in complex_types {
            if_chain! {
                if let Some(n) = t.name.as_ref();
                if n.0 == root_tn;
                then {
                    root_ts.append_all(t.codegen(ctx));
                } else {
                    debug!("genning for {:?}", t);
                    defs.append_all(t.codegen(ctx));
                }
            };
        }
        quote!(
            use serde_derive::{Deserialize, Serialize};
            use std::collections::HashMap;

            #root_doc
            #root_ts
            #defs
        )
    }
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
