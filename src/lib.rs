#![warn(clippy::pedantic)]

pub mod utils;
pub mod weather;

use crate::utils::usernames::get_telegram_handle;
use teloxide::{
    prelude::*,
    types::{ParseMode, ReplyParameters},
    utils::command::BotCommands,
};
use utils::usernames::Username;
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
    Marvel,
}

/// Executes the specified command.
///
/// # Arguments
///
/// * `bot` - The bot instance.
/// * `msg` - The message that triggered the command.
/// * `cmd` - The command to execute.
///
/// # Errors
///
/// Returns an error if there was a problem executing the command.
#[allow(clippy::too_many_lines)]
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
                    "(Dota 2) {}, {}, {}, {}, {}, {}",
                    get_telegram_handle(Username::TheXiao77),
                    get_telegram_handle(Username::Javi),
                    get_telegram_handle(Username::DarkTrainer),
                    get_telegram_handle(Username::Dvdgg),
                    get_telegram_handle(Username::Victor),
                    get_telegram_handle(Username::Mario),
                ),
            )
            .await?
        }
        Command::Civ => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "(Civilization V) {}, {}, {}, {}, {}, {}",
                    get_telegram_handle(Username::Javi),
                    get_telegram_handle(Username::DarkTrainer),
                    get_telegram_handle(Username::Sauturn),
                    get_telegram_handle(Username::Awe),
                    get_telegram_handle(Username::Jaime),
                    get_telegram_handle(Username::Davas),
                ),
            )
            .await?
        }
        Command::Cs => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "(CS2) {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
                    get_telegram_handle(Username::TheXiao77),
                    get_telegram_handle(Username::DarkTrainer),
                    get_telegram_handle(Username::Sauturn),
                    get_telegram_handle(Username::Awe),
                    get_telegram_handle(Username::Jaime),
                    get_telegram_handle(Username::Davas),
                    get_telegram_handle(Username::Javi),
                    get_telegram_handle(Username::Red),
                    get_telegram_handle(Username::DrDvd),
                    get_telegram_handle(Username::Toxic),
                ),
            )
            .await?
        }
        Command::Hunt => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "(Hunt) {}, {}, {}, {}, {}",
                    get_telegram_handle(Username::DarkTrainer),
                    get_telegram_handle(Username::Sauturn),
                    get_telegram_handle(Username::Davas),
                    get_telegram_handle(Username::Mario),
                    get_telegram_handle(Username::Toxic),
                ),
            )
            .await?
        }
        Command::Pokemongo => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "(Pokemon Go) {}, {}, {}, {}, {}",
                    get_telegram_handle(Username::Javi),
                    get_telegram_handle(Username::DarkTrainer),
                    get_telegram_handle(Username::Sauturn),
                    get_telegram_handle(Username::Garfu),
                    get_telegram_handle(Username::Mario),
                ),
            )
            .await?
        }
        Command::Telefonillo => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "(Gartic/Pinturillo) {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
                    get_telegram_handle(Username::TheXiao77),
                    get_telegram_handle(Username::Javi),
                    get_telegram_handle(Username::Awe),
                    get_telegram_handle(Username::Sauturn),
                    get_telegram_handle(Username::Davas),
                    get_telegram_handle(Username::Dvdgg),
                    get_telegram_handle(Username::Victor),
                    get_telegram_handle(Username::DrDvd),
                    get_telegram_handle(Username::Jaime),
                    get_telegram_handle(Username::DarkTrainer),
                ),
            )
            .await?
        }
        Command::Marvel => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "(Marvel) {}, {}, {}, {}",
                    get_telegram_handle(Username::Javi),
                    get_telegram_handle(Username::Victor),
                    get_telegram_handle(Username::Mario),
                    get_telegram_handle(Username::DarkTrainer),
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
                if weather.name.is_empty() {
                    bot.send_message(msg.chat.id, "Ni puta idea de donde esta eso")
                        .reply_parameters(ReplyParameters::new(msg.id))
                        .await?
                } else {
                    let message = format!(
                        r"🌍 <b>{}, {}</b>
📍 Coordinates: {:.2}°, {:.2}°

🌡️ <b>Temperature</b>
• Current: {}°C (Feels like: {}°C)
• Max: {}°C | Min: {}°C

🌤️ <b>Weather Conditions</b>
• {}, {}

💨 <b>Wind & Atmosphere</b>
• Wind: {}m/s at {}°
• Pressure: {}hPa
• Humidity: {}%
• Visibility: {}m",
                        weather.name,
                        weather.sys.country,
                        weather.coord.lon,
                        weather.coord.lat,
                        weather.main.temp,
                        weather.main.feels_like,
                        weather.main.temp_max,
                        weather.main.temp_min,
                        weather.weather[0].main,
                        weather.weather[0].description,
                        weather.wind.speed,
                        weather.wind.deg,
                        weather.main.pressure,
                        weather.main.humidity,
                        weather.visibility
                    );
                    bot.send_message(msg.chat.id, message)
                        .parse_mode(ParseMode::Html)
                        .reply_parameters(ReplyParameters::new(msg.id))
                        .await?
                }
            }
        }
    };
    Ok(())
}
