use serde_derive::{Deserialize, Serialize};
include!("./primitives.rs");

#[doc = " GPX is the root element in the XML file."]
#[serde(rename = "gpx")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GpxType {
    version: xsd::String,
    creator: xsd::String,
    #[serde(rename = "$value")]
    body: Vec<GpxTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum GpxTypeBody {
    Metadata(MetadataType),
    Wpt(WptType),
    Rte(RteType),
    Trk(TrkType),
    Extensions(ExtensionsType),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct LatitudeType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct LongitudeType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct DegreesType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct FixType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct DgpsStationType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetadataType {
    #[serde(rename = "$value")]
    body: Vec<MetadataTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MetadataTypeBody {
    Name(xsd::String),
    Desc(xsd::String),
    Author(PersonType),
    Copyright(CopyrightType),
    Link(LinkType),
    Time(xsd::DateTime),
    Keywords(xsd::String),
    Bounds(BoundsType),
    Extensions(ExtensionsType),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WptType {
    lat: LatitudeType,
    lon: LongitudeType,
    #[serde(rename = "$value")]
    body: Vec<WptTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum WptTypeBody {
    Ele(xsd::Decimal),
    Time(xsd::DateTime),
    Magvar(DegreesType),
    Geoidheight(xsd::Decimal),
    Name(xsd::String),
    Cmt(xsd::String),
    Desc(xsd::String),
    Src(xsd::String),
    Link(LinkType),
    Sym(xsd::String),
    Type(xsd::String),
    Fix(FixType),
    Sat(xsd::NonNegativeInteger),
    Hdop(xsd::Decimal),
    Vdop(xsd::Decimal),
    Pdop(xsd::Decimal),
    Ageofdgpsdata(xsd::Decimal),
    Dgpsid(DgpsStationType),
    Extensions(ExtensionsType),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RteType {
    #[serde(rename = "$value")]
    body: Vec<RteTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum RteTypeBody {
    Name(xsd::String),
    Cmt(xsd::String),
    Desc(xsd::String),
    Src(xsd::String),
    Link(LinkType),
    Number(xsd::NonNegativeInteger),
    Type(xsd::String),
    Extensions(ExtensionsType),
    Rtept(WptType),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrkType {
    #[serde(rename = "$value")]
    body: Vec<TrkTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TrkTypeBody {
    Name(xsd::String),
    Cmt(xsd::String),
    Desc(xsd::String),
    Src(xsd::String),
    Link(LinkType),
    Number(xsd::NonNegativeInteger),
    Type(xsd::String),
    Extensions(ExtensionsType),
    Trkseg(TrksegType),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionsType {
    #[serde(rename = "$value")]
    body: Vec<ExtensionsTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ExtensionsTypeBody {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrksegType {
    #[serde(rename = "$value")]
    body: Vec<TrksegTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TrksegTypeBody {
    Trkpt(WptType),
    Extensions(ExtensionsType),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CopyrightType {
    author: xsd::String,
    #[serde(rename = "$value")]
    body: Vec<CopyrightTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum CopyrightTypeBody {
    Year(xsd::GYear),
    License(xsd::AnyUri),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinkType {
    href: xsd::AnyUri,
    #[serde(rename = "$value")]
    body: Vec<LinkTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum LinkTypeBody {
    Text(xsd::String),
    Type(xsd::String),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmailType {
    id: xsd::String,
    domain: xsd::String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PersonType {
    #[serde(rename = "$value")]
    body: Vec<PersonTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PersonTypeBody {
    Name(xsd::String),
    Email(EmailType),
    Link(LinkType),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PtType {
    lat: LatitudeType,
    lon: LongitudeType,
    #[serde(rename = "$value")]
    body: Vec<PtTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PtTypeBody {
    Ele(xsd::Decimal),
    Time(xsd::DateTime),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PtsegType {
    #[serde(rename = "$value")]
    body: Vec<PtsegTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PtsegTypeBody {
    Pt(PtType),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BoundsType {
    minlat: LatitudeType,
    minlon: LongitudeType,
    maxlat: LatitudeType,
    maxlon: LongitudeType,
}
