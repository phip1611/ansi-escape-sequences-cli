#!/usr/bin/env bash

set -euo pipefail

cargo build --release
export PATH="target/release:$PATH"

echo "### FOREGROUND COLORS ###"
echo -e "$(ansi black)black$(ansi reset)"
echo -e "$(ansi red)red$(ansi reset)"
echo -e "$(ansi green)green$(ansi reset)"
echo -e "$(ansi yellow)yellow$(ansi reset)"
echo -e "$(ansi blue)blue$(ansi reset)"
echo -e "$(ansi magenta)magenta$(ansi reset)"
echo -e "$(ansi cyan)cyan$(ansi reset)"
echo -e "$(ansi white)white$(ansi reset)"
echo -e "$(ansi gray)gray$(ansi clear)"
echo -e "$(ansi bright-red)bright-red$(ansi reset)"
echo -e "$(ansi bright-green)bright-green$(ansi reset)"
echo -e "$(ansi bright-yellow)bright-yellow$(ansi reset)"
echo -e "$(ansi bright-blue)bright-blue$(ansi reset)"
echo -e "$(ansi bright-magenta)bright-magenta$(ansi reset)"
echo -e "$(ansi bright-cyan)bright-cyan$(ansi reset)"
echo -e "$(ansi bright-white)bright-white$(ansi reset)"

echo "### BACKGROUND COLORS ###"
echo -e "$(ansi bg-black)bg-black$(ansi reset)"
echo -e "$(ansi bg-red)bg-red$(ansi reset)"
echo -e "$(ansi bg-green)bg-green$(ansi reset)"
echo -e "$(ansi bg-yellow)bg-yellow$(ansi reset)"
echo -e "$(ansi bg-blue)bg-blue$(ansi reset)"
echo -e "$(ansi bg-magenta)bg-magenta$(ansi reset)"
echo -e "$(ansi bg-cyan)bg-cyan$(ansi reset)"
echo -e "$(ansi bg-white)bg-white$(ansi reset)"
echo -e "$(ansi bg-gray)bg-gray$(ansi clear)"
echo -e "$(ansi bg-bright-red)bg-bright-red$(ansi reset)"
echo -e "$(ansi bg-bright-green)bg-bright-green$(ansi reset)"
echo -e "$(ansi bg-bright-yellow)bg-bright-yellow$(ansi reset)"
echo -e "$(ansi bg-bright-blue)bg-bright-blue$(ansi reset)"
echo -e "$(ansi bg-bright-magenta)bg-bright-magenta$(ansi reset)"
echo -e "$(ansi bg-bright-cyan)bg-bright-cyan$(ansi reset)"
echo -e "$(ansi bg-bright-white)bg-bright-white$(ansi reset)"

echo "### STYLES ###"
echo -e "$(ansi bold)bold$(ansi reset)"
echo -e "$(ansi dimmed)dimmed$(ansi reset)"
echo -e "$(ansi italic)italic$(ansi reset)"
echo -e "$(ansi underline)underline$(ansi reset)"
echo -e "$(ansi blink)blink$(ansi reset)"
echo -e "$(ansi hidden)hidden$(ansi reset)"
echo -e "$(ansi strike)strike-through$(ansi reset)"

echo "### COMBINATIONS (SUBSET) ###"
echo -e "$(ansi red bold)red bold$(ansi reset)"
echo -e "$(ansi bright-red bold)bright-red bold$(ansi reset)"
echo -e "$(ansi dimmed green)dimmed green$(ansi reset)"

# README example
echo -e "$(ansi yellow bold)WARNING$(ansi reset): There was a problem"
# Or without "-e":
echo "$(ansi yellow bold --escape-style=direct)WARNING$(ansi --escape-style=direct reset): There was a problem"
