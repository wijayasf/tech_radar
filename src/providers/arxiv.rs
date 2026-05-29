use roxmltree::Document;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Paper {
    pub title: String,
    pub summary: String,
}

pub async fn fetch_latest_papers() -> Result<Vec<Paper>, Box<dyn std::error::Error>> {
    let url = "http://export.arxiv.org/api/query?search_query=%28all:%22LLM+multi-agent+orchestration%22+OR+all:%22Autonomous+software+engineering+agents%22+OR+all:%22Microservices+architecture+patterns%22+OR+all:%22Cloud-native+system+design%22+OR+all:%22Kubernetes+scaling+patterns%22+OR+all:%22Event-driven+architecture%22%29&sortBy=submittedDate&sortOrder=descending&max_results=6";
    let response = reqwest::get(url).await?;
    let xml = response.text().await?;
    let document = Document::parse(&xml)?;

    let papers = document
        .descendants()
        .filter(|node| node.has_tag_name("entry"))
        .map(|entry| {
            let title = entry
                .children()
                .find(|node| node.has_tag_name("title"))
                .and_then(|node| node.text())
                .map(str::trim)
                .unwrap_or_default()
                .to_string();

            let summary = entry
                .children()
                .find(|node| node.has_tag_name("summary"))
                .and_then(|node| node.text())
                .map(str::trim)
                .unwrap_or_default()
                .to_string();

            Paper { title, summary }
        })
        .collect();

    Ok(papers)
}
