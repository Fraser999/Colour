//! Macros for creating coloured console output.
//!
//! The macros are based on the following colours:
//!
//! * grey
//! * red
//! * green
//! * yellow
//! * blue
//! * magenta
//! * cyan
//! * black
//! * white
//!
//! To view the colour palette, run:
//!
//! ```sh
//! cargo test print_stdout_no_newline
//! ```
//!
//! There are dark versions of each colour except black and white, and for all colours there is a
//! version with suffix `_ln` which appends a newline (similar to [`print!`] and [`println!`]).
//!
//! Every variant also has a version with prefix `e_`.  Variants with this prefix output to stderr,
//! while those without the `e_` prefix output to stdout.
//!
//! Every variant also has a version with prefix `write_`.  Variants with this prefix output to a
//! provided buffer, where the buffer implements `std::io::Write` (similar to [`write!`] and
//! [`writeln!`]).
//!
//! There are also `prnt!`, `prnt_ln!`, `wrte!` and `wrte_ln!` available which print using the
//! current default foreground colour.

/// Helpers and consts which should not be used directly outside of this crate.
#[doc(hidden)]
pub mod internal;

#[rustfmt::skip]
macro_rules! make_print_macro {
    ($dollar:tt $name:ident, $newline:literal, $colour:ident, $output:ident) => {
        #[macro_export]
        macro_rules! $name {
            () => {
                $crate::internal::write::<_, $newline>(
                    std::io::$output().lock(),
                    stringify!($output),
                    $crate::internal::Colour::$colour,
                    None
                )
            };
            ($dollar($dollar args:tt)*) => {
                $crate::internal::write::<_, $newline>(
                    std::io::$output().lock(),
                    stringify!($output),
                    $crate::internal::Colour::$colour,
                    Some(format_args!($dollar($dollar args)*))
                )
            };
        }
    };
}

make_print_macro!($ black, false, Black, stdout);
make_print_macro!($ e_black, false, Black, stderr);
make_print_macro!($ black_ln, true, Black, stdout);
make_print_macro!($ e_black_ln, true, Black, stderr);

make_print_macro!($ dark_red, false, DarkRed, stdout);
make_print_macro!($ e_dark_red, false, DarkRed, stderr);
make_print_macro!($ dark_red_ln, true, DarkRed, stdout);
make_print_macro!($ e_dark_red_ln, true, DarkRed, stderr);

make_print_macro!($ dark_green, false, DarkGreen, stdout);
make_print_macro!($ e_dark_green, false, DarkGreen, stderr);
make_print_macro!($ dark_green_ln, true, DarkGreen, stdout);
make_print_macro!($ e_dark_green_ln, true, DarkGreen, stderr);

make_print_macro!($ dark_yellow, false, DarkYellow, stdout);
make_print_macro!($ e_dark_yellow, false, DarkYellow, stderr);
make_print_macro!($ dark_yellow_ln, true, DarkYellow, stdout);
make_print_macro!($ e_dark_yellow_ln, true, DarkYellow, stderr);

make_print_macro!($ dark_blue, false, DarkBlue, stdout);
make_print_macro!($ e_dark_blue, false, DarkBlue, stderr);
make_print_macro!($ dark_blue_ln, true, DarkBlue, stdout);
make_print_macro!($ e_dark_blue_ln, true, DarkBlue, stderr);

make_print_macro!($ dark_magenta, false, DarkMagenta, stdout);
make_print_macro!($ e_dark_magenta, false, DarkMagenta, stderr);
make_print_macro!($ dark_magenta_ln, true, DarkMagenta, stdout);
make_print_macro!($ e_dark_magenta_ln, true, DarkMagenta, stderr);

make_print_macro!($ dark_cyan, false, DarkCyan, stdout);
make_print_macro!($ e_dark_cyan, false, DarkCyan, stderr);
make_print_macro!($ dark_cyan_ln, true, DarkCyan, stdout);
make_print_macro!($ e_dark_cyan_ln, true, DarkCyan, stderr);

make_print_macro!($ grey, false, Grey, stdout);
make_print_macro!($ e_grey, false, Grey, stderr);
make_print_macro!($ grey_ln, true, Grey, stdout);
make_print_macro!($ e_grey_ln, true, Grey, stderr);

make_print_macro!($ dark_grey, false, DarkGrey, stdout);
make_print_macro!($ e_dark_grey, false, DarkGrey, stderr);
make_print_macro!($ dark_grey_ln, true, DarkGrey, stdout);
make_print_macro!($ e_dark_grey_ln, true, DarkGrey, stderr);

