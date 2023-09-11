use std::path::{Path};
use std::io::{self, Read};
use std::io::IsTerminal;
use crate::solarized::{
    print_colored, print_fancy,
    VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA,
    BOLD, UNDERLINED, ITALIC
};

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
        ("Image file is generated in current directory.\n", CYAN, vec![]),
        ("Have fun and enjoy the squares!.", YELLOW, vec![]),
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
