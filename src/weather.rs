#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct WeatherData {
    pub HeWeather6: Vec<Data>,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Data {
    pub basic: Basic,
    pub update: Update,
    pub status: String,
    pub daily_forecast: Vec<Daily>,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Update {
    pub loc: String,
    pub utc: String,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Basic {
    pub cid: String,
    pub location: String,
    pub parent_city: String,
    pub admin_area: String,
    pub cnty: String,
    pub lat: String,
    pub lon: String,
    pub tz: String,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Daily {
    pub cond_code_d: String,
    pub cond_code_n: String,
    pub cond_txt_d: String,
    pub cond_txt_n: String,
    pub date: String,
    pub hum: String,
    pub mr: String,
    pub ms: String,
    pub pcpn: String,
    pub pop: String,
    pub pres: String,
    pub sr: String,
    pub ss: String,
    pub tmp_max: String,
    pub tmp_min: String,
    pub uv_index: String,
    pub vis: String,
    pub wind_deg: String,
    pub wind_dir: String,
    pub wind_sc: String,
    pub wind_spd: String,
}
