use chrono::{DateTime, Duration, FixedOffset};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct AtomNcName(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct AtomMediaType(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct AtomLanguageTag(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct AtomEmailAddress(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Name(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Uri(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Email(AtomEmailAddress);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Feed(FeedType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Entry(EntryType);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Content {
	#[serde(flatten)]
	other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Author(AtomPersonConstruct);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Category {
	#[serde(flatten)]
	other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Contributor(AtomPersonConstruct);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Generator {
	#[serde(flatten)]
	other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Icon {
	#[serde(flatten)]
	other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Id {
	#[serde(flatten)]
	other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Logo {
	#[serde(flatten)]
	other: HashMap<String, String>,
}
#[doc = "The \"atom:link\" element defines a reference from an\n                                entry or feed to a Web resource. This specification\n                                assigns no\n                                meaning to the content (if any) of this\n                                element."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Link {
	#[serde(flatten)]
	other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Published(AtomDateConstruct);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Rights(AtomTextConstruct);
#[doc = "atom:source is used to preserve metadata of a feed\n                                when\n                                an entry is copied from a feed to another feed."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Source {
	#[serde(flatten)]
	other: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Subtitle(AtomTextConstruct);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Summary(AtomTextConstruct);
#[doc = "The \"atom:title\" element is a Text construct that\n                                conveys a human- readable title for an entry or feed.\n                                atomTitle =\n                                element atom:title { atomTextConstruct }."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Title(AtomTextConstruct);
#[doc = "The \"atom:updated\" element is a Date construct\n                                indicating the most recent instant in time when an entry\n                                or feed was\n                                modified in a way the publisher considers\n                                significant. Therefore, not\n                                all modifications\n                                necessarily result in a changed atom:updated value.\n                                atomUpdated = element atom:updated { atomDateConstruct\n                                }. Publishers\n                                MAY change the value of this element over\n                                time."]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Updated(AtomDateConstruct);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AtomTextConstruct {
	r#type: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AtomPersonConstruct {
	#[serde(rename = "$value")]
	body: Vec<UpcaseAtomPersonConstructBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseAtomPersonConstructBody {
	Name(Name),
	Uri(Uri),
	Email(Email),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AtomDateConstruct {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeedType {
	#[serde(rename = "$value")]
	body: Vec<UpcaseFeedTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseFeedTypeBody {
	Entry(Entry),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EntryType {
	#[serde(rename = "$value")]
	body: Vec<UpcaseEntryTypeBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseEntryTypeBody {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UndefinedContent {
	#[serde(rename = "$value")]
	body: Vec<UpcaseUndefinedContentBody>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UpcaseUndefinedContentBody {}
