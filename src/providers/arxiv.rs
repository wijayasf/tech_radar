use chrono::{DateTime, Duration as ChronoDuration, Utc};
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
    let max_attempts = 4;

    for attempt in 1..=max_attempts {
        let response = match reqwest::get(url).await {
            Ok(response) => response,
            Err(error) => {
                last_error = Some(format!("Request error on attempt {}: {}", attempt, error));
                if attempt < max_attempts {
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
                if attempt < max_attempts {
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
                break;
            }
        };

        if status.as_u16() == 429 {
            let wait_seconds = 5_u64 * 2_u64.pow((attempt - 1) as u32);
            println!("arXiv rate limit on attempt {}: status={}", attempt, status);
            println!("arXiv rate limit body: {}", body);
            last_error = Some(format!("arXiv returned HTTP 429 after attempt {}", attempt));

            if attempt < max_attempts {
                println!("Retrying arXiv fetch in {} seconds...", wait_seconds);
                sleep(Duration::from_secs(wait_seconds)).await;
                continue;
            }

            println!("arXiv rate limit persisted after {} attempts. Skipping fetch safely.", max_attempts);
            return Ok(Vec::new());
        }

        if !status.is_success() {
            println!("arXiv HTTP error on attempt {}: status={}", attempt, status);
            println!("arXiv error body: {}", body);
            last_error = Some(format!("arXiv returned non-success status {}", status));
            if attempt < max_attempts {
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
                if attempt < max_attempts {
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
                break;
            }
        };

        let cutoff = Utc::now() - ChronoDuration::days(3);

        let papers = document
            .descendants()
            .filter(|node| node.has_tag_name("entry"))
            .filter_map(|entry| {
                let published = entry
                    .children()
                    .find(|node| node.has_tag_name("published"))
                    .and_then(|node| node.text())
                    .and_then(|text| DateTime::parse_from_rfc3339(text).ok())
                    .map(|dt| dt.with_timezone(&Utc));

                if published.is_none_or(|published| published < cutoff) {
                    return None;
                }

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

                Some(Paper { title, summary })
            })
            .collect();

        return Ok(papers);
    }

    Err(last_error
        .unwrap_or_else(|| "Unknown arXiv fetch error".to_string())
        .into())
}
