pub mod utils;
pub mod weather;

use teloxide::{
    payloads::AnswerWebAppQuerySetters,
    prelude::*,
    types::{ParseMode, VideoChatEnded},
    utils::command::BotCommands,
};
use utils::usernames::{
    AWE, DARKTRAINER, DAVAS, DRDVD, DVDGG, GARFU, JAIME, JAVI, MARIO, MCKAY, RED, SAUTURN,
    THEXIAO77, TOXIC, VICTOR,
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
    Dota,
    #[command(description = "Let's play some civilization!")]
    Civ,
    #[command(description = "Let's play some csgo!")]
    Cs,
    #[command(description = "Let's play some csgo!")]
    Hunt,
    #[command(description = "Let's play some hunt!")]
    Pokemongo,
    #[command(description = "Let's play some pokemongoles!")]
    Telefonillo,
    #[command(description = "Let's play some garticphone!")]
    Weather { location: String },
    #[command(description = "everyone")]
    Everyone,
}

pub async fn commands(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Dota => {
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
        Command::Hunt => {
            bot.send_message(
                msg.chat.id,
                format!("(Hunt) {}, {}, {}, {}", DARKTRAINER, SAUTURN, DAVAS, MARIO),
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
                    THEXIAO77, JAVI, AWE, SAUTURN, DAVAS, DVDGG, VICTOR, DRDVD, JAIME, DARKTRAINER,
                ),
            )
            .await?
        }
        Command::Weather { location } => {
            if location.is_empty() {
                bot.send_message(
                    msg.chat.id,
                    "Tienes que poner la localizacion a consultar, por ejemplo:\n /weather Toledo",
                )
                .await?
            } else {
                let weather = get_weather(&location).await;
                if !weather.name.is_empty() {
                    let message = format!(
                        r"<b>Location</b>: {},{} ({},{})
<b>Current Temperature:</b> {}ºC
<b>Current weather:</b> {},{}
<b>Max Temperature:</b> {}ºC
<b>Min Temperature:</b> {}ºC
<b>Temperature feels like:</b> {}ºC
<b>Wind:</b> {}m/s, {}º
<b>Pressure:</b> {}hPa
<b>Humidty:</b> {}%
<b>Visibility:</b> {}m",
                        weather.name,
                        weather.sys.country,
                        weather.coord.lon,
                        weather.coord.lat,
                        weather.main.temp,
                        weather.weather[0].main,
                        weather.weather[0].description,
                        weather.main.temp_max,
                        weather.main.temp_min,
                        weather.main.feels_like,
                        weather.wind.speed,
                        weather.wind.deg,
                        weather.main.pressure,
                        weather.main.humidity,
                        weather.visibility
                    );
                    bot.send_message(msg.chat.id, message)
                        .parse_mode(ParseMode::Html)
                        .reply_to_message_id(msg.id)
                        .await?
                } else {
                    bot.send_message(msg.chat.id, "Ni puta idea de donde esta eso")
                        .reply_to_message_id(msg.id)
                        .await?
                }
            }
        }
        Command::Everyone => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "@everyone: {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
                    THEXIAO77,
                    JAVI,
                    VICTOR,
                    DARKTRAINER,
                    AWE,
                    TOXIC,
                    RED,
                    DVDGG,
                    GARFU,
                    SAUTURN,
                    MARIO,
                    JAIME,
                    DAVAS,
                    DRDVD,
                    MCKAY,
                ),
            )
            .await?
        }
    };
    Ok(())
}
