use image::{ GenericImageView };
use image::imageops::FilterType;
use std::{thread, time};
use std::fs;

fn main() {
    let time_to_wait = time::Duration::from_millis(120);
    
    // image directory
    let directory = "./badapple_frames_vlc";

    let paths = fs::read_dir(directory).unwrap();

    for path in paths {
        if path.as_ref().unwrap().file_name().to_str().is_some() {
            get_image_by_pixels(
                format!("{}/{}", directory, path.unwrap().file_name().to_str().unwrap()).as_str(),
                10
            );
            thread::sleep(time_to_wait);
        }
    }
}

fn get_image_by_pixels(dir: &str, resolution: u32) {
    let mut last_y = 0;

    // get image from path
    let img = image::open(dir).unwrap();

    let resized_img = img.resize(img.width() / resolution, img.height() / resolution, FilterType::Nearest);

    // println!("original size: {:?}, resized size: {:?}", img.dimensions(), resized_img.dimensions());

    for pixel in resized_img.pixels() {
        // pixel: (347, 154, Rgba([0, 0, 0, 255])) // width, height

        // new line to be inserted
        if last_y != pixel.1 {
            last_y = pixel.1;
            println!("");
        }
        
        let mut rgb: u32 = pixel.2.0[0].into();
        rgb += pixel.2.0[1] as u32;
        rgb += pixel.2.0[2] as u32;

        // println!("rgb: {rgb}");

        let brightness: f32 = (rgb / 3) as f32;

        print!("{}", get_str_ascii(brightness));
        print!("{}", get_str_ascii(brightness));
    }

    println!(":)");
}

fn get_str_ascii(brightness: f32) -> &'static str{
    let character_set = [" ", ".", ",", "-", "~", "+", "=", "@"]; // len of 8
    // [" ", ".", ",", "-", "~", "+", "=", "@"]

    let index = ((brightness / 255.0) * (character_set.len() as f32 - 1.0)).round();
    return character_set[index as usize];
}