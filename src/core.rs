use serde_json::json;

pub async fn analyze_papers(
    papers: Vec<crate::providers::Paper>,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let combined_papers = papers
        .into_iter()
        .enumerate()
        .map(|(index, paper)| {
            format!(
                "Paper {}:\nTitle: {}\nSummary: {}\n---",
                index + 1,
                paper.title,
                paper.summary
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    if combined_papers.trim().is_empty() {
        return Err("Data paper dari arXiv kosong".into());
    }

    let endpoint = "https://api.openai.com/v1/chat/completions";

    let payload = serde_json::json!({
        "model": "gpt-4o-mini",
        "messages": [
            {
                "role": "system",
                "content": "You are a Senior Tech Lead. Analyze the supplied engineering research papers objectively, technically, and from a developer value-add perspective. Remove all references to real estate or township. Produce clean Markdown titled 'Tech Radar: Weekly Engineering Intelligence'. For each topic, include exactly these sections: What's New, Architectural Impact, and Radar Classification. Radar Classification must use one of these labels only: [ADOPT], [TRIAL], or [ASSESS]. Focus on engineering relevance such as latency, coupling, extensibility, operability, scaling, platform maturity, and production readiness."
            },
            {
                "role": "user",
                "content": combined_papers
            }
        ],
        "temperature": 0.2
    });

    let client = reqwest::Client::new();
    let response = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await?;
        println!("DEBUG GEMINI ERROR BODY: {}", error_body);
        return Err(format!("Gemini API request failed with status {}", status).into());
    }

    let res_json: serde_json::Value = response.json().await?;

    let text_output = res_json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("Gagal mengambil text dari OpenAI response")?
        .to_string();

    Ok(text_output)
}
