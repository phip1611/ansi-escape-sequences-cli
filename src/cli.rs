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
    Black,
    /// Black background color.
    BgBlack,
    /// Red foreground color.
    Red,
    /// Red background color.
    BgRed,
    /// Green foreground color.
    Green,
    /// Green background color.
    BgGreen,
    /// Yellow foreground color.
    Yellow,
    /// Yellow background color.
    BgYellow,
    /// Blue foreground color.
    Blue,
    /// Blue background color.
    BgBlue,
    /// Purple foreground color.
    Magenta,
    /// Purple background color.
    BgMagenta,
    /// Cyan foreground color.
    Cyan,
    /// Cyan background color.
    BgCyan,
    /// White foreground color.
    White,
    /// White background color.
    BgWhite,

    /// Gray foreground color.
    Gray,
    /// Gray background color.
    BgGray,
    /// Bright red foreground color.
    BrightRed,
    /// Bright red background color.
    BgBrightRed,
    /// Bright green foreground color.
    BrightGreen,
    /// Bright green background color.
    BgBrightGreen,
    /// Bright yellow foreground color.
    BrightYellow,
    /// Bright yellow background color.
    BgBrightYellow,
    /// Bright blue foreground color.
    BrightBlue,
    /// Bright blue background color.
    BgBrightBlue,
    /// Pink foreground color.
    BrightMagenta,
    /// Pink background color.
    BgBrightMagenta,
    /// Bright cyan foreground color.
    BrightCyan,
    /// Bright cyan background color.
    BgBrightCyan,
    /// Bright white foreground color.
    BrightWhite,
    /// Bright white background color.
    BgBrightWhite,

    /// Reset/Clear all style.
    Clear,
    /// Reset/Clear all style.
    Reset,

    // /// Normal font.
    // Normal,
    /// Bold font.
    Bold,
    /// Dimmed font.
    Dimmed,
    /// Italic font.
    Italic,
    /// Underlined font.
    Underline,
    /// Blinking font.
    Blink,
    /// Hidden font.
    Hidden,
    /// Strikethrough font.
    Strike,
    /// Strikethrough font.
    Strikethrough,
}

impl AnsiKeyword {
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
