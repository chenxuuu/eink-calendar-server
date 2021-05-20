#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct WeatherData {
    pub code: String,
    pub daily: Vec<Data>,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Data {
    pub fxDate: String,
    pub sunrise: String,
    pub sunset: String,
    pub moonrise: String,
    pub moonset: String,
    pub moonPhase: String,
    pub tempMax: String,
    pub tempMin: String,
    pub iconDay: String,
    pub textDay: String,
    pub iconNight: String,
    pub textNight: String,
    pub wind360Day: String,
    pub windDirDay: String,
    pub windScaleDay: String,
    pub windSpeedDay: String,
    pub wind360Night: String,
    pub windDirNight: String,
    pub windScaleNight: String,
    pub windSpeedNight: String,
    pub humidity: String,
    pub precip: String,
    pub pressure: String,
    pub vis: String,
    pub cloud: String,
    pub uvIndex: String,
}

#[derive(Deserialize, Serialize)]
pub struct Hitokoto {
//    pub from: String,
//    pub from_who: String,
    pub hitokoto: String,
    pub length: u32,
}
