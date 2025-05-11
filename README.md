# 'ansi' - a CLI utility to quickly get ANSI escape codes
This Rust project called `ansi-escape-sequences-cli` provides an executable called `ansi`
which can be used on the Terminal to easily colorize/style your output.

## Usage
- Add `ansi` to your `PATH`
- Embed it in your shell scripts like this:
  ```bash
  echo "$(ansi yellow bold)WARNING$(ansi reset): There was a problem"
  ```
- Type `--help` for more guidance!

## Install
`$ cargo install ansi-escape-sequences-cli` (the binary is just called `ansi`!)

## MSRV
Ths MSRV is `1.85.1`.
