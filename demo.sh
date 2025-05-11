#!/usr/bin/env bash

set -euo pipefail

cargo build --release
export PATH="target/release:$PATH"


echo "Foreground colors:"
echo "$(ansi black)black$(ansi clear)"
echo "$(ansi red)red$(ansi reset)"
echo "$(ansi green)green$(ansi reset)"
echo "$(ansi yellow)yellow$(ansi reset)"
echo "$(ansi blue)blue$(ansi reset)"
echo "$(ansi magenta)magenta$(ansi reset)"
echo "$(ansi cyan)cyan$(ansi reset)"
echo "$(ansi white)white$(ansi reset)"


echo "Background colors:"
echo "$(ansi bg-black)bg-black$(ansi reset)"
echo "$(ansi bg-red)bg-red$(ansi reset)"
echo "$(ansi bg-green)bg-green$(ansi reset)"
echo "$(ansi bg-yellow)bg-yellow$(ansi reset)"
echo "$(ansi bg-blue)bg-blue$(ansi reset)"
echo "$(ansi bg-magenta)bg-magenta$(ansi reset)"
echo "$(ansi bg-cyan)bg-cyan$(ansi reset)"
echo "$(ansi bg-white)bg-white$(ansi reset)"

echo "Styles:"
echo "$(ansi bold)bold$(ansi reset)"
echo "$(ansi dimmed)dimmed$(ansi reset)"
echo "$(ansi italic)italic$(ansi reset)"
echo "$(ansi underline)underline$(ansi reset)"
echo "$(ansi blink)blink$(ansi reset)"
echo "$(ansi hidden)hidden$(ansi reset)"
echo "$(ansi strike)strike-through$(ansi reset)"

echo "Combinations:"
echo "$(ansi red bold)red bold$(ansi reset)"
echo "$(ansi dimmed green)dimmed green$(ansi reset)"