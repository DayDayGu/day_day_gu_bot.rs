use async_std::task;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static TOKEN: Lazy<String> = Lazy::new(|| std::env::var("TG_TOKEN").unwrap());
static CHAT_ID: Lazy<String> = Lazy::new(|| std::env::var("TG_CHAT_ID").unwrap());
static TG_API: &str = "https://api.telegram.org/bot";

fn tg_api(action: &str) -> String {
    [TG_API, &TOKEN, "/", action].concat()
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    // a telegram message
    chat_id: String,
    text: String,
    parse_mode: String,
    disable_web_page_preview: bool,
    disable_notification: bool,
}

impl<'a> Message {
    pub fn new(text: &'a str) -> Message {
        Message {
            chat_id: CHAT_ID.to_owned(),
            text: text.to_owned(),
            parse_mode: "Markdown".to_owned(),
            disable_notification: false,
            disable_web_page_preview: false,
        }
    }
}

pub async fn send_message<'a>(text: &'a str) {
    let msg = Message::new(text);
    let uri = tg_api("sendMessage");
    task::spawn(async move {
        if let Err(e) = surf::post(uri).body_json(&msg).unwrap().await {
            log::error!("Error when send message: {}", e);
        }
    });
}
