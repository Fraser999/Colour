#![forbid(warnings)]
#![warn(
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#![deny(clippy::all, clippy::pedantic)]

use colour::*;

#[test]
fn colours() {
    grey_ln!("This is {}.", "grey");
    grey_ln!("So is this.");
    dark_grey_ln!("This is {}.", "dark_grey");
    dark_grey_ln!("So is this.");
    red_ln!("This is {}.", "red");
    red_ln!("So is this.");
    dark_red_ln!("This is {}.", "dark_red");
    dark_red_ln!("So is this.");
    green_ln!("This is {}.", "green");
    green_ln!("So is this.");
    dark_green_ln!("This is {}.", "dark_green");
    dark_green_ln!("So is this.");
    yellow_ln!("This is {}.", "yellow");
    yellow_ln!("So is this.");
    dark_yellow_ln!("This is {}.", "dark_yellow");
    dark_yellow_ln!("So is this.");
    blue_ln!("This is {}.", "blue");
    blue_ln!("So is this.");
    dark_blue_ln!("This is {}.", "dark_blue");
    dark_blue_ln!("So is this.");
    magenta_ln!("This is {}.", "magenta");
    magenta_ln!("So is this.");
    dark_magenta_ln!("This is {}.", "dark_magenta");
    dark_magenta_ln!("So is this.");
    cyan_ln!("This is {}.", "cyan");
    cyan_ln!("So is this.");
    dark_cyan_ln!("This is {}.", "dark_cyan");
    dark_cyan_ln!("So is this.");
    white_ln!("This is {}.", "white");
    white_ln!("So is this.");
    black_ln!("This is {}.", "black");
    black_ln!("So is this.");
    prnt_ln!("This is {}.", "the default colour");
    prnt_ln!("So is this.");

    grey!("g");
    grey!("{}, ", "rey");
    dark_grey!("d");
    dark_grey!("{}, ", "ark_grey");
    red!("r");
    red!("{}, ", "ed");
    dark_red!("d");
    dark_red!("{}, ", "ark_red");
    green!("g");
    green!("{}, ", "reen");
    dark_green!("d");
    dark_green!("{}, ", "ark_green");
    yellow!("y");
    yellow!("{}, ", "ellow");
    dark_yellow!("d");
    dark_yellow!("{}, ", "ark_yellow");
    blue!("b");
    blue!("{}, ", "lue");
    dark_blue!("d");
    dark_blue!("{}, ", "ark_blue");
    magenta!("m");
    magenta!("{}, ", "agenta");
    dark_magenta!("d");
    dark_magenta!("{}, ", "ark_magenta");
    cyan!("c");
    cyan!("{}, ", "yan");
    dark_cyan!("d");
    dark_cyan!("{}, ", "ark_cyan");
    white!("w");
    white!("{}, ", "hite");
    black!("b");
    black!("{}, ", "lack");
    prnt!("d");
    prnt!("efault {}", "colour\n\n");
}
