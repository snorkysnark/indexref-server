use yaserde_derive::YaDeserialize;

#[derive(Debug, YaDeserialize)]
#[yaserde(
    prefix = "RDF",
    root = "RDF",
    namespace = "RDF: http://www.w3.org/1999/02/22-rdf-syntax-ns#",
    namespace = "NS1: http://amb.vis.ne.jp/mozilla/scrapbook-rdf#"
)]
pub struct Rdf {
    #[yaserde(rename = "Description")]
    pub descriptions: Vec<RdfDescription>,
    #[yaserde(rename = "Seq")]
    pub sequences: Vec<RdfSeq>,
}

#[derive(Debug, YaDeserialize)]
pub struct RdfDescription {
    #[yaserde(attribute, prefix = "RDF")]
    pub about: String,
    #[yaserde(attribute, prefix = "NS1")]
    pub id: String,
    #[yaserde(attribute, rename = "type", prefix = "NS1")]
    pub r#type: String,
    #[yaserde(attribute, prefix = "NS1")]
    pub title: String,
    #[yaserde(attribute, prefix = "NS1")]
    pub chars: String,
    #[yaserde(attribute, prefix = "NS1")]
    pub comment: String,
    #[yaserde(attribute, prefix = "NS1")]
    pub icon: String,
    #[yaserde(attribute, prefix = "NS1")]
    pub source: String,
}
//
// impl From<RdfDescription> for entity::types::ScrapbookData {
//     fn from(value: RdfDescription) -> Self {
//         fn none_if_empty(string: String) -> Option<String> {
//             match string.as_str() {
//                 "" => None,
//                 _ => Some(string),
//             }
//         }
//
//         Self {
//             about: value.about,
//             id: value.id,
//             r#type: none_if_empty(value.r#type),
//             title: none_if_empty(value.title),
//             chars: none_if_empty(value.chars),
//             comment: none_if_empty(value.comment),
//             icon: none_if_empty(value.icon),
//             source: none_if_empty(value.source),
//         }
//     }
// }

#[derive(Debug, YaDeserialize)]
pub struct RdfSeq {
    #[yaserde(attribute, prefix = "RDF")]
    pub about: String,
    #[yaserde(rename = "li")]
    pub items: Vec<RdfLi>,
}

#[derive(Debug, YaDeserialize)]
pub struct RdfLi {
    #[yaserde(attribute, prefix = "RDF")]
    pub resource: String,
}
