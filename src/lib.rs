#![warn(clippy::pedantic)]

pub mod utils;
pub mod weather;

use crate::utils::usernames::get_telegram_handle;
use chrono::DateTime;
use teloxide::{
    prelude::*,
    types::{ParseMode, ReplyParameters},
    utils::command::BotCommands,
};
use utils::usernames::Username;
use weather::openweather::get_weather;

/// Convert timestamp to formatted time string
fn format_time(timestamp: i64) -> String {
    DateTime::from_timestamp(timestamp, 0)
        .map_or_else(|| "N/A".to_string(), |dt| dt.format("%H:%M").to_string())
}

/// Get wind direction with emoji based on degree
fn get_wind_direction(deg: i64) -> &'static str {
    match deg {
        0..=22 => "N ⬇️",
        23..=67 => "NE ↙️",
        68..=112 => "E ⬅️",
        113..=157 => "SE ↖️",
        158..=202 => "S ⬆️",
        203..=247 => "SW ↗️",
        248..=292 => "W ➡️",
        _ => "NW ↘️",
    }
}

/// Format visibility from meters to read-friendly format
fn format_visibility(visibility_meters: i64) -> String {
    let km = visibility_meters / 1000;
    let remainder = (visibility_meters % 1000) / 100;
    format!("{visibility_meters}m ({km}.{remainder}km)")
}

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
    #[command(description = "Let's play some deadlock!")]
    Deadlock,
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

        Command::Deadlock | Command::Dota => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "🎮 <b>Defensa del viejales</b>\n\n⚔️ <i>Dota/Deadlock</i>\n{} {} {} {} {} {}",
                    get_telegram_handle(Username::TheXiao77),
                    get_telegram_handle(Username::Javi),
                    get_telegram_handle(Username::DarkTrainer),
                    get_telegram_handle(Username::Dvdgg),
                    get_telegram_handle(Username::Victor),
                    get_telegram_handle(Username::Mario),
                ),
            )
            .parse_mode(ParseMode::Html)
            .await?
        }

        Command::Civ => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "🏛️ <b>BUILD AN EMPIRE!</b>\n\n🗺️ <i>Civilization V</i>\n{} {} {} {} {} {}",
                    get_telegram_handle(Username::Javi),
                    get_telegram_handle(Username::DarkTrainer),
                    get_telegram_handle(Username::Sauturn),
                    get_telegram_handle(Username::Awe),
                    get_telegram_handle(Username::Jaime),
                    get_telegram_handle(Username::Davas),
                ),
            )
            .parse_mode(ParseMode::Html)
            .await?
        }

        Command::Cs => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "🔫 <b>Rush B!</b>\n\n🎯 <i>CS2</i>\n{} {} {} {} {} {} {} {} {} {}",
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
            .parse_mode(ParseMode::Html)
            .await?
        }
        Command::Hunt => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "🔍 <b>TIME TO HUNT!</b>\n\n👹 <i>Hunt</i>\n{} {} {} {} {}",
                    get_telegram_handle(Username::DarkTrainer),
                    get_telegram_handle(Username::Sauturn),
                    get_telegram_handle(Username::Davas),
                    get_telegram_handle(Username::Mario),
                    get_telegram_handle(Username::Toxic),
                ),
            )
            .parse_mode(ParseMode::Html)
            .await?
        }
        Command::Pokemongo => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "🔴 <b>GOTTA CATCH 'EM ALL!</b>\n\n⚡ <i>Pokémon GO Squad</i>\n{} {} {} {} {}",
                    get_telegram_handle(Username::Javi),
                    get_telegram_handle(Username::DarkTrainer),
                    get_telegram_handle(Username::Sauturn),
                    get_telegram_handle(Username::Garfu),
                    get_telegram_handle(Username::Mario),
                ),
            )
            .parse_mode(ParseMode::Html)
            .await?
        }

        Command::Telefonillo => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "🎨 <b>A dibujar mortadelos!</b>\n\n🖌️ <i>Gartic/Pinturillo</i>\n{} {} {} {} {} {} {} {} {} {}",
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
            .parse_mode(ParseMode::Html)
            .await?
        }

        Command::Weather { location } => {
            if location.is_empty() {
                bot.send_message(
                    msg.chat.id,
                    "🌍 <b>Weather Check!</b>\n\nUsage: <code>/weather [location]</code>\n\nExample: <code>/weather Toledo</code>",
                )
                .parse_mode(ParseMode::Html)
                .await?
            } else {
                let weather = get_weather(&location).await;
                if weather.name.is_empty() {
                    bot.send_message(msg.chat.id, "❌ No sé dónde está eso, colega. Revisa la ortografía.")
                        .reply_parameters(ReplyParameters::new(msg.id))
                        .await?
                } else {
                let sunrise = format_time(weather.sys.sunrise);
                let sunset = format_time(weather.sys.sunset);
                let wind_direction = get_wind_direction(weather.wind.deg);
                let visibility_str = format_visibility(weather.visibility);
                let message = format!(
                    r"🌍 <b>{}, {}</b>
📍 Coordinates: {:.2}°, {:.2}°

🌡️ <b>Temperature</b>
• Current: <b>{}°C</b> (Feels like: {}°C)
• Max: {}°C | Min: {}°C

🌦️ <b>Weather Conditions</b>
• <b>{}</b>
• {}
• ☁️ Cloud Coverage: {}%

💨 <b>Wind & Atmosphere</b>
• Wind: {}m/s {} ({:.0}°)
• Pressure: {}hPa
• Humidity: {}%
• Visibility: {}

🌅 <b>Sun Times</b>
• Sunrise: {} | Sunset: {}",
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
                        weather.clouds.all,
                        weather.wind.speed,
                        wind_direction,
                        weather.wind.deg,
                        weather.main.pressure,
                        weather.main.humidity,
                        visibility_str,
                        sunrise,
                        sunset
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
