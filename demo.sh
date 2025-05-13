#!/usr/bin/env bash

set -euo pipefail

cargo build --release
export PATH="target/release:$PATH"


echo "### FOREGROUND COLORS ###"
echo "$(ansi black)black$(ansi clear)"
echo "$(ansi red)red$(ansi reset)"
echo "$(ansi green)green$(ansi reset)"
echo "$(ansi yellow)yellow$(ansi reset)"
echo "$(ansi blue)blue$(ansi reset)"
echo "$(ansi magenta)magenta$(ansi reset)"
echo "$(ansi cyan)cyan$(ansi reset)"
echo "$(ansi white)white$(ansi reset)"
echo "$(ansi gray)gray$(ansi clear)"
echo "$(ansi bright-red)bright-red$(ansi reset)"
echo "$(ansi bright-green)bright-green$(ansi reset)"
echo "$(ansi bright-yellow)bright-yellow$(ansi reset)"
echo "$(ansi bright-blue)bright-blue$(ansi reset)"
echo "$(ansi bright-magenta)bright-magenta$(ansi reset)"
echo "$(ansi bright-cyan)bright-cyan$(ansi reset)"
echo "$(ansi bright-white)bright-white$(ansi reset)"


echo "### BACKGROUND COLORS ###"
echo "$(ansi bg-black)bg-black$(ansi reset)"
echo "$(ansi bg-red)bg-red$(ansi reset)"
echo "$(ansi bg-green)bg-green$(ansi reset)"
echo "$(ansi bg-yellow)bg-yellow$(ansi reset)"
echo "$(ansi bg-blue)bg-blue$(ansi reset)"
echo "$(ansi bg-magenta)bg-magenta$(ansi reset)"
echo "$(ansi bg-cyan)bg-cyan$(ansi reset)"
echo "$(ansi bg-white)bg-white$(ansi reset)"
echo "$(ansi bg-gray)bg-gray$(ansi clear)"
echo "$(ansi bg-bright-red)bg-bright-red$(ansi reset)"
echo "$(ansi bg-bright-green)bg-bright-green$(ansi reset)"
echo "$(ansi bg-bright-yellow)bg-bright-yellow$(ansi reset)"
echo "$(ansi bg-bright-blue)bg-bright-blue$(ansi reset)"
echo "$(ansi bg-bright-magenta)bg-bright-magenta$(ansi reset)"
echo "$(ansi bg-bright-cyan)bg-bright-cyan$(ansi reset)"
echo "$(ansi bg-bright-white)bg-bright-white$(ansi reset)"

echo "### STYLES ###"
echo "$(ansi bold)bold$(ansi reset)"
echo "$(ansi dimmed)dimmed$(ansi reset)"
echo "$(ansi italic)italic$(ansi reset)"
echo "$(ansi underline)underline$(ansi reset)"
echo "$(ansi blink)blink$(ansi reset)"
echo "$(ansi hidden)hidden$(ansi reset)"
echo "$(ansi strike)strike-through$(ansi reset)"

echo "### COMBINATIONS (SUBSET) ###"
echo "$(ansi red bold)red bold$(ansi reset)"
echo "$(ansi bright-red bold)bright-red bold$(ansi reset)"
echo "$(ansi dimmed green)dimmed green$(ansi reset)"
