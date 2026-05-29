use serde_json::json;

const DISCORD_MESSAGE_LIMIT: usize = 2_000;

pub async fn send_to_discord(
    webhook_url: &str,
    content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    for chunk in split_message(content, DISCORD_MESSAGE_LIMIT) {
        let payload = json!({ "content": chunk });

        client
            .post(webhook_url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;
    }

    Ok(())
}

fn split_message(content: &str, max_chars: usize) -> Vec<String> {
    if content.is_empty() {
        return vec![String::new()];
    }

    let mut chunks = Vec::new();
    let mut current = String::new();

    for line in content.lines() {
        if line.chars().count() > max_chars {
            if !current.is_empty() {
                chunks.push(current);
                current = String::new();
            }

            let mut oversized = String::new();
            for ch in line.chars() {
                oversized.push(ch);
                if oversized.chars().count() == max_chars {
                    chunks.push(oversized);
                    oversized = String::new();
                }
            }

            if !oversized.is_empty() {
                current = oversized;
            }

            continue;
        }

        let candidate = if current.is_empty() {
            line.to_string()
        } else {
            format!("{}\n{}", current, line)
        };

        if candidate.chars().count() <= max_chars {
            current = candidate;
        } else {
            chunks.push(current);
            current = line.to_string();
        }
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    if chunks.is_empty() {
        chunks.push(String::new());
    }

    chunks
}
