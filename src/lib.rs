#![doc(html_root_url = "https://docs.rs/colour", test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]

/// Helpers and consts which should not be used directly outside of this crate.
#[doc(hidden)]
pub mod internal;

pub use internal::AlreadySet;

/// Inject ANSI colour escape codes into the output, regardless of the value of the `NO_COLOR`
/// environment variable, or whether the output terminal (if any) supports them.
///
/// This should normally only be called by a binary application, not a library.  It should be
/// called before using any of the `print` or `write` macros.
///
/// # Errors
///
/// Returns an error if this library has already been initialised to not inject ANSI colour escape
/// codes, e.g. by calling [`force_no_colour`] of calling one of the macros where the environment
/// was detected to require no coloured output.
pub fn force_colour() -> Result<(), AlreadySet> {
    internal::force(true)
}

/// As per [`force_colour`].
pub fn force_color() -> Result<(), AlreadySet> {
    internal::force(true)
}

/// Do not inject ANSI colour escape codes into the output, regardless of the value of the
/// `CLICOLOR_FORCE` environment variable, or whether the output terminal (if any) supports them.
///
/// This should normally only be called by a binary application, not a library.  It should be
/// called before using any of the `print` or `write` macros.
///
/// # Errors
///
/// Returns an error if this library has already been initialised to inject ANSI colour escape
/// codes, e.g. by calling [`force_colour`] of calling one of the macros where the environment
/// was detected to require coloured output.
pub fn force_no_colour() -> Result<(), AlreadySet> {
    internal::force(false)
}

/// As per [`force_no_colour`].
pub fn force_no_color() -> Result<(), AlreadySet> {
    internal::force(false)
}

#[rustfmt::skip]
macro_rules! make_print_macro {
    ($dollar:tt $name:ident, $colour:ident) => {
        #[macro_export]
        macro_rules! $name {
            ($dollar($dollar args:tt)*) => {{
                if $crate::internal::should_colour() {
                    ::std::print!(
                        "{}{}{}",
                        $crate::internal::$colour,
                        ::core::format_args!($dollar($dollar args)*),
                        $crate::internal::DEFAULT_COLOUR,
                    )
                } else {
                    ::std::print!($dollar($dollar args)*)
                }
            }};
        }
    };
}

#[rustfmt::skip]
macro_rules! make_println_macro {
    ($dollar:tt $name:ident, $colour:ident) => {
        #[macro_export]
        macro_rules! $name {
            () => {
                if $crate::internal::should_colour() {
                    ::std::println!(
                        "{}\n{}",
                        $crate::internal::$colour,
                        $crate::internal::DEFAULT_COLOUR,
                    )
                } else {
                    ::std::println!()
                }
            };
            ($dollar($dollar args:tt)*) => {{
                if $crate::internal::should_colour() {
                    ::std::println!(
                        "{}{}{}",
                        $crate::internal::$colour,
                        ::core::format_args!($dollar($dollar args)*),
                        $crate::internal::DEFAULT_COLOUR,
                    )
                } else {
                    ::std::println!($dollar($dollar args)*)
                }
            }};
        }
    };
}

#[rustfmt::skip]
macro_rules! make_eprint_macro {
    ($dollar:tt $name:ident, $colour:ident) => {
        #[macro_export]
        macro_rules! $name {
            ($dollar($dollar args:tt)*) => {{
                if $crate::internal::should_colour() {
                    ::std::eprint!(
                        "{}{}{}",
                        $crate::internal::$colour,
                        ::core::format_args!($dollar($dollar args)*),
                        $crate::internal::DEFAULT_COLOUR,
                    )
                } else {
                    ::std::eprint!($dollar($dollar args)*)
                }
            }};
        }
    };
}

#[rustfmt::skip]
macro_rules! make_eprintln_macro {
    ($dollar:tt $name:ident, $colour:ident) => {
        #[macro_export]
        macro_rules! $name {
            () => {
                if $crate::internal::should_colour() {
                    ::std::eprintln!(
                        "{}\n{}",
                        $crate::internal::$colour,
                        $crate::internal::DEFAULT_COLOUR,
                    )
                } else {
                    ::std::eprintln!()
                }
            };
            ($dollar($dollar args:tt)*) => {{
                if $crate::internal::should_colour() {
                    ::std::eprintln!(
                        "{}{}{}",
                        $crate::internal::$colour,
                        ::core::format_args!($dollar($dollar args)*),
                        $crate::internal::DEFAULT_COLOUR,
                    )
                } else {
                    ::std::eprintln!($dollar($dollar args)*)
                }
            }};
        }
    };
}

