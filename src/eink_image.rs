use image::{ImageBuffer, GrayImage,open};
use imageproc::drawing;
use rusttype::{Font,Scale};
use crate::weather;
use chrono::prelude::*;
use lazy_static::*;
use std::fs::File;
use std::io::Read;

//获取资源文件路径
pub fn get_path() -> String{
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
    let on_top = open(get_path()+"bw-64/"+&n.to_string()+".png").expect("open weather image error").into_luma8();
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

pub fn get_eink_image(w: &weather::WeatherData, h: &weather::Hitokoto, _imei: u64, v: u32, gray: bool) -> Result<Vec<u8>, &'static str>{
    // 构建具有指定宽度和高度的RGB图像缓冲区。
    let mut img: GrayImage = ImageBuffer::new(WIDTH, HEIGHT);

    //刷白
    drawing::draw_filled_rect_mut(&mut img,imageproc::rect::Rect::at(0, 0).of_size(WIDTH, HEIGHT),WHITE);

    //当前时间（加点时间防止刷新完过了
    let dt = Local::now() + chrono::Duration::seconds(45);

    //显示天气
    if w.code == "200" {
        let offset = if w.daily.get(0).expect("get weather day error").fxDate == dt.format("%Y-%m-%d").to_string() {
            0
        }else{
            1
        };
        for n in 0..6 {
            let now = w.daily.get(n+offset).expect("get weather day error");
            put_weather_img(&mut img,now.iconDay.to_string().parse::<u32>().expect("parse error"),66*n as u32,172);
            put_weather_img(&mut img,now.iconNight.to_string().parse::<u32>().expect("parse error"),66*n as u32,236);
            match n {
                0 => drawing::draw_text_mut(&mut img, BLACK, 15+66*n as u32,228, Scale {x: 22.0,y: 22.0 }, &FONT_PIXEL,"今天"),
                1 => drawing::draw_text_mut(&mut img, BLACK, 15+66*n as u32,228, Scale {x: 22.0,y: 22.0 }, &FONT_PIXEL,"明天"),
                2 => drawing::draw_text_mut(&mut img, BLACK, 15+66*n as u32,228, Scale {x: 22.0,y: 22.0 }, &FONT_PIXEL,"后天"),
                _ => {
                    let dt_now = dt + chrono::Duration::days(n as i64);
                    drawing::draw_text_mut(&mut img, BLACK, 15+66*n as u32,228, Scale {x: 22.0,y: 22.0 }, &FONT_PIXEL,&(dt_now.day().to_string()+"日"))
                }
            }
        }
        //当日天气信息
        let now = w.daily.get(offset).expect("get weather day error");
        drawing::draw_text_mut(&mut img, BLACK, 5,120, Scale {x: 35.0,y: 35.0 }, &FONT_STATIC,&format!("{}~{}℃",now.tempMax,now.tempMin));
        drawing::draw_text_mut(&mut img, BLACK, 143,140, Scale {x: 20.0,y: 20.0 }, &FONT_STATIC,&format!("相对湿度{}%",now.humidity));
        drawing::draw_text_mut(&mut img, BLACK, 10,160, Scale {x: 20.0,y: 20.0 }, &FONT_STATIC,
            &format!("白天{}{}级 夜间{}{}级",now.windDirDay,now.windScaleDay,now.windDirNight,now.windScaleNight));
    }

    //写上日期
    drawing::draw_text_mut(&mut img, BLACK, 250,0, Scale {x: 170.0,y: 170.0 }, &FONT_ART, &format!("{:02}",dt.day()));
    drawing::draw_text_mut(&mut img, BLACK, 250+10,0, Scale {x: 35.0,y: 35.0 }, &FONT_ART, &format!("{:04}",dt.year()));
    drawing::draw_text_mut(&mut img, BLACK, 250+85,0, Scale {x: 35.0,y: 35.0 }, &FONT_ART, &format!("{:2}月",dt.month()));
    drawing::draw_text_mut(&mut img, BLACK, 250+35,140, Scale {x: 35.0,y: 35.0 }, &FONT_ART,
        &format!("星期{}",match dt.weekday().number_from_monday() {
            1 => "一",2 => "二",3 => "三",4 => "四",5 => "五",6 => "六",7 => "日",
            _ => ""
        }
    ));

    //一言
    {
        let split = h.hitokoto.len()/3;
        let offset = (8-(split as u32)/2)*16;
        let split = split/2*3;
        drawing::draw_text_mut(&mut img, BLACK, offset,15, Scale {x: 40.0,y: 40.0 }, &FONT_ART, &h.hitokoto[0..split]);
        drawing::draw_text_mut(&mut img, BLACK, offset,70, Scale {x: 40.0,y: 40.0 }, &FONT_ART, &h.hitokoto[split..]);
    }

    //电量
    {
        let battery: f64 = v as f64 * 5.0 / 3000.0 - 6.0;
        let battery = match battery {
            _ if battery > 1.0 => 1.0,
            _ if battery < 0.0 => 0.0,
            _ => battery
        };
        drawing::draw_text_mut(&mut img, BLACK, 26,291, Scale {x: 11.0,y: 11.0 }, &FONT_PIXEL, &format!("{:.0}%",battery*100.0));
        drawing::draw_filled_rect_mut(&mut img,imageproc::rect::Rect::at(3, 290).of_size(22, 10),BLACK);
        drawing::draw_filled_rect_mut(&mut img,imageproc::rect::Rect::at(1, 293).of_size(2, 4),BLACK);
        drawing::draw_filled_rect_mut(&mut img,imageproc::rect::Rect::at(4, 291).of_size((20.0*(1.0-battery)) as u32, 8),WHITE);
    }

    //更新时间
    drawing::draw_text_mut(&mut img, BLACK, 260,291, Scale {x: 11.0,y: 11.0 }, &FONT_PIXEL, &dt.format("上次更新: %Y-%m-%d %H:%M:%S").to_string());

    Ok(generate_eink_bytes(&img,gray))
}

//生成最终的图片序列
fn generate_eink_bytes(img: &GrayImage, gray: bool)->Vec<u8>{
    let mut r1:Vec<u8> = Vec::new();//第一张
    let mut r2:Vec<u8> = Vec::new();//第二张
    for y in 0..HEIGHT {
        for l in 0..WIDTH/8 {
            let mut temp1:u8 = 0;
            let mut temp2:u8 = 0;
            for i in 0..8 {
                let p:u8 = img.get_pixel(l*8+i,y)[0];
                //匹配像素点颜色
                let (t1,t2) = if gray {
                    match p {
                        192..=255 => (1,1),
                        128..=191 => (1,0),
                        64 ..=127 => (0,1),
                        0  ..=63  => (0,0),
                    }
                }else{
                    match p {
                        128..=255 => (1,1),
                        0 ..=127 => (0,0),
                    }
                };
                temp1+=t1<<(7-i);
                temp2+=t2<<(7-i);
            }
            r1.push(temp1);
            r2.push(temp2);
        }
    }
    if gray {
        r1.append(&mut r2);
    }
    r1
}
