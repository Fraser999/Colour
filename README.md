# Colour

Macros for creating coloured console output.

## General

The following macros are provided:

|Colour|Dark Variant|With LF|Dark With LF|
|:---|:---|:---|:---|
|`black!`|`dark_black!`|`black_ln!`|`dark_black_ln!`|
|`red!`|`dark_red!`|`red_ln!`|`dark_red_ln!`|
|`green!`|`dark_green!`|`green_ln!`|`dark_green_ln!`|
|`yellow!`|`dark_yellow!`|`yellow_ln!`|`dark_yellow_ln!`|
|`blue!`|`dark_blue!`|`blue_ln!`|`dark_blue_ln!`|
|`magenta!`|`dark_magenta!`|`magenta_ln!`|`dark_magenta_ln!`|
|`cyan!`|`dark_cyan!`|`cyan_ln!`|`dark_cyan_ln!`|
|`white!`|`dark_white!`|`white_ln!`|`dark_white_ln!`|

The versions with suffix `_ln` append a newline (similar to
[`print!`](https://doc.servo.org/std/macro.print.html) versus
[`println!`](https://doc.servo.org/std/macro.println.html)).

There are also `prnt!` and `prnt_ln!` available which print using the current default foreground
colour.

## Example

```rust
#[macro_use]
extern crate colour;

fn foo() {
    let err: Result<(), u8> = Err(1);
    prnt_ln!("Failed on {}", 9);
    yellow!("Error details: ");
    red_ln!("{:?}", err);
}

fn all_colours() {
    black!("black ");
    red!("red ");
    green!("green ");
    yellow!("yellow ");
    blue!("blue ");
    magenta!("magenta ");
    cyan!("cyan ");
    white!("white ");
    dark_black!("dark_black ");
    dark_red!("dark_red ");
    dark_green!("dark_green ");
    dark_yellow!("dark_yellow ");
    dark_blue!("dark_blue ");
    dark_magenta!("dark_magenta ");
    dark_cyan!("dark_cyan ");
    dark_white!("dark_white ");
    prnt!("default colour\n\n");
}
```

## Performance

Internally, the macros use a global static handle to stdout which is locked on each call.  At
the end of each call, stdout is flushed.  This results in slower performance compared to
`print!` or `println!`, but avoids interleaving of the output in a multi-threaded environment.

##  License

`colour` is distributed under the terms of the General Public License (GPL), version 3
([COPYING](COPYING) or http://www.gnu.org/licenses/gpl-3.0.en.html).
