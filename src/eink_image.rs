use image::{ImageBuffer, GrayImage};
use imageproc::drawing;
use rusttype::{Font,Scale};
use crate::weather;
use chrono::prelude::*;
use lazy_static::*;
use std::fs::File;
use std::io::Read;

//加载字体文件
fn load_font(path: String) -> Font<'static>{
    let mut file = File::open(path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    Font::try_from_vec(data).unwrap()
}
//静态加载字体
lazy_static! {
    static ref FONT_STATIC: Font<'static> = load_font(std::env::args().nth(1).expect("sarasa-regular.ttc"));
    static ref FONT_PIXEL: Font<'static> = load_font(std::env::args().nth(2).expect("LanaPixel.ttf"));
    static ref FONT_ART: Font<'static> = load_font(std::env::args().nth(3).expect("ShangShouShaoNianTi.ttf"));
}

//屏幕长宽
const WIDTH:u32 = 400;
const HEIGHT:u32 = 300;

//四种颜色
const WHITE:image::Luma<u8> = image::Luma([223]);
const GRAY1:image::Luma<u8> = image::Luma([159]);
const GRAY2:image::Luma<u8> = image::Luma([ 96]);
const BLACK:image::Luma<u8> = image::Luma([ 32]);

pub fn get_eink_image(w: &weather::WeatherData, imei: u64, v: u32) -> Vec<u8>{
    // 构建具有指定宽度和高度的RGB图像缓冲区。
    let mut img: GrayImage = ImageBuffer::new(WIDTH, HEIGHT);
    //刷白
    drawing::draw_filled_rect_mut(&mut img,imageproc::rect::Rect::at(0, 0).of_size(WIDTH, HEIGHT),image::Luma([255]));

    //当前时间（加点时间防止刷新完过了
    let dt = Local::now() + chrono::Duration::seconds(45);

    //写字
    drawing::draw_text_mut(&mut img, BLACK, 0,0, Scale {x: 40.0,y: 40.0 }, &FONT_ART,&(dt.month().to_string()+"月"));
    drawing::draw_text_mut(&mut img, BLACK, 0,40, Scale {x: 80.0,y: 80.0 }, &FONT_ART, &dt.day().to_string());


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
