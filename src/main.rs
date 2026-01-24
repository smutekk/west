use clap::Parser;
use dotenv::dotenv;
use std::env;
// use reqwest::blocking;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Weather report written in Rust using OpenWeather",
    override_usage = "uvi <COUNTRY PROVINCE CITY FORMAT>"
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

    let args = Args::parse();

    let area = args.area;
    let country_code = &area[0];
    let province_code = &area[1];
    let city_code = &area[2];

    println!("{:?}", &country_code);

    let api_key = env::var("API");

    Ok(())
}
