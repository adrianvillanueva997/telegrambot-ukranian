use std::env;

use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use super::model::OpenWeather;

/// Retrieves the weather information for a given city.
///
/// # Arguments
///
/// * `city` - The name of the city.
///
/// # Panics
///
/// This function will panic if the environment key for `OpenWeather` is not set up.
pub async fn get_weather(city: &str) -> OpenWeather {
    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            // TODO: Update openapi to 3.0
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
            city,
            env::var("openweather_key").expect("Environment key not set up")
        ))
        .header(AUTHORIZATION, "Bearer [AUTH_TOKEN]")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return OpenWeather::default();
    }
    let result = response.json::<OpenWeather>().await;
    result.unwrap_or_else(|err| {
        panic!("{}", err);
    })
}
