//TODO:

use clap::Parser;
use compile_dotenv::compile_env;
use geolocation;
use reqwest::{self, blocking};
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Weather report written in Rust using OpenWeather",
    override_usage = "west <COUNTRY PROVINCE CITY FORMAT>"
)]

struct Args {
    /// ie.. CA ON TO
    #[arg(long)]
    area: Option<Vec<String>>,
    // #[arg(long)]
    // forecast: Option<bool>, // probably not, forecast api is weird
    /// Wind speed in metres/second
    #[arg(short)]
    wind: bool,

    /// Fahrenheit
    #[arg(short)]
    fahrenheit: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = compile_env!("API");
    let api_str = api_key.to_string();

    let args = Args::parse();
    let country_code;
    let province_code;
    let city_code;

    let lat;
    let long;

    let mut wtr_url = String::new();

    if let Some(area) = &args.area {
        if area.len() >= 3 {
            country_code = &area[0];
            province_code = &area[1];
            city_code = &area[2];
            // println!("Args specified");

            let geo_url = format!(
                "http://api.openweathermap.org/geo/1.0/direct?q={city_code},{province_code},{country_code}&limit=1&appid={api_str}"
            );

            let geo_response = blocking::get(&geo_url)?;
            // println!("Geo response: {:?}", geo_response.status());

            let geo_content = geo_response.text()?;

            let geo_v: Value = serde_json::from_str(&geo_content)?;

            let f_lat = geo_v[0]["lat"].as_f64().unwrap_or(0.0);
            let f_lon = geo_v[0]["lon"].as_f64().unwrap_or(0.0);

            wtr_url = format!(
                "https://api.openweathermap.org/data/2.5/weather?lat={f_lat}&lon={f_lon}&appid={api_str}"
            )
        }
    } else {
        let ip_str = reqwest::blocking::get("https://api.ipify.org")
            .unwrap()
            .text()
            .unwrap();

        let area_info = geolocation::find(&ip_str).unwrap();

        long = &area_info.longitude;
        lat = &area_info.latitude;

        wtr_url = format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={long}&appid={api_str}"
        );
    }

    let wtr_response = blocking::get(&wtr_url)?;
    let wtr_content = wtr_response.text()?;

    let wtr_v: Value = serde_json::from_str(&wtr_content)?;

    let current_temp = wtr_v["main"]["feels_like"].as_f64().unwrap_or(0.0);
    let current_cond = wtr_v["weather"][0]["main"].as_str().unwrap_or("Sunny");
    let current_wind = wtr_v["wind"]["speed"].as_f64().unwrap_or(0.0);

    let mut celsius_temp = current_temp - 273.15;

    let icon = match current_cond {
        "Rain" => "",
        "Thunderstorm" => "",
        "Drizzle" => "",
        "Clouds" => "",
        "Snow" => "",
        "Sunny" => "",
        _ => "",
    };

    if args.fahrenheit {
        celsius_temp = celsius_temp * 1.8 + 32.0;
    }

    if !args.wind {
        println!("{}C {}", celsius_temp.floor(), icon);
    } else {
        let wind_speed = current_wind / 1.0;

        println!("{}m/s", wind_speed)
    }

    Ok(())
}
