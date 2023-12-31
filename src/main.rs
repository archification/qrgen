mod common;
mod generation;
mod decode;

use std::path::{Path, PathBuf};
use std::process::exit;
use std::env;
use common::{
    read_file_to_string, is_file_path,
    load_stdin, usage, unsupported
};
use generation::{
    png, svg, chunked,
    chaos, chunked_chaos,
    colors, chunked_colors,
    chunked_hypno
};
use decode::decode;
use solarized::clear;

const MAX_TEXT_SIZE: usize = 2048;

fn main() {
    clear();
    let args: Vec<String> = env::args().collect();
    let (text, filename) = if args.len() == 1 {
        (load_stdin().unwrap_or_else(|_| {
            String::new()
        }).to_string(), "output.png".to_string())
    } else if args.len() == 2 {
        match &args[1][..] {
            "help" | "--help" | "-h" => {
                usage(args.clone());
                exit(0);
            },
            "hypno" | "hypnosis" => {
                (String::new(), args[1].clone())
            },
            _ => {
                (load_stdin().unwrap_or_else(|_| {
                    String::new()
                }).to_string(), args[1].clone())
            }
        }
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
        exit(0);
    };
    let mut final_filename = filename.clone();
    if final_filename == "output.png" {
        let mut counter = 1;
        while Path::new(&final_filename).exists() {
            final_filename = format!("output_{}.png", counter);
            counter += 1;
        }
    }
    if text == "decode" {
        decode(&filename);
    } else if filename.trim() == "chaos" || filename.trim() == "chaos.png" {
        if text.len() > MAX_TEXT_SIZE {
            chunked_chaos(&text, &filename);
        } else {
            chaos(&text, &filename);
        };
    } else if filename.trim() == "colors" || filename.trim() == "colors.png" {
        if text.len() > MAX_TEXT_SIZE {
            chunked_colors(&text, &filename);
        } else {
            colors(&text, &filename);
        };
    } else if filename.trim() == "hypno" || filename.trim() == "hypnosis" {
        chunked_hypno(&filename);
    } else if text.len() > MAX_TEXT_SIZE {
        chunked(&text, &final_filename);
    } else {
        let extension = Path::new(&final_filename).extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        match extension {
            "png" => png(&text, &final_filename),
            "svg" => svg(&text, &final_filename),
            _ => {
                let mut new_filename = PathBuf::from(final_filename);
                new_filename.set_extension("png");
                let new_filename_str = new_filename.to_str().unwrap_or("output.png");
                unsupported(new_filename_str);
                png(&text, new_filename_str)
            },
        }
    }
}
