mod solarized;

use std::path::{Path, PathBuf};
use std::io::{self, Read};
use std::env;
use solarized::{
    print_colored, print_fancy,
    VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA,
    BOLD, UNDERLINED, ITALIC
};
use qrcode_generator::QrCodeEcc;

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

fn usage(args: Vec<String>) {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let (url, filename) = if args.len() == 1 {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        let parts: Vec<&str> = buffer.trim().split_whitespace().collect();
        if parts.len() == 1 {
            (parts[0].to_string(), "file_output.png".to_string())
        } else if parts.len() >= 2 {
            (parts[0].to_string(), parts[1].to_string())
        } else {
            usage(args);
            return;
        }
    } else if args.len() == 2 {
        (args[1].clone(), "file_output.png".to_string())
    } else if args.len() == 3 {
        (args[1].clone(), args[2].clone())
    } else {
        usage(args);
        return;
    };
    let extension = Path::new(&filename).extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    match extension {
        "png" => png(&url, &filename),
        "svg" => svg(&url, &filename),
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
            png(&url, new_filename_str)
        },
    }
}
