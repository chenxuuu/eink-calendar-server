use image::{ImageBuffer, GrayImage,Pixel};
use imageproc::drawing;
use rusttype::{Font,Scale};
use crate::weather;

//屏幕长宽
const WIDTH:u32 = 400;
const HEIGHT:u32 = 300;

//四种颜色
const WHITE:u8 = 255;
const GRAY1:u8 = 200;
const GRAY2:u8 = 100;
const BLACK:u8 = 0;

pub fn get_eink_image(w: &weather::Data, imei: u64, v: u32) -> Vec<u8>{
    // 构建具有指定宽度和高度的RGB图像缓冲区。
    let mut img: GrayImage = ImageBuffer::new(WIDTH, HEIGHT);
    //刷白
    drawing::draw_filled_rect_mut(&mut img,imageproc::rect::Rect::at(0, 0).of_size(WIDTH-1, HEIGHT-1),image::Luma([255]));
    //加载字体
    let font = Vec::from(include_bytes!("sarasa-regular.ttc") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();
    //写字
    drawing::draw_text_mut(&mut img, image::Luma([BLACK]), 0,0, Scale {x: 20.0,y: 20.0 }, &font, "hello 测试中文");
    img.save("done.png").unwrap();

    generate_eink_bytes(img)
}

//生成最终的图片序列
fn generate_eink_bytes(img: GrayImage)->Vec<u8>{
    let mut r1:Vec<u8> = Vec::new();//第一张
    let mut r2:Vec<u8> = Vec::new();//第二张
    for y in 0..HEIGHT {
        for l in 0..WIDTH/8 {
            let mut temp1:u8 = 0;
            let mut temp2:u8 = 0;
            for i in 0..8 {
                let p:u8 = img.get_pixel(l*8+i,y)[0];
                let (t1,t2) = match p {//匹配像素点颜色
                    201..=255 => (1,1),
                    101..=200 => (1,0),
                    1..=100 => (0,1),
                    _ => (0,0),
                };
                temp1+=t1<<i;
                temp2+=t2<<i;
            }
            r1.push(temp1);
            r2.push(temp2);
        }
    }

    r1.append(&mut r2);
    r1
}
