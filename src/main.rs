#![warn(clippy::pedantic)]
use log::info;
use std::env;
use std::str::FromStr;
use telegrambot_ukranian::{commands, Command};
use teloxide::prelude::*;

use teloxide::{update_listeners::webhooks, Bot};
use url::Url;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let bot = Bot::from_env();
    let addr = ([0, 0, 0, 0], 8080).into();
    let url = env::var("url").expect("URL is not set");
    let url = Url::from_str(&url).unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url.clone()))
        .await
        .expect("Couldn't setup webhook");
    info!("Running on {url} {addr}");

    Command::repl_with_listener(bot, commands, listener).await;
}
