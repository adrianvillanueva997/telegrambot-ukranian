use teloxide::{prelude::*, utils::command::BotCommands};
use utils::usernames::{
    AWE, DARKTRAINER, DAVAS, DRDVD, DVDGG, GARFU, JAIME, JAVI, MARIO, RED, SAUTURN, THEXIAO77,
    VICTOR,
};

pub mod utils;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Let's play some dota!")]
    Call,
    #[command(description = "Let's play some civilization!")]
    Civ,
    #[command(description = "Let's play some csgo!")]
    Cs,
    #[command(description = "Let's play some csgo!")]
    Ow,
    #[command(description = "Let's play some csgo!")]
    Pokemongo,
    #[command(description = "Let's play some csgo!")]
    Telefonillo,
    #[command(description = "Let's play some csgo!")]
    Weather { location: String },
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Call => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "(Dota 2) {}, {}, {}, {}, {}",
                    THEXIAO77, JAVI, DARKTRAINER, DVDGG, VICTOR
                ),
            )
            .await?
        }
        Command::Civ => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "(Civilization V) {}, {}, {}, {}, {}, {}",
                    JAVI, DARKTRAINER, SAUTURN, AWE, JAIME, DAVAS
                ),
            )
            .await?
        }
        Command::Cs => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "(CS:GO) {}, {}, {}, {}, {}, {}, {}, {}, {}",
                    THEXIAO77, DARKTRAINER, SAUTURN, AWE, JAIME, DAVAS, JAVI, RED, DRDVD
                ),
            )
            .await?
        }
        Command::Ow => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "(Overwatch) {}, {}, {}, {}, {}, {}",
                    THEXIAO77, DARKTRAINER, AWE, JAVI, RED, VICTOR
                ),
            )
            .await?
        }
        Command::Pokemongo => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "(Pokemon Go) {}, {}, {}, {}, {}",
                    JAVI, DARKTRAINER, SAUTURN, GARFU, MARIO
                ),
            )
            .await?
        }
        Command::Telefonillo => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "(Gartic/Pinturillo) {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
                    THEXIAO77, JAVI, AWE, SAUTURN, DAVAS, DVDGG, VICTOR, DRDVD, JAIME, DARKTRAINER
                ),
            )
            .await?
        }
        Command::Weather { location } => bot.send_message(msg.chat.id, &location).await?,
    };

    Ok(())
}

pub async fn send_message(bot: Bot, message: Message, text: String) {
    let result = bot.send_message(message.chat.id, text).await;
    let a = result.unwrap();
}
