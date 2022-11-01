use telegrambot_ukranian::{answer, Command};
use teloxide::prelude::*;
use teloxide::Bot;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let bot = Bot::from_env();
    Command::repl(bot, answer).await;
}
