/*
MIT License

Copyright (c) 2021 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
//! A CLI utility installed as "ansi" to quickly get ASCII escape sequences. Supports the most basic
//! ones, like colors and styles as bold or italic. The lifecycle of this utility usually is really
//! short, rather it is invoked often/multiple times. It can be used like this:
//! `$ echo "$(ansi bg-green)Hello World $(ansi reset)$(ansi red)$(ansi bold)$(ansi underline)Red Warning$(ansi reset)"`

mod input;

use crate::input::print_help;
use ansi_term::Color::Red;
use ansi_term::{Colour, Style};
use std::str::FromStr;

/// Determines how the special `ESC`-character gets ASCII-encoded before printed to stdout.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EscapeStyle {
    /// Bash (and many other tools) support `\e` as escaped version of `ESC` code.
    Bash,
    /// Many tools allow hex values in the following notation: `\x1b`, i.e. an escaped version of `ESC` code.
    Hex,
    /// Many tools allow unicode values in the following notation: `\u001b`, i.e. an escaped version of `ESC` code.
    Unicode,
    /// Rust uses unicode in the following form: `\u{1b}`, i.e. an escaped version of `ESC` code.
    UnicodeRust,
}

impl EscapeStyle {

    /// Help test for each variant.
    fn help_text(&self) -> &str {
        match self {
            EscapeStyle::Bash => "Bash (and many other tools) support `\\e` as escaped version of `ESC` code",
            EscapeStyle::Hex => "Many tools allow hex values in the following notation: `\\x1b`, i.e. an escaped version of `ESC` code.",
            EscapeStyle::Unicode => "Many tools allow unicode values in the following notation: `\\u001b`, i.e. an escaped version of `ESC` code.",
            EscapeStyle::UnicodeRust => "Rust uses unicode in the following form: `\\u{1b}`, i.e. an escaped version of `ESC` code.",
        }
    }

    /// When we get the escape sequence from `ansi_term` it directly contains the ASCII value
    /// of `ESC`. Normally we don't want this but an escaped version of the `ESC`-later using
    /// regular characters.
    fn escape_esc_character(&self, escape_sequence: String) -> String {
        const ASCII_ESCAPE: &str = "\u{1b}";
        match self {
            EscapeStyle::UnicodeRust => escape_sequence.replace(ASCII_ESCAPE, "\\u{1b}"),
            EscapeStyle::Unicode => escape_sequence.replace(ASCII_ESCAPE, "\\u001b"),
            EscapeStyle::Bash => escape_sequence.replace(ASCII_ESCAPE, "\\e"),
            EscapeStyle::Hex => escape_sequence.replace(ASCII_ESCAPE, "\\x1b"),
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
            _ => None,
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
        // reset is the same for all commands
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
        "bg-purple" => Style::new().on(Colour::Purple).prefix().to_string(),
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

/// A CLI utility installed as "ansi" to quickly get ASCII escape sequences. Supports the most basic
/// ones, like colors and styles as bold or italic. The lifecycle of this utility usually is really
/// short, rather it is invoked often/multiple times. It can be used like this:
/// `$ echo "$(ansi bg-green)Hello World $(ansi reset)$(ansi red)$(ansi bold)$(ansi underline)Red Warning$(ansi reset)"`
fn main() {
    let args = std::env::args();
    let args = args.collect::<Vec<String>>();
    let args = args.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    let (params, cmd) =
        input::analyze_args(&args).expect("Input must be valid. Type '--help' for assistance.");

    if cmd == "help" {
        print_help();
    } else {
        let escape_sequence = command_to_escape_code(&cmd);

        if !params.no_ascii_escape() {
            print!("{}", params.escape_style().escape_esc_character(escape_sequence));
        } else {
            print!("{}", escape_sequence);
        }
        if params.new_line() {
            println!();
        }
    }
}
