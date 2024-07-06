impl repo_content {
    async fn fetch_file_content(client: &reqwest::Client, url: &str) -> Result<String, Error> {
        let res = client
            .get(url)
            .header("User-Agent", "request")
            .send()
            .await?;

        let content = res.text().await?;
        Ok(content)
    }
}
