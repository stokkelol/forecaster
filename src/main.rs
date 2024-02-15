extern crate dotenv;

use dotenv::dotenv;
use frankenstein::Api;
use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;
use serde::{Deserialize, Serialize};
use std::env;

const MIN: f32 = 273.15;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    wind: Wind,
    clouds: Clouds,
    sys: Sys,
    name: String,
    visibility: i32,
    timezone: i32,
    id: i32,
    cod: i32,
    dt: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f32,
    lat: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Main {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: i32,
    humidity: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f32,
    deg: i32,
    //gust: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    #[serde(rename(deserialize = "type"))]
    typ: i32,
    id: i32,
    country: String,
    sunrise: i64,
    sunset: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let resp = req().await?;
    let api = Api::new(env::var("TELEGRAM_API_KEY").unwrap().as_str());
    let str = format!(
        "Current temperature in Kyiv is {:.4}°C, feels like {:.2}°C; {}.",
        to_celsius(resp.main.temp).to_string(), to_celsius(resp.main.feels_like), resp.weather.first().unwrap().description
    );

    let params = SendMessageParams::builder()
        .chat_id(env::var("TELEGRAM_CHANNEL_ID").unwrap().to_string())
        .text(str)
        .build();

    api.send_message(&params).expect("Error sending API request.");

    Ok(())
}

async fn req() -> Result<Data, Box<dyn std::error::Error>> {
    let resp = reqwest::get(format!(
        "https://api.openweathermap.org/data/2.5/weather?q=Kyiv&appid={}",
        env::var("OPENWEATHER_API_KEY").unwrap().to_string()
    ))
    .await?
    .json::<Data>()
    .await?;
    Ok(resp)
}

fn to_celsius(temp: f32) -> f32 {
    temp - MIN
}
