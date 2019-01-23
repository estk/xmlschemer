use heck::*;
use if_chain::if_chain;
use log::debug;
use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use syn::{self, Ident};

// default unqualified
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
    // #all
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
        for e in self.body.as_ref()? {
            if let ElementBody::Annotation(a) = e {
                return a.get_doc();
            }
        }

        None
    }
}

impl CodeGenerator for Element {
    fn codegen(&self, ctx: &mut Context) -> TokenStream {
        unimplemented!()
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
        for e in self.body.as_ref()? {
            if let SimpleBody::Annotation(a) = e {
                return a.get_doc();
            }
        }
        None
    }
}

impl CodeGenerator for SimpleType {
    fn codegen(&self, ctx: &mut Context) -> TokenStream {
        let name = &self.name.as_ref().unwrap().0.to_camel_case();
        let name_id = Ident::new(name, Span::call_site());
        let mut doc = TokenStream::new();
        if let Some(s) = self.get_doc() {
            doc.append_all(TokenStream::from_str(&format!("/// {}", s)))
        }
        doc.append_all(quote!(
            #[derive(Serialize, Deserialize, Debug)]
            #[serde(rename_all = "camelCase")]
        ));

        doc.append_all(quote!(
            #[serde(transparent)]
        ));
        if name == "String" {
            quote!(
                #doc
                pub struct #name_id(std::string::String);
            )
        } else {
            quote!(
                #doc
                pub struct #name_id(String);
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
    r#abstract: Option<bool>,
    r#type: Option<QName>,
    #[serde(rename = "$value")]
    body: Option<Vec<ComplexBody>>,
}

impl CodeGenerator for ComplexType {
    fn codegen(&self, ctx: &mut Context) -> TokenStream {
        let name_str = if let Some(n) = ctx.name.as_ref() {
            n.to_camel_case()
        } else {
            self.name.as_ref().unwrap().0.to_camel_case()
        };
        debug!("Building complex type: {}", name_str);
        let name = Ident::new(&name_str, Span::call_site());
        let doc = quote!(
            #[derive(Serialize, Deserialize, Debug)]
            #[serde(rename_all = "camelCase")]
        );

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
                #doc
                pub struct #name {
                    #fields
                }
            )
        } else {
            let mut body_ctx = ctx.with_name(&format!("{}Body", name));
            let body = sequence.first().unwrap().codegen(&mut body_ctx);
            let defs = body_ctx.defs;

            quote!(
                #doc
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
    Annotation(Annotation),
    Sequence(Sequence),
    Attribute(Attribute),
    AnyAttribute(AnyAttribute),
    ComplexContent(ComplexContent),
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
    Group(Group),
    Attribute(Attribute),
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
    body: Option<Vec<SimpleType>>,
}
impl CodeGenerator for Attribute {
    fn codegen(&self, ctx: &mut Context) -> TokenStream {
        let name = self.name.as_ref().unwrap();
        let name_id = Ident::new(&name, Span::call_site());
        let ty = to_type_path(&self.r#type.as_ref().unwrap().0);
        let mut doc = TokenStream::new();
        quote!(
            #doc
            #name_id: #ty
        )
    }
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
        let name = Ident::new(
            &ctx.name.as_ref().unwrap().to_camel_case(),
            Span::call_site(),
        );
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
                let name = Ident::new(&e.name.as_ref().unwrap().to_camel_case(), Span::call_site());
                let ty_name = to_type_path(&e.r#type.as_ref().unwrap().0);
                quote!(
                    #name(#ty_name)
                )
            })
            .collect();

        let mut variant_stream = TokenStream::new();
        for v in variants {
            variant_stream = quote!(
                #variant_stream
                #v,
            )
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

fn to_type_path(s: &str) -> syn::TypePath {
    let mut split = s.split(':').map(|e| e.to_string()).collect::<Vec<String>>();

    split.last_mut().map(|x| *x = x.to_camel_case());
    let join = split.join("::");

    syn::parse_str(&join).unwrap()
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
    Annotation(Annotation),
    Choice(Choice),
    Sequence(Sequence),
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
    AppInfo(Any),
    Documentation(Documentation),
}

#[serde(rename = "appinfo")]
#[derive(Serialize, Deserialize, Debug)]
pub struct AppInfo();

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
pub enum SchemaBody {
    Include,
    Import(Import),
    Redefine,
    Override,
    Annotation(Annotation),
    DefaultOpenContent,
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
        let mut ts = TokenStream::new();
        let body = match self {
            Self::Include => TokenStream::new(),
            Self::Import(i) => TokenStream::new(),
            Self::Redefine => TokenStream::new(),
            Self::Override => TokenStream::new(),
            Self::Annotation(i) => TokenStream::new(),
            Self::DefaultOpenContent => TokenStream::new(),
            Self::SimpleType(i) => i.codegen(ctx),
            Self::ComplexType(i) => i.codegen(ctx),
            Self::Group(i) => TokenStream::new(),
            Self::AttributeGroup(i) => TokenStream::new(),
            Self::Element(i) => i.codegen(ctx),
            Self::Attribute => TokenStream::new(),
            Self::Notation(i) => TokenStream::new(),
        };
        ts.append_all(body);
        ts
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
    #[serde(rename = "lang")]
    xml_lang: Option<String>,

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
}
impl Context {
    pub fn with_name(&self, name: &str) -> Self {
        let mut me = self.clone();
        me.name.replace(name.to_string());
        me
    }
}

impl Default for Context {
    fn default() -> Self {
        Context {
            name: None,
            defs: TokenStream::new(),
        }
    }
}

impl CodeGenerator for Schema {
    fn codegen(&self, ctx: &mut Context) -> TokenStream {
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
        let root_name = Ident::new(&root_name_str, Span::call_site());

        let (root_tn, root_doc) = if let Some(r) = root {
            let doc = r
                .get_doc()
                .map(|d| format!("/// {}", d))
                .unwrap_or("".to_string());
            let mut doc_ts = TokenStream::from_str(&doc).unwrap();
            doc_ts.append_all(quote!(
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
        // for e in elements {
        //     defs.append_all(e.codegen(ctx))
        // }
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

            #root_doc
            #root_ts
            #defs
        )
    }
}
