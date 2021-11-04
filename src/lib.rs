use serde::{Deserialize, Serialize};

// MediaHaven structs
//
// All MediaHaven nodes
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaHavenResult {
    #[serde(rename = "TotalNrOfResults")]
    pub total_nr_of_results: i64,
    #[serde(rename = "StartIndex")]
    pub start_index: i64,
    #[serde(rename = "MediaDataList")]
    pub media_data_list: Vec<MediaDataList>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaDataList {
    #[serde(rename = "Technical")]
    pub technical: ::serde_json::Value,
    #[serde(rename = "Administrative")]
    pub administrative: ::serde_json::Value,
    #[serde(rename = "Descriptive")]
    pub descriptive: ::serde_json::Value,
    #[serde(rename = "RightsManagement")]
    pub rights_management: ::serde_json::Value,
    #[serde(rename = "Internal")]
    pub internal: ::serde_json::Value,
    #[serde(rename = "Dynamic")]
    pub dynamic: ::serde_json::Value,
    #[serde(rename = "Structural")]
    pub structural: ::serde_json::Value,
    #[serde(rename = "Context")]
    pub context: ::serde_json::Value,
}
