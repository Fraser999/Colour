# Colour

Macros for creating coloured console output.

## General

To view the colour palette:

```sh
cargo run --example println
```

The following macros are provided:

<table>
  <tr>
    <th colspan="2">Like <code>print!</code> (writes to stdout, no newline)</th>
    <th colspan="2">Like <code>println!</code> (writes to stdout, with newline)</th>
  </tr>
  <tr>
    <th>Normal Colour</th>
    <th>Dark Colour</th>
    <th>Normal Colour</th>
    <th>Dark Colour</th>
  </tr>
  <tr>
    <td><code>black!</code></td>
    <td>N/A</td>
    <td><code>black_ln!</code></td>
    <td>N/A</td>
  </tr>
  <tr>
    <td><code>red!</code></td>
    <td><code>dark_red!</code></td>
    <td><code>red_ln!</code></td>
    <td><code>dark_red_ln!</code></td>
  </tr>
  <tr>
    <td><code>green!</code></td>
    <td><code>dark_green!</code></td>
    <td><code>green_ln!</code></td>
    <td><code>dark_green_ln!</code></td>
  </tr>
  <tr>
    <td><code>yellow!</code></td>
    <td><code>dark_yellow!</code></td>
    <td><code>yellow_ln!</code></td>
    <td><code>dark_yellow_ln!</code></td>
  </tr>
  <tr>
    <td><code>blue!</code></td>
    <td><code>dark_blue!</code></td>
    <td><code>blue_ln!</code></td>
    <td><code>dark_blue_ln!</code></td>
  </tr>
  <tr>
    <td><code>magenta!</code></td>
    <td><code>dark_magenta!</code></td>
    <td><code>magenta_ln!</code></td>
    <td><code>dark_magenta_ln!</code></td>
  </tr>
  <tr>
    <td><code>cyan!</code></td>
    <td><code>dark_cyan!</code></td>
    <td><code>cyan_ln!</code></td>
    <td><code>dark_cyan_ln!</code></td>
  </tr>
  <tr>
    <td><code>grey!</code></td>
    <td><code>dark_grey!</code></td>
    <td><code>grey_ln!</code></td>
    <td><code>dark_grey_ln!</code></td>
  </tr>
  <tr>
    <td><code>gray!</code></td>
    <td><code>dark_gray!</code></td>
    <td><code>gray_ln!</code></td>
    <td><code>dark_gray_ln!</code></td>
  </tr>
  <tr>
    <td><code>white!</code></td>
    <td>N/A</td>
    <td><code>white_ln!</code></td>
    <td>N/A</td>
  </tr>
</table>

<table>
  <tr>
    <th colspan="2">Like <code>eprint!</code> (writes to stderr, no newline)</th>
    <th colspan="2">Like <code>eprintln!</code> (writes to stderr, with newline)</th>
  </tr>
  <tr>
    <th>Normal Colour</th>
    <th>Dark Colour</th>
    <th>Normal Colour</th>
    <th>Dark Colour</th>
  </tr>
  <tr>
    <td><code>e_black!</code></td>
    <td>N/A</td>
    <td><code>e_black_ln!</code></td>
    <td>N/A</td>
  </tr>
  <tr>
    <td><code>e_red!</code></td>
    <td><code>e_dark_red!</code></td>
    <td><code>e_red_ln!</code></td>
    <td><code>e_dark_red_ln!</code></td>
  </tr>
  <tr>
    <td><code>e_green!</code></td>
    <td><code>e_dark_green!</code></td>
    <td><code>e_green_ln!</code></td>
    <td><code>e_dark_green_ln!</code></td>
  </tr>
  <tr>
    <td><code>e_yellow!</code></td>
    <td><code>e_dark_yellow!</code></td>
    <td><code>e_yellow_ln!</code></td>
    <td><code>e_dark_yellow_ln!</code></td>
  </tr>
  <tr>
    <td><code>e_blue!</code></td>
    <td><code>e_dark_blue!</code></td>
    <td><code>e_blue_ln!</code></td>
    <td><code>e_dark_blue_ln!</code></td>
  </tr>
  <tr>
    <td><code>e_magenta!</code></td>
    <td><code>e_dark_magenta!</code></td>
    <td><code>e_magenta_ln!</code></td>
    <td><code>e_dark_magenta_ln!</code></td>
  </tr>
  <tr>
    <td><code>e_cyan!</code></td>
    <td><code>e_dark_cyan!</code></td>
    <td><code>e_cyan_ln!</code></td>
    <td><code>e_dark_cyan_ln!</code></td>
  </tr>
  <tr>
    <td><code>e_grey!</code></td>
    <td><code>e_dark_grey!</code></td>
    <td><code>e_grey_ln!</code></td>
    <td><code>e_dark_grey_ln!</code></td>
  </tr>
  <tr>
    <td><code>e_gray!</code></td>
    <td><code>e_dark_gray!</code></td>
    <td><code>e_gray_ln!</code></td>
    <td><code>e_dark_gray_ln!</code></td>
  </tr>
  <tr>
    <td><code>e_white!</code></td>
    <td>N/A</td>
    <td><code>e_white_ln!</code></td>
    <td>N/A</td>
  </tr>
</table>