make_print_macro!($ black, BLACK);
make_eprint_macro!($ e_black, BLACK);
make_println_macro!($ black_ln, BLACK);
make_eprintln_macro!($ e_black_ln, BLACK);
make_print_macro!($ black_bold, BOLD_BLACK);
make_eprint_macro!($ e_black_bold, BOLD_BLACK);
make_println_macro!($ black_ln_bold, BOLD_BLACK);
make_eprintln_macro!($ e_black_ln_bold, BOLD_BLACK);

make_print_macro!($ dark_red, DARK_RED);
make_eprint_macro!($ e_dark_red, DARK_RED);
make_println_macro!($ dark_red_ln, DARK_RED);
make_eprintln_macro!($ e_dark_red_ln, DARK_RED);
make_print_macro!($ dark_red_bold, BOLD_DARK_RED);
make_eprint_macro!($ e_dark_red_bold, BOLD_DARK_RED);
make_println_macro!($ dark_red_ln_bold, BOLD_DARK_RED);
make_eprintln_macro!($ e_dark_red_ln_bold, BOLD_DARK_RED);

make_print_macro!($ dark_green, DARK_GREEN);
make_eprint_macro!($ e_dark_green, DARK_GREEN);
make_println_macro!($ dark_green_ln, DARK_GREEN);
make_eprintln_macro!($ e_dark_green_ln, DARK_GREEN);
make_print_macro!($ dark_green_bold, BOLD_DARK_GREEN);
make_eprint_macro!($ e_dark_green_bold, BOLD_DARK_GREEN);
make_println_macro!($ dark_green_ln_bold, BOLD_DARK_GREEN);
make_eprintln_macro!($ e_dark_green_ln_bold, BOLD_DARK_GREEN);

make_print_macro!($ dark_yellow, DARK_YELLOW);
make_eprint_macro!($ e_dark_yellow, DARK_YELLOW);
make_println_macro!($ dark_yellow_ln, DARK_YELLOW);
make_eprintln_macro!($ e_dark_yellow_ln, DARK_YELLOW);
make_print_macro!($ dark_yellow_bold, BOLD_DARK_YELLOW);
make_eprint_macro!($ e_dark_yellow_bold, BOLD_DARK_YELLOW);
make_println_macro!($ dark_yellow_ln_bold, BOLD_DARK_YELLOW);
make_eprintln_macro!($ e_dark_yellow_ln_bold, BOLD_DARK_YELLOW);

make_print_macro!($ dark_blue, DARK_BLUE);
make_eprint_macro!($ e_dark_blue, DARK_BLUE);
make_println_macro!($ dark_blue_ln, DARK_BLUE);
make_eprintln_macro!($ e_dark_blue_ln, DARK_BLUE);
make_print_macro!($ dark_blue_bold, BOLD_DARK_BLUE);
make_eprint_macro!($ e_dark_blue_bold, BOLD_DARK_BLUE);
make_println_macro!($ dark_blue_ln_bold, BOLD_DARK_BLUE);
make_eprintln_macro!($ e_dark_blue_ln_bold, BOLD_DARK_BLUE);

make_print_macro!($ dark_magenta, DARK_MAGENTA);
make_eprint_macro!($ e_dark_magenta, DARK_MAGENTA);
make_println_macro!($ dark_magenta_ln, DARK_MAGENTA);
make_eprintln_macro!($ e_dark_magenta_ln, DARK_MAGENTA);
make_print_macro!($ dark_magenta_bold, BOLD_DARK_MAGENTA);
make_eprint_macro!($ e_dark_magenta_bold, BOLD_DARK_MAGENTA);
make_println_macro!($ dark_magenta_ln_bold, BOLD_DARK_MAGENTA);
make_eprintln_macro!($ e_dark_magenta_ln_bold, BOLD_DARK_MAGENTA);

make_print_macro!($ dark_cyan, DARK_CYAN);
make_eprint_macro!($ e_dark_cyan, DARK_CYAN);
make_println_macro!($ dark_cyan_ln, DARK_CYAN);
make_eprintln_macro!($ e_dark_cyan_ln, DARK_CYAN);
make_print_macro!($ dark_cyan_bold, BOLD_DARK_CYAN);
make_eprint_macro!($ e_dark_cyan_bold, BOLD_DARK_CYAN);
make_println_macro!($ dark_cyan_ln_bold, BOLD_DARK_CYAN);
make_eprintln_macro!($ e_dark_cyan_ln_bold, BOLD_DARK_CYAN);

