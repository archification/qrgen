use std::path::Path;
use image::ImageError;
use crate::solarized::{
    /*print_colored, */print_fancy,
    VIOLET, /*BLUE, */CYAN/*, GREEN, YELLOW, ORANGE, RED, MAGENTA,
    BOLD, UNDERLINED, ITALIC*/
};

pub fn decode_qrcode_from_png(png_path: &str) -> Result<Option<String>, ImageError> {
    let img = match image::open(&Path::new(png_path)) {
        Ok(image) => image.to_luma8(),
        Err(err) => return Err(err),
    };
    let mut decoder = rqrr::PreparedImage::prepare(img);
    let grids = decoder.detect_grids();
    for grid in grids {
        let decoded = grid.decode();
        if let Ok((_, content)) = decoded {
            return Ok(Some(content));
        }
    }
    Ok(None)
}

pub fn decode(png_path: &str) {
    match decode_qrcode_from_png(png_path) {
        Ok(Some(content)) => print_fancy(&[
                ("Decoded content: ", CYAN, vec![]),
                (&format!("{}", content), VIOLET, vec![]),
            ]),
        Ok(None) => print_fancy(&[
            ("Image at ", CYAN, vec![]),
            (&format!("{}", png_path), VIOLET, vec![]),
            (" does not contain a valid QR code.", CYAN, vec![]),
        ]),
        Err(_) => print_fancy(&[
            ("The file ", CYAN, vec![]),
            (&format!("{}", png_path), VIOLET, vec![]),
            (" does not contain a valid PNG image.", CYAN, vec![]),
        ])
    }
}
