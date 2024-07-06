impl repo_content {
    #[tokio::main]
    async fn fetch_repo_contents() -> Result<(), Error> {
        let owner = "cnhornbeck";
        let repo = "MCModProfiles";
        let url = format!("https://api.github.com/repos/{}/{}/contents", owner, repo);

        let client = reqwest::Client::new();
        let res = client
            .get(&url)
            .header("User-Agent", "request")
            .send()
            .await?;

        let contents: Vec<RepoContent> = res.json().await?;

        use std::iter::Iterator;

        let files: Vec<&RepoContent> = contents
        .iter()
        .filter(
            |item| matches!(item, RepoContent::File {content_type,  ..} if content_type == "file"),
        )
        .collect();

        for file in &files {
            if let RepoContent::File {
                name,
                download_url: Some(url),
                ..
            } = file
            {
                println!("Fetching content for file: {}", name);
                let file_content = fetch_file_content(&client, url).await?;
                println!("Content of {}: \n{}", name, file_content);
            }
        }

        println!("Files: {:?}", files);

        Ok(())
    }
}
