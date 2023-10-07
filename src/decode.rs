use std::fs;
use std::path::Path;
use image::ImageError;
use solarized::{
    print_fancy,
    VIOLET, CYAN,
    PrintMode::NewLine
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
        Ok(Some(content)) => {
            print_fancy(&[
                ("Decoded content: ", CYAN, vec![]),
                (&format!("{}", content), VIOLET, vec![]),
            ], NewLine);
            let txt_path = Path::new(png_path).with_extension("txt");
            if let Err(err) = fs::write(&txt_path, &content) {
                print_fancy(&[
                    ("Failed to write to ", CYAN, vec![]),
                    (&format!("{}", txt_path.display()), VIOLET, vec![]),
                    (&format!(" due to error: {}", err), CYAN, vec![]),
                ], NewLine);
            } else {
                print_fancy(&[
                    ("Successfully written to ", CYAN, vec![]),
                    (&format!("{}", txt_path.display()), VIOLET, vec![]),
                ], NewLine);
            }
        }
        Ok(None) => print_fancy(&[
            ("Image at ", CYAN, vec![]),
            (&format!("{}", png_path), VIOLET, vec![]),
            (" does not contain a valid QR code.", CYAN, vec![]),
        ], NewLine),
        Err(_) => print_fancy(&[
            ("The file ", CYAN, vec![]),
            (&format!("{}", png_path), VIOLET, vec![]),
            (" does not contain a valid PNG image.", CYAN, vec![]),
        ], NewLine)
    }
}

/*
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
*/
