use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum RepoContent {
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
