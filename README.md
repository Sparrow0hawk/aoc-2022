# Advent of Code 2022

Solutions for advent of code 2022 using Rust!

## Usage

This code will download the relevant task data if you include your AoC session code as an environment variable.

To get your session code:
1. Log in at https://adventofcode.com/2022
2. In FireFox, right-click and "Inspect"
3. Select the "Storage" tab and under Cookies you'll have a cookie called session
4. Copy the value of the session cookie

To use this with this repository:

```bash
$ cd aoc-2022

$ COOKIE=XXXX cargo run -- --task 1
```

Where you replace `XXXX` with the value of the session cookie