make_print_macro!($ red, false, Red, stdout);
make_print_macro!($ e_red, false, Red, stderr);
make_print_macro!($ red_ln, true, Red, stdout);
make_print_macro!($ e_red_ln, true, Red, stderr);

make_print_macro!($ green, false, Green, stdout);
make_print_macro!($ e_green, false, Green, stderr);
make_print_macro!($ green_ln, true, Green, stdout);
make_print_macro!($ e_green_ln, true, Green, stderr);

make_print_macro!($ yellow, false, Yellow, stdout);
make_print_macro!($ e_yellow, false, Yellow, stderr);
make_print_macro!($ yellow_ln, true, Yellow, stdout);
make_print_macro!($ e_yellow_ln, true, Yellow, stderr);

make_print_macro!($ blue, false, Blue, stdout);
make_print_macro!($ e_blue, false, Blue, stderr);
make_print_macro!($ blue_ln, true, Blue, stdout);
make_print_macro!($ e_blue_ln, true, Blue, stderr);

make_print_macro!($ magenta, false, Magenta, stdout);
make_print_macro!($ e_magenta, false, Magenta, stderr);
make_print_macro!($ magenta_ln, true, Magenta, stdout);
make_print_macro!($ e_magenta_ln, true, Magenta, stderr);

make_print_macro!($ cyan, false, Cyan, stdout);
make_print_macro!($ e_cyan, false, Cyan, stderr);
make_print_macro!($ cyan_ln, true, Cyan, stdout);
make_print_macro!($ e_cyan_ln, true, Cyan, stderr);

make_print_macro!($ white, false, White, stdout);
make_print_macro!($ e_white, false, White, stderr);
make_print_macro!($ white_ln, true, White, stdout);
make_print_macro!($ e_white_ln, true, White, stderr);

make_print_macro!($ prnt, false, Reset, stdout);
make_print_macro!($ e_prnt, false, Reset, stderr);
make_print_macro!($ prnt_ln, true, Reset, stdout);
make_print_macro!($ e_prnt_ln, true, Reset, stderr);

#[rustfmt::skip]
macro_rules! make_write_macro {
    ($dollar:tt $name:ident, $newline:literal, $colour:ident) => {
        #[macro_export]
        macro_rules! $name {
            ($dollar dst:expr, $dollar($dollar args:tt)*) => {
                $crate::internal::write::<_, $newline>(
                    $dollar dst,
                    stringify!($dollar dst),
                    $crate::internal::Colour::$colour,
                    Some(format_args!($dollar($dollar args)*))
                )
            };
        }
    };
}

make_write_macro!($ write_black, false, Black);
make_write_macro!($ write_black_ln, true, Black);

make_write_macro!($ write_dark_red, false, DarkRed);
make_write_macro!($ write_dark_red_ln, true, DarkRed);

make_write_macro!($ write_dark_green, false, DarkGreen);
make_write_macro!($ write_dark_green_ln, true, DarkGreen);

make_write_macro!($ write_dark_yellow, false, DarkYellow);
make_write_macro!($ write_dark_yellow_ln, true, DarkYellow);

make_write_macro!($ write_dark_blue, false, DarkBlue);
make_write_macro!($ write_dark_blue_ln, true, DarkBlue);

make_write_macro!($ write_dark_magenta, false, DarkMagenta);
make_write_macro!($ write_dark_magenta_ln, true, DarkMagenta);

make_write_macro!($ write_dark_cyan, false, DarkCyan);
make_write_macro!($ write_dark_cyan_ln, true, DarkCyan);

make_write_macro!($ write_grey, false, Grey);
make_write_macro!($ write_grey_ln, true, Grey);

make_write_macro!($ write_dark_grey, false, DarkGrey);
make_write_macro!($ write_dark_grey_ln, true, DarkGrey);

make_write_macro!($ write_red, false, Red);
make_write_macro!($ write_red_ln, true, Red);

make_write_macro!($ write_green, false, Green);
make_write_macro!($ write_green_ln, true, Green);

make_write_macro!($ write_yellow, false, Yellow);
make_write_macro!($ write_yellow_ln, true, Yellow);

make_write_macro!($ write_blue, false, Blue);
make_write_macro!($ write_blue_ln, true, Blue);

make_write_macro!($ write_magenta, false, Magenta);
make_write_macro!($ write_magenta_ln, true, Magenta);

make_write_macro!($ write_cyan, false, Cyan);
make_write_macro!($ write_cyan_ln, true, Cyan);

make_write_macro!($ write_white, false, White);
make_write_macro!($ write_white_ln, true, White);

make_write_macro!($ wrte, false, Reset);
make_write_macro!($ wrte_ln, true, Reset);
