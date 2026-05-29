use feed_rs::parser;

pub async fn fetch_from_rss(
    feed_url: &str,
) -> Result<Vec<crate::providers::Paper>, Box<dyn std::error::Error>> {
    let response = reqwest::get(feed_url).await?;
    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        println!("RSS HTTP error: status={}", status);
        println!("RSS error body: {}", body);
        return Err(format!("RSS returned non-success status {}", status).into());
    }

    let feed = parser::parse(body.as_bytes())?;

    let papers = feed
        .entries
        .into_iter()
        .map(|entry| {
            let title = entry
                .title
                .as_ref()
                .map(|title| title.content.clone())
                .unwrap_or_default();

            let summary = entry
                .summary
                .as_ref()
                .map(|summary| summary.content.clone())
                .unwrap_or_else(|| {
                    entry
                        .content
                        .as_ref()
                        .and_then(|content| content.body.clone())
                        .unwrap_or_default()
                });

            crate::providers::Paper { title, summary }
        })
        .collect();

    Ok(papers)
}
