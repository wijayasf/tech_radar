use roxmltree::Document;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Paper {
    pub title: String,
    pub summary: String,
}

pub async fn fetch_latest_papers() -> Result<Vec<Paper>, Box<dyn std::error::Error>> {
    let url = "http://export.arxiv.org/api/query?search_query=%28all:%22LLM+multi-agent+orchestration%22+OR+all:%22Autonomous+software+engineering+agents%22+OR+all:%22Microservices+architecture+patterns%22+OR+all:%22Cloud-native+system+design%22+OR+all:%22Kubernetes+scaling+patterns%22+OR+all:%22Event-driven+architecture%22%29&sortBy=submittedDate&sortOrder=descending&max_results=6";

    let mut last_error: Option<String> = None;

    for attempt in 1..=3 {
        let response = match reqwest::get(url).await {
            Ok(response) => response,
            Err(error) => {
                last_error = Some(format!("Request error on attempt {}: {}", attempt, error));
                if attempt < 3 {
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
                break;
            }
        };

        let status = response.status();
        let body = match response.text().await {
            Ok(body) => body,
            Err(error) => {
                last_error = Some(format!("Failed reading arXiv response body on attempt {}: {}", attempt, error));
                if attempt < 3 {
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
                break;
            }
        };

        if !status.is_success() {
            println!("arXiv HTTP error on attempt {}: status={}", attempt, status);
            println!("arXiv error body: {}", body);
            last_error = Some(format!("arXiv returned non-success status {}", status));
            if attempt < 3 {
                sleep(Duration::from_secs(5)).await;
                continue;
            }
            break;
        }

        let document = match Document::parse(&body) {
            Ok(document) => document,
            Err(error) => {
                println!("arXiv parse error on attempt {}: {}", attempt, error);
                println!("arXiv raw body: {}", body);
                last_error = Some(format!("Failed parsing arXiv XML: {}", error));
                if attempt < 3 {
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
                break;
            }
        };

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

        return Ok(papers);
    }

    Err(last_error
        .unwrap_or_else(|| "Unknown arXiv fetch error".to_string())
        .into())
}
