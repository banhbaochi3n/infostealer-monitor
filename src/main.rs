#![allow(unused)]
use std::time::Duration;

use crate::error::Error;
// use crate::check_keyword::check_keyword;
pub use crate::utils::Result;

use grammers_client::types::message::Message;
use grammers_client::types::Media::{Document, Photo};
use grammers_client::{Client, Config, InitParams, Update};
use grammers_mtsender::ReconnectionPolicy;
use grammers_session::{PackedChat, PackedType, Session};

// mod check_keyword;
mod error;
mod utils;

// Session file for later use
const SESSION_FILE: &str = "./session";

fn main() -> Result<()> {
    pretty_env_logger::init();

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(build_client())
}

async fn build_client() -> Result<()> {
    dotenvy::dotenv()?;

    let api_id: i32 = dotenvy::var("api_id")?.parse()?;
    let api_hash = dotenvy::var("api_hash")?;
    let _webhook_url = dotenvy::var("webhook_url")?;

    // check_keyword::search_keyword()?;

    log::info!("[*] Connecting to Telegram...");
    let _client = Client::connect(Config {
        session: Session::load_file_or_create(SESSION_FILE)?,
        api_id,
        api_hash: api_hash.clone(),
        params: InitParams {
            catch_up: true,
            reconnection_policy: &Policy,
            ..Default::default()
        },
    })
    .await?;

    log::info!("[*] Connected");

    let mut sign_out = false;

    if !_client.is_authorized().await? {
        log::info!("[*] Signing in...");
        _client.session().save_to_file(SESSION_FILE)?;
        log::info!("[*] Signed in");
    }

    let url: &str = "https://t.me/RedlineViper";

    if let Some(chat) = _client.resolve_username(url).await? {
        log::info!("Found chat: {:?}", chat.name());
    }

    // if let Ok(Some(chat)) = _client.join_chat("RedlineViper").await {
    //     log::info!("Successfully joined channel {url}");
    // } else {
    //     log::warn!("Failed to join channel {url}");
    // };

    // Constantly check for new updates (messages, mentions, etc)
    while let Some(update) = _client.next_update().await? {
        match update {
            Update::NewMessage(message) if !message.outgoing() => {
                todo!()
            }
            Update::MessageEdited(message) => {
                todo!()
            }
            _ => (),
        }
    }

    Ok(())
}

struct Policy;

impl ReconnectionPolicy for Policy {
    fn should_retry(&self, attempts: usize) -> std::ops::ControlFlow<(), std::time::Duration> {
        let duration = u64::pow(2, attempts as _);
        std::ops::ControlFlow::Continue(Duration::from_millis(duration))
    }
}

async fn search_monitored_keyword_in_data_leak(downloaded_files: &str) -> Result<()> {
    if downloaded_files.ends_with(".rar") || downloaded_files.ends_with(".zip") {
        log::info!("[*] Found data leak!");
        return Ok(());
    }

    Ok(())
}

async fn handle_new_data_leak_msg(event: Message) -> Result<()> {
    let urls_list = contains_url_in_msg(event);

    Ok(())
}

fn detect_telegram_link(url_list: Vec<String>) -> (Vec<String>, Vec<String>) {
    let (telegram_urls_extracted_list, urls_need_reviewing_list) =
        url_list.into_iter().partition(|url| url.contains("t.me/"));

    (telegram_urls_extracted_list, urls_need_reviewing_list)
}

fn contains_url_in_msg(event: Message) -> Vec<String> {
    let msg = event.text().split_whitespace();

    msg.filter(|word| word.starts_with("https:") || word.starts_with("http:"))
        .map(|w| w.to_string())
        .collect::<Vec<String>>()
}
