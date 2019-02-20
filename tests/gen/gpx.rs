use chrono::{DateTime, Duration, FixedOffset};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
#[doc = "The latitude of the point.  Decimal degrees, WGS84 datum."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct LatitudeType(String);
#[doc = "The longitude of the point.  Decimal degrees, WGS84 datum."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct LongitudeType(String);
#[doc = "Used for bearing, heading, course.  Units are decimal degrees, true (not magnetic)."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct DegreesType(String);
#[doc = "Type of GPS fix.  none means GPS had no fix.  To signify \"the fix info is unknown, leave out fixType entirely. pps = military signal used"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct FixType(String);
#[doc = "Represents a differential GPS station."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct DgpsStationType(String);
#[doc = "GPX is the root element in the XML file."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Gpx(GpxType);
#[doc = "GPX documents contain a metadata header, followed by waypoints, routes, and tracks.  You can add your own elements\r\n\t\tto the extensions section of the GPX document."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GpxType {
    #[doc = "You must include the version number in your GPX document."]
    version: String,
    #[doc = "You must include the name or URL of the software that created your GPX document.  This allows others to\r\n\t\tinform the creator of a GPX instance document that fails to validate."]
    creator: String,
    #[serde(rename = "$value")]
    body: Vec<UpcaseGpxTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseGpxTypeBody {
    Metadata(MetadataType),
    Wpt(WptType),
    Rte(RteType),
    Trk(TrkType),
    Extensions(ExtensionsType),
}
#[doc = "Information about the GPX file, author, and copyright restrictions goes in the metadata section.  Providing rich,\r\n\t\tmeaningful information about your GPX files allows others to search for and use your GPS data."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetadataType {
    #[serde(rename = "$value")]
    body: Vec<UpcaseMetadataTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseMetadataTypeBody {
    Name(String),
    Desc(String),
    Author(PersonType),
    Copyright(CopyrightType),
    Link(LinkType),
    Time(DateTime<FixedOffset>),
    Keywords(String),
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
    body: Vec<UpcaseWptTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseWptTypeBody {
    Ele(String),
    Time(DateTime<FixedOffset>),
    Magvar(DegreesType),
    Geoidheight(String),
    Name(String),
    Cmt(String),
    Desc(String),
    Src(String),
    Link(LinkType),
    Sym(String),
    Type(String),
    Fix(FixType),
    Sat(String),
    Hdop(String),
    Vdop(String),
    Pdop(String),
    Ageofdgpsdata(String),
    Dgpsid(DgpsStationType),
    Extensions(ExtensionsType),
}
#[doc = "rte represents route - an ordered list of waypoints representing a series of turn points leading to a destination."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RteType {
    #[serde(rename = "$value")]
    body: Vec<UpcaseRteTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseRteTypeBody {
    Name(String),
    Cmt(String),
    Desc(String),
    Src(String),
    Link(LinkType),
    Number(String),
    Type(String),
    Extensions(ExtensionsType),
    Rtept(WptType),
}
#[doc = "trk represents a track - an ordered list of points describing a path."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrkType {
    #[serde(rename = "$value")]
    body: Vec<UpcaseTrkTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseTrkTypeBody {
    Name(String),
    Cmt(String),
    Desc(String),
    Src(String),
    Link(LinkType),
    Number(String),
    Type(String),
    Extensions(ExtensionsType),
    Trkseg(TrksegType),
}
#[doc = "You can add extend GPX by adding your own elements from another schema here."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionsType {
    #[serde(rename = "$value")]
    body: Vec<UpcaseExtensionsTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseExtensionsTypeBody {}
#[doc = "A Track Segment holds a list of Track Points which are logically connected in order. To represent a single GPS track where GPS reception was lost, or the GPS receiver was turned off, start a new Track Segment for each continuous span of track data."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrksegType {
    #[serde(rename = "$value")]
    body: Vec<UpcaseTrksegTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseTrksegTypeBody {
    Trkpt(WptType),
    Extensions(ExtensionsType),
}
#[doc = "Information about the copyright holder and any license governing use of this file.  By linking to an appropriate license,\r\n\t you may place your data into the public domain or grant additional usage rights."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CopyrightType {
    #[doc = "Copyright holder (TopoSoft, Inc.)"]
    author: String,
    #[serde(rename = "$value")]
    body: Vec<UpcaseCopyrightTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseCopyrightTypeBody {
    Year(i32),
    License(String),
}
#[doc = "A link to an external resource (Web page, digital photo, video clip, etc) with additional information."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinkType {
    #[doc = "URL of hyperlink."]
    href: String,
    #[serde(rename = "$value")]
    body: Vec<UpcaseLinkTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseLinkTypeBody {
    Text(String),
    Type(String),
}
#[doc = "An email address.  Broken into two parts (id and domain) to help prevent email harvesting."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmailType {
    #[doc = "id half of email address (billgates2004)"]
    id: String,
    #[doc = "domain half of email address (hotmail.com)"]
    domain: String,
}
#[doc = "A person or organization."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PersonType {
    #[serde(rename = "$value")]
    body: Vec<UpcasePersonTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcasePersonTypeBody {
    Name(String),
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
    body: Vec<UpcasePtTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcasePtTypeBody {
    Ele(String),
    Time(DateTime<FixedOffset>),
}
#[doc = "An ordered sequence of points.  (for polygons or polylines, e.g.)"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PtsegType {
    #[serde(rename = "$value")]
    body: Vec<UpcasePtsegTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcasePtsegTypeBody {
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
