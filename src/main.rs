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
//! A CLI utility installed as "ansi" to quickly get ANSI escape sequences. Supports the most basic
//! ones, like colors and styles as bold or italic. The lifecycle of this utility usually is really
//! short, rather it is invoked often/multiple times. It can be used like this:
//! `$ echo "$(ansi bg-green)Hello World $(ansi reset)$(ansi red)$(ansi bold)$(ansi underline)Red Warning$(ansi reset)"`

use crate::cli::{AnsiKeyword, EscEscapeStyle};
use clap::Parser;

mod cli;

fn to_ansi_escape_sequence(
    esc_encode_style: EscEscapeStyle,
    ansi_keywords: &[AnsiKeyword],
) -> String {
    const ANSI_ESP_SEP: char = ';';
    const ANSI_ESC_SUFFIX: &str = "m";

    let prefix = {
        let esc = esc_encode_style.escape_sequence();
        format!("{esc}[")
    };

    let codes: String =
        ansi_keywords
            .iter()
            .map(|ak| ak.to_ansi_code())
            .fold(String::new(), |mut acc, next| {
                if acc.is_empty() {
                    acc.push_str(next);
                } else {
                    acc.push(ANSI_ESP_SEP);
                    acc.push_str(next);
                }
                acc
            });
    format!("{prefix}{codes}{suffix}", suffix = ANSI_ESC_SUFFIX)
}

/// A CLI utility installed as "ansi" to quickly get ANSI escape sequences. Supports the most basic
/// ones, like colors and styles as bold or italic. The lifecycle of this utility usually is really
/// short, rather it is invoked often/multiple times. It can be used like this:
/// `$ echo "$(ansi bg-green)Hello World $(ansi reset)$(ansi red)$(ansi bold)$(ansi underline)Red Warning$(ansi reset)"`
fn main() {
    let args = cli::Cli::parse();
    print!(
        "{}",
        to_ansi_escape_sequence(args.escape_style, &args.keywords)
    );
    if args.new_line {
        println!();
    }
}
