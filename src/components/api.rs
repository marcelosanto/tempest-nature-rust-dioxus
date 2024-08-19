use reqwest;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    lat: f64,
    lon: f64,
}

#[derive(Deserialize, Debug, Default)]
pub struct WeatherResponse {
    pub  weather: Vec<Weather>,
    pub  main: Main,
    pub wind: Wind,
    pub   name: String,
    pub  sys: Sys,
}

#[derive(Deserialize, Debug, Default)]
pub struct Weather {
    pub description: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct Main {
    pub  temp: f64,
    pub humidity: f64,
    pub pressure: f64,
}

#[derive(Deserialize, Debug, Default)]
pub struct Wind {
    pub  speed: f64,
}

#[derive(Deserialize, Debug, Default)]
pub struct Sys {
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64
}

pub async fn get_weather(city_name: String) -> reqwest::Result<WeatherResponse> {
    // let response = reqwest::get("https://api.ipify.org").await?;
    // let ip = response.text().await?;

    // let mut location_lat_long = Location { lat: 0.0, lon: 0.0 };

    // if city_name != "none" {
    //     location_lat_long = get_lat_lon(ip.clone()).await?;

    //     println!("{:?}", location_lat_long);
    // }

    // println!("{:?}", location_lat_long);

    dotenv().ok();

let api_key = env::var("OPENWEATHER_API_KEY").expect("A variável OPENWEATHER_API_KEY deve estar definida");
    let city: Vec<&str> = city_name.split(",").collect();
    let resp_weather = reqwest::get(
        format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}", city[0], city[1], api_key),
    )
    .await?
    .json::<WeatherResponse>()
    .await?;

    // println!("{:?}", city);

    // if city.len() == 3 {
    //     resp_weather = reqwest::get(format!("https://api.openweathermap.org/data/2.5/weather?q={},{},{}&units=metric&appid=11a70b5b6c6d329b4725068885de6f6d", city[0], city[1], city[2])).await?.json::<WeatherResponse>().await?;
    //     println!("w -> {:?}", resp_weather);
    // }

    // if city.len() == 2 {
    //     resp_weather = reqwest::get(
    //         format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid=11a70b5b6c6d329b4725068885de6f6d", city[0], city[1]),
    //     )
    //     .await?
    //     .json::<WeatherResponse>()
    //     .await?;

    //     println!("w -> {:?}", resp_weather);
    // }
    //let url_weather = format!("https://api.openweathermap.org/data/2.5/weather?q={city_name},{state_code},{country_code}&appid={api_key}");

    Ok(resp_weather)
}

pub async fn get_lat_lon(ip: String) -> reqwest::Result<Location> {
    let url = format!("{}{}", "http://ip-api.com/json/", ip);

    let response = reqwest::get(&url).await?.json::<Location>().await?;
    let local: Location = Location {
        lat: response.lat,
        lon: response.lon,
    };

    Ok(local)
}