make_print_macro!($ grey, GREY);
make_eprint_macro!($ e_grey, GREY);
make_println_macro!($ grey_ln, GREY);
make_eprintln_macro!($ e_grey_ln, GREY);
make_print_macro!($ grey_bold, BOLD_GREY);
make_eprint_macro!($ e_grey_bold, BOLD_GREY);
make_println_macro!($ grey_ln_bold, BOLD_GREY);
make_eprintln_macro!($ e_grey_ln_bold, BOLD_GREY);

make_print_macro!($ gray, GREY);
make_eprint_macro!($ e_gray, GREY);
make_println_macro!($ gray_ln, GREY);
make_eprintln_macro!($ e_gray_ln, GREY);
make_print_macro!($ gray_bold, BOLD_GREY);
make_eprint_macro!($ e_gray_bold, BOLD_GREY);
make_println_macro!($ gray_ln_bold, BOLD_GREY);
make_eprintln_macro!($ e_gray_ln_bold, BOLD_GREY);

make_print_macro!($ dark_grey, DARK_GREY);
make_eprint_macro!($ e_dark_grey, DARK_GREY);
make_println_macro!($ dark_grey_ln, DARK_GREY);
make_eprintln_macro!($ e_dark_grey_ln, DARK_GREY);
make_print_macro!($ dark_grey_bold, BOLD_DARK_GREY);
make_eprint_macro!($ e_dark_grey_bold, BOLD_DARK_GREY);
make_println_macro!($ dark_grey_ln_bold, BOLD_DARK_GREY);
make_eprintln_macro!($ e_dark_grey_ln_bold, BOLD_DARK_GREY);

make_print_macro!($ dark_gray, DARK_GREY);
make_eprint_macro!($ e_dark_gray, DARK_GREY);
make_println_macro!($ dark_gray_ln, DARK_GREY);
make_eprintln_macro!($ e_dark_gray_ln, DARK_GREY);
make_print_macro!($ dark_gray_bold, BOLD_DARK_GREY);
make_eprint_macro!($ e_dark_gray_bold, BOLD_DARK_GREY);
make_println_macro!($ dark_gray_ln_bold, BOLD_DARK_GREY);
make_eprintln_macro!($ e_dark_gray_ln_bold, BOLD_DARK_GREY);

make_print_macro!($ red, RED);
make_eprint_macro!($ e_red, RED);
make_println_macro!($ red_ln, RED);
make_eprintln_macro!($ e_red_ln, RED);
make_print_macro!($ red_bold, BOLD_RED);
make_eprint_macro!($ e_red_bold, BOLD_RED);
make_println_macro!($ red_ln_bold, BOLD_RED);
make_eprintln_macro!($ e_red_ln_bold, BOLD_RED);

make_print_macro!($ green, GREEN);
make_eprint_macro!($ e_green, GREEN);
make_println_macro!($ green_ln, GREEN);
make_eprintln_macro!($ e_green_ln, GREEN);
make_print_macro!($ green_bold, BOLD_GREEN);
make_eprint_macro!($ e_green_bold, BOLD_GREEN);
make_println_macro!($ green_ln_bold, BOLD_GREEN);
make_eprintln_macro!($ e_green_ln_bold, BOLD_GREEN);

make_print_macro!($ yellow, YELLOW);
make_eprint_macro!($ e_yellow, YELLOW);
make_println_macro!($ yellow_ln, YELLOW);
make_eprintln_macro!($ e_yellow_ln, YELLOW);
make_print_macro!($ yellow_bold, BOLD_YELLOW);
make_eprint_macro!($ e_yellow_bold, BOLD_YELLOW);
make_println_macro!($ yellow_ln_bold, BOLD_YELLOW);
make_eprintln_macro!($ e_yellow_ln_bold, BOLD_YELLOW);

make_print_macro!($ blue, BLUE);
make_eprint_macro!($ e_blue, BLUE);
make_println_macro!($ blue_ln, BLUE);
make_eprintln_macro!($ e_blue_ln, BLUE);
make_print_macro!($ blue_bold, BOLD_BLUE);
make_eprint_macro!($ e_blue_bold, BOLD_BLUE);
make_println_macro!($ blue_ln_bold, BOLD_BLUE);
make_eprintln_macro!($ e_blue_ln_bold, BOLD_BLUE);

