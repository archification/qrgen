use crossterm::style::{Attribute, SetAttribute, SetBackgroundColor, SetForegroundColor, ResetColor, Color};
use std::io::stdout;
use crossterm::{
    ExecutableCommand,
    cursor,
    terminal::{
        Clear,
        ClearType
    }
};

pub const BACK: Color = Color::Rgb { r:7, g:54, b:66 };
pub const VIOLET: Color = Color::Rgb { r:108, g:113, b:196 };
pub const BLUE: Color = Color::Rgb { r:38, g:139, b:210 };
pub const CYAN: Color = Color::Rgb { r:42, g:161, b:152 };
pub const GREEN: Color = Color::Rgb { r:133, g:153, b:0 };
pub const YELLOW: Color = Color::Rgb { r:181, g:137, b:0 };
pub const ORANGE: Color = Color::Rgb { r:203, g:75, b:22 };
pub const RED: Color = Color::Rgb { r:211, g:1, b:2 };
pub const MAGENTA: Color = Color::Rgb { r:211, g:54, b:130 };
pub const BOLD: Attribute = Attribute::Bold;
pub const UNDERLINED: Attribute = Attribute::Underlined;
pub const ITALIC: Attribute = Attribute::Italic;

pub fn print_colored(message: &[&str], colors: &[Color]) {
    let mut formatted_message = String::new();
    for (i, m) in message.iter().enumerate() {
        let color = &colors[i % colors.len()];
        formatted_message.push_str(&format!("{}{}", SetForegroundColor(*color), m));
    }
    println!("{}{}{}", SetBackgroundColor(BACK), formatted_message, ResetColor);
}

pub fn print_fancy(message_fragments: &[(&str, Color, Vec<Attribute>)]) {
    let mut formatted_message = String::new();
    for (message, color, attributes) in message_fragments {
        formatted_message.push_str(&format!("{}", SetBackgroundColor(BACK)));
        formatted_message.push_str(&format!("{}", SetForegroundColor(*color)));
        for attribute in attributes {
            formatted_message.push_str(&format!("{}", SetAttribute(*attribute)));
        }
        formatted_message.push_str(message);
        formatted_message.push_str(&format!("{}", SetAttribute(Attribute::Reset)));
    }
    println!("{}{}{}", SetBackgroundColor(BACK), formatted_message, ResetColor);
}

pub fn clear() {
    stdout()
        .execute(Clear(ClearType::All)).unwrap()
        .execute(cursor::MoveTo(0, 0)).unwrap();
}
