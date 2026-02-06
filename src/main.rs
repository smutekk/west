use clap::Parser;
use dotenv::dotenv;
use reqwest::{self, blocking};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Weather report written in Rust using OpenWeather",
    override_usage = "west <COUNTRY PROVINCE CITY FORMAT>"
)]

struct Args {
    /// ie.. CA ON TO
    area: Vec<String>,

    // /// Or state if you're from one of THOSE countries
    // #[arg(long)]
    // province: String,

    // #[arg(long)]
    // city: String,
    /// ie.. F for Ferenheit, C for Celcius
    #[arg(long)]
    format: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("API");

    let args = Args::parse();

    let area = args.area;
    let country_code = &area[0];
    let province_code = &area[1];
    let city_code = &area[2];

    let api_str = api_key.unwrap().to_string();

    let geo_url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={city_code},{province_code},{country_code}&limit=1&appid={api_str}"
    );

    let response = blocking::get(&geo_url)?;
    let content = response.text();

    // TODO: get lat and long from content

    // println!("{:?}", &content);

    println!("{}", &geo_url);
    println!("{:?}", &country_code);

    Ok(())
}
