use std::path::{Path, PathBuf};
use crate::solarized::{
    print_colored, print_fancy,
    VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA,
};
use crate::{unsupported};
use qrcode_generator::QrCodeEcc;

const MAX_TEXT_SIZE: usize = 2048;

pub fn png(text: &str, filename: &str) {
    print_fancy(&[
        ("text: ", YELLOW, vec![]),
        (text.trim(), VIOLET, vec![]),
        ("\nfilename: ", YELLOW, vec![]),
        (filename.trim(), VIOLET, vec![]),
    ]);
    print_colored(
        &["c", "r", "e", "a", "t", "i", "n", "g"],
        &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA]
    );
    qrcode_generator::to_png_to_file(text, QrCodeEcc::Low, 1024, filename).unwrap();
}

pub fn svg(text: &str, filename: &str) {
    print_fancy(&[
        ("text: ", YELLOW, vec![]),
        (text.trim(), VIOLET, vec![]),
        ("\nfilename: ", YELLOW, vec![]),
        (filename.trim(), VIOLET, vec![]),
    ]);
    print_colored(
        &["c", "r", "e", "a", "t", "i", "n", "g"],
        &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA]
    );
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
