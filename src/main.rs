use clap::Parser;
use dotenvy::dotenv;
use reqwest::{self, blocking};
use serde_json::Value;
use std::env;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Weather report written in Rust using OpenWeather",
    override_usage = "west <COUNTRY PROVINCE CITY FORMAT>"
)]

struct Args {
    /// ie.. CA ON TO
    #[arg(default_values = &["CA", "ON", "TO"])]
    area: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("API").expect("API key not found");

    let args = Args::parse();

    let area = args.area;
    let country_code = &area[0];
    let province_code = &area[1];
    let city_code = &area[2];

    let api_str = api_key.to_string();

    let geo_url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={city_code},{province_code},{country_code}&limit=1&appid={api_str}"
    );

    let geo_response = blocking::get(&geo_url)?;
    let geo_content = geo_response.text()?;

    let geo_v: Value = serde_json::from_str(&geo_content)?;

    let lat = geo_v[0]["lat"].as_f64().unwrap_or(0.0);
    let lon = geo_v[0]["lon"].as_f64().unwrap_or(0.0);

    let wtr_url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={api_str}"
    );

    let wtr_response = blocking::get(&wtr_url)?;
    let wtr_content = wtr_response.text()?;

    let wtr_v: Value = serde_json::from_str(&wtr_content)?;

    let current_temp = wtr_v["main"]["temp"].as_f64().unwrap_or(0.0);

    let celcius_temp = current_temp - 273.15;

    println!("{}", celcius_temp.floor());

    Ok(())
}
