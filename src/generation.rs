use std::path::{Path, PathBuf};
//use std::fs::{self, File};
use crate::solarized::{
    print_colored,
    ORANGE,
//    clear
};
use crate::common::{unsupported, feedback};
use qrcode_generator::QrCodeEcc;
//use image::{ImageBuffer, Luma, Rgb};
//use gif;
//use rand::Rng;

const MAX_TEXT_SIZE: usize = 2048;

pub fn png(text: &str, filename: &str) {
    feedback(&text, &filename);
    qrcode_generator::to_png_to_file(text, QrCodeEcc::Low, 1024, filename).unwrap();
}

pub fn svg(text: &str, filename: &str) {
    feedback(&text, &filename);
    qrcode_generator::to_svg_to_file(text, QrCodeEcc::Low, 1024, None::<&str>, filename).unwrap();
}

pub fn chunked(text: &str, filename: &str) {
    print_colored(
        &["Size exceeds 2K > chunking."],
        &[ORANGE],
    );
    let mut base_filename = String::from(filename);
    if filename == "output.png" {
        let mut counter = 1;
        loop {
            base_filename = format!("output_{}.png", counter);
            if !Path::new(&format!("{}_part_0.png", base_filename)).exists() {
                break;
            }
            counter += 1;
        }
    }
    let chunks = text.as_bytes().chunks(MAX_TEXT_SIZE);
    let extension = Path::new(&base_filename).extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    match extension {
        "png" => {
            for (i, chunk) in chunks.enumerate() {
                let chunk_str = String::from_utf8_lossy(chunk);
                let filename_with_index = format!("{}_part_{}.png", base_filename, i);
                png(&chunk_str, &filename_with_index);
                print_colored(
                    &["chunk written."],
                    &[ORANGE],
                );
            }
            return;
        }
        "svg" => {
            for (i, chunk) in chunks.enumerate() {
                let chunk_str = String::from_utf8_lossy(chunk);
                let filename_with_index = format!("{}_part_{}.svg", base_filename, i);
                svg(&chunk_str, &filename_with_index);
                print_colored(
                    &["chunk written."],
                    &[ORANGE],
                );
            }
            return;
        }
        _ => {
            let mut new_filename = PathBuf::from(base_filename);
            new_filename.set_extension("png");
            let new_filename_str = new_filename.to_str().unwrap_or("output.png");
            unsupported(new_filename_str);
            for (i, chunk) in chunks.enumerate() {
                let chunk_str = String::from_utf8_lossy(chunk);
                let filename_with_index = format!("{}_part_{}.png", new_filename_str, i);
                png(&chunk_str, &filename_with_index);
                print_colored(
                    &["chunk written."],
                    &[ORANGE],
                );
            }
            return;
        }
    }
}

