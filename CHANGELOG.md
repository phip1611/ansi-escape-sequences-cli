# CHANGELOG

## v0.2.1 (2025-05-11)
- Added support for various bright colors. Type `--help` to get a list of them.

## v0.2.0 (2025-05-11)
- **BREAKING** MSRV is now `1.85.1`
- CLI parsing was refactored: now uses `clap`; CLI should be mostly compatible
- You can now use multiple args: `echo "$(ansi red bold)hey$(ansi reset)"` for
  example.

## v0.1.4 (2021-05-02)
