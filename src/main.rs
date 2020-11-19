use image::Rgba;
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::env;
use std::path::Path;

fn main() {
    let sb_img = image::open(&Path::new("./source.png"))
        .ok()
        .expect("Can't open the source image.");

    let text = if env::args().count() == 3 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter search text")
    };
    let text2 = if env::args().count() == 3 {
        env::args().nth(2).unwrap()
    } else {
        panic!("Please enter suggestion text")
    };

    let path = Path::new("result.png");

    let mut image = sb_img.to_rgba8();

    let font = Vec::from(include_bytes!("LiberationSans-Regular.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();
    let font2 = Vec::from(include_bytes!("LiberationSans-BoldItalic.ttf") as &[u8]);
    let font2 = Font::try_from_vec(font2).unwrap();
    let height = 18f32;
    let scale = Scale {
        x: height * 1.0,
        y: height,
    };

    draw_text_mut(
        &mut image,
        Rgba([0u8, 0u8, 0u8, 255u8]),
        160,
        27,
        scale,
        &font,
        &text,
    );

    draw_text_mut(
        &mut image,
        Rgba([26u8, 13u8, 171u8, 255u8]),
        282,
        170,
        scale,
        &font2,
        &text2,
    );

    let _ = image.save(path).unwrap();
}
