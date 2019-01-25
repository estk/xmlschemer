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
#[serde(rename = "address")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct Anglepos90Type(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct Angle90Type(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct Anglepos180Type(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct Angle180Type(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct Angle360Type(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct AltitudeModeEnumType(String);
#[doc = "aabbggrr\n        \n        ffffffff: opaque white\n        ff000000: opaque black"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct ColorType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct CoordinatesType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct ColorModeEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct DateTimeType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct DisplayModeEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct GridOriginEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct ItemIconStateType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct ItemIconStateEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct ListItemTypeEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct RefreshModeEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct ViewRefreshModeEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct ShapeEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct StyleStateEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct UnitsEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vec2Type {
    x: Double,
    y: Double,
    xunits: UnitsEnumType,
    yunits: UnitsEnumType,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbstractObjectType {
    #[serde(rename = "$value")]
    body: Vec<AbstractObjectTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AbstractObjectTypeBody {
    ObjectSimpleExtensionGroup(ObjectSimpleExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbstractFeatureType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SnippetType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbstractViewType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LookAtType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CameraType {}
#[doc = "MetadataType deprecated in 2.2"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetadataType {
    #[serde(rename = "$value")]
    body: Vec<MetadataTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MetadataTypeBody {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedDataType {
    #[serde(rename = "$value")]
    body: Vec<ExtendedDataTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ExtendedDataTypeBody {
    Data(Data),
    SchemaData(SchemaData),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaDataType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimpleDataType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbstractContainerType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbstractGeometryType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbstractOverlayType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbstractStyleSelectorType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbstractTimePrimitiveType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KmlType {
    hint: String,
    #[serde(rename = "$value")]
    body: Vec<KmlTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum KmlTypeBody {
    NetworkLinkControl(NetworkLinkControl),
    AbstractFeatureGroup(AbstractFeatureGroup),
    KmlSimpleExtensionGroup(KmlSimpleExtensionGroup),
    KmlObjectExtensionGroup(KmlObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkLinkControlType {
    #[serde(rename = "$value")]
    body: Vec<NetworkLinkControlTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum NetworkLinkControlTypeBody {
    MinRefreshPeriod(MinRefreshPeriod),
    MaxSessionLength(MaxSessionLength),
    Cookie(Cookie),
    Message(Message),
    LinkName(LinkName),
    LinkDescription(LinkDescription),
    LinkSnippet(LinkSnippet),
    Expires(Expires),
    Update(Update),
    AbstractViewGroup(AbstractViewGroup),
    NetworkLinkControlSimpleExtensionGroup(NetworkLinkControlSimpleExtensionGroup),
    NetworkLinkControlObjectExtensionGroup(NetworkLinkControlObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DocumentType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaType {
    name: String,
    id: Id,
    #[serde(rename = "$value")]
    body: Vec<SchemaTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SchemaTypeBody {
    SimpleField(SimpleField),
    SchemaExtension(SchemaExtension),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimpleFieldType {
    r#type: String,
    name: String,
    #[serde(rename = "$value")]
    body: Vec<SimpleFieldTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SimpleFieldTypeBody {
    DisplayName(DisplayName),
    SimpleFieldExtension(SimpleFieldExtension),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FolderType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlacemarkType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkLinkType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegionType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LatLonAltBoxType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LodType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinkType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MultiGeometryType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PointType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LineStringType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinearRingType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PolygonType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BoundaryType {
    #[serde(rename = "$value")]
    body: Vec<BoundaryTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BoundaryTypeBody {
    LinearRing(LinearRing),
    BoundarySimpleExtensionGroup(BoundarySimpleExtensionGroup),
    BoundaryObjectExtensionGroup(BoundaryObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModelType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocationType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrientationType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScaleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMapType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AliasType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GroundOverlayType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbstractLatLonBoxType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LatLonBoxType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScreenOverlayType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PhotoOverlayType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewVolumeType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImagePyramidType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StyleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StyleMapType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PairType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbstractSubStyleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbstractColorStyleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IconStyleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasicLinkType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LabelStyleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LineStyleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PolyStyleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BalloonStyleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListStyleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemIconType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeStampType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSpanType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateType {
    #[serde(rename = "$value")]
    body: Vec<UpdateTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpdateTypeBody {
    TargetHref(TargetHref),
    UpdateExtensionGroup(UpdateExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateType {
    #[serde(rename = "$value")]
    body: Vec<CreateTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum CreateTypeBody {
    AbstractContainerGroup(AbstractContainerGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteType {
    #[serde(rename = "$value")]
    body: Vec<DeleteTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum DeleteTypeBody {
    AbstractFeatureGroup(AbstractFeatureGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeType {
    #[serde(rename = "$value")]
    body: Vec<ChangeTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ChangeTypeBody {
    AbstractObjectGroup(AbstractObjectGroup),
}
