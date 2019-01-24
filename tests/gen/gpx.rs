mod xsd {
    use serde_derive::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct String(std::string::String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Boolean(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Float(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Double(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Decimal(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct DateTime(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Duration(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct HexBinary(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Base64Binary(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct AnyUri(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Id(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Idref(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Entity(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Notation(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct NormalizedString(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Token(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Language(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Idrefs(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Entities(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Nmtoken(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Nmtokens(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Name(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct QName(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct NcName(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Integer(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct NonNegativeInteger(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct PositiveInteger(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct NonPositiveInteger(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct NegativeInteger(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Byte(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Int(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Long(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Short(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct UnsignedByte(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct UnsignedInt(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct UnsignedLong(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct UnsignedShort(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Date(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Time(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct GYearMonth(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct GYear(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct GMonthDay(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct GDay(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct GMonth(String);
}
use serde_derive::{Deserialize, Serialize};
#[doc = "GPX is the root element in the XML file."]
#[serde(rename = "gpx")]
#[doc = "GPX documents contain a metadata header, followed by waypoints, routes, and tracks.  You can add your own elements\r\n\t\tto the extensions section of the GPX document."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GpxType {
    #[doc = "You must include the version number in your GPX document."]
    version: xsd::String,
    #[doc = "You must include the name or URL of the software that created your GPX document.  This allows others to\r\n\t\tinform the creator of a GPX instance document that fails to validate."]
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
#[doc = "The latitude of the point.  Decimal degrees, WGS84 datum."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct LatitudeType(String);
#[doc = "The longitude of the point.  Decimal degrees, WGS84 datum."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct LongitudeType(String);
#[doc = "Used for bearing, heading, course.  Units are decimal degrees, true (not magnetic)."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct DegreesType(String);
#[doc = "Type of GPS fix.  none means GPS had no fix.  To signify \"the fix info is unknown, leave out fixType entirely. pps = military signal used"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct FixType(String);
#[doc = "Represents a differential GPS station."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct DgpsStationType(String);
#[doc = "Information about the GPX file, author, and copyright restrictions goes in the metadata section.  Providing rich,\r\n\t\tmeaningful information about your GPX files allows others to search for and use your GPS data."]
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
#[doc = "wpt represents a waypoint, point of interest, or named feature on a map."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WptType {
    #[doc = "The latitude of the point.  This is always in decimal degrees, and always in WGS84 datum."]
    lat: LatitudeType,
    #[doc = "The longitude of the point.  This is always in decimal degrees, and always in WGS84 datum."]
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
#[doc = "rte represents route - an ordered list of waypoints representing a series of turn points leading to a destination."]
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
#[doc = "trk represents a track - an ordered list of points describing a path."]
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
#[doc = "You can add extend GPX by adding your own elements from another schema here."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionsType {
    #[serde(rename = "$value")]
    body: Vec<ExtensionsTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ExtensionsTypeBody {}
#[doc = "A Track Segment holds a list of Track Points which are logically connected in order. To represent a single GPS track where GPS reception was lost, or the GPS receiver was turned off, start a new Track Segment for each continuous span of track data."]
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
#[doc = "Information about the copyright holder and any license governing use of this file.  By linking to an appropriate license,\r\n\t you may place your data into the public domain or grant additional usage rights."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CopyrightType {
    #[doc = "Copyright holder (TopoSoft, Inc.)"]
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
#[doc = "A link to an external resource (Web page, digital photo, video clip, etc) with additional information."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinkType {
    #[doc = "URL of hyperlink."]
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
#[doc = "An email address.  Broken into two parts (id and domain) to help prevent email harvesting."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmailType {
    #[doc = "id half of email address (billgates2004)"]
    id: xsd::String,
    #[doc = "domain half of email address (hotmail.com)"]
    domain: xsd::String,
}
#[doc = "A person or organization."]
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
#[doc = "A geographic point with optional elevation and time.  Available for use by other schemas."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PtType {
    #[doc = "The latitude of the point.  Decimal degrees, WGS84 datum."]
    lat: LatitudeType,
    #[doc = "The latitude of the point.  Decimal degrees, WGS84 datum."]
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
#[doc = "An ordered sequence of points.  (for polygons or polylines, e.g.)"]
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
#[doc = "Two lat/lon pairs defining the extent of an element."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BoundsType {
    #[doc = "The minimum latitude."]
    minlat: LatitudeType,
    #[doc = "The minimum longitude."]
    minlon: LongitudeType,
    #[doc = "The maximum latitude."]
    maxlat: LatitudeType,
    #[doc = "The maximum longitude."]
    maxlon: LongitudeType,
}