make_print_macro!($ magenta, MAGENTA);
make_eprint_macro!($ e_magenta, MAGENTA);
make_println_macro!($ magenta_ln, MAGENTA);
make_eprintln_macro!($ e_magenta_ln, MAGENTA);
make_print_macro!($ magenta_bold, BOLD_MAGENTA);
make_eprint_macro!($ e_magenta_bold, BOLD_MAGENTA);
make_println_macro!($ magenta_ln_bold, BOLD_MAGENTA);
make_eprintln_macro!($ e_magenta_ln_bold, BOLD_MAGENTA);

make_print_macro!($ cyan, CYAN);
make_eprint_macro!($ e_cyan, CYAN);
make_println_macro!($ cyan_ln, CYAN);
make_eprintln_macro!($ e_cyan_ln, CYAN);
make_print_macro!($ cyan_bold, BOLD_CYAN);
make_eprint_macro!($ e_cyan_bold, BOLD_CYAN);
make_println_macro!($ cyan_ln_bold, BOLD_CYAN);
make_eprintln_macro!($ e_cyan_ln_bold, BOLD_CYAN);

make_print_macro!($ white, WHITE);
make_eprint_macro!($ e_white, WHITE);
make_println_macro!($ white_ln, WHITE);
make_eprintln_macro!($ e_white_ln, WHITE);
make_print_macro!($ white_bold, BOLD_WHITE);
make_eprint_macro!($ e_white_bold, BOLD_WHITE);
make_println_macro!($ white_ln_bold, BOLD_WHITE);
make_eprintln_macro!($ e_white_ln_bold, BOLD_WHITE);

make_print_macro!($ print_bold, BOLD_DEFAULT_COLOUR);
make_eprint_macro!($ eprint_bold, BOLD_DEFAULT_COLOUR);
make_eprint_macro!($ e_print_bold, BOLD_DEFAULT_COLOUR);
make_println_macro!($ println_bold, BOLD_DEFAULT_COLOUR);
make_println_macro!($ print_ln_bold, BOLD_DEFAULT_COLOUR);
make_eprintln_macro!($ eprintln_bold, BOLD_DEFAULT_COLOUR);
make_eprintln_macro!($ e_print_ln_bold, BOLD_DEFAULT_COLOUR);

#[rustfmt::skip]
macro_rules! make_write_macro {
    ($dollar:tt $name:ident, $colour:ident) => {
        #[macro_export]
        macro_rules! $name {
            ($dollar dst:expr, $dollar($dollar args:tt)*) => {
                if $crate::internal::should_colour() {
                    ::core::write!(
                        $dollar dst,
                        "{}{}{}",
                        $crate::internal::$colour,
                        ::core::format_args!($dollar($dollar args)*),
                        $crate::internal::DEFAULT_COLOUR,
                    )
                } else {
                    ::core::write!($dollar dst, $dollar($dollar args)*)
                }
            };
        }
    }
}

#[rustfmt::skip]
macro_rules! make_writeln_macro {
    ($dollar:tt $name:ident, $colour:ident) => {
        #[macro_export]
        macro_rules! $name {
            ($dollar dst:expr, $dollar($dollar args:tt)*) => {
                if $crate::internal::should_colour() {
                    ::core::writeln!(
                        $dollar dst,
                        "{}{}{}",
                        $crate::internal::$colour,
                        ::core::format_args!($dollar($dollar args)*),
                        $crate::internal::DEFAULT_COLOUR,
                    )
                } else {
                    ::core::writeln!($dollar dst, $dollar($dollar args)*)
                }
            };
        }
    }
}

make_write_macro!($ write_black, BLACK);
make_writeln_macro!($ writeln_black, BLACK);
make_write_macro!($ write_black_bold, BOLD_BLACK);
make_writeln_macro!($ writeln_black_bold, BOLD_BLACK);

make_write_macro!($ write_dark_red, DARK_RED);
make_writeln_macro!($ writeln_dark_red, DARK_RED);
make_write_macro!($ write_dark_red_bold, BOLD_DARK_RED);
make_writeln_macro!($ writeln_dark_red_bold, BOLD_DARK_RED);

make_write_macro!($ write_dark_green, DARK_GREEN);
make_writeln_macro!($ writeln_dark_green, DARK_GREEN);
make_write_macro!($ write_dark_green_bold, BOLD_DARK_GREEN);
make_writeln_macro!($ writeln_dark_green_bold, BOLD_DARK_GREEN);

