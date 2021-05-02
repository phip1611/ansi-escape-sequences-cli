mod input;

use ansi_term::Color::Red;
use std::env::args;
use ansi_term::{Style, Color, Colour};
use std::str::FromStr;
use crate::input::print_help;

/// Determines how the special `ESC`-character gets ASCII-encoded in ASCII escape sequences.
///
/// Example (bash script):
/// `echo "normal$(ansi red)red$(ansi clear)$(ansi underline)$(ansi cyan)cyan underlined$(ansi clear)"`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EscapeStyle {
    /// Bash (and many other tools) support `\e` that gets replaced by ASCIIs `ESC` code.
    Bash,
    /// Many tools allow hex values in the following notation: `\x1b`, i.e. ASCIIs `ESC` code.
    Hex,
    /// Many tools allow unicode values in the following notation: `\u001b`, i.e. ASCIIs `ESC` code.
    Unicode,
    /// Rust uses unicode in the following form: `\u{1b}`, i.e. ASCIIs `ESC` code.
    UnicodeRust,
}

impl EscapeStyle {
    fn asciify(&self, escape_sequence: String) -> String {
        const ASCII_ESCAPE: &str = "\u{1b}";
        match self {
            EscapeStyle::UnicodeRust => escape_sequence.replace(ASCII_ESCAPE, "\\u{1b}"),
            EscapeStyle::Unicode => escape_sequence.replace(ASCII_ESCAPE, "\\u001b"),
            EscapeStyle::Bash => escape_sequence.replace(ASCII_ESCAPE, "\\e"),
            EscapeStyle::Hex => escape_sequence.replace(ASCII_ESCAPE, "\\x1b"),
        }
    }

    fn help_text(&self) -> &str {
        match self {
            EscapeStyle::Bash => "Bash (and many other tools) support `\\e` that gets replaced by ASCIIs `ESC` code.",
            EscapeStyle::Hex => "Many tools allow hex values in the following notation: `\x1b`, i.e. ASCIIs `ESC` code.",
            EscapeStyle::Unicode => "Many tools allow unicode values in the following notation: `\\u001b`, i.e. ASCIIs `ESC` code.",
            EscapeStyle::UnicodeRust => "Rust uses unicode in the following form: `\u{1b}`, i.e. ASCIIs `ESC` code.",
        }
    }
}

impl FromStr for EscapeStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_ascii_lowercase();
        let es = match s.as_str() {
            "bash" => Some(EscapeStyle::Bash),
            "unicode" => Some(EscapeStyle::Unicode),
            "unicode-rust" => Some(EscapeStyle::UnicodeRust),
            "hex" => Some(EscapeStyle::Hex),
            _ => None
        };
        es.ok_or(format!("escape style format '{}' is unknown!", s))
    }
}

impl Default for EscapeStyle {
    fn default() -> Self {
        EscapeStyle::Bash
    }
}

pub fn command_to_escape_code(cmd: &str) -> String {
    match cmd {
        // clear is the same for all commands
        "clear" => Red.suffix().to_string(),
        "reset" => Red.suffix().to_string(),

        "black" => Style::new().fg(Colour::Black).prefix().to_string(),
        "bg-black" => Style::new().on(Colour::Black).prefix().to_string(),
        "red" => Style::new().fg(Colour::Red).prefix().to_string(),
        "bg-red" => Style::new().on(Colour::Red).prefix().to_string(),
        "green" => Style::new().fg(Colour::Green).prefix().to_string(),
        "bg-green" => Style::new().on(Colour::Green).prefix().to_string(),
        "yellow" => Style::new().fg(Colour::Yellow).prefix().to_string(),
        "bg-yellow" => Style::new().on(Colour::Yellow).prefix().to_string(),
        "blue" => Style::new().fg(Colour::Blue).prefix().to_string(),
        "bg-blue" => Style::new().on(Colour::Blue).prefix().to_string(),
        "purple" => Style::new().fg(Colour::Purple).prefix().to_string(),
        "bg-Purple" => Style::new().on(Colour::Purple).prefix().to_string(),
        "cyan" => Style::new().fg(Colour::Cyan).prefix().to_string(),
        "bg-cyan" => Style::new().on(Colour::Cyan).prefix().to_string(),
        "white" => Style::new().fg(Colour::White).prefix().to_string(),
        "bg-white" => Style::new().on(Colour::White).prefix().to_string(),

        // clear is the same for all commands
        "normal" => Red.suffix().to_string(),
        "bold" => Style::new().bold().prefix().to_string(),
        "dimmed" => Style::new().dimmed().prefix().to_string(),
        "italic" => Style::new().italic().prefix().to_string(),
        "underline" => Style::new().underline().prefix().to_string(),
        "blink" => Style::new().blink().prefix().to_string(),
        "hidden" => Style::new().hidden().prefix().to_string(),
        "strike" | "strikethrough" => Style::new().strikethrough().prefix().to_string(),

        _ => panic!(),
    }
}

fn main() {
    let args = std::env::args();
    let args = args.collect::<Vec<String>>();
    let args = args.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    let (params, cmd) = input::analyze_args(
        &args
    ).expect("Input must be valid. Type '--help' for assistance.");

    if cmd == "help" {
        print_help();
    } else {
        let escape_sequence = command_to_escape_code(&cmd);

        if !params.no_ascii_escape() {
            print!("{}", params.escape_style().asciify(escape_sequence));
        } else {
            print!("{}", escape_sequence);
        }
        if params.new_line() {
            println!();
        }
    }

}
