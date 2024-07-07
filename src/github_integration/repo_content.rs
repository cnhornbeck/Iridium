use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum RepoContent {
    File {
        name: String,
        path: String,
        #[serde(rename = "type")]
        content_type: String,
        download_url: Option<String>,
    },
    Directory {
        name: String,
        path: String,
        #[serde(rename = "type")]
        content_type: String,
    },
}
