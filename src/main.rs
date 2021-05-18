//#![deny(warnings)]

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::Filter;
mod weather;
mod eink_image;
#[macro_use]
extern crate log;

#[derive(Deserialize, Serialize)]
struct Calendar {
    voltage: u32,
}

#[repr(C)]
#[derive(Deserialize, Serialize)]
struct TimeStruct {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

async fn get_eink_data(
    imei: u64,
    lng: f64,
    lat: f64,
    device: Calendar,
) -> Result<impl warp::Reply, Infallible> {
    //当前时间（加点时间防止刷新完过了
    let dt = Local::now() + chrono::Duration::seconds(45);
    let now = TimeStruct {
        year: dt.year() as u16,
        month: dt.month() as u8,
        day: dt.day() as u8,
        hour: dt.hour() as u8,
        minute: dt.minute() as u8,
        second: dt.second() as u8,
    };
    //第二天零点
    let dt = dt + chrono::Duration::days(1);
    let next = TimeStruct {
        year: dt.year() as u16,
        month: dt.month() as u8,
        day: dt.day() as u8,
        hour: 0,
        minute: 0,
        second: 0,
    };

    let resp = reqwest::get(
        &format!(
            "https://free-api.heweather.com/s6/weather/forecast?location={},{}&key={}",
            lng,
            lat,
            String::from_utf8_lossy(include_bytes!("weather.key"))
        )
    ).await.unwrap().text().await.unwrap();

    let v: weather::WeatherData = serde_json::from_str(&resp).unwrap();
    let weather_data = v.HeWeather6.get(0).unwrap();

    //用来存放最终返回的结果
    let mut reply: Vec<u8> = Vec::new();
    //当前时间
    reply.append(&mut bincode::serialize(&now).unwrap());
    //下次开机的时间
    reply.append(&mut bincode::serialize(&next).unwrap());

    //最终的图片
    reply.append(&mut eink_image::get_eink_image(weather_data,imei,device.voltage));

    Ok(reply)
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // POST /eink-calendar/:imei/:lat/:lng  {"voltage":4100}
    let eink = warp::post()
        .and(warp::path("eink-calendar"))
        .and(warp::path::param::<u64>())
        .and(warp::path::param::<f64>())
        .and(warp::path::param::<f64>())
        .and(warp::body::content_length_limit(1024 * 2))
        .and(warp::body::json())
        .and_then(get_eink_data);

    info!("{}", "server start");
    warp::serve(eink).run(([127, 0, 0, 1], 10241)).await
}
