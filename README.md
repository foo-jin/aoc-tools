# AoC-tools
This project is a simple toolset to retrieve [Advent of Code](https://adventofcode.com/) problem instances and submit solutions to them. I made this mainly for personal use, but feedback is always welcome.

## Installation
Clone the source repository, and then run `cargo install --path .`. This will use cargo to install the project in the current directory. The resulting binary will be placed in `$HOME/.cargo/bin` under default settings.

## Usage
If the binary is in PATH, you can run `aoc --help` for the usage instructions.

## Configuration
`aoc` will look for a aoc.toml file in the current directory and it's parent directories. You can either add one manually, or create it with `aoc init`. Here is an example config:
```
api_key = <secret cookie>         # required
year = 2018                       # required
leaderboards = [<leaderboard id>] # can be empty
```
Since Advent of Code does not have an official API some of these values need to be digged up using a browser.
The api key can be found in your cookies, it looks like this: `session=<lots of random characters and numbers>`.
The leaderboard ID's can be found in the url of the leaderboard: `https://adventofcode.com/2018/leaderboard/private/view/<leaderboard id>`
