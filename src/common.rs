use std::path::Path;
use std::io::{self, Read};
use crate::solarized::{
    print_colored, print_fancy,
    VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA,
    BOLD, UNDERLINED, ITALIC
};
use qrcode_generator::QrCodeEcc;

pub fn png(url: &str, filename: &str) {
    print_colored(
        &["c", "r", "e", "a", "t", "i", "n", "g"],
        &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA]
    );
    qrcode_generator::to_png_to_file(url, QrCodeEcc::Low, 1024, filename).unwrap();
}

pub fn svg(url: &str, filename: &str) {
    print_colored(
        &["c", "r", "e", "a", "t", "i", "n", "g"],
        &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA]
    );
    qrcode_generator::to_svg_to_file(url, QrCodeEcc::Low, 1024, None::<&str>, filename).unwrap();
}

pub fn read_file_to_string(filename: &str) -> io::Result<String> {
    let mut file = std::fs::File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
    //2K cap
}

pub fn is_file_path(path: &str) -> bool {
    Path::new(path).is_file()
}

pub fn usage(args: Vec<String>) {
    print_fancy(&[
        ("Usage: ", CYAN, vec![BOLD]),
        (&format!("{}", args[0]), VIOLET, vec![]),
        (" <text>", MAGENTA, vec![]),
        (" <fileName>\n", GREEN, vec![]),
        ("Example: ", CYAN, vec![BOLD]),
        (&format!("{} ", args[0]), BLUE, vec![]),
        ("\"", MAGENTA, vec![]),
        ("https://google.com/", MAGENTA, vec![UNDERLINED, ITALIC]),
        ("\"", MAGENTA, vec![]),
        (" GoogleQrImage.png\n", GREEN, vec![]),
        ("<text>", MAGENTA, vec![]),
        (" is required. Put your url here in quotes.\n", CYAN, vec![]),
        ("<fileName>", GREEN, vec![]),
        (" is optional. Can be png or svg. Defaults to \"file_output.png\".\n", CYAN, vec![]),
        ("Image file is generated in current directory.", CYAN, vec![]),
    ]);
}

pub fn unsupported(new_filename_str: &str) {
    print_fancy(&[
        ("Unsupported", RED, vec![BOLD]),
        (" file extension.\n", CYAN, vec![]),
        ("Defaulting to ", CYAN, vec![]),
        ("PNG", GREEN, vec![BOLD]),
        (". Saving as ", CYAN, vec![]),
        (new_filename_str, BLUE, vec![BOLD]),
    ]);
}
