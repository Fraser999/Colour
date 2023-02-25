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
//! There are dark versions of each colour except black and white, and for all colours there is a
//! version with suffix `_ln` which appends a newline (similar to [`print!`] and [`println!`]).
//!
//! There are also `prnt!` and `prnt_ln!` available which print using the current default foreground
//! colour.
//!
//! Every variant also has a version with prefix `e_`.  Variants with this prefix output to stderr,
//! while those without the `e_` prefix output to stdout.

/// Helpers and consts which should not be used directly outside of this crate.
#[doc(hidden)]
pub mod internal;

#[rustfmt::skip]
macro_rules! make_macro {
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
            ($dollar($dollar args:expr),*) => {
                $crate::internal::write::<_, $newline>(
                    std::io::$output().lock(),
                    stringify!($output),
                    $crate::internal::Colour::$colour,
                    Some(format_args!($dollar($dollar args),*))
                )
            };
        }
    };
}

make_macro!($ black, false, Black, stdout);
make_macro!($ e_black, false, Black, stderr);
make_macro!($ black_ln, true, Black, stdout);
make_macro!($ e_black_ln, true, Black, stderr);

make_macro!($ dark_red, false, DarkRed, stdout);
make_macro!($ e_dark_red, false, DarkRed, stderr);
make_macro!($ dark_red_ln, true, DarkRed, stdout);
make_macro!($ e_dark_red_ln, true, DarkRed, stderr);

make_macro!($ dark_green, false, DarkGreen, stdout);
make_macro!($ e_dark_green, false, DarkGreen, stderr);
make_macro!($ dark_green_ln, true, DarkGreen, stdout);
make_macro!($ e_dark_green_ln, true, DarkGreen, stderr);

make_macro!($ dark_yellow, false, DarkYellow, stdout);
make_macro!($ e_dark_yellow, false, DarkYellow, stderr);
make_macro!($ dark_yellow_ln, true, DarkYellow, stdout);
make_macro!($ e_dark_yellow_ln, true, DarkYellow, stderr);

make_macro!($ dark_blue, false, DarkBlue, stdout);
make_macro!($ e_dark_blue, false, DarkBlue, stderr);
make_macro!($ dark_blue_ln, true, DarkBlue, stdout);
make_macro!($ e_dark_blue_ln, true, DarkBlue, stderr);

make_macro!($ dark_magenta, false, DarkMagenta, stdout);
make_macro!($ e_dark_magenta, false, DarkMagenta, stderr);
make_macro!($ dark_magenta_ln, true, DarkMagenta, stdout);
make_macro!($ e_dark_magenta_ln, true, DarkMagenta, stderr);

make_macro!($ dark_cyan, false, DarkCyan, stdout);
make_macro!($ e_dark_cyan, false, DarkCyan, stderr);
make_macro!($ dark_cyan_ln, true, DarkCyan, stdout);
make_macro!($ e_dark_cyan_ln, true, DarkCyan, stderr);

make_macro!($ grey, false, Grey, stdout);
make_macro!($ e_grey, false, Grey, stderr);
make_macro!($ grey_ln, true, Grey, stdout);
make_macro!($ e_grey_ln, true, Grey, stderr);

make_macro!($ dark_grey, false, DarkGrey, stdout);
make_macro!($ e_dark_grey, false, DarkGrey, stderr);
make_macro!($ dark_grey_ln, true, DarkGrey, stdout);
make_macro!($ e_dark_grey_ln, true, DarkGrey, stderr);

make_macro!($ red, false, Red, stdout);
make_macro!($ e_red, false, Red, stderr);
make_macro!($ red_ln, true, Red, stdout);
make_macro!($ e_red_ln, true, Red, stderr);

make_macro!($ green, false, Green, stdout);
make_macro!($ e_green, false, Green, stderr);
make_macro!($ green_ln, true, Green, stdout);
make_macro!($ e_green_ln, true, Green, stderr);

make_macro!($ yellow, false, Yellow, stdout);
make_macro!($ e_yellow, false, Yellow, stderr);
make_macro!($ yellow_ln, true, Yellow, stdout);
make_macro!($ e_yellow_ln, true, Yellow, stderr);

make_macro!($ blue, false, Blue, stdout);
make_macro!($ e_blue, false, Blue, stderr);
make_macro!($ blue_ln, true, Blue, stdout);
make_macro!($ e_blue_ln, true, Blue, stderr);

make_macro!($ magenta, false, Magenta, stdout);
make_macro!($ e_magenta, false, Magenta, stderr);
make_macro!($ magenta_ln, true, Magenta, stdout);
make_macro!($ e_magenta_ln, true, Magenta, stderr);

make_macro!($ cyan, false, Cyan, stdout);
make_macro!($ e_cyan, false, Cyan, stderr);
make_macro!($ cyan_ln, true, Cyan, stdout);
make_macro!($ e_cyan_ln, true, Cyan, stderr);

make_macro!($ white, false, White, stdout);
make_macro!($ e_white, false, White, stderr);
make_macro!($ white_ln, true, White, stdout);
make_macro!($ e_white_ln, true, White, stderr);

make_macro!($ prnt, false, Reset, stdout);
make_macro!($ e_prnt, false, Reset, stderr);
make_macro!($ prnt_ln, true, Reset, stdout);
make_macro!($ e_prnt_ln, true, Reset, stderr);
