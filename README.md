# Colour

Macros for creating coloured console output.

## General

To view the colour palette:

```sh
cargo test print_stdout_no_newline
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
    <td><code>write_white!</code></td>
    <td>N/A</td>
    <td><code>write_white_ln!</code></td>
    <td>N/A</td>
  </tr>
</table>

There are also `prnt!`, `prnt_ln!`, `wrte!` and `wrte_ln!` available which use the current default
foreground colour.

## Example

```rust
use colour::*;

fn foo() {
    let err: Result<(), u8> = Err(1);
    prnt_ln!("Failed on {}", 9);
    yellow!("Error details: ");
    red_ln!("{:?}", err);
}

fn bar() {
    use std::io::Write as _;

    let mut v = vec![];
    let err: Result<(), u8> = Err(1);
    wrte_ln!(&mut v, "Failed on {}", 9);
    write_yellow!(&mut v, "Error details: ");
    write_red_ln!(&mut v, "{:?}", err);
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
