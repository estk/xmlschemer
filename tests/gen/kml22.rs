use chrono::{DateTime, Duration, FixedOffset};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Coordinates(CoordinatesType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Message(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseItemIcon(UpcaseItemIconType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseImagePyramidObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseTimeStampObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractOverlayObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseUpdateOpExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseBoundarySimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Cookie(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct OverlayXy(Vec2Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BottomFov(Angle90Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcasePolyStyleObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BgColor(ColorType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct SourceHref(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseMultiGeometry(UpcaseMultiGeometryType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MaxSessionLength(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLineStyle(UpcaseLineStyleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractLatLonBoxSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLineStringSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcasePlacemarkObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseNetworkLinkControlObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseCameraObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseExtendedData(UpcaseExtendedDataType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct LinkSnippet(UpcaseSnippetType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractViewObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseRegionSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Range(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLabelStyle(UpcaseLabelStyleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLatLonBoxObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcasePlacemark(UpcasePlacemarkType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Fill(bool);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractColorStyleGroup(UpcaseAbstractColorStyleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseViewVolumeSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseItemIconSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct X(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractViewSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseKmlSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseNetworkLinkControl(UpcaseNetworkLinkControlType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAlias(UpcaseAliasType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct TopFov(Angle90Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseResourceMapObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseViewVolume(UpcaseViewVolumeType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Value(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseGroundOverlaySimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct East(Angle180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct RotationXy(Vec2Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcasePolygonObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseSchemaExtension {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseStyleMapSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractObjectGroup(UpcaseAbstractObjectType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseStyleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLatLonAltBoxSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcasePairSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcasePoint(UpcasePointType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAliasObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseCameraSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLabelStyleObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseListStyleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractContainerGroup(UpcaseAbstractContainerType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct AltitudeModeGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseSchemaDataExtension {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Scale(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseUpdate(UpcaseUpdateType);
#[doc = "not anyURI due to $[x] substitution in\n      PhotoOverlay"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Href(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLatLonAltBox(UpcaseLatLonAltBoxType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseObjectSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcasePair(UpcasePairType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLatLonBox(UpcaseLatLonBoxType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseStyleObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseBalloonStyle(UpcaseBalloonStyleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseScreenOverlayObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Description(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Size(Vec2Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseBalloonStyleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MaxHeight(i32);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct LinkDescription(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcasePointObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Heading(Angle360Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcasePolyStyleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcasePointSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Address(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseOrientationSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MaxWidth(i32);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseRegionObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractFeatureSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Color(ColorType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Expires(DateTimeType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct TextColor(ColorType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct DisplayName(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseGroundOverlay(UpcaseGroundOverlayType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Name(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseBasicLinkObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseFolderSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct InnerBoundaryIs(UpcaseBoundaryType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseDocumentSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseDocumentObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLookAtObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseSimpleFieldExtension {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractTimePrimitiveGroup(UpcaseAbstractTimePrimitiveType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MinLodPixels(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct PhoneNumber(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct AltitudeMode(AltitudeModeEnumType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MinAltitude(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLineStyleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct OuterBoundaryIs(UpcaseBoundaryType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ViewFormat(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Text(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ScreenXy(Vec2Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Near(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseResourceMapSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct HotSpot(Vec2Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Outline(bool);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseTimeSpanSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Key(StyleStateEnumType);
#[doc = "<kml> is the root element."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Kml(UpcaseKmlType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLinkObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseStyleMapObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseItemIconObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractGeometryObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractStyleSelectorObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseScreenOverlaySimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseChange(UpcaseChangeType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseStyleMap(UpcaseStyleMapType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractColorStyleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct LeftFov(Angle180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct South(Angle180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractFeatureObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ViewBoundScale(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct RefreshInterval(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct TargetHref(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractContainerSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Latitude(Angle90Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseModelObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractSubStyleObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseListStyleObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractFeatureGroup(UpcaseAbstractFeatureType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractViewGroup(UpcaseAbstractViewType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLinearRing(UpcaseLinearRingType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseViewVolumeObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseImagePyramidSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseUpdateExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[doc = "Url deprecated in 2.2"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseUrl(UpcaseLinkType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseIconStyleObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Tilt(Anglepos180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLineStyleObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Open(bool);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MaxFadeExtent(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseModelSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcasePolyStyle(UpcasePolyStyleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct North(Angle180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct GridOrigin(GridOriginEnumType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcasePolygon(UpcasePolygonType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLod(UpcaseLodType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Longitude(Angle180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractLatLonBoxObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseIconStyleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct RightFov(Angle180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MinRefreshPeriod(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseNetworkLinkSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct DisplayMode(DisplayModeEnumType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseNetworkLinkControlSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Extrude(bool);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MaxAltitude(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseCreate(UpcaseCreateType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseResourceMap(UpcaseResourceMapType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractColorStyleObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseKmlObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct RefreshVisibility(bool);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Altitude(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseRegion(UpcaseRegionType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLodSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseFolder(UpcaseFolderType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Snippet(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct West(Angle180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Tessellate(bool);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct FlyToView(bool);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcasePairObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLocationObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseMultiGeometryObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcasePlacemarkSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ViewRefreshTime(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractContainerObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseIcon(UpcaseLinkType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseListStyle(UpcaseListStyleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractStyleSelectorGroup(UpcaseAbstractStyleSelectorType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseSchema(UpcaseSchemaType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct RefreshMode(RefreshModeEnumType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseFolderObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractOverlaySimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLatLonBoxSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLabelStyleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseMultiGeometrySimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseNetworkLinkObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MaxSnippetLines(i32);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLatLonAltBoxObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Y(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseGroundOverlayObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseTimeStampSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseNetworkLink(UpcaseNetworkLinkType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseIconStyle(UpcaseIconStyleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseScale(UpcaseScaleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcasePhotoOverlaySimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseScaleObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseTimeStamp(UpcaseTimeStampType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractGeometryGroup(UpcaseAbstractGeometryType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseScaleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseOrientation(UpcaseOrientationType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseScreenOverlay(UpcaseScreenOverlayType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcasePolygonSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ViewRefreshMode(ViewRefreshModeEnumType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLink(UpcaseLinkType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Visibility(bool);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseTimeSpan(UpcaseTimeSpanType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractTimePrimitiveObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractSubStyleGroup(UpcaseAbstractSubStyleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLocation(UpcaseLocationType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLinkSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseSimpleField(UpcaseSimpleFieldType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseDelete(UpcaseDeleteType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Z(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct LinkName(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseDocument(UpcaseDocumentType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseBoundaryObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLookAtSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractTimePrimitiveSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAliasSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseDataExtension {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseStyle(UpcaseStyleType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLineStringObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseOrientationObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseSimpleData(UpcaseSimpleDataType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Roll(Angle180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct StyleUrl(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLookAt(UpcaseLookAtType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseBalloonStyleObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseSnippet(UpcaseSnippetType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseModel(UpcaseModelType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseTimeSpanObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractOverlayGroup(UpcaseAbstractOverlayType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct State(ItemIconStateType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractGeometrySimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct End(DateTimeType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ColorMode(ColorModeEnumType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct DrawOrder(i32);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseBasicLinkSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLinearRingObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractSubStyleSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseCamera(UpcaseCameraType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Begin(DateTimeType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLineString(UpcaseLineStringType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseSchemaData(UpcaseSchemaDataType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Rotation(Angle180Type);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct When(DateTimeType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLinearRingSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct HttpQuery(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Shape(ShapeEnumType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLodObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcasePhotoOverlay(UpcasePhotoOverlayType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseLocationSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MaxLodPixels(f64);
#[doc = "Metadata deprecated in 2.2"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseMetadata(UpcaseMetadataType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseAbstractStyleSelectorSimpleExtensionGroup(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ListItemType(ListItemTypeEnumType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Width(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseData(UpcaseDataType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcasePhotoOverlayObjectExtensionGroup {
    #[serde(flatten)]
    other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct TileSize(i32);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpcaseImagePyramid(UpcaseImagePyramidType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MinFadeExtent(f64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseLinkType {
    #[serde(rename = "$value")]
    body: UpcaseLinkTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseLinkTypeBody {
    base: UpcaseBasicLinkType,
    body: UpcaseLinkTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseLinkTypeBodyExtension {
    RefreshMode(RefreshMode),
    RefreshInterval(RefreshInterval),
    ViewRefreshMode(ViewRefreshMode),
    ViewRefreshTime(ViewRefreshTime),
    ViewBoundScale(ViewBoundScale),
    ViewFormat(ViewFormat),
    HttpQuery(HttpQuery),
    UpcaseLinkSimpleExtensionGroup(UpcaseLinkSimpleExtensionGroup),
    UpcaseLinkObjectExtensionGroup(UpcaseLinkObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseScreenOverlayType {
    #[serde(rename = "$value")]
    body: UpcaseScreenOverlayTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseScreenOverlayTypeBody {
    base: UpcaseAbstractOverlayType,
    body: UpcaseScreenOverlayTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseScreenOverlayTypeBodyExtension {
    OverlayXy(OverlayXy),
    ScreenXy(ScreenXy),
    RotationXy(RotationXy),
    Size(Size),
    Rotation(Rotation),
    UpcaseScreenOverlaySimpleExtensionGroup(UpcaseScreenOverlaySimpleExtensionGroup),
    UpcaseScreenOverlayObjectExtensionGroup(UpcaseScreenOverlayObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseOrientationType {
    #[serde(rename = "$value")]
    body: UpcaseOrientationTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseOrientationTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseOrientationTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseOrientationTypeBodyExtension {
    Heading(Heading),
    Tilt(Tilt),
    Roll(Roll),
    UpcaseOrientationSimpleExtensionGroup(UpcaseOrientationSimpleExtensionGroup),
    UpcaseOrientationObjectExtensionGroup(UpcaseOrientationObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseLatLonBoxType {
    #[serde(rename = "$value")]
    body: UpcaseLatLonBoxTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseLatLonBoxTypeBody {
    base: UpcaseAbstractLatLonBoxType,
    body: UpcaseLatLonBoxTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseLatLonBoxTypeBodyExtension {
    Rotation(Rotation),
    UpcaseLatLonBoxSimpleExtensionGroup(UpcaseLatLonBoxSimpleExtensionGroup),
    UpcaseLatLonBoxObjectExtensionGroup(UpcaseLatLonBoxObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseAliasType {
    #[serde(rename = "$value")]
    body: UpcaseAliasTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseAliasTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseAliasTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseAliasTypeBodyExtension {
    TargetHref(TargetHref),
    SourceHref(SourceHref),
    UpcaseAliasSimpleExtensionGroup(UpcaseAliasSimpleExtensionGroup),
    UpcaseAliasObjectExtensionGroup(UpcaseAliasObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcasePlacemarkType {
    #[serde(rename = "$value")]
    body: UpcasePlacemarkTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcasePlacemarkTypeBody {
    base: UpcaseAbstractFeatureType,
    body: UpcasePlacemarkTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcasePlacemarkTypeBodyExtension {
    UpcaseAbstractGeometryGroup(UpcaseAbstractGeometryGroup),
    UpcasePlacemarkSimpleExtensionGroup(UpcasePlacemarkSimpleExtensionGroup),
    UpcasePlacemarkObjectExtensionGroup(UpcasePlacemarkObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParentAbstractFeatureType {
    #[serde(rename = "$value")]
    body: UpcaseParentAbstractFeatureTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseParentAbstractFeatureTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseParentAbstractFeatureTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseParentAbstractFeatureTypeBodyExtension {
    Name(Name),
    Visibility(Visibility),
    Open(Open),
    Author(atom::Author),
    Link(atom::Link),
    Address(Address),
    UpcaseAddressDetails(xal::UpcaseAddressDetails),
    PhoneNumber(PhoneNumber),
    Description(Description),
    UpcaseAbstractViewGroup(UpcaseAbstractViewGroup),
    UpcaseAbstractTimePrimitiveGroup(UpcaseAbstractTimePrimitiveGroup),
    StyleUrl(StyleUrl),
    UpcaseAbstractStyleSelectorGroup(UpcaseAbstractStyleSelectorGroup),
    UpcaseRegion(UpcaseRegion),
    UpcaseAbstractFeatureSimpleExtensionGroup(UpcaseAbstractFeatureSimpleExtensionGroup),
    UpcaseAbstractFeatureObjectExtensionGroup(UpcaseAbstractFeatureObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseLocationType {
    #[serde(rename = "$value")]
    body: UpcaseLocationTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseLocationTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseLocationTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseLocationTypeBodyExtension {
    Longitude(Longitude),
    Latitude(Latitude),
    Altitude(Altitude),
    UpcaseLocationSimpleExtensionGroup(UpcaseLocationSimpleExtensionGroup),
    UpcaseLocationObjectExtensionGroup(UpcaseLocationObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseFolderType {
    #[serde(rename = "$value")]
    body: UpcaseFolderTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseFolderTypeBody {
    base: UpcaseAbstractContainerType,
    body: UpcaseFolderTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseFolderTypeBodyExtension {
    UpcaseAbstractFeatureGroup(UpcaseAbstractFeatureGroup),
    UpcaseFolderSimpleExtensionGroup(UpcaseFolderSimpleExtensionGroup),
    UpcaseFolderObjectExtensionGroup(UpcaseFolderObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseSchemaDataType {
    #[serde(rename = "$value")]
    body: UpcaseSchemaDataTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseSchemaDataTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseSchemaDataTypeBodyExtension,
    schemaUrl: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseSchemaDataTypeBodyExtension {
    UpcaseSimpleData(UpcaseSimpleData),
    UpcaseSchemaDataExtension(UpcaseSchemaDataExtension),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseLabelStyleType {
    #[serde(rename = "$value")]
    body: UpcaseLabelStyleTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseLabelStyleTypeBody {
    base: UpcaseAbstractColorStyleType,
    body: UpcaseLabelStyleTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseLabelStyleTypeBodyExtension {
    Scale(Scale),
    UpcaseLabelStyleSimpleExtensionGroup(UpcaseLabelStyleSimpleExtensionGroup),
    UpcaseLabelStyleObjectExtensionGroup(UpcaseLabelStyleObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcasePhotoOverlayType {
    #[serde(rename = "$value")]
    body: UpcasePhotoOverlayTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcasePhotoOverlayTypeBody {
    base: UpcaseAbstractOverlayType,
    body: UpcasePhotoOverlayTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcasePhotoOverlayTypeBodyExtension {
    Rotation(Rotation),
    UpcaseViewVolume(UpcaseViewVolume),
    UpcaseImagePyramid(UpcaseImagePyramid),
    UpcasePoint(UpcasePoint),
    Shape(Shape),
    UpcasePhotoOverlaySimpleExtensionGroup(UpcasePhotoOverlaySimpleExtensionGroup),
    UpcasePhotoOverlayObjectExtensionGroup(UpcasePhotoOverlayObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseStyleMapType {
    #[serde(rename = "$value")]
    body: UpcaseStyleMapTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseStyleMapTypeBody {
    base: UpcaseAbstractStyleSelectorType,
    body: UpcaseStyleMapTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseStyleMapTypeBodyExtension {
    UpcasePair(UpcasePair),
    UpcaseStyleMapSimpleExtensionGroup(UpcaseStyleMapSimpleExtensionGroup),
    UpcaseStyleMapObjectExtensionGroup(UpcaseStyleMapObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParentAbstractContainerType {
    #[serde(rename = "$value")]
    body: UpcaseParentAbstractContainerTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseParentAbstractContainerTypeBody {
    base: UpcaseAbstractFeatureType,
    body: UpcaseParentAbstractContainerTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseParentAbstractContainerTypeBodyExtension {
    UpcaseAbstractContainerSimpleExtensionGroup(UpcaseAbstractContainerSimpleExtensionGroup),
    UpcaseAbstractContainerObjectExtensionGroup(UpcaseAbstractContainerObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseSimpleDataType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseLineStringType {
    #[serde(rename = "$value")]
    body: UpcaseLineStringTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseLineStringTypeBody {
    base: UpcaseAbstractGeometryType,
    body: UpcaseLineStringTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseLineStringTypeBodyExtension {
    Extrude(Extrude),
    Tessellate(Tessellate),
    AltitudeModeGroup(AltitudeModeGroup),
    Coordinates(Coordinates),
    UpcaseLineStringSimpleExtensionGroup(UpcaseLineStringSimpleExtensionGroup),
    UpcaseLineStringObjectExtensionGroup(UpcaseLineStringObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseRegionType {
    #[serde(rename = "$value")]
    body: UpcaseRegionTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseRegionTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseRegionTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseRegionTypeBodyExtension {
    UpcaseLatLonAltBox(UpcaseLatLonAltBox),
    UpcaseLod(UpcaseLod),
    UpcaseRegionSimpleExtensionGroup(UpcaseRegionSimpleExtensionGroup),
    UpcaseRegionObjectExtensionGroup(UpcaseRegionObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseNetworkLinkType {
    #[serde(rename = "$value")]
    body: UpcaseNetworkLinkTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseNetworkLinkTypeBody {
    base: UpcaseAbstractFeatureType,
    body: UpcaseNetworkLinkTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseNetworkLinkTypeBodyExtension {
    RefreshVisibility(RefreshVisibility),
    FlyToView(FlyToView),
    UpcaseNetworkLinkSimpleExtensionGroup(UpcaseNetworkLinkSimpleExtensionGroup),
    UpcaseNetworkLinkObjectExtensionGroup(UpcaseNetworkLinkObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParentAbstractGeometryType {
    #[serde(rename = "$value")]
    body: UpcaseParentAbstractGeometryTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseParentAbstractGeometryTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseParentAbstractGeometryTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseParentAbstractGeometryTypeBodyExtension {
    UpcaseAbstractGeometrySimpleExtensionGroup(UpcaseAbstractGeometrySimpleExtensionGroup),
    UpcaseAbstractGeometryObjectExtensionGroup(UpcaseAbstractGeometryObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParentAbstractSubStyleType {
    #[serde(rename = "$value")]
    body: UpcaseParentAbstractSubStyleTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseParentAbstractSubStyleTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseParentAbstractSubStyleTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseParentAbstractSubStyleTypeBodyExtension {
    UpcaseAbstractSubStyleSimpleExtensionGroup(UpcaseAbstractSubStyleSimpleExtensionGroup),
    UpcaseAbstractSubStyleObjectExtensionGroup(UpcaseAbstractSubStyleObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseCameraType {
    #[serde(rename = "$value")]
    body: UpcaseCameraTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseCameraTypeBody {
    base: UpcaseAbstractViewType,
    body: UpcaseCameraTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseCameraTypeBodyExtension {
    Longitude(Longitude),
    Latitude(Latitude),
    Altitude(Altitude),
    Heading(Heading),
    Tilt(Tilt),
    Roll(Roll),
    AltitudeModeGroup(AltitudeModeGroup),
    UpcaseCameraSimpleExtensionGroup(UpcaseCameraSimpleExtensionGroup),
    UpcaseCameraObjectExtensionGroup(UpcaseCameraObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseBoundaryType {
    #[serde(rename = "$value")]
    body: Vec<UpcaseBoundaryTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseBoundaryTypeBody {
    UpcaseLinearRing(UpcaseLinearRing),
    UpcaseBoundarySimpleExtensionGroup(UpcaseBoundarySimpleExtensionGroup),
    UpcaseBoundaryObjectExtensionGroup(UpcaseBoundaryObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParentAbstractOverlayType {
    #[serde(rename = "$value")]
    body: UpcaseParentAbstractOverlayTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseParentAbstractOverlayTypeBody {
    base: UpcaseAbstractFeatureType,
    body: UpcaseParentAbstractOverlayTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseParentAbstractOverlayTypeBodyExtension {
    Color(Color),
    DrawOrder(DrawOrder),
    UpcaseIcon(UpcaseIcon),
    UpcaseAbstractOverlaySimpleExtensionGroup(UpcaseAbstractOverlaySimpleExtensionGroup),
    UpcaseAbstractOverlayObjectExtensionGroup(UpcaseAbstractOverlayObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParentAbstractColorStyleType {
    #[serde(rename = "$value")]
    body: UpcaseParentAbstractColorStyleTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseParentAbstractColorStyleTypeBody {
    base: UpcaseAbstractSubStyleType,
    body: UpcaseParentAbstractColorStyleTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseParentAbstractColorStyleTypeBodyExtension {
    Color(Color),
    ColorMode(ColorMode),
    UpcaseAbstractColorStyleSimpleExtensionGroup(UpcaseAbstractColorStyleSimpleExtensionGroup),
    UpcaseAbstractColorStyleObjectExtensionGroup(UpcaseAbstractColorStyleObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseLatLonAltBoxType {
    #[serde(rename = "$value")]
    body: UpcaseLatLonAltBoxTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseLatLonAltBoxTypeBody {
    base: UpcaseAbstractLatLonBoxType,
    body: UpcaseLatLonAltBoxTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseLatLonAltBoxTypeBodyExtension {
    MinAltitude(MinAltitude),
    MaxAltitude(MaxAltitude),
    AltitudeModeGroup(AltitudeModeGroup),
    UpcaseLatLonAltBoxSimpleExtensionGroup(UpcaseLatLonAltBoxSimpleExtensionGroup),
    UpcaseLatLonAltBoxObjectExtensionGroup(UpcaseLatLonAltBoxObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseImagePyramidType {
    #[serde(rename = "$value")]
    body: UpcaseImagePyramidTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseImagePyramidTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseImagePyramidTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseImagePyramidTypeBodyExtension {
    TileSize(TileSize),
    MaxWidth(MaxWidth),
    MaxHeight(MaxHeight),
    GridOrigin(GridOrigin),
    UpcaseImagePyramidSimpleExtensionGroup(UpcaseImagePyramidSimpleExtensionGroup),
    UpcaseImagePyramidObjectExtensionGroup(UpcaseImagePyramidObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseModelType {
    #[serde(rename = "$value")]
    body: UpcaseModelTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseModelTypeBody {
    base: UpcaseAbstractGeometryType,
    body: UpcaseModelTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseModelTypeBodyExtension {
    AltitudeModeGroup(AltitudeModeGroup),
    UpcaseLocation(UpcaseLocation),
    UpcaseOrientation(UpcaseOrientation),
    UpcaseScale(UpcaseScale),
    UpcaseLink(UpcaseLink),
    UpcaseResourceMap(UpcaseResourceMap),
    UpcaseModelSimpleExtensionGroup(UpcaseModelSimpleExtensionGroup),
    UpcaseModelObjectExtensionGroup(UpcaseModelObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseBalloonStyleType {
    #[serde(rename = "$value")]
    body: UpcaseBalloonStyleTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseBalloonStyleTypeBody {
    base: UpcaseAbstractSubStyleType,
    body: UpcaseBalloonStyleTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseBalloonStyleTypeBodyExtension {
    TextColor(TextColor),
    Text(Text),
    DisplayMode(DisplayMode),
    UpcaseBalloonStyleSimpleExtensionGroup(UpcaseBalloonStyleSimpleExtensionGroup),
    UpcaseBalloonStyleObjectExtensionGroup(UpcaseBalloonStyleObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParentAbstractTimePrimitiveType {
    #[serde(rename = "$value")]
    body: UpcaseParentAbstractTimePrimitiveTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseParentAbstractTimePrimitiveTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseParentAbstractTimePrimitiveTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseParentAbstractTimePrimitiveTypeBodyExtension {
    UpcaseAbstractTimePrimitiveSimpleExtensionGroup(
        UpcaseAbstractTimePrimitiveSimpleExtensionGroup,
    ),
    UpcaseAbstractTimePrimitiveObjectExtensionGroup(
        UpcaseAbstractTimePrimitiveObjectExtensionGroup,
    ),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcasePairType {
    #[serde(rename = "$value")]
    body: UpcasePairTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcasePairTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcasePairTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcasePairTypeBodyExtension {
    Key(Key),
    StyleUrl(StyleUrl),
    UpcaseAbstractStyleSelectorGroup(UpcaseAbstractStyleSelectorGroup),
    UpcasePairSimpleExtensionGroup(UpcasePairSimpleExtensionGroup),
    UpcasePairObjectExtensionGroup(UpcasePairObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParentAbstractViewType {
    #[serde(rename = "$value")]
    body: UpcaseParentAbstractViewTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseParentAbstractViewTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseParentAbstractViewTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseParentAbstractViewTypeBodyExtension {
    UpcaseAbstractViewSimpleExtensionGroup(UpcaseAbstractViewSimpleExtensionGroup),
    UpcaseAbstractViewObjectExtensionGroup(UpcaseAbstractViewObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcasePolygonType {
    #[serde(rename = "$value")]
    body: UpcasePolygonTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcasePolygonTypeBody {
    base: UpcaseAbstractGeometryType,
    body: UpcasePolygonTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcasePolygonTypeBodyExtension {
    Extrude(Extrude),
    Tessellate(Tessellate),
    AltitudeModeGroup(AltitudeModeGroup),
    OuterBoundaryIs(OuterBoundaryIs),
    InnerBoundaryIs(InnerBoundaryIs),
    UpcasePolygonSimpleExtensionGroup(UpcasePolygonSimpleExtensionGroup),
    UpcasePolygonObjectExtensionGroup(UpcasePolygonObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParentAbstractObjectType {
    #[serde(rename = "$value")]
    body: Vec<UpcaseParentAbstractObjectTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseParentAbstractObjectTypeBody {
    UpcaseObjectSimpleExtensionGroup(UpcaseObjectSimpleExtensionGroup),
}
#[doc = "MetadataType deprecated in 2.2"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseMetadataType {
    #[serde(rename = "$value")]
    body: Vec<UpcaseMetadataTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseMetadataTypeBody {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseTimeStampType {
    #[serde(rename = "$value")]
    body: UpcaseTimeStampTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseTimeStampTypeBody {
    base: UpcaseAbstractTimePrimitiveType,
    body: UpcaseTimeStampTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseTimeStampTypeBodyExtension {
    When(When),
    UpcaseTimeStampSimpleExtensionGroup(UpcaseTimeStampSimpleExtensionGroup),
    UpcaseTimeStampObjectExtensionGroup(UpcaseTimeStampObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseIconStyleType {
    #[serde(rename = "$value")]
    body: UpcaseIconStyleTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseIconStyleTypeBody {
    base: UpcaseAbstractColorStyleType,
    body: UpcaseIconStyleTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseIconStyleTypeBodyExtension {
    Scale(Scale),
    Heading(Heading),
    UpcaseIcon(UpcaseBasicLinkType),
    HotSpot(HotSpot),
    UpcaseIconStyleSimpleExtensionGroup(UpcaseIconStyleSimpleExtensionGroup),
    UpcaseIconStyleObjectExtensionGroup(UpcaseIconStyleObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcasePointType {
    #[serde(rename = "$value")]
    body: UpcasePointTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcasePointTypeBody {
    base: UpcaseAbstractGeometryType,
    body: UpcasePointTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcasePointTypeBodyExtension {
    Extrude(Extrude),
    AltitudeModeGroup(AltitudeModeGroup),
    Coordinates(Coordinates),
    UpcasePointSimpleExtensionGroup(UpcasePointSimpleExtensionGroup),
    UpcasePointObjectExtensionGroup(UpcasePointObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseMultiGeometryType {
    #[serde(rename = "$value")]
    body: UpcaseMultiGeometryTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseMultiGeometryTypeBody {
    base: UpcaseAbstractGeometryType,
    body: UpcaseMultiGeometryTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseMultiGeometryTypeBodyExtension {
    UpcaseAbstractGeometryGroup(UpcaseAbstractGeometryGroup),
    UpcaseMultiGeometrySimpleExtensionGroup(UpcaseMultiGeometrySimpleExtensionGroup),
    UpcaseMultiGeometryObjectExtensionGroup(UpcaseMultiGeometryObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseSchemaType {
    name: String,
    id: String,
    #[serde(rename = "$value")]
    body: Vec<UpcaseSchemaTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseSchemaTypeBody {
    UpcaseSimpleField(UpcaseSimpleField),
    UpcaseSchemaExtension(UpcaseSchemaExtension),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseNetworkLinkControlType {
    #[serde(rename = "$value")]
    body: Vec<UpcaseNetworkLinkControlTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseNetworkLinkControlTypeBody {
    MinRefreshPeriod(MinRefreshPeriod),
    MaxSessionLength(MaxSessionLength),
    Cookie(Cookie),
    Message(Message),
    LinkName(LinkName),
    LinkDescription(LinkDescription),
    LinkSnippet(LinkSnippet),
    Expires(Expires),
    UpcaseUpdate(UpcaseUpdate),
    UpcaseAbstractViewGroup(UpcaseAbstractViewGroup),
    UpcaseNetworkLinkControlSimpleExtensionGroup(UpcaseNetworkLinkControlSimpleExtensionGroup),
    UpcaseNetworkLinkControlObjectExtensionGroup(UpcaseNetworkLinkControlObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseKmlType {
    hint: String,
    #[serde(rename = "$value")]
    body: Vec<UpcaseKmlTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseKmlTypeBody {
    UpcaseNetworkLinkControl(UpcaseNetworkLinkControl),
    UpcaseAbstractFeatureGroup(UpcaseAbstractFeatureGroup),
    UpcaseKmlSimpleExtensionGroup(UpcaseKmlSimpleExtensionGroup),
    UpcaseKmlObjectExtensionGroup(UpcaseKmlObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseItemIconType {
    #[serde(rename = "$value")]
    body: UpcaseItemIconTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseItemIconTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseItemIconTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseItemIconTypeBodyExtension {
    State(State),
    Href(Href),
    UpcaseItemIconSimpleExtensionGroup(UpcaseItemIconSimpleExtensionGroup),
    UpcaseItemIconObjectExtensionGroup(UpcaseItemIconObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseDataType {
    #[serde(rename = "$value")]
    body: UpcaseDataTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseDataTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseDataTypeBodyExtension,
    name: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseDataTypeBodyExtension {
    DisplayName(DisplayName),
    Value(Value),
    UpcaseDataExtension(UpcaseDataExtension),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseDocumentType {
    #[serde(rename = "$value")]
    body: UpcaseDocumentTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseDocumentTypeBody {
    base: UpcaseAbstractContainerType,
    body: UpcaseDocumentTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseDocumentTypeBodyExtension {
    UpcaseSchema(UpcaseSchema),
    UpcaseAbstractFeatureGroup(UpcaseAbstractFeatureGroup),
    UpcaseDocumentSimpleExtensionGroup(UpcaseDocumentSimpleExtensionGroup),
    UpcaseDocumentObjectExtensionGroup(UpcaseDocumentObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseUpdateType {
    #[serde(rename = "$value")]
    body: Vec<UpcaseUpdateTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseUpdateTypeBody {
    TargetHref(TargetHref),
    UpcaseUpdateExtensionGroup(UpcaseUpdateExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseSimpleFieldType {
    r#type: String,
    name: String,
    #[serde(rename = "$value")]
    body: Vec<UpcaseSimpleFieldTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseSimpleFieldTypeBody {
    DisplayName(DisplayName),
    UpcaseSimpleFieldExtension(UpcaseSimpleFieldExtension),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseListStyleType {
    #[serde(rename = "$value")]
    body: UpcaseListStyleTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseListStyleTypeBody {
    base: UpcaseAbstractSubStyleType,
    body: UpcaseListStyleTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseListStyleTypeBodyExtension {
    ListItemType(ListItemType),
    BgColor(BgColor),
    UpcaseItemIcon(UpcaseItemIcon),
    MaxSnippetLines(MaxSnippetLines),
    UpcaseListStyleSimpleExtensionGroup(UpcaseListStyleSimpleExtensionGroup),
    UpcaseListStyleObjectExtensionGroup(UpcaseListStyleObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseLinearRingType {
    #[serde(rename = "$value")]
    body: UpcaseLinearRingTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseLinearRingTypeBody {
    base: UpcaseAbstractGeometryType,
    body: UpcaseLinearRingTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseLinearRingTypeBodyExtension {
    Extrude(Extrude),
    Tessellate(Tessellate),
    AltitudeModeGroup(AltitudeModeGroup),
    Coordinates(Coordinates),
    UpcaseLinearRingSimpleExtensionGroup(UpcaseLinearRingSimpleExtensionGroup),
    UpcaseLinearRingObjectExtensionGroup(UpcaseLinearRingObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseLodType {
    #[serde(rename = "$value")]
    body: UpcaseLodTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseLodTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseLodTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseLodTypeBodyExtension {
    MinLodPixels(MinLodPixels),
    MaxLodPixels(MaxLodPixels),
    MinFadeExtent(MinFadeExtent),
    MaxFadeExtent(MaxFadeExtent),
    UpcaseLodSimpleExtensionGroup(UpcaseLodSimpleExtensionGroup),
    UpcaseLodObjectExtensionGroup(UpcaseLodObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseChangeType {
    #[serde(rename = "$value")]
    body: Vec<UpcaseChangeTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseChangeTypeBody {
    UpcaseAbstractObjectGroup(UpcaseAbstractObjectGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseGroundOverlayType {
    #[serde(rename = "$value")]
    body: UpcaseGroundOverlayTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseGroundOverlayTypeBody {
    base: UpcaseAbstractOverlayType,
    body: UpcaseGroundOverlayTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseGroundOverlayTypeBodyExtension {
    Altitude(Altitude),
    AltitudeModeGroup(AltitudeModeGroup),
    UpcaseLatLonBox(UpcaseLatLonBox),
    UpcaseGroundOverlaySimpleExtensionGroup(UpcaseGroundOverlaySimpleExtensionGroup),
    UpcaseGroundOverlayObjectExtensionGroup(UpcaseGroundOverlayObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseViewVolumeType {
    #[serde(rename = "$value")]
    body: UpcaseViewVolumeTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseViewVolumeTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseViewVolumeTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseViewVolumeTypeBodyExtension {
    LeftFov(LeftFov),
    RightFov(RightFov),
    BottomFov(BottomFov),
    TopFov(TopFov),
    Near(Near),
    UpcaseViewVolumeSimpleExtensionGroup(UpcaseViewVolumeSimpleExtensionGroup),
    UpcaseViewVolumeObjectExtensionGroup(UpcaseViewVolumeObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParentBasicLinkType {
    #[serde(rename = "$value")]
    body: UpcaseParentBasicLinkTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseParentBasicLinkTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseParentBasicLinkTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseParentBasicLinkTypeBodyExtension {
    Href(Href),
    UpcaseBasicLinkSimpleExtensionGroup(UpcaseBasicLinkSimpleExtensionGroup),
    UpcaseBasicLinkObjectExtensionGroup(UpcaseBasicLinkObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseLookAtType {
    #[serde(rename = "$value")]
    body: UpcaseLookAtTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseLookAtTypeBody {
    base: UpcaseAbstractViewType,
    body: UpcaseLookAtTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseLookAtTypeBodyExtension {
    Longitude(Longitude),
    Latitude(Latitude),
    Altitude(Altitude),
    Heading(Heading),
    Tilt(Tilt),
    Range(Range),
    AltitudeModeGroup(AltitudeModeGroup),
    UpcaseLookAtSimpleExtensionGroup(UpcaseLookAtSimpleExtensionGroup),
    UpcaseLookAtObjectExtensionGroup(UpcaseLookAtObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParentAbstractLatLonBoxType {
    #[serde(rename = "$value")]
    body: UpcaseParentAbstractLatLonBoxTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseParentAbstractLatLonBoxTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseParentAbstractLatLonBoxTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseParentAbstractLatLonBoxTypeBodyExtension {
    North(North),
    South(South),
    East(East),
    West(West),
    UpcaseAbstractLatLonBoxSimpleExtensionGroup(UpcaseAbstractLatLonBoxSimpleExtensionGroup),
    UpcaseAbstractLatLonBoxObjectExtensionGroup(UpcaseAbstractLatLonBoxObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseResourceMapType {
    #[serde(rename = "$value")]
    body: UpcaseResourceMapTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseResourceMapTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseResourceMapTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseResourceMapTypeBodyExtension {
    UpcaseAlias(UpcaseAlias),
    UpcaseResourceMapSimpleExtensionGroup(UpcaseResourceMapSimpleExtensionGroup),
    UpcaseResourceMapObjectExtensionGroup(UpcaseResourceMapObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseTimeSpanType {
    #[serde(rename = "$value")]
    body: UpcaseTimeSpanTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseTimeSpanTypeBody {
    base: UpcaseAbstractTimePrimitiveType,
    body: UpcaseTimeSpanTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseTimeSpanTypeBodyExtension {
    Begin(Begin),
    End(End),
    UpcaseTimeSpanSimpleExtensionGroup(UpcaseTimeSpanSimpleExtensionGroup),
    UpcaseTimeSpanObjectExtensionGroup(UpcaseTimeSpanObjectExtensionGroup),
}
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
pub struct UpcaseExtendedDataType {
    #[serde(rename = "$value")]
    body: Vec<UpcaseExtendedDataTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseExtendedDataTypeBody {
    UpcaseData(UpcaseData),
    UpcaseSchemaData(UpcaseSchemaData),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseStyleType {
    #[serde(rename = "$value")]
    body: UpcaseStyleTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseStyleTypeBody {
    base: UpcaseAbstractStyleSelectorType,
    body: UpcaseStyleTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseStyleTypeBodyExtension {
    UpcaseIconStyle(UpcaseIconStyle),
    UpcaseLabelStyle(UpcaseLabelStyle),
    UpcaseLineStyle(UpcaseLineStyle),
    UpcasePolyStyle(UpcasePolyStyle),
    UpcaseBalloonStyle(UpcaseBalloonStyle),
    UpcaseListStyle(UpcaseListStyle),
    UpcaseStyleSimpleExtensionGroup(UpcaseStyleSimpleExtensionGroup),
    UpcaseStyleObjectExtensionGroup(UpcaseStyleObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseScaleType {
    #[serde(rename = "$value")]
    body: UpcaseScaleTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseScaleTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseScaleTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseScaleTypeBodyExtension {
    X(X),
    Y(Y),
    Z(Z),
    UpcaseScaleSimpleExtensionGroup(UpcaseScaleSimpleExtensionGroup),
    UpcaseScaleObjectExtensionGroup(UpcaseScaleObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParentAbstractStyleSelectorType {
    #[serde(rename = "$value")]
    body: UpcaseParentAbstractStyleSelectorTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseParentAbstractStyleSelectorTypeBody {
    base: UpcaseAbstractObjectType,
    body: UpcaseParentAbstractStyleSelectorTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseParentAbstractStyleSelectorTypeBodyExtension {
    UpcaseAbstractStyleSelectorSimpleExtensionGroup(
        UpcaseAbstractStyleSelectorSimpleExtensionGroup,
    ),
    UpcaseAbstractStyleSelectorObjectExtensionGroup(
        UpcaseAbstractStyleSelectorObjectExtensionGroup,
    ),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseLineStyleType {
    #[serde(rename = "$value")]
    body: UpcaseLineStyleTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcaseLineStyleTypeBody {
    base: UpcaseAbstractColorStyleType,
    body: UpcaseLineStyleTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseLineStyleTypeBodyExtension {
    Width(Width),
    UpcaseLineStyleSimpleExtensionGroup(UpcaseLineStyleSimpleExtensionGroup),
    UpcaseLineStyleObjectExtensionGroup(UpcaseLineStyleObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcasePolyStyleType {
    #[serde(rename = "$value")]
    body: UpcasePolyStyleTypeBody,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpcasePolyStyleTypeBody {
    base: UpcaseAbstractColorStyleType,
    body: UpcasePolyStyleTypeBodyExtension,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcasePolyStyleTypeBodyExtension {
    Fill(Fill),
    Outline(Outline),
    UpcasePolyStyleSimpleExtensionGroup(UpcasePolyStyleSimpleExtensionGroup),
    UpcasePolyStyleObjectExtensionGroup(UpcasePolyStyleObjectExtensionGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseCreateType {
    #[serde(rename = "$value")]
    body: Vec<UpcaseCreateTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseCreateTypeBody {
    UpcaseAbstractContainerGroup(UpcaseAbstractContainerGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseDeleteType {
    #[serde(rename = "$value")]
    body: Vec<UpcaseDeleteTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseDeleteTypeBody {
    UpcaseAbstractFeatureGroup(UpcaseAbstractFeatureGroup),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpcaseSnippetType {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct AltitudeModeEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct CoordinatesType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Angle90Type(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct DateTimeType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct RefreshModeEnumType(String);
#[doc = "aabbggrr\n        \n        ffffffff: opaque white\n        ff000000: opaque black"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ColorType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct StyleStateEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct GridOriginEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct DisplayModeEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Anglepos180Type(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ItemIconStateType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ViewRefreshModeEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ShapeEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ColorModeEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UnitsEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Anglepos90Type(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Angle180Type(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Angle360Type(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ListItemTypeEnumType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ItemIconStateEnumType(String);
enum UpcaseAbstractViewType {
    ParentAbstractViewType(ParentAbstractViewType),
    UpcaseCameraType(UpcaseCameraType),
    UpcaseLookAtType(UpcaseLookAtType),
}
enum UpcaseAbstractContainerType {
    ParentAbstractContainerType(ParentAbstractContainerType),
    UpcaseFolderType(UpcaseFolderType),
    UpcaseDocumentType(UpcaseDocumentType),
}
enum UpcaseAbstractTimePrimitiveType {
    ParentAbstractTimePrimitiveType(ParentAbstractTimePrimitiveType),
    UpcaseTimeStampType(UpcaseTimeStampType),
    UpcaseTimeSpanType(UpcaseTimeSpanType),
}
enum UpcaseAbstractOverlayType {
    ParentAbstractOverlayType(ParentAbstractOverlayType),
    UpcaseScreenOverlayType(UpcaseScreenOverlayType),
    UpcasePhotoOverlayType(UpcasePhotoOverlayType),
    UpcaseGroundOverlayType(UpcaseGroundOverlayType),
}
enum UpcaseAbstractFeatureType {
    ParentAbstractFeatureType(ParentAbstractFeatureType),
    UpcasePlacemarkType(UpcasePlacemarkType),
    ParentAbstractContainerType(ParentAbstractContainerType),
    UpcaseNetworkLinkType(UpcaseNetworkLinkType),
    ParentAbstractOverlayType(ParentAbstractOverlayType),
}
enum UpcaseAbstractColorStyleType {
    ParentAbstractColorStyleType(ParentAbstractColorStyleType),
    UpcaseLabelStyleType(UpcaseLabelStyleType),
    UpcaseIconStyleType(UpcaseIconStyleType),
    UpcaseLineStyleType(UpcaseLineStyleType),
    UpcasePolyStyleType(UpcasePolyStyleType),
}
enum UpcaseAbstractObjectType {
    ParentAbstractObjectType(ParentAbstractObjectType),
    UpcaseOrientationType(UpcaseOrientationType),
    UpcaseAliasType(UpcaseAliasType),
    ParentAbstractFeatureType(ParentAbstractFeatureType),
    UpcaseLocationType(UpcaseLocationType),
    UpcaseSchemaDataType(UpcaseSchemaDataType),
    UpcaseRegionType(UpcaseRegionType),
    ParentAbstractGeometryType(ParentAbstractGeometryType),
    UpcaseAbstractSubStyleType(UpcaseAbstractSubStyleType),
    UpcaseImagePyramidType(UpcaseImagePyramidType),
    UpcaseAbstractTimePrimitiveType(UpcaseAbstractTimePrimitiveType),
    UpcasePairType(UpcasePairType),
    ParentAbstractViewType(ParentAbstractViewType),
    UpcaseItemIconType(UpcaseItemIconType),
    UpcaseDataType(UpcaseDataType),
    UpcaseLodType(UpcaseLodType),
    UpcaseViewVolumeType(UpcaseViewVolumeType),
    ParentBasicLinkType(ParentBasicLinkType),
    ParentAbstractLatLonBoxType(ParentAbstractLatLonBoxType),
    UpcaseResourceMapType(UpcaseResourceMapType),
    UpcaseScaleType(UpcaseScaleType),
    ParentAbstractStyleSelectorType(ParentAbstractStyleSelectorType),
}
enum UpcaseAbstractStyleSelectorType {
    ParentAbstractStyleSelectorType(ParentAbstractStyleSelectorType),
    UpcaseStyleMapType(UpcaseStyleMapType),
    UpcaseStyleType(UpcaseStyleType),
}
enum UpcaseBasicLinkType {
    ParentBasicLinkType(ParentBasicLinkType),
    UpcaseLinkType(UpcaseLinkType),
}
enum UpcaseAbstractLatLonBoxType {
    ParentAbstractLatLonBoxType(ParentAbstractLatLonBoxType),
    UpcaseLatLonBoxType(UpcaseLatLonBoxType),
    UpcaseLatLonAltBoxType(UpcaseLatLonAltBoxType),
}
enum UpcaseAbstractGeometryType {
    ParentAbstractGeometryType(ParentAbstractGeometryType),
    UpcaseLineStringType(UpcaseLineStringType),
    UpcaseModelType(UpcaseModelType),
    UpcasePolygonType(UpcasePolygonType),
    UpcasePointType(UpcasePointType),
    UpcaseMultiGeometryType(UpcaseMultiGeometryType),
    UpcaseLinearRingType(UpcaseLinearRingType),
}
enum UpcaseAbstractSubStyleType {
    ParentAbstractSubStyleType(ParentAbstractSubStyleType),
    ParentAbstractColorStyleType(ParentAbstractColorStyleType),
    UpcaseBalloonStyleType(UpcaseBalloonStyleType),
    UpcaseListStyleType(UpcaseListStyleType),
}
