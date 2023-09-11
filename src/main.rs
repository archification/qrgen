mod solarized;
mod common;

use std::path::{Path, PathBuf};
use std::io::{self, Read};
use std::env;
use solarized::{
    print_fancy,
    BLUE, CYAN, GREEN, RED,
    BOLD,
};
use common::{
    png, svg, read_file_to_string, is_file_path, usage
};

const MAX_TEXT_SIZE: usize = 2048;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (text, filename) = if args.len() == 1 {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        (buffer.trim().to_string(), "file_output.png".to_string())
    } else if args.len() == 2 {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        (buffer.trim().to_string(), args[1].clone())
    } else if args.len() == 3 {
        if is_file_path(&args[1]) {
            let file_content = read_file_to_string(&args[1]);
            match file_content {
                Ok(content) => (content.trim().to_string(), args[2].clone()),
                Err(_) => {
                    usage(args);
                    return;
                }
            }
        } else {
            (args[1].clone(), args[2].clone())
        }
    } else {
        usage(args);
        return;
    };
    if text.len() > MAX_TEXT_SIZE {
        let chunks = text.as_bytes().chunks(MAX_TEXT_SIZE);
        for (i, chunk) in chunks.enumerate() {
            let chunk_str = String::from_utf8_lossy(chunk);
            let filename_with_index = format!("{}_part_{}.png", filename, i);
            png(&chunk_str, &filename_with_index);
        }
        return;
    }
    let extension = Path::new(&filename).extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    match extension {
        "png" => png(&text, &filename),
        "svg" => svg(&text, &filename),
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
            png(&text, new_filename_str)
        },
    }
}
