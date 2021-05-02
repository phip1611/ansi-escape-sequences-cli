# 'ansi' - a CLI utility to quickly get ANSI escape codes
This Rust project called `ansi-escape-sequences-cli` provides an executable called `ansi`
which can be used on the Terminal to easily colorize/style your output.

## Install
`$ cargo install ansi-escape-sequences-cli` (the binary is just called `ansi`!)

## Usage example 1: in Terminal
`$ echo "$(ansi bg-purple)Hello World $(ansi reset)$(ansi red)$(ansi bold)$(ansi underline)Red Warning$(ansi reset)"` 

![Colorful example of terminal output](demo.png "Colorful example of terminal output")
  
## Usage example 2: multiple uses in a bash script
test.sh
```bash
# with "-e" we can prevent unnecessary ASCII-escaping of the "ESC" symbol
# otherwise it can happen that your shell script outputs 
# "Foo=\e[31mbar" instead of the colored output you wanted
reset=$(ansi -e reset)
red=$(ansi -e red)
echo "Foo=${red}bar"
echo "Bar=${reset}foo"
```

## zsh auto completion file
In `res/zsh-completion/_ansi` is a completion file. Install it for example into
`/usr/local/share/zsh/site-functions/`.

## Options/Parameters
![Image of colorful help page, text version is below](help.png "Image of colorful help page, text version is below")
*1:1 copy of "help"-page*
```text
ANSI-ESCAPE-SEQUENCES-CLI ('ansi') @ version 0.1.0
Made by Philipp Schuster <phip1611@gmail.com>
See - https://crates.io/crates/ansi-escape-sequences-cli or
      https://github.com/phip1611/ansi-escape-sequences-cli
----------------------------------------------------
SYNOPSIS:
 ansi red|green|bold|strike|bg-cyan|... 
      [-n|--new-line] [-e|--no-escape] [-h|--help]
      [-s|--escape-style bash|unicode|unicode-rust|hex]
PARAMETERS:
  -n: add new line character to output (default: false)
  -e: don't ASCII-escape output, i.e. it will return real ASCII value of `ESC` instead of `\e`
  -s: only useful if `-e` is NOT provided: style of the ASCII string escape format
COMMANDS:
  For a full list visit: https://crates.io/crates/ansi-escape-sequences-cli
  The most basic ones are all supported. For example:
    clear/reset
    black
    bg-black
    red
    bg-red
    green
    bg-green
    yellow
    bg-yellow
    blue
    bg-blue
    purple
    bg-purple
    cyan
    bg-cyan
    white
    bg-white
    normal
    bold
    dimmed
    italic
    underline
    blink
    hidden (hidden)
    strike | strikethrough
ESCAPE STYLES
  bash:         Bash (and many other tools) support `\e` that gets replaced by ASCIIs `ESC` code.
  hex:          Many tools allow hex values in the following notation: `\x1b`, i.e. ASCIIs `ESC` code.
  unicode     : Many tools allow unicode values in the following notation: `\u001b`, i.e. ASCIIs `ESC` code.
  unicode-rust: Rust uses unicode in the following form: `\u{1b}`, i.e. ASCIIs `ESC` code.

```