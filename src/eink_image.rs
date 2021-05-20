use image::{ImageBuffer, GrayImage,open};
use imageproc::drawing;
use rusttype::{Font,Scale};
use crate::weather;
use chrono::prelude::*;
use lazy_static::*;
use std::fs::File;
use std::io::Read;
use std::convert::TryInto;

//获取资源文件路径
fn get_path() -> String{
    std::env::args().nth(1).expect("static/")
}
//加载字体文件
fn load_font(path: String) -> Font<'static>{
    let mut file = File::open(&path).expect(&path);
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    Font::try_from_vec(data).unwrap()
}
//静态加载字体
lazy_static! {
    static ref FONT_STATIC: Font<'static> = load_font(get_path() + "sarasa-regular.ttc");
    static ref FONT_PIXEL: Font<'static> = load_font(get_path() + "LanaPixel.ttf");
    static ref FONT_ART: Font<'static> = load_font(get_path() + "ShangShouShaoNianTi.ttf");
}

//摆放天气图片
fn put_weather_img(img: &mut GrayImage,n: u32,x: u32, y: u32) {
    let on_top = open(get_path()+"bw-64/"+&n.to_string()+".png").unwrap().into_luma8();
    image::imageops::overlay(img, &on_top, x, y);
}

//屏幕长宽
const WIDTH:u32 = 400;
const HEIGHT:u32 = 300;

//四种颜色
const WHITE:image::Luma<u8> = image::Luma([255]);
const GRAY1:image::Luma<u8> = image::Luma([159]);
const GRAY2:image::Luma<u8> = image::Luma([ 96]);
const BLACK:image::Luma<u8> = image::Luma([ 0]);

pub fn get_eink_image(w: &weather::WeatherData, h: &weather::Hitokoto, _imei: u64, v: u32) -> Vec<u8>{
    // 构建具有指定宽度和高度的RGB图像缓冲区。
    let mut img: GrayImage = ImageBuffer::new(WIDTH, HEIGHT);

    //刷白
    drawing::draw_filled_rect_mut(&mut img,imageproc::rect::Rect::at(0, 0).of_size(WIDTH, HEIGHT),WHITE);

    //当前时间（加点时间防止刷新完过了
    let dt = Local::now() + chrono::Duration::seconds(45);

    //显示天气
    if w.code == "200" {
        for n in 0..6 {
            let now = w.daily.get(n).unwrap();
            put_weather_img(&mut img,now.iconDay.to_string().parse::<u32>().unwrap(),66*n as u32,172);
            put_weather_img(&mut img,now.iconNight.to_string().parse::<u32>().unwrap(),66*n as u32,236);
            match n {
                0 => drawing::draw_text_mut(&mut img, BLACK, 16+66*n as u32,228, Scale {x: 20.0,y: 20.0 }, &FONT_STATIC,"今天"),
                1 => drawing::draw_text_mut(&mut img, BLACK, 16+66*n as u32,228, Scale {x: 20.0,y: 20.0 }, &FONT_STATIC,"明天"),
                2 => drawing::draw_text_mut(&mut img, BLACK, 16+66*n as u32,228, Scale {x: 20.0,y: 20.0 }, &FONT_STATIC,"后天"),
                _ => {
                    let dt_now = dt + chrono::Duration::days(n as i64);
                    drawing::draw_text_mut(&mut img, BLACK, 16+66*n as u32,228, Scale {x: 20.0,y: 20.0 }, &FONT_STATIC,&(dt_now.day().to_string()+"日"))
                }
            }
        }
    }

    //写上日期
    drawing::draw_text_mut(&mut img, BLACK, 0,0, Scale {x: 170.0,y: 170.0 }, &FONT_ART, &format!("{:02}",dt.day()));
    drawing::draw_text_mut(&mut img, BLACK, 10,0, Scale {x: 35.0,y: 35.0 }, &FONT_ART, &format!("{:04}",dt.year()));
    drawing::draw_text_mut(&mut img, BLACK, 85,0, Scale {x: 35.0,y: 35.0 }, &FONT_ART, &format!("{:2}月",dt.month()));
    drawing::draw_text_mut(&mut img, BLACK, 35,140, Scale {x: 35.0,y: 35.0 }, &FONT_ART,
        &format!("星期{}",match dt.weekday().number_from_monday() {
            1 => "一",2 => "二",3 => "三",4 => "四",5 => "五",6 => "六",7 => "日",
            _ => ""
        }
    ));

    //一言
    drawing::draw_text_mut(&mut img, BLACK, 160,30, Scale {x: 40.0,y: 40.0 }, &FONT_ART, &h.hitokoto[0..21]);
    drawing::draw_text_mut(&mut img, BLACK, 160,90, Scale {x: 40.0,y: 40.0 }, &FONT_ART, &h.hitokoto[24..45]);

    //电量
    let battery: f64 = (v as f64 - 3400.0)/700.0;
    let battery = match battery {
        _ if battery > 1.0 => 1.0,
        _ if battery < 0.0 => 0.0,
        _ => battery
    };
    drawing::draw_text_mut(&mut img, BLACK, 0,290, Scale {x: 12.0,y: 12.0 }, &FONT_PIXEL, &format!("{:.0}%",battery*100.0));

    generate_eink_bytes(&img)
}

//生成最终的图片序列
fn generate_eink_bytes(img: &GrayImage)->Vec<u8>{
    let mut r1:Vec<u8> = Vec::new();//第一张
    let mut r2:Vec<u8> = Vec::new();//第二张
    for y in 0..HEIGHT {
        for l in 0..WIDTH/8 {
            let mut temp1:u8 = 0;
            let mut temp2:u8 = 0;
            for i in 0..8 {
                let p:u8 = img.get_pixel(l*8+i,y)[0];
                let (t1,t2) = match p {//匹配像素点颜色
                    192..=255 => (1,1),
                    128..=191 => (1,0),
                    64 ..=127 => (0,1),
                    0  ..=63  => (0,0),
                };
                temp1+=t1<<(7-i);
                temp2+=t2<<(7-i);
            }
            r1.push(temp1);
            r2.push(temp2);
        }
    }

    r1.append(&mut r2);
    r1
}
