mod core;
mod providers;

#[tokio::main]
async fn main() {
    let gemini_api_key = match std::env::var("GEMINI_API_KEY") {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Environment variable GEMINI_API_KEY tidak ditemukan.");
            std::process::exit(1);
        }
    };

    let discord_webhook_url = match std::env::var("DISCORD_WEBHOOK_URL") {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Environment variable DISCORD_WEBHOOK_URL tidak ditemukan.");
            std::process::exit(1);
        }
    };

    println!("Mengambil data paper...");
    let papers = match providers::fetch_latest_papers().await {
        Ok(papers) => papers,
        Err(error) => {
            eprintln!("Gagal mengambil data paper dari ArXiv: {error}");
            std::process::exit(1);
        }
    };

    if papers.is_empty() {
        eprintln!("Tidak ada paper yang ditemukan dari ArXiv.");
        std::process::exit(1);
    }

    println!("Menganalisis dengan Gemini...");
    let analysis = match core::analyze_papers(papers, &gemini_api_key).await {
        Ok(result) => result,
        Err(error) => {
            eprintln!("Gagal menganalisis paper dengan Gemini: {error}");
            std::process::exit(1);
        }
    };

    println!("Mengirim ke Discord...");
    if let Err(error) = providers::send_to_discord(&discord_webhook_url, &analysis).await {
        eprintln!("Gagal mengirim hasil ke Discord: {error}");
        std::process::exit(1);
    }

    println!("Selesai. Ringkasan berhasil dikirim ke Discord.");
}
