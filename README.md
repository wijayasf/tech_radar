# Tech Intelligence Radar

Sistem ini mengambil paper teknik terbaru dari arXiv, menganalisisnya dengan persona **Senior Tech Lead**, lalu mengirim hasil analisis Markdown ke Discord.

## Fokus Topik

- Agentic AI / multi-agent orchestration
- Autonomous software engineering agents
- Microservices architecture patterns
- Cloud-native system design
- Kubernetes scaling patterns
- Event-driven architecture

## Alur Kerja

1. [`fetch_latest_papers()`](src/providers/arxiv.rs:10) mengambil paper terbaru dari query arXiv yang lebih presisi.
2. [`analyze_papers()`](src/core.rs:3) mengirim judul dan ringkasan paper ke OpenAI untuk disusun menjadi dokumen Markdown dengan format:
   - `What's New`
   - `Architectural Impact`
   - `Radar Classification`
3. [`send_to_discord()`](src/providers/discord.rs:5) memecah hasil analisis ke beberapa chunk aman, lalu mengirimnya ke Discord webhook.

## Environment Variables

- `GEMINI_API_KEY` pada [`src/main.rs`](src/main.rs:6) sekarang dipakai sebagai API key OpenAI oleh [`analyze_papers()`](src/core.rs:3). Sebaiknya nanti diganti nama menjadi `OPENAI_API_KEY` bila Anda ingin merapikan implementasi.
- `DISCORD_WEBHOOK_URL` dibaca di [`src/main.rs`](src/main.rs:14).

## Menjalankan Sistem

```bash
export GEMINI_API_KEY='sk-...'
export DISCORD_WEBHOOK_URL='https://discord.com/api/webhooks/...'
cargo run
```

## Catatan Operasional

- Jika arXiv mengembalikan `Rate exceeded`, tunggu beberapa saat lalu jalankan ulang.
- Query di [`src/providers/arxiv.rs`](src/providers/arxiv.rs:11) sudah dipersempit untuk mengurangi noise dan risiko rate limit.
- Jika ingin hasil lebih konsisten, ganti nama environment variable `GEMINI_API_KEY` menjadi `OPENAI_API_KEY` di [`src/main.rs`](src/main.rs:6) dan [`src/core.rs`](src/core.rs:5).
