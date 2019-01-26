mod xsd {
    use serde_derive::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct String(std::string::String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct bool(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct Float(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct f64(String);
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
    pub struct BigId(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct BigIdref(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct BigEntity(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct BigNotation(String);
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
    pub struct BigIdrefs(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct BigEntities(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct BigNmtoken(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct BigNmtokens(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct BigName(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct BigQName(String);
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(transparent)]
    pub struct BigNcName(String);
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
    pub struct i64(String);
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
#[serde(transparent)]
pub struct Altitude(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct AltitudeMode(AltitudeModeEnumType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Begin(DateTimeType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BgColor(ColorType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BottomFov(Angle90Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Color(ColorType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ColorMode(ColorModeEnumType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Cookie(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Coordinates(CoordinatesType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Description(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct DisplayName(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct DisplayMode(DisplayModeEnumType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct DrawOrder(i64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct East(Angle180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct End(DateTimeType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Expires(DateTimeType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Extrude(bool);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Fill(bool);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct FlyToView(bool);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct GridOrigin(GridOriginEnumType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Heading(Angle360Type);
#[doc = "not anyURI due to $[x] substitution in\n      PhotoOverlay"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Href(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct HttpQuery(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct HotSpot(Vec2Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Key(StyleStateEnumType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Latitude(Angle90Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct LeftFov(Angle180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct LinkDescription(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct LinkName(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct LinkSnippet(BigSnippetType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ListItemType(ListItemTypeEnumType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Longitude(Angle180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MaxSnippetLines(i64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MaxSessionLength(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Message(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MinAltitude(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MinFadeExtent(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MinLodPixels(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MinRefreshPeriod(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MaxAltitude(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MaxFadeExtent(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MaxLodPixels(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MaxHeight(i64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MaxWidth(i64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Name(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Near(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct North(Angle180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Open(bool);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Outline(bool);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct OverlayXy(Vec2Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct PhoneNumber(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Range(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct RefreshMode(RefreshModeEnumType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct RefreshInterval(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct RefreshVisibility(bool);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct RightFov(Angle180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Roll(Angle180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Rotation(Angle180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct RotationXy(Vec2Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Scale(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ScreenXy(Vec2Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Shape(ShapeEnumType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Size(Vec2Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct South(Angle180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct SourceHref(AnyUri);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Snippet(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct State(ItemIconStateType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct StyleUrl(AnyUri);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct TargetHref(AnyUri);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Tessellate(bool);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Text(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct TextColor(ColorType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct TileSize(i64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Tilt(Anglepos180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct TopFov(Angle90Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Value(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ViewBoundScale(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ViewFormat(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ViewRefreshMode(ViewRefreshModeEnumType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ViewRefreshTime(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Visibility(bool);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct West(Angle180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct When(DateTimeType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Width(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct X(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Y(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Z(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractObjectGroup(BigAbstractObjectType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigObjectSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractFeatureGroup(BigAbstractFeatureType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractFeatureSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigSnippet(BigSnippetType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractViewGroup(BigAbstractViewType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractViewSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLookAt(BigLookAtType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLookAtSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigCamera(BigCameraType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigCameraSimpleExtensionGroup(String);
#[doc = "Metadata deprecated in 2.2"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigMetadata(BigMetadataType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigExtendedData(BigExtendedDataType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigSchemaData(BigSchemaDataType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigSimpleData(BigSimpleDataType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigData(BigDataType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractContainerGroup(BigAbstractContainerType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractContainerSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractGeometryGroup(BigAbstractGeometryType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractGeometrySimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractOverlayGroup(BigAbstractOverlayType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractOverlaySimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractStyleSelectorGroup(BigAbstractStyleSelectorType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractStyleSelectorSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractTimePrimitiveGroup(BigAbstractTimePrimitiveType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractTimePrimitiveSimpleExtensionGroup(String);
#[doc = "<kml> is the root element."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Kml(BigKmlType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigKmlSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigNetworkLinkControl(BigNetworkLinkControlType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigNetworkLinkControlSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigDocument(BigDocumentType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigDocumentSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigSchema(BigSchemaType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigSimpleField(BigSimpleFieldType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigFolder(BigFolderType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigFolderSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigPlacemark(BigPlacemarkType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigPlacemarkSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigNetworkLink(BigNetworkLinkType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigNetworkLinkSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigRegion(BigRegionType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigRegionSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLatLonAltBox(BigLatLonAltBoxType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLatLonAltBoxSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLod(BigLodType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLodSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigIcon(BigLinkType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLink(BigLinkType);
#[doc = "Url deprecated in 2.2"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigUrl(BigLinkType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLinkSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigMultiGeometry(BigMultiGeometryType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigMultiGeometrySimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigPoint(BigPointType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigPointSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLineString(BigLineStringType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLineStringSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLinearRing(BigLinearRingType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLinearRingSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigPolygon(BigPolygonType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigPolygonSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct OuterBoundaryIs(BigBoundaryType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct InnerBoundaryIs(BigBoundaryType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigBoundarySimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigModel(BigModelType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigModelSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLocation(BigLocationType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLocationSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigOrientation(BigOrientationType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigOrientationSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigScale(BigScaleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigScaleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigResourceMap(BigResourceMapType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigResourceMapSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAlias(BigAliasType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAliasSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigGroundOverlay(BigGroundOverlayType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigGroundOverlaySimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractLatLonBoxSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLatLonBox(BigLatLonBoxType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLatLonBoxSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigScreenOverlay(BigScreenOverlayType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigScreenOverlaySimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigPhotoOverlay(BigPhotoOverlayType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigPhotoOverlaySimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigViewVolume(BigViewVolumeType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigViewVolumeSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigImagePyramid(BigImagePyramidType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigImagePyramidSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigStyle(BigStyleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigStyleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigStyleMap(BigStyleMapType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigStyleMapSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigPair(BigPairType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigPairSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractSubStyleGroup(BigAbstractSubStyleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractSubStyleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractColorStyleGroup(BigAbstractColorStyleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigAbstractColorStyleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigIconStyle(BigIconStyleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigIconStyleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigBasicLinkSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLabelStyle(BigLabelStyleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLabelStyleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLineStyle(BigLineStyleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigLineStyleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigPolyStyle(BigPolyStyleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigPolyStyleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigBalloonStyle(BigBalloonStyleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigBalloonStyleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigListStyle(BigListStyleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigListStyleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigItemIcon(BigItemIconType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigItemIconSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigTimeStamp(BigTimeStampType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigTimeStampSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigTimeSpan(BigTimeSpanType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigTimeSpanSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigUpdate(BigUpdateType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigCreate(BigCreateType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigDelete(BigDeleteType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BigChange(BigChangeType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vec2Type {
    x: f64,
    y: f64,
    xunits: UnitsEnumType,
    yunits: UnitsEnumType,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigAbstractObjectType {
    #[serde(rename = "$value")]
    body: Vec<BigBigAbstractObjectTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BigBigAbstractObjectTypeBody {
    BigObjectSimpleExtensionGroup(BigObjectSimpleExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigAbstractFeatureType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigSnippetType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigAbstractViewType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigLookAtType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigCameraType {}
#[doc = "MetadataType deprecated in 2.2"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigMetadataType {
    #[serde(rename = "$value")]
    body: Vec<BigBigMetadataTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BigBigMetadataTypeBody {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigExtendedDataType {
    #[serde(rename = "$value")]
    body: Vec<BigBigExtendedDataTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BigBigExtendedDataTypeBody {
    BigData(BigData),
    BigSchemaData(BigSchemaData),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigSchemaDataType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigSimpleDataType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigDataType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigAbstractContainerType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigAbstractGeometryType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigAbstractOverlayType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigAbstractStyleSelectorType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigAbstractTimePrimitiveType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigKmlType {
    hint: String,
    #[serde(rename = "$value")]
    body: Vec<BigBigKmlTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BigBigKmlTypeBody {
    BigNetworkLinkControl(BigNetworkLinkControl),
    BigAbstractFeatureGroup(BigAbstractFeatureGroup),
    BigKmlSimpleExtensionGroup(BigKmlSimpleExtensionGroup),
    BigKmlObjectExtensionGroup(BigKmlObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigNetworkLinkControlType {
    #[serde(rename = "$value")]
    body: Vec<BigBigNetworkLinkControlTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BigBigNetworkLinkControlTypeBody {
    MinRefreshPeriod(MinRefreshPeriod),
    MaxSessionLength(MaxSessionLength),
    Cookie(Cookie),
    Message(Message),
    LinkName(LinkName),
    LinkDescription(LinkDescription),
    LinkSnippet(LinkSnippet),
    Expires(Expires),
    BigUpdate(BigUpdate),
    BigAbstractViewGroup(BigAbstractViewGroup),
    BigNetworkLinkControlSimpleExtensionGroup(BigNetworkLinkControlSimpleExtensionGroup),
    BigNetworkLinkControlObjectExtensionGroup(BigNetworkLinkControlObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigDocumentType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigSchemaType {
    name: String,
    id: BigId,
    #[serde(rename = "$value")]
    body: Vec<BigBigSchemaTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BigBigSchemaTypeBody {
    BigSimpleField(BigSimpleField),
    BigSchemaExtension(BigSchemaExtension),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigSimpleFieldType {
    r#type: String,
    name: String,
    #[serde(rename = "$value")]
    body: Vec<BigBigSimpleFieldTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BigBigSimpleFieldTypeBody {
    DisplayName(DisplayName),
    BigSimpleFieldExtension(BigSimpleFieldExtension),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigFolderType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigPlacemarkType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigNetworkLinkType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigRegionType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigLatLonAltBoxType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigLodType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigLinkType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigMultiGeometryType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigPointType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigLineStringType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigLinearRingType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigPolygonType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigBoundaryType {
    #[serde(rename = "$value")]
    body: Vec<BigBigBoundaryTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BigBigBoundaryTypeBody {
    BigLinearRing(BigLinearRing),
    BigBoundarySimpleExtensionGroup(BigBoundarySimpleExtensionGroup),
    BigBoundaryObjectExtensionGroup(BigBoundaryObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigModelType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigLocationType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigOrientationType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigScaleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigResourceMapType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigAliasType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigGroundOverlayType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigAbstractLatLonBoxType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigLatLonBoxType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigScreenOverlayType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigPhotoOverlayType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigViewVolumeType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigImagePyramidType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigStyleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigStyleMapType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigPairType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigAbstractSubStyleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigAbstractColorStyleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigIconStyleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigBasicLinkType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigLabelStyleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigLineStyleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigPolyStyleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigBalloonStyleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigListStyleType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigItemIconType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigTimeStampType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigTimeSpanType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigUpdateType {
    #[serde(rename = "$value")]
    body: Vec<BigBigUpdateTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BigBigUpdateTypeBody {
    TargetHref(TargetHref),
    BigUpdateExtensionGroup(BigUpdateExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigCreateType {
    #[serde(rename = "$value")]
    body: Vec<BigBigCreateTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BigBigCreateTypeBody {
    BigAbstractContainerGroup(BigAbstractContainerGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigDeleteType {
    #[serde(rename = "$value")]
    body: Vec<BigBigDeleteTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BigBigDeleteTypeBody {
    BigAbstractFeatureGroup(BigAbstractFeatureGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BigChangeType {
    #[serde(rename = "$value")]
    body: Vec<BigBigChangeTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BigBigChangeTypeBody {
    BigAbstractObjectGroup(BigAbstractObjectGroup),
}
