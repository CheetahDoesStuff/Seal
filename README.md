# Seal
**A simple and intuitive TUI for testing out and writing REGEX queries.**
![image](https://github.com/CheetahDoesStuff/Seal/blob/main/readme/Screenshot%202026-10-04%20at%2013.27.12.png)

Seal is a lightweight TUI written in rust, made for instant feedback when developing regex queries and right there in your terminal, with a simple interface that does one job, and it does it well.

It consists of 3 main components, the input, regex and output. The input and regex is quite self explanatory, give a regex string, give an input to run said regex string against, get an output.

The output can be visualized in 3 ways:
- highlight: displaying the entire input text, and highlighting any matches in repeating light and dark blue to clearly state where each match ends, starts and what it encompasses.
- extract: displays a numbered list of all matches and the body of said matches, great if you use characters such as `?` or `*` that can result in multiple different matches that arent identical, or if you arent interested in the non-matching part of the input.
- extract-raw: identical to the extract, but in a raw format (eg `1. test 2. testing 3. test123` becomes `testtestingtest123`), removing decorations and just returning the matches next to each other, highlighted of course, to see where one starts and the other one ends. Good if your matches are supposed to form something when combined (eg `1. test 2. ing 3.  stuff` could be intended (and become with extract-raw) to be viewed as `testing stuff`).

## Tech stack
- Rust
- Ratatui
- fancy-regex

## Installation
The package is distributed on crates.io, to install (assuming cargo is already installed), simply type
```sh
cargo install ch-seal-tui
```
into your terminal.

You can then open it any time just by running `ch-seal-tui` in your terminal.

_Why is it called ch-seal-tui? well, both seal and seal-tui was taken, and so i put my name as a prefix._

## Usage
Controls are described in the app itself and usage is self-explanatory. Put in query in the query box, an input in the input box and see the result through the different output formats in the output box!
