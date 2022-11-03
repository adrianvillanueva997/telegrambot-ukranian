use std::env;

use super::weather_model::Weather;

pub async fn get_weather(city: &str) -> Result<Weather, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let body = client
        .get(format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
            city,
            env::var("openweather_key").expect("Environment key not set up")
        ))
        .send()
        .await?
        .json::<Weather>()
        .await?;

    Ok(body)
}
