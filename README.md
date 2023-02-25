# Colour

Macros for creating coloured console output.

## General

The following macros are provided:

| Colour     | Dark Variant    | With LF       | Dark With LF       |
|:-----------|:----------------|:--------------|:-------------------|
| `black!`   | N/A             | `black_ln!`   | N/A                |
| `red!`     | `dark_red!`     | `red_ln!`     | `dark_red_ln!`     |
| `green!`   | `dark_green!`   | `green_ln!`   | `dark_green_ln!`   |
| `yellow!`  | `dark_yellow!`  | `yellow_ln!`  | `dark_yellow_ln!`  |
| `blue!`    | `dark_blue!`    | `blue_ln!`    | `dark_blue_ln!`    |
| `magenta!` | `dark_magenta!` | `magenta_ln!` | `dark_magenta_ln!` |
| `cyan!`    | `dark_cyan!`    | `cyan_ln!`    | `dark_cyan_ln!`    |
| `grey!`    | `dark_grey!`    | `grey_ln!`    | `dark_grey_ln!`    |
| `white!`   | N/A             | `white_ln!`   | N/A                |

The versions with suffix `_ln` append a newline (similar to
[`print!`](https://doc.rust-lang.org/std/macro.print.html) versus
[`println!`](https://doc.rust-lang.org/std/macro.println.html)).

There are also `prnt!` and `prnt_ln!` available which print using the current default foreground
colour.

Every variant also has a version with prefix `e_`.  Variants with this prefix output to stderr,
while those without the `e_` prefix output to stdout.

## Example

```rust
use colour::*;

fn foo() {
    let err: Result<(), u8> = Err(1);
    prnt_ln!("Failed on {}", 9);
    yellow!("Error details: ");
    red_ln!("{:?}", err);
}

fn main() {
    black!("black ");
    red!("red ");
    green!("green ");
    yellow!("yellow ");
    blue!("blue ");
    magenta!("magenta ");
    cyan!("cyan ");
    grey!("grey ");
    white!("white ");
    dark_red!("dark_red ");
    dark_green!("dark_green ");
    dark_yellow!("dark_yellow ");
    dark_blue!("dark_blue ");
    dark_magenta!("dark_magenta ");
    dark_cyan!("dark_cyan ");
    dark_grey!("dark_grey ");
    prnt!("default colour\n\n");
}
```

## Minimum Rust Version

The crate can be compiled with Rust versions 1.58.0 and newer.

##  License

`colour` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.
