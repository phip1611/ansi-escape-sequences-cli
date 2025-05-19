/*
MIT License

Copyright (c) 2025 Philipp Schuster

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
//! CLI parsing.

use crate::to_ansi_escape_sequence;
use clap::{Parser, ValueEnum};

/// Determines how the special `ESC` symbol (character) is escaped in normal
/// ASCII characters before being printed to stdout.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
pub enum EscEscapeStyle {
    /// Use `\e` (Bash style) for encoding.
    #[default]
    Bash,
    /// Use `\x1b` (hex style) for encoding.
    Hex,
    /// Use `\u001b` (unicode style) for encoding.
    Unicode,
    /// Use `\u{1b}` (Rust unicode style) for encoding.
    UnicodeRust,
    /// Print the `ESC` symbol as is and not encoded.
    Direct,
}

impl EscEscapeStyle {
    /// Returns the (encoded) ESC escape sequence.
    pub const fn escape_sequence(&self) -> &'static str {
        match self {
            Self::Bash => "\\e",
            Self::Hex => "\\x1b",
            Self::Unicode => "\\u001b",
            Self::UnicodeRust => "\\u{1b}",
            Self::Direct => "\u{1b}",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum AnsiKeyword {
    /// Black foreground color.
    #[value(help = crate::cli::AnsiKeyword::Black.to_help_text())]
    Black,
    /// Black background color.
    #[value(help = crate::cli::AnsiKeyword::BgBlack.to_help_text())]
    BgBlack,
    /// Red foreground color.
    #[value(help = crate::cli::AnsiKeyword::Red.to_help_text())]
    Red,
    /// Red background color.
    #[value(help = crate::cli::AnsiKeyword::BgRed.to_help_text())]
    BgRed,
    /// Green foreground color.
    #[value(help = crate::cli::AnsiKeyword::Green.to_help_text())]
    Green,
    /// Green background color.
    #[value(help = crate::cli::AnsiKeyword::BgGreen.to_help_text())]
    BgGreen,
    /// Yellow foreground color.
    #[value(help = crate::cli::AnsiKeyword::Yellow.to_help_text())]
    Yellow,
    /// Yellow background color.
    #[value(help = crate::cli::AnsiKeyword::BgYellow.to_help_text())]
    BgYellow,
    /// Blue foreground color.
    #[value(help = crate::cli::AnsiKeyword::Blue.to_help_text())]
    Blue,
    /// Blue background color.
    #[value(help = crate::cli::AnsiKeyword::BgBlue.to_help_text())]
    BgBlue,
    /// Purple foreground color.
    #[value(help = crate::cli::AnsiKeyword::Magenta.to_help_text())]
    Magenta,
    /// Purple background color.
    #[value(help = crate::cli::AnsiKeyword::BgMagenta.to_help_text())]
    BgMagenta,
    /// Cyan foreground color.
    #[value(help = crate::cli::AnsiKeyword::Cyan.to_help_text())]
    Cyan,
    /// Cyan background color.
    #[value(help = crate::cli::AnsiKeyword::BgCyan.to_help_text())]
    BgCyan,
    /// White foreground color.
    #[value(help = crate::cli::AnsiKeyword::White.to_help_text())]
    White,
    /// White background color.
    #[value(help = crate::cli::AnsiKeyword::BgWhite.to_help_text())]
    BgWhite,

    /// Gray foreground color.
    #[value(help = crate::cli::AnsiKeyword::Gray.to_help_text())]
    Gray,
    /// Gray background color.
    #[value(help = crate::cli::AnsiKeyword::BgGray.to_help_text())]
    BgGray,
    /// Bright red foreground color.
    #[value(help = crate::cli::AnsiKeyword::BrightRed.to_help_text())]
    BrightRed,
    /// Bright red background color.
    #[value(help = crate::cli::AnsiKeyword::BgBrightRed.to_help_text())]
    BgBrightRed,
    /// Bright green foreground color.
    #[value(help = crate::cli::AnsiKeyword::BrightGreen.to_help_text())]
    BrightGreen,
    /// Bright green background color.
    #[value(help = crate::cli::AnsiKeyword::BgBrightGreen.to_help_text())]
    BgBrightGreen,
    /// Bright yellow foreground color.
    #[value(help = crate::cli::AnsiKeyword::BrightYellow.to_help_text())]
    BrightYellow,
    /// Bright yellow background color.
    #[value(help = crate::cli::AnsiKeyword::BgBrightYellow.to_help_text())]
    BgBrightYellow,
    /// Bright blue foreground color.
    #[value(help = crate::cli::AnsiKeyword::BrightBlue.to_help_text())]
    BrightBlue,
    /// Bright blue background color.
    #[value(help = crate::cli::AnsiKeyword::BgBrightBlue.to_help_text())]
    BgBrightBlue,
    /// Pink foreground color.
    #[value(help = crate::cli::AnsiKeyword::BrightMagenta.to_help_text())]
    BrightMagenta,
    /// Pink background color.
    #[value(help = crate::cli::AnsiKeyword::BgBrightMagenta.to_help_text())]
    BgBrightMagenta,
    /// Bright cyan foreground color.
    #[value(help = crate::cli::AnsiKeyword::BrightCyan.to_help_text())]
    BrightCyan,
    /// Bright cyan background color.
    #[value(help = crate::cli::AnsiKeyword::BgBrightCyan.to_help_text())]
    BgBrightCyan,
    /// Bright white foreground color.
    #[value(help = crate::cli::AnsiKeyword::BrightWhite.to_help_text())]
    BrightWhite,
    /// Bright white background color.
    #[value(help = crate::cli::AnsiKeyword::BgBrightWhite.to_help_text())]
    BgBrightWhite,

    /// Reset/Clear all style.
    Clear,
    /// Reset/Clear all style.
    Reset,

    // /// Normal font.
    // Normal,
    /// Bold font.
    #[value(help = crate::cli::AnsiKeyword::Bold.to_help_text())]
    Bold,
    /// Dimmed font.
    #[value(help = crate::cli::AnsiKeyword::Dimmed.to_help_text())]
    Dimmed,
    /// Italic font.
    #[value(help = crate::cli::AnsiKeyword::Italic.to_help_text())]
    Italic,
    /// Underlined font.
    #[value(help = crate::cli::AnsiKeyword::Underline.to_help_text())]
    Underline,
    /// Blinking font.
    #[value(help = crate::cli::AnsiKeyword::Blink.to_help_text())]
    Blink,
    /// Hidden font.
    #[value(help = crate::cli::AnsiKeyword::Hidden.to_help_text())]
    Hidden,
    /// Strikethrough font.
    #[value(help = crate::cli::AnsiKeyword::Strike.to_help_text())]
    Strike,
    /// Strikethrough font.
    #[value(help = crate::cli::AnsiKeyword::Strikethrough.to_help_text())]
    Strikethrough,
}

impl AnsiKeyword {
    fn to_help_text(self) -> String {
        match self {
            AnsiKeyword::Black
            | AnsiKeyword::Red
            | AnsiKeyword::Green
            | AnsiKeyword::Yellow
            | AnsiKeyword::Blue
            | AnsiKeyword::Magenta
            | AnsiKeyword::Cyan
            | AnsiKeyword::White
            | AnsiKeyword::Gray
            | AnsiKeyword::BrightRed
            | AnsiKeyword::BrightGreen
            | AnsiKeyword::BrightYellow
            | AnsiKeyword::BrightBlue
            | AnsiKeyword::BrightMagenta
            | AnsiKeyword::BrightCyan
            | AnsiKeyword::BrightWhite => {
                let begin_sequence = to_ansi_escape_sequence(EscEscapeStyle::Direct, &[self]);
                let end_sequence = to_ansi_escape_sequence(EscEscapeStyle::Direct, &[Self::Clear]);
                format!("{begin_sequence}{self:?}{end_sequence} foreground color")
            }
            AnsiKeyword::BgBlack
            | AnsiKeyword::BgRed
            | AnsiKeyword::BgGreen
            | AnsiKeyword::BgYellow
            | AnsiKeyword::BgBlue
            | AnsiKeyword::BgMagenta
            | AnsiKeyword::BgCyan
            | AnsiKeyword::BgWhite
            | AnsiKeyword::BgGray
            | AnsiKeyword::BgBrightRed
            | AnsiKeyword::BgBrightGreen
            | AnsiKeyword::BgBrightYellow
            | AnsiKeyword::BgBrightBlue
            | AnsiKeyword::BgBrightMagenta
            | AnsiKeyword::BgBrightCyan
            | AnsiKeyword::BgBrightWhite => {
                let begin_sequence = to_ansi_escape_sequence(EscEscapeStyle::Direct, &[self]);
                let end_sequence = to_ansi_escape_sequence(EscEscapeStyle::Direct, &[Self::Clear]);
                format!("{begin_sequence}{self:?}{end_sequence} background color")
            }
            AnsiKeyword::Bold
            | AnsiKeyword::Dimmed
            | AnsiKeyword::Italic
            | AnsiKeyword::Underline
            | AnsiKeyword::Blink
            | AnsiKeyword::Hidden
            | AnsiKeyword::Strike
            | AnsiKeyword::Strikethrough => {
                let begin_sequence = to_ansi_escape_sequence(EscEscapeStyle::Direct, &[self]);
                let end_sequence = to_ansi_escape_sequence(EscEscapeStyle::Direct, &[Self::Clear]);
                format!("{begin_sequence}{self:?}{end_sequence} font")
            }
            _ => unimplemented!("No help text for {self:?}"),
        }
    }

    /// Returns the ANSI code of the style.
    pub const fn to_ansi_code(self) -> &'static str {
        match self {
            AnsiKeyword::Black => "30",
            AnsiKeyword::BgBlack => "40",
            AnsiKeyword::Red => "31",
            AnsiKeyword::BgRed => "41",
            AnsiKeyword::Green => "32",
            AnsiKeyword::BgGreen => "42",
            AnsiKeyword::Yellow => "33",
            AnsiKeyword::BgYellow => "43",
            AnsiKeyword::Blue => "34",
            AnsiKeyword::BgBlue => "44",
            AnsiKeyword::Magenta => "35",
            AnsiKeyword::BgMagenta => "45",
            AnsiKeyword::Cyan => "36",
            AnsiKeyword::BgCyan => "46",
            AnsiKeyword::White => "37",
            AnsiKeyword::BgWhite => "47",
            AnsiKeyword::Gray => "90",
            AnsiKeyword::BgGray => "100",
            AnsiKeyword::BrightRed => "91",
            AnsiKeyword::BgBrightRed => "101",
            AnsiKeyword::BrightGreen => "92",
            AnsiKeyword::BgBrightGreen => "102",
            AnsiKeyword::BrightYellow => "93",
            AnsiKeyword::BgBrightYellow => "103",
            AnsiKeyword::BrightBlue => "94",
            AnsiKeyword::BgBrightBlue => "104",
            AnsiKeyword::BrightMagenta => "95",
            AnsiKeyword::BgBrightMagenta => "105",
            AnsiKeyword::BrightCyan => "96",
            AnsiKeyword::BgBrightCyan => "106",
            AnsiKeyword::BrightWhite => "97",
            AnsiKeyword::BgBrightWhite => "107",
            AnsiKeyword::Clear | AnsiKeyword::Reset => "0",
            AnsiKeyword::Bold => "1",
            AnsiKeyword::Dimmed => "2",
            AnsiKeyword::Italic => "3",
            AnsiKeyword::Underline => "4",
            AnsiKeyword::Blink => "5",
            AnsiKeyword::Hidden => "8",
            AnsiKeyword::Strike | AnsiKeyword::Strikethrough => "9",
        }
    }
}

/// ANSI escape sequences CLI utility.
///
/// Please note: Your terminal app may use a theme (color palette) that doesn't
/// match all color names with the actual color.
#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about)]
pub struct Cli {
    /// Add a trailing new-line character (`\n`) to the command output.
    #[arg(short = 'n', long)]
    pub new_line: bool,
    /// Determines the style to encode the `ESC` symbol.
    #[arg(long, value_enum, default_value_t)]
    pub escape_style: EscEscapeStyle,
    /// ANSI escape sequences. Please note that not all combination makes
    /// sense.
    #[arg(required = true)]
    pub keywords: Vec<AnsiKeyword>,
}
