use std::path::{Path, PathBuf};
use crate::solarized::{
    print_colored,
    ORANGE,
};
use crate::common::{unsupported, feedback};
use qrcode_generator::QrCodeEcc;
use image::{ImageBuffer, Luma, Rgb};
use rand::Rng;

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
    let chunks = text.as_bytes().chunks(MAX_TEXT_SIZE);
    let extension = Path::new(&filename).extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    match extension {
        "png" => {
            for (i, chunk) in chunks.enumerate() {
                let chunk_str = String::from_utf8_lossy(chunk);
                let filename_with_index = format!("{}_part_{}.png", filename, i);
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
                let filename_with_index = format!("{}_part_{}.svg", filename, i);
                svg(&chunk_str, &filename_with_index);
                print_colored(
                    &["chunk written."],
                    &[ORANGE],
                );
            }
            return;
        }
        _ => {
            let mut new_filename = PathBuf::from(filename);
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
