mod solarized;

use solarized::{
    print_colored, print_fancy,
    VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA,
    BOLD, UNDERLINED, ITALIC
};
use qrcode_generator::QrCodeEcc;
use std::path::{Path, PathBuf};
use std::env;

fn png(url: &str, filename: &str) {
    print_colored(
        &["c", "r", "e", "a", "t", "i", "n", "g"],
        &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA]
    );
    qrcode_generator::to_png_to_file(url, QrCodeEcc::Low, 1024, filename).unwrap();
}

fn svg(url: &str, filename: &str) {
    print_colored(
        &["c", "r", "e", "a", "t", "i", "n", "g"],
        &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA]
    );
    qrcode_generator::to_svg_to_file(url, QrCodeEcc::Low, 1024, None::<&str>, filename).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 && args.len() != 3 {
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
        return;
    }
    let url = &args[1];
    let filename = match args.as_slice() {
        [_] | [_ , _] => "file_output.png",
        [_, _, filename, ..] => filename,
        _ => unreachable!(),
    };
    let extension = Path::new(filename).extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    match extension {
        "png" => png(url, filename),
        "svg" => svg(url, filename),
        _ => {
            let mut new_filename = PathBuf::from(filename);
            new_filename.set_extension("png");
            let new_filename_str = new_filename.to_str().unwrap_or("file_output.png");
            print_fancy(&[
                ("Unsupported", RED, vec![BOLD]),
                (" file extension.\n", CYAN, vec![]),
                ("Defaulting to ", CYAN, vec![]),
                ("PNG", GREEN, vec![BOLD]),
                (". Saving as ", CYAN, vec![]),
                (new_filename_str, BLUE, vec![BOLD]),
            ]);
            png(url, new_filename_str)
        },
    }
}
