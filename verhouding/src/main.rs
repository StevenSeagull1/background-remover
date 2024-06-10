use image::{DynamicImage, GenericImageView, Rgba};
use std::path::Path;
use image::GenericImage;
use image::RgbaImage;
const TOLERANCE:u8 = 25;
const TOLERANCE_BLACK:i16 = 50;


fn pixels_are_similar1(pixel1: [u8; 4], pixel2: [u8; 4], tolerance: u8) -> bool {
    let r_diff = (pixel1[0] as i16 - pixel2[0] as i16).abs();
    let g_diff = (pixel1[1] as i16 - pixel2[1] as i16).abs();
    let b_diff = (pixel1[2] as i16 - pixel2[2] as i16).abs();

    r_diff <= tolerance as i16 && g_diff <= tolerance as i16 && b_diff <= 
    tolerance as i16
}

fn pixels_are_similar2(pixel1: [u8; 4], pixel2: [u8; 4]) -> bool {
    let r1 = pixel1[0]  as f32 + 10.0;
    let g1 = pixel1[1]  as f32 + 10.0;
    let b1 = pixel1[2]  as f32 + 10.0;
 
    let r2 = pixel2[0]  as f32 + 10.0;
    let g2 = pixel2[1]  as f32 + 10.0;
    let b2 = pixel2[2]  as f32 + 10.0;

    let t: f32 = 25.0;

    if (r1 < t && g1 < t && b1 < t) || (r2 < t && g2 < t && b2 < t){
        let same = pixels_are_similar1(pixel1, pixel2, TOLERANCE);
        return same
    }
     
    let Av = 1.0;
    let x = ((r1 / r2 - Av).abs() + (g1 / g2 - Av).abs() + (b1 / b2 - Av).abs()) / 3.0;
    let tolerance = 0.5;

     x <= tolerance
}

fn remove_background(base_image: &DynamicImage, comparison_image: 
    &DynamicImage, tolerance: f32) -> RgbaImage {
    let (width, height) = base_image.dimensions();
    let mut output = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let base_pixel = base_image.get_pixel(x, y).0;
            let comparison_pixel = comparison_image.get_pixel(x, y).0;
            // let result = pixels_are_similar2(base_pixel, comparison_pixel);
            // println!("{}", result);
            if pixels_are_similar2(base_pixel, comparison_pixel) {

                output.put_pixel(x, y, base_image.get_pixel(x, y));
            }
        }
    }

    output
}

fn main() {
    let base_image = image::open("selfies/selfie02.png").unwrap();
    let comparison_image = image::open("selfies/leeg01.png").unwrap();
    
    let tolerance = 0.1; 
    let output_image = remove_background(&base_image, &comparison_image,
        tolerance);

    output_image.save("ja.png").unwrap();
}