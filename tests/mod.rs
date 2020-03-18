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
fn stdout() {
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

#[test]
fn stderr() {
    e_grey_ln!("This is {}.", "grey");
    e_grey_ln!("So is this.");
    e_dark_grey_ln!("This is {}.", "dark_grey");
    e_dark_grey_ln!("So is this.");
    e_red_ln!("This is {}.", "red");
    e_red_ln!("So is this.");
    e_dark_red_ln!("This is {}.", "dark_red");
    e_dark_red_ln!("So is this.");
    e_green_ln!("This is {}.", "green");
    e_green_ln!("So is this.");
    e_dark_green_ln!("This is {}.", "dark_green");
    e_dark_green_ln!("So is this.");
    e_yellow_ln!("This is {}.", "yellow");
    e_yellow_ln!("So is this.");
    e_dark_yellow_ln!("This is {}.", "dark_yellow");
    e_dark_yellow_ln!("So is this.");
    e_blue_ln!("This is {}.", "blue");
    e_blue_ln!("So is this.");
    e_dark_blue_ln!("This is {}.", "dark_blue");
    e_dark_blue_ln!("So is this.");
    e_magenta_ln!("This is {}.", "magenta");
    e_magenta_ln!("So is this.");
    e_dark_magenta_ln!("This is {}.", "dark_magenta");
    e_dark_magenta_ln!("So is this.");
    e_cyan_ln!("This is {}.", "cyan");
    e_cyan_ln!("So is this.");
    e_dark_cyan_ln!("This is {}.", "dark_cyan");
    e_dark_cyan_ln!("So is this.");
    e_white_ln!("This is {}.", "white");
    e_white_ln!("So is this.");
    e_black_ln!("This is {}.", "black");
    e_black_ln!("So is this.");
    e_prnt_ln!("This is {}.", "the default colour");
    e_prnt_ln!("So is this.");

    e_grey!("g");
    e_grey!("{}, ", "rey");
    e_dark_grey!("d");
    e_dark_grey!("{}, ", "ark_grey");
    e_red!("r");
    e_red!("{}, ", "ed");
    e_dark_red!("d");
    e_dark_red!("{}, ", "ark_red");
    e_green!("g");
    e_green!("{}, ", "reen");
    e_dark_green!("d");
    e_dark_green!("{}, ", "ark_green");
    e_yellow!("y");
    e_yellow!("{}, ", "ellow");
    e_dark_yellow!("d");
    e_dark_yellow!("{}, ", "ark_yellow");
    e_blue!("b");
    e_blue!("{}, ", "lue");
    e_dark_blue!("d");
    e_dark_blue!("{}, ", "ark_blue");
    e_magenta!("m");
    e_magenta!("{}, ", "agenta");
    e_dark_magenta!("d");
    e_dark_magenta!("{}, ", "ark_magenta");
    e_cyan!("c");
    e_cyan!("{}, ", "yan");
    e_dark_cyan!("d");
    e_dark_cyan!("{}, ", "ark_cyan");
    e_white!("w");
    e_white!("{}, ", "hite");
    e_black!("b");
    e_black!("{}, ", "lack");
    e_prnt!("d");
    e_prnt!("efault {}", "colour\n\n");
}
