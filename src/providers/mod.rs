pub mod arxiv;
pub mod discord;
pub mod rss_reader;

pub use arxiv::{fetch_latest_papers, Paper};
pub use discord::send_to_discord;
pub use rss_reader::fetch_from_rss;
