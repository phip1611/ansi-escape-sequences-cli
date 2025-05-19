# 'ansi' - a CLI utility to quickly get ANSI escape codes

`ansi-escape-sequences-cli` provides an executable called `ansi` which can be
used in a shell or in a shell script to easily colorize and style terminal
output.

## Usage
- Add `ansi` to your `PATH`
- Embed it in your shell scripts like this:
  ```bash
  # You need "-e" to let echo replace the escaped ESC symbol
  echo -e "$(ansi yellow bold)WARNING$(ansi reset): There was a problem"
  # Or use echo without "-e":
  echo "$(ansi yellow bold --escape-style=direct)WARNING$(ansi reset): There was a problem"
  ```
- Type `--help` for more guidance!

## Install
`$ cargo install ansi-escape-sequences-cli` (the binary is just called `ansi`!)

## Supported Platforms
- ✔️ Linux
- ✔️ MacOS
- ✔️ Windows

## MSRV
Ths MSRV is `1.85.1`.
