use std::path::{Path, PathBuf};
use std::io::{self, Read};
use std::io::IsTerminal;
use crate::solarized::{
    print_colored, print_fancy,
    VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA,
    BOLD, UNDERLINED, ITALIC
};
use qrcode_generator::QrCodeEcc;

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

const MAX_TEXT_SIZE: usize = 2048;

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

pub fn load_stdin() -> io::Result<String> {
    if std::io::stdin().is_terminal() {
        print_fancy(&[
            ("There is no ", RED, vec![]),
            ("stdin", RED, vec![BOLD, UNDERLINED]),
            (" or ", RED, vec![]),
            ("text", RED, vec![BOLD, UNDERLINED]),
            (".\n", RED, vec![]),
            ("Generating ", ORANGE, vec![]),
            ("empty", ORANGE, vec![ITALIC]),
            (" code.", ORANGE, vec![]),
        ]);
        print_colored(
            &["Run with \"help\", \"--help\", or \"-h\" to see usage."],
            &[CYAN]
        );
        return Err(io::Error::new(io::ErrorKind::Other, "stdin not redirected"));
    }
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    return Ok(buffer);
}

pub fn usage(args: Vec<String>) {
    print_fancy(&[
        ("Usage: ", CYAN, vec![BOLD]),
        (&format!("{}", args[0]), BLUE, vec![]),
        (" <text/file>", MAGENTA, vec![]),
        (" <fileName>\n", GREEN, vec![]),

        //example with both arguments
        ("Example: ", CYAN, vec![BOLD]),
        (&format!("{} ", args[0]), BLUE, vec![]),
        ("\"", MAGENTA, vec![]),
        ("https://google.com/", MAGENTA, vec![UNDERLINED]),
        ("\"", MAGENTA, vec![]),
        (" GoogleQrImage.png\n", GREEN, vec![]),

        //example with stdin
        ("Example: ", CYAN, vec![BOLD]),
        ("cat file | ", MAGENTA, vec![]),
        (&format!("{} ", args[0]), BLUE, vec![]),
        (" GoogleQrImage.png\n", GREEN, vec![]),

        ("<text>", MAGENTA, vec![]),
        (" can be supplied by an argument or stdin.\n", CYAN, vec![]),
        ("This argument can be text or a file in the current directory.\n", CYAN, vec![]),
        ("<fileName>", GREEN, vec![]),
        (" is optional. Can be png or svg. Defaults to ", CYAN, vec![]),
        ("\"output.png\"", VIOLET, vec![]),
        (".\n", CYAN, vec![]),
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
