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
//! Module for parsing CLI input/arguments.

use crate::EscapeStyle;
use ansi_term::{Colour, Style};
use std::str::FromStr;

// env!: inspects an environment variable at compile time
const CRATE_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");
const CRATE_URL: &'static str = env!("CARGO_PKG_REPOSITORY");

/// All parameters that can affect the behaviour of this CLI.
#[derive(Debug, Eq, PartialEq)]
pub struct Parameters {
    /// Add a trailing new-line character (`\n`) to the command output (default: false).
    new_line: bool,
    /// Don't escape ESC-symbol, i.e. directly write the ASCII-code of `ESC` to the response
    /// instead of `"\e"` for example. (default: false).
    no_esc_escape: bool,
    /// Only used if [`no_esc_escape`] is false. Determines the output style of the special
    /// `ESC` char. See [`crate::EscapeStyle`].
    escape_style: EscapeStyle,
}

impl Parameters {
    pub fn new_line(&self) -> bool {
        self.new_line
    }
    pub fn no_esc_escape(&self) -> bool {
        self.no_esc_escape
    }
    pub fn escape_style(&self) -> EscapeStyle {
        self.escape_style
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            new_line: false,
            no_esc_escape: false,
            escape_style: EscapeStyle::default(),
        }
    }
}

/// Analyzes the provided args and returns a tuple of
/// * the parameters,
/// * the command to execute.
///
/// If "help" was requested in parameter-style (e.g. `-h` or `--help`)
/// then "help" is returned as command.
pub fn analyze_args(args: &[&str]) -> Option<(Parameters, String)> {
    let mut params = Parameters::default();
    let mut cmd = None;

    // we skip the first one because this is the binary name
    let mut i = 1;
    while i < args.len() {
        let arg = args[i];
        let maybe_next_arg = args.get(i + 1);
        if arg.starts_with("--") || arg.starts_with("-") {
            match arg {
                "-n" | "--new-line" => params.new_line = true,
                "-e" | "--no-escape" => params.no_esc_escape = true,
                "-s" | "--escape-style" => {
                    params.escape_style =
                        EscapeStyle::from_str(maybe_next_arg.expect("Must provide escape style"))
                            .expect(
                            "You must provide an valid escape style. Type '--help' for assistance.",
                        );
                    i += 1;
                }
                "-h" | "--help" => cmd = Some("help"),
                _ => panic!("Unknown parameter '{}'! Type '--help' for assistance.", arg),
            }
        } else if cmd.is_none() {
            cmd = Some(arg);
        } else {
            panic!("Provided too many arguments/commands! Type '--help' for assistance.");
        }

        i += 1;
    }

    Some((
        params,
        cmd.expect("Provided no argument/command! Type '--help' for assistance.")
            .to_string(),
    ))
}

