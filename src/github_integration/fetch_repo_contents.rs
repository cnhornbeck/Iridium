use super::repo_content::RepoContent;
use reqwest::Error;

impl RepoContent {
    pub async fn fetch_repo_contents() -> Result<String, Error> {
        let owner = "cnhornbeck";
        let repo = "MCModProfiles";
        let url = format!("https://api.github.com/repos/{}/{}/contents", owner, repo);

        let client = reqwest::Client::new();
        let res = client
            .get(&url)
            .header("User-Agent", "my-app")
            .send()
            .await?;

        let contents: Vec<RepoContent> = res.json().await?;

        let files: Vec<RepoContent> = contents
            .into_iter()
            .filter(|item| matches!(item, RepoContent::File { .. }))
            .collect();

        let mut file_content = String::new();

        if let Some(first_file) = files.first() {
            if let RepoContent::File {
                download_url: Some(url),
                ..
            } = first_file
            {
                let client = reqwest::Client::new();
                file_content = fetch_file_content(&client, url).await?;
            }
        }

        Ok(file_content)
    }
}

async fn fetch_file_content(client: &reqwest::Client, url: &str) -> Result<String, Error> {
    let res = client
        .get(url)
        .header("User-Agent", "FeriumManager")
        .send()
        .await?
        .error_for_status()?; // This will automatically handle non-success status codes

    let content: String = res.text().await?;
    Ok(content)
}
