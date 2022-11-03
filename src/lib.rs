pub mod utils;
pub mod weather;

use teloxide::{prelude::*, utils::command::BotCommands};
use utils::usernames::{
    AWE, DARKTRAINER, DAVAS, DRDVD, DVDGG, GARFU, JAIME, JAVI, MARIO, RED, SAUTURN, THEXIAO77,
    VICTOR,
};
use weather::openweather::get_weather;

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
    #[command(description = "Let's play some overwatch!")]
    Pokemongo,
    #[command(description = "Let's play some pokemongoles!")]
    Telefonillo,
    #[command(description = "Let's play some garticphone!")]
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
        Command::Weather { location } => {
            if location.is_empty() {
                bot.send_message(
                    msg.chat.id,
                    "Tienes que poner la localizacion a consultar, por ejemplo:\n /weather Manila",
                )
                .await?
            } else {
                let result = get_weather(&location).await;
                let a = result.unwrap_or_else(|error| panic!("There was an error: {:?}", error));
                bot.send_message(msg.chat.id, a.id.to_string()).await?
            }
        }
    };
    Ok(())
}
