pub mod arxiv;
pub mod discord;

pub use arxiv::{fetch_latest_papers, Paper};
pub use discord::send_to_discord;