make_write_macro!($ write_dark_yellow, DARK_YELLOW);
make_writeln_macro!($ writeln_dark_yellow, DARK_YELLOW);
make_write_macro!($ write_dark_yellow_bold, BOLD_DARK_YELLOW);
make_writeln_macro!($ writeln_dark_yellow_bold, BOLD_DARK_YELLOW);

make_write_macro!($ write_dark_blue, DARK_BLUE);
make_writeln_macro!($ writeln_dark_blue, DARK_BLUE);
make_write_macro!($ write_dark_blue_bold, BOLD_DARK_BLUE);
make_writeln_macro!($ writeln_dark_blue_bold, BOLD_DARK_BLUE);

make_write_macro!($ write_dark_magenta, DARK_MAGENTA);
make_writeln_macro!($ writeln_dark_magenta, DARK_MAGENTA);
make_write_macro!($ write_dark_magenta_bold, BOLD_DARK_MAGENTA);
make_writeln_macro!($ writeln_dark_magenta_bold, BOLD_DARK_MAGENTA);

make_write_macro!($ write_dark_cyan, DARK_CYAN);
make_writeln_macro!($ writeln_dark_cyan, DARK_CYAN);
make_write_macro!($ write_dark_cyan_bold, BOLD_DARK_CYAN);
make_writeln_macro!($ writeln_dark_cyan_bold, BOLD_DARK_CYAN);

make_write_macro!($ write_dark_grey, DARK_GREY);
make_writeln_macro!($ writeln_dark_grey, DARK_GREY);
make_write_macro!($ write_dark_grey_bold, BOLD_DARK_GREY);
make_writeln_macro!($ writeln_dark_grey_bold, BOLD_DARK_GREY);

make_write_macro!($ write_dark_gray, DARK_GREY);
make_writeln_macro!($ writeln_dark_gray, DARK_GREY);
make_write_macro!($ write_dark_gray_bold, BOLD_DARK_GREY);
make_writeln_macro!($ writeln_dark_gray_bold, BOLD_DARK_GREY);

make_write_macro!($ write_grey, GREY);
make_writeln_macro!($ writeln_grey, GREY);
make_write_macro!($ write_grey_bold, BOLD_GREY);
make_writeln_macro!($ writeln_grey_bold, BOLD_GREY);

make_write_macro!($ write_gray, GREY);
make_writeln_macro!($ writeln_gray, GREY);
make_write_macro!($ write_gray_bold, BOLD_GREY);
make_writeln_macro!($ writeln_gray_bold, BOLD_GREY);

make_write_macro!($ write_red, RED);
make_writeln_macro!($ writeln_red, RED);
make_write_macro!($ write_red_bold, BOLD_RED);
make_writeln_macro!($ writeln_red_bold, BOLD_RED);

make_write_macro!($ write_green, GREEN);
make_writeln_macro!($ writeln_green, GREEN);
make_write_macro!($ write_green_bold, BOLD_GREEN);
make_writeln_macro!($ writeln_green_bold, BOLD_GREEN);

make_write_macro!($ write_yellow, YELLOW);
make_writeln_macro!($ writeln_yellow, YELLOW);
make_write_macro!($ write_yellow_bold, BOLD_YELLOW);
make_writeln_macro!($ writeln_yellow_bold, BOLD_YELLOW);

make_write_macro!($ write_blue, BLUE);
make_writeln_macro!($ writeln_blue, BLUE);
make_write_macro!($ write_blue_bold, BOLD_BLUE);
make_writeln_macro!($ writeln_blue_bold, BOLD_BLUE);

make_write_macro!($ write_magenta, MAGENTA);
make_writeln_macro!($ writeln_magenta, MAGENTA);
make_write_macro!($ write_magenta_bold, BOLD_MAGENTA);
make_writeln_macro!($ writeln_magenta_bold, BOLD_MAGENTA);

make_write_macro!($ write_cyan, CYAN);
make_writeln_macro!($ writeln_cyan, CYAN);
make_write_macro!($ write_cyan_bold, BOLD_CYAN);
make_writeln_macro!($ writeln_cyan_bold, BOLD_CYAN);

make_write_macro!($ write_white, WHITE);
make_writeln_macro!($ writeln_white, WHITE);
make_write_macro!($ write_white_bold, BOLD_WHITE);
make_writeln_macro!($ writeln_white_bold, BOLD_WHITE);

make_write_macro!($ write_bold, BOLD_DEFAULT_COLOUR);
make_writeln_macro!($ writeln_bold, BOLD_DEFAULT_COLOUR);