<table>
  <tr>
    <th colspan="2">Like <code>write!</code> (no newline)</th>
    <th colspan="2">Like <code>writeln!</code> (with newline)</th>
  </tr>
  <tr>
    <th>Normal Colour</th>
    <th>Dark Colour</th>
    <th>Normal Colour</th>
    <th>Dark Colour</th>
  </tr>
  <tr>
    <td><code>write_black!</code></td>
    <td>N/A</td>
    <td><code>write_black_ln!</code></td>
    <td>N/A</td>
  </tr>
  <tr>
    <td><code>write_red!</code></td>
    <td><code>write_dark_red!</code></td>
    <td><code>write_red_ln!</code></td>
    <td><code>write_dark_red_ln!</code></td>
  </tr>
  <tr>
    <td><code>write_green!</code></td>
    <td><code>write_dark_green!</code></td>
    <td><code>write_green_ln!</code></td>
    <td><code>write_dark_green_ln!</code></td>
  </tr>
  <tr>
    <td><code>write_yellow!</code></td>
    <td><code>write_dark_yellow!</code></td>
    <td><code>write_yellow_ln!</code></td>
    <td><code>write_dark_yellow_ln!</code></td>
  </tr>
  <tr>
    <td><code>write_blue!</code></td>
    <td><code>write_dark_blue!</code></td>
    <td><code>write_blue_ln!</code></td>
    <td><code>write_dark_blue_ln!</code></td>
  </tr>
  <tr>
    <td><code>write_magenta!</code></td>
    <td><code>write_dark_magenta!</code></td>
    <td><code>write_magenta_ln!</code></td>
    <td><code>write_dark_magenta_ln!</code></td>
  </tr>
  <tr>
    <td><code>write_cyan!</code></td>
    <td><code>write_dark_cyan!</code></td>
    <td><code>write_cyan_ln!</code></td>
    <td><code>write_dark_cyan_ln!</code></td>
  </tr>
  <tr>
    <td><code>write_grey!</code></td>
    <td><code>write_dark_grey!</code></td>
    <td><code>write_grey_ln!</code></td>
    <td><code>write_dark_grey_ln!</code></td>
  </tr>
  <tr>
    <td><code>write_gray!</code></td>
    <td><code>write_dark_gray!</code></td>
    <td><code>write_gray_ln!</code></td>
    <td><code>write_dark_gray_ln!</code></td>
  </tr>
  <tr>
    <td><code>write_white!</code></td>
    <td>N/A</td>
    <td><code>write_white_ln!</code></td>
    <td>N/A</td>
  </tr>
</table>

All of these macros are available with bold formatting by appending `_bold` (for example
`blue_ln_bold`). 

There following macros which use the default foreground colour are also available:
- `print_bold!`
- `eprint_bold!` and `e_print_bold!`
- `println_bold!` and `print_ln_bold!`
- `eprintln_bold!` and `e_print_ln_bold!`
- `write_bold!`
- `writeln_bold!`

## Configuration

The crate follows the recommendations in
[Standard for ANSI Colors in Terminals](http://bixense.com/clicolors), meaning that the environment
variables `NO_COLOR` and `CLICOLOR_FORCE` are respected.  The library acts as if `CLICOLOR` is set,
so that environment variable has no effect.

The environment variable `TERM` is also considered: if set to `dumb`, coloured output is disabled,
and if set to any other value, coloured output is enabled.

The order of precedence is `NO_COLOR`, followed by `CLICOLOR_FORCE` and then `TERM`.

### Usage in Binaries

Binaries can override these environment variables and the automatic detection of a terminal/tty by
calling `force_colour` or `force_no_colour`.  The binary should call only one of these, and the call
should happen before any potential calls to any of the macros this crate provides.

Libraries should generally never call these functions.

## Example

```rust
use colour::*;

fn _foo() {
    let err: Result<(), u8> = Err(1);
    yellow_ln!("Failed on {}", 9);
    print!("Error details: ");
    dark_red_ln_bold!("{:?}", err);
}

fn main() {
    grey_ln!("grey");
    grey_ln_bold!("bold grey");
    dark_grey_ln!("dark grey");
    dark_grey_ln_bold!("bold dark grey");
    red_ln!("red");
    red_ln_bold!("bold red");
    dark_red_ln!("dark red");
    dark_red_ln_bold!("bold dark red");
    green_ln!("green");
    green_ln_bold!("bold green");
    dark_green_ln!("dark green");
    dark_green_ln_bold!("bold dark green");
    yellow_ln!("yellow");
    yellow_ln_bold!("bold yellow");
    dark_yellow_ln!("dark yellow");
    dark_yellow_ln_bold!("bold dark yellow");
    blue_ln!("blue");
    blue_ln_bold!("bold blue");
    dark_blue_ln!("dark blue");
    dark_blue_ln_bold!("bold dark blue");
    magenta_ln!("magenta");
    magenta_ln_bold!("bold magenta");
    dark_magenta_ln!("dark magenta");
    dark_magenta_ln_bold!("bold dark magenta");
    cyan_ln!("cyan");
    cyan_ln_bold!("bold cyan");
    dark_cyan_ln!("dark cyan");
    dark_cyan_ln_bold!("bold dark cyan");
    black_ln!("black");
    black_ln_bold!("bold black");
    white_ln!("white");
    white_ln_bold!("bold white");
    println!("default colour");
    println_bold!("bold default colour");
}
```

## Minimum Rust Version

The crate can be compiled with Rust versions 1.70.0 and newer.

##  License

`colour` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.