pub fn print_help() {
    println!(
        "{}",
        Colour::Green.bold().paint(format!(
            "{} ('ansi') @ version {}",
            CRATE_NAME.to_ascii_uppercase(), CRATE_VERSION
        ))
    );
    println!("Made by Philipp Schuster <phip1611@gmail.com>");
    println!("See - https://crates.io/crates/{} or", CRATE_NAME);
    println!("      {}", CRATE_URL);
    println!("----------------------------------------------------");
    println!("{}", Style::default().bold().paint("SYNOPSIS:"));
    println!(
        " ansi {}|{}|{}|{}|{}|... ",
        Colour::Red.paint("red"),
        Colour::Green.paint("green"),
        Style::default().bold().paint("bold"),
        Style::default().strikethrough().paint("strike"),
        Style::default().on(Colour::Cyan).paint("bg-cyan"),
    );
    println!("      [-n|--new-line] [-e|--no-escape] [-h|--help]");
    println!("      [-s|--escape-style bash|unicode|unicode-rust|hex]");

    println!("{}", Style::default().bold().paint("PARAMETERS:"));
    println!("  -n: add new line character to output (default: false)");
    println!("  -e: don't escape 'ESC'-symbol, i.e. return the real ASCII value of `ESC` instead of `\\e`");
    println!("  -s: only useful if `-e` is NOT provided: style of the `ESC`-character-escaping");

    println!("{}", Style::default().bold().paint("COMMANDS:"));
    println!(
        "  For a full list visit: https://crates.io/crates/{}",
        CRATE_NAME
    );
    println!("  The most basic ones are all supported. For example:");
    println!("    reset");
    println!("    {}", Style::default().fg(Colour::Black).paint("black"));
    println!(
        "    {}",
        Style::default().on(Colour::Black).paint("bg-black")
    );
    println!("    {}", Style::default().fg(Colour::Red).paint("red"));
    println!("    {}", Style::default().on(Colour::Red).paint("bg-red"));
    println!("    {}", Style::default().fg(Colour::Green).paint("green"));
    println!(
        "    {}",
        Style::default().on(Colour::Green).paint("bg-green")
    );
    println!(
        "    {}",
        Style::default().fg(Colour::Yellow).paint("yellow")
    );
    println!(
        "    {}",
        Style::default().on(Colour::Yellow).paint("bg-yellow")
    );
    println!("    {}", Style::default().fg(Colour::Blue).paint("blue"));
    println!("    {}", Style::default().on(Colour::Blue).paint("bg-blue"));
    println!(
        "    {}",
        Style::default().fg(Colour::Purple).paint("purple")
    );
    println!(
        "    {}",
        Style::default().on(Colour::Purple).paint("bg-purple")
    );
    println!("    {}", Style::default().fg(Colour::Cyan).paint("cyan"));
    println!("    {}", Style::default().on(Colour::Cyan).paint("bg-cyan"));
    println!("    {}", Style::default().fg(Colour::White).paint("white"));
    println!(
        "    {}",
        Style::default().on(Colour::White).paint("bg-white")
    );
    println!("    normal");
    println!("    {}", Style::default().bold().paint("bold"));
    println!("    {}", Style::default().dimmed().paint("dimmed"));
    println!("    {}", Style::default().italic().paint("italic"));
    println!("    {}", Style::default().underline().paint("underline"));
    println!("    {}", Style::default().blink().paint("blink"));
    println!("    {} (hidden)", Style::default().hidden().paint("hidden"));
    println!(
        "    {}",
        Style::default()
            .strikethrough()
            .paint("strike | strikethrough")
    );

    println!("{}", Style::default().bold().paint("ESCAPE STYLES"));
    println!("  bash:         {}", EscapeStyle::Bash.help_text());
    println!("  hex:          {}", EscapeStyle::Hex.help_text());
    println!("  unicode     : {}", EscapeStyle::Unicode.help_text());
    println!("  unicode-rust: {}", EscapeStyle::UnicodeRust.help_text());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_analyze_args_1() {
        // first arg is on UNIX always the name of the invoked binary
        let args = ["ansi", "red"];
        let (actual_params_1, actual_cmd_1) = analyze_args(&args).expect("Must be valid");
        assert_eq!(actual_params_1.new_line(), false);
        assert_eq!(actual_params_1.no_esc_escape(), false);
        assert_eq!(actual_params_1.escape_style(), EscapeStyle::default());
        assert_eq!(actual_cmd_1, "red");
    }

    #[test]
    #[should_panic]
    fn test_analyze_args_2() {
        // first arg is on UNIX always the name of the invoked binary
        let args = ["ansi"];
        let (actual_params_1, actual_cmd_1) = analyze_args(&args).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_analyze_args_3() {
        // unknown parameter g
        let args = ["ansi", "red", "-g", "-n", "--escape-style", "unicode-rust"];
        let (actual_params_1, actual_cmd_1) = analyze_args(&args).expect("Must be valid");
    }

    #[test]
    fn test_analyze_args_4() {
        // first arg is on UNIX always the name of the invoked binary
        let args = ["ansi", "red", "-n", "--escape-style", "unicode-rust"];
        let (actual_params_1, actual_cmd_1) = analyze_args(&args).expect("Must be valid");
        assert_eq!(actual_params_1.new_line(), true);
        assert_eq!(actual_params_1.no_esc_escape(), false);
        assert_eq!(actual_params_1.escape_style(), EscapeStyle::UnicodeRust);
        assert_eq!(actual_cmd_1, "red");
    }
}