/*
pub fn colors(text: &str, filename: &str) {
    feedback(&text, &filename);
    let img_u8: ImageBuffer<Luma<u8>, Vec<u8>> = qrcode_generator::to_image_buffer(text, QrCodeEcc::Low, 1024)
        .unwrap();
    let mut colored_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(img_u8.width(), img_u8.height());
    for (x, y, pixel) in img_u8.enumerate_pixels() {
        let Luma([value]) = *pixel;
        let new_color: Rgb<u8> = if value == 0 {
            Rgb([(0), (200), (200)])
        } else {
            Rgb([(200), (0), (200)])
        };
        colored_buffer.put_pixel(x, y, new_color);
    }
    let mut new_filename = PathBuf::from(filename);
    new_filename.set_extension("png");
    let new_filename_str = new_filename.to_str().unwrap_or("output.png");
    colored_buffer.save(new_filename_str).unwrap();
}

pub fn chunked_colors(text: &str, filename: &str) {
    print_colored(
        &["Size exceeds 2K > chunking."],
        &[ORANGE],
    );
    let chunks = text.as_bytes().chunks(MAX_TEXT_SIZE);
    let mut new_filename = PathBuf::from(filename);
    new_filename.set_extension("png");
    let new_filename_str = new_filename.to_str().unwrap_or("output.png");
    unsupported(new_filename_str);
    for (i, chunk) in chunks.enumerate() {
        let chunk_str = String::from_utf8_lossy(chunk);
        let filename_with_index = format!("{}_part_{}.png", new_filename_str, i);
        colors(&chunk_str, &filename_with_index);
        print_colored(
            &["chunk written."],
            &[ORANGE],
        );
    }
    return;
}

pub fn chaos(text: &str, filename: &str) {
    feedback(&text, &filename);
    let img_u8: ImageBuffer<Luma<u8>, Vec<u8>> = qrcode_generator::to_image_buffer(text, QrCodeEcc::Low, 1024)
        .unwrap();
    let mut colored_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(img_u8.width(), img_u8.height());
    let mut rng = rand::thread_rng();
    for (x, y, pixel) in img_u8.enumerate_pixels() {
        let Luma([value]) = *pixel;
        let new_color: Rgb<u8> = if value == 0 {
            Rgb([rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)])
        } else {
            Rgb([rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)])
        };
        colored_buffer.put_pixel(x, y, new_color);
    }
    let mut new_filename = PathBuf::from(filename);
    new_filename.set_extension("png");
    let new_filename_str = new_filename.to_str().unwrap_or("output.png");
    colored_buffer.save(new_filename_str).unwrap();
}

pub fn chunked_chaos(text: &str, filename: &str) {
    print_colored(
        &["Size exceeds 2K > chunking."],
        &[ORANGE],
    );
    let chunks = text.as_bytes().chunks(MAX_TEXT_SIZE);
    let mut new_filename = PathBuf::from(filename);
    new_filename.set_extension("png");
    let new_filename_str = new_filename.to_str().unwrap_or("output.png");
    unsupported(new_filename_str);
    for (i, chunk) in chunks.enumerate() {
        let chunk_str = String::from_utf8_lossy(chunk);
        let filename_with_index = format!("{}_part_{}.png", new_filename_str, i);
        chaos(&chunk_str, &filename_with_index);
        print_colored(
            &["chunk written."],
            &[ORANGE],
        );
    }
    return;
}

pub fn hypno(text: &str, filename: &str, red: u8, green: u8, blue: u8) {
    feedback(&text, &filename);
    let img_u8: ImageBuffer<Luma<u8>, Vec<u8>> = qrcode_generator::to_image_buffer(text, QrCodeEcc::Low, 1024)
        .unwrap();
    let mut colored_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(img_u8.width(), img_u8.height());
    for (x, y, pixel) in img_u8.enumerate_pixels() {
        let Luma([value]) = *pixel;
        let new_color: Rgb<u8> = if value == 0 {
            Rgb([(red), (green), (blue)])
        } else {
            Rgb([(0), (0), (0)])
        };
        colored_buffer.put_pixel(x, y, new_color);
    }
    let mut new_filename = PathBuf::from(filename);
    new_filename.set_extension("png");
    let new_filename_str = new_filename.to_str().unwrap_or("output.png");
    colored_buffer.save(new_filename_str).unwrap();
}

pub fn chunked_hypno(filename: &str) {
    //create random data
    let mut rng = rand::thread_rng();
    let random_data: String = (0..614400)
        .map(|_| (rng.gen_range(33..127) as u8) as char)
        .collect();
    //create chunks
    let chunks = random_data.as_bytes().chunks(MAX_TEXT_SIZE);
    let mut new_filename = PathBuf::from(filename);
    new_filename.set_extension("png");
    let new_filename_str = new_filename.to_str().unwrap_or("output.png");
    unsupported(new_filename_str);
    let mut rng = rand::thread_rng();
    let mut red: u8 = rng.gen_range(1..=255);
    let mut green: u8 = rng.gen_range(1..=255);
    let mut blue: u8 = rng.gen_range(1..=255);
    for (i, chunk) in chunks.enumerate() {
        clear();
        let chunk_str = String::from_utf8_lossy(chunk);
        let filename_with_index = format!("hypno_part_{}.png", i);
        hypno(&chunk_str, &filename_with_index, red, green, blue);
        red = (red + 1) % 254;
        green = (green + 2) % 253;
        blue = (blue + 3) % 252;
        print_colored(
            &["chunk written."],
            &[ORANGE],
        );
    }
    let mut png_paths: Vec<String> = Vec::new();
    for entry in fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "png" {
                    if let Some(filename) = path.to_str() {
                        png_paths.push(filename.to_string());
                    }
                }
            }
        }
    }
    println!("creating gif file");
    let mut image = File::create("output.gif").unwrap();
    let mut encoder = gif::Encoder::new(&mut image, 1024, 1024, &[]).unwrap();
    encoder.set_repeat(gif::Repeat::Infinite).unwrap();
    let mut filen = 0;
    for path in png_paths {
        clear();
        println!("encoding {}", filen);
        let img = image::open(&Path::new(&path)).unwrap();
        let mut rgba = img.to_rgba8();
        let mut frame = gif::Frame::from_rgba_speed(1024, 1024, &mut *rgba, 30);
        frame.delay = 2;
        encoder.write_frame(&frame).unwrap();
        filen += 1;
    }
    return;
}
*/
