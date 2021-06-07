//#![deny(warnings)]

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::Filter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing::{debug,info};
mod weather;
mod eink_image;
extern crate log;

#[derive(Deserialize, Serialize)]
struct Calendar {
    voltage: u32,
    two_color: bool,
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
) -> Result<Vec<u8>, &'static str> {
    info!("new request");

    //当前时间（加点时间防止刷新完过了
    let mut dt = Local::now() + chrono::Duration::seconds(45);
    let now = TimeStruct {
        year: dt.year() as u16,
        month: dt.month() as u8,
        day: dt.day() as u8,
        hour: dt.hour() as u8,
        minute: dt.minute() as u8,
        second: dt.second() as u8,
    };

    //第二天零点
    if dt.hour() >= 16 {
        dt = dt + chrono::Duration::days(1);
    }
    let next = TimeStruct {
        year: dt.year() as u16,
        month: dt.month() as u8,
        day: dt.day() as u8,
        hour: if dt.hour() >= 16 {0}else if dt.hour() >= 8 {16}else{8},
        minute: 0,
        second: 0,
    };

    let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(5)).build().expect("http client build error");
    debug!("get weather");
    let resp = client.get(&format!(
        "https://devapi.qweather.com/v7/weather/7d?location={},{}&key={}",
        lng,
        lat,
        String::from_utf8_lossy(include_bytes!("weather.key"))
    )).send().await.expect("http send error").text().await.expect("http recv error");
    //debug用，使用固定的天气信息
    //let resp = String::from_utf8_lossy(include_bytes!("weather_json.json"));
    let weather_data: weather::WeatherData = serde_json::from_str(&resp).expect("json decode error");

    //一言
    debug!("get hitokoto");
    let hitokoto = if dt.hour() < 8 {
        weather::Hitokoto{
            hitokoto : String::from("早上好呀！诶嘿嘿。。"),
            length : 30,
        }
    }
    else {
        let resp = client.get("https://v1.hitokoto.cn/?c=i&max_length=16")
        .send().await.expect("http send error").text().await.expect("http recv error");
        serde_json::from_str(&resp).expect("json decode error")
    };

    //用来存放最终返回的结果
    let mut reply: Vec<u8> = Vec::new();
    //当前时间
    reply.append(&mut bincode::serialize(&now).expect("bincode encode error"));
    //下次开机的时间
    reply.append(&mut bincode::serialize(&next).expect("bincode encode error"));

    //最终的图片
    debug!("get img");
    reply.append(&mut eink_image::get_eink_image(&weather_data,&hitokoto,imei,device.voltage,!device.two_color)?);

    debug!("all done");
    Ok(reply)
}

async fn eink_server(
    imei: u64,
    lng: f64,
    lat: f64,
    device: Calendar,
) -> Result<impl warp::Reply, Infallible> {
    Ok(get_eink_data(imei,lng,lat,device).await.unwrap())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        //.with_env_filter(filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // POST /eink-calendar/:imei/:lat/:lng  {"voltage":4100}
    let eink = warp::post()
        .and(warp::path("eink-calendar"))
        .and(warp::path::param::<u64>())
        .and(warp::path::param::<f64>())
        .and(warp::path::param::<f64>())
        .and(warp::body::content_length_limit(1024 * 2))
        .and(warp::body::json())
        .and_then(eink_server)
        .with(warp::trace::named("log"));

    let hello = warp::get().and(warp::path("eink-calendar")).and(warp::path("hello"))
        .map(||"hello world!");

    warp::serve(eink.or(hello)).run(([127, 0, 0, 1], 10241)).await
}
