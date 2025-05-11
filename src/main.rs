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

use clap::Parser;

mod cli;

const ESCAPE: &str = "\x1b";
const ANSI_ESC_PREFIX: &str = "\x1b[";
const ANSI_ESC_SUFFIX: &str = "m";
const ANSI_ESP_SEP: char = ';';

/// A CLI utility installed as "ansi" to quickly get ANSI escape sequences. Supports the most basic
/// ones, like colors and styles as bold or italic. The lifecycle of this utility usually is really
/// short, rather it is invoked often/multiple times. It can be used like this:
/// `$ echo "$(ansi bg-green)Hello World $(ansi reset)$(ansi red)$(ansi bold)$(ansi underline)Red Warning$(ansi reset)"`
fn main() {
    let args = cli::Cli::parse();
    let prefix = args
        .escape_style
        .map(|escape_style| {
            let replace = escape_style.escape_sequence();
            ANSI_ESC_PREFIX.replace(ESCAPE, replace)
        })
        .unwrap_or_else(|| ANSI_ESC_PREFIX.to_string());
    let codes: String = args
        .keywords
        .into_iter()
        .map(|ak| ak.to_ansi_sequence())
        .fold(String::new(), |mut acc, next| {
            if acc.is_empty() {
                acc.push_str(next);
            } else {
                acc.push(ANSI_ESP_SEP);
                acc.push_str(next);
            }
            acc
        });
    print!("{prefix}{codes}{ANSI_ESC_SUFFIX}");
    if args.new_line {
        println!();
    }
}
