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
//! There are dark versions of each colour, and for all colours there is version with suffix `_ln`
//! which appends a newline (similar to [`print!`](https://doc.servo.org/std/macro.print.html) and
//! [`println!`](https://doc.servo.org/std/macro.println.html)).
//!
//! There are also `prnt!` and `prnt_ln!` available which print using the current default foreground
//! colour.
//!
//! Every variant also has a version with prefix `e_`.  Variants with this prefix output to stderr,
//! while those without output to stdout.

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

/// Helpers and consts which should not be used directly outside of this crate.
#[doc(hidden)]
pub mod unnamed;

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using the current
/// default foreground colour.
#[macro_export]
macro_rules! prnt_ln {
    ($arg:tt) => {
        $crate::unnamed::write(None, $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(None, &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using grey as the
/// foreground colour.
#[macro_export]
macro_rules! grey_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::Grey), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::Grey), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using red as the
/// foreground colour.
#[macro_export]
macro_rules! red_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::Red), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::Red), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using green as the
/// foreground colour.
#[macro_export]
macro_rules! green_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::Green), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::Green), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using yellow as the
/// foreground colour.
#[macro_export]
macro_rules! yellow_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::Yellow), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::Yellow), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using blue as the
/// foreground colour.
#[macro_export]
macro_rules! blue_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::Blue), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::Blue), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using magenta as the
/// foreground colour.
#[macro_export]
macro_rules! magenta_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::Magenta), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::Magenta), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using cyan as the
/// foreground colour.
#[macro_export]
macro_rules! cyan_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::Cyan), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::Cyan), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using white as the
/// foreground colour.
#[macro_export]
macro_rules! white_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::White), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::White), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using black as the
/// foreground colour.
#[macro_export]
macro_rules! black_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::Black), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::Black), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using dark grey as
/// the foreground colour.
#[macro_export]
macro_rules! dark_grey_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkGrey), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkGrey), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using dark red as
/// the foreground colour.
#[macro_export]
macro_rules! dark_red_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkRed), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkRed), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using dark green as
/// the foreground colour.
#[macro_export]
macro_rules! dark_green_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkGreen), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkGreen), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using dark yellow as
/// the foreground colour.
#[macro_export]
macro_rules! dark_yellow_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkYellow), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkYellow), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using dark blue as
/// the foreground colour.
#[macro_export]
macro_rules! dark_blue_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkBlue), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkBlue), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using dark magenta
/// as the foreground colour.
#[macro_export]
macro_rules! dark_magenta_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkMagenta), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkMagenta), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using dark cyan as
/// the foreground colour.
#[macro_export]
macro_rules! dark_cyan_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkCyan), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkCyan), &format!($($arg)*), true);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using the current
/// default foreground colour.
#[macro_export]
macro_rules! prnt {
    ($arg:tt) => {
        $crate::unnamed::write(None, $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(None, &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using grey as the
/// foreground colour.
#[macro_export]
macro_rules! grey {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::Grey), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::Grey), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using red as the
/// foreground colour.
#[macro_export]
macro_rules! red {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::Red), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::Red), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using green as the
/// foreground colour.
#[macro_export]
macro_rules! green {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::Green), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::Green), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using yellow as the
/// foreground colour.
#[macro_export]
macro_rules! yellow {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::Yellow), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::Yellow), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using blue as the
/// foreground colour.
#[macro_export]
macro_rules! blue {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::Blue), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::Blue), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using magenta as the
/// foreground colour.
#[macro_export]
macro_rules! magenta {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::Magenta), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::Magenta), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using cyan as the
/// foreground colour.
#[macro_export]
macro_rules! cyan {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::Cyan), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::Cyan), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using white as the
/// foreground colour.
#[macro_export]
macro_rules! white {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::White), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::White), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using black as the
/// foreground colour.
#[macro_export]
macro_rules! black {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::Black), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::Black), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using dark grey as the
/// foreground colour.
#[macro_export]
macro_rules! dark_grey {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkGrey), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkGrey), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using dark red as the
/// foreground colour.
#[macro_export]
macro_rules! dark_red {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkRed), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkRed), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using dark green as the
/// foreground colour.
#[macro_export]
macro_rules! dark_green {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkGreen), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkGreen), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using dark yellow as the
/// foreground colour.
#[macro_export]
macro_rules! dark_yellow {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkYellow), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkYellow), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using dark blue as the
/// foreground colour.
#[macro_export]
macro_rules! dark_blue {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkBlue), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkBlue), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using dark magenta as
/// the foreground colour.
#[macro_export]
macro_rules! dark_magenta {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkMagenta), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkMagenta), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using dark cyan as the
/// foreground colour.
#[macro_export]
macro_rules! dark_cyan {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkCyan), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::write(Some($crate::unnamed::Colour::DarkCyan), &format!($($arg)*), false);
    };
}

/// Macro similar to [`eprintln!`](https://doc.servo.org/std/macro.eprintln.html) using the current
/// default foreground colour.
#[macro_export]
macro_rules! e_prnt_ln {
    ($arg:tt) => {
        $crate::unnamed::ewrite(None, $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(None, &format!($($arg)*), true);
    };
}

/// Macro similar to [`eprintln!`](https://doc.servo.org/std/macro.eprintln.html) using grey as the
/// foreground colour.
#[macro_export]
macro_rules! e_grey_ln {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Grey), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Grey), &format!($($arg)*), true);
    };
}

/// Macro similar to [`eprintln!`](https://doc.servo.org/std/macro.eprintln.html) using red as the
/// foreground colour.
#[macro_export]
macro_rules! e_red_ln {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Red), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Red), &format!($($arg)*), true);
    };
}

/// Macro similar to [`eprintln!`](https://doc.servo.org/std/macro.eprintln.html) using green as the
/// foreground colour.
#[macro_export]
macro_rules! e_green_ln {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Green), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Green), &format!($($arg)*), true);
    };
}

/// Macro similar to [`eprintln!`](https://doc.servo.org/std/macro.eprintln.html) using yellow as
/// the foreground colour.
#[macro_export]
macro_rules! e_yellow_ln {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Yellow), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Yellow), &format!($($arg)*), true);
    };
}

/// Macro similar to [`eprintln!`](https://doc.servo.org/std/macro.eprintln.html) using blue as the
/// foreground colour.
#[macro_export]
macro_rules! e_blue_ln {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Blue), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Blue), &format!($($arg)*), true);
    };
}

/// Macro similar to [`eprintln!`](https://doc.servo.org/std/macro.eprintln.html) using magenta as
/// the foreground colour.
#[macro_export]
macro_rules! e_magenta_ln {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Magenta), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Magenta), &format!($($arg)*), true);
    };
}

/// Macro similar to [`eprintln!`](https://doc.servo.org/std/macro.eprintln.html) using cyan as the
/// foreground colour.
#[macro_export]
macro_rules! e_cyan_ln {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Cyan), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Cyan), &format!($($arg)*), true);
    };
}

/// Macro similar to [`eprintln!`](https://doc.servo.org/std/macro.eprintln.html) using white as the
/// foreground colour.
#[macro_export]
macro_rules! e_white_ln {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::White), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::White), &format!($($arg)*), true);
    };
}

/// Macro similar to [`eprintln!`](https://doc.servo.org/std/macro.eprintln.html) using black as the
/// foreground colour.
#[macro_export]
macro_rules! e_black_ln {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Black), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Black), &format!($($arg)*), true);
    };
}

/// Macro similar to [`eprintln!`](https://doc.servo.org/std/macro.eprintln.html) using dark grey as
/// the foreground colour.
#[macro_export]
macro_rules! e_dark_grey_ln {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkGrey), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkGrey), &format!($($arg)*), true);
    };
}

/// Macro similar to [`eprintln!`](https://doc.servo.org/std/macro.eprintln.html) using dark red as
/// the foreground colour.
#[macro_export]
macro_rules! e_dark_red_ln {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkRed), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkRed), &format!($($arg)*), true);
    };
}

/// Macro similar to [`eprintln!`](https://doc.servo.org/std/macro.eprintln.html) using dark green
/// as the foreground colour.
#[macro_export]
macro_rules! e_dark_green_ln {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkGreen), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkGreen), &format!($($arg)*), true);
    };
}

/// Macro similar to [`eprintln!`](https://doc.servo.org/std/macro.eprintln.html) using dark yellow
/// as the foreground colour.
#[macro_export]
macro_rules! e_dark_yellow_ln {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkYellow), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkYellow), &format!($($arg)*), true);
    };
}

/// Macro similar to [`eprintln!`](https://doc.servo.org/std/macro.eprintln.html) using dark blue as
/// the foreground colour.
#[macro_export]
macro_rules! e_dark_blue_ln {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkBlue), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkBlue), &format!($($arg)*), true);
    };
}

/// Macro similar to [`eprintln!`](https://doc.servo.org/std/macro.eprintln.html) using dark magenta
/// as the foreground colour.
#[macro_export]
macro_rules! e_dark_magenta_ln {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkMagenta), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkMagenta), &format!($($arg)*), true);
    };
}

/// Macro similar to [`eprintln!`](https://doc.servo.org/std/macro.eprintln.html) using dark cyan as
/// the foreground colour.
#[macro_export]
macro_rules! e_dark_cyan_ln {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkCyan), $arg, true);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkCyan), &format!($($arg)*), true);
    };
}

/// Macro similar to [`eprint!`](https://doc.servo.org/std/macro.eprint.html) using the current
/// default foreground colour.
#[macro_export]
macro_rules! e_prnt {
    ($arg:tt) => {
        $crate::unnamed::ewrite(None, $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(None, &format!($($arg)*), false);
    };
}

/// Macro similar to [`eprint!`](https://doc.servo.org/std/macro.eprint.html) using grey as the
/// foreground colour.
#[macro_export]
macro_rules! e_grey {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Grey), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Grey), &format!($($arg)*), false);
    };
}

/// Macro similar to [`eprint!`](https://doc.servo.org/std/macro.eprint.html) using red as the
/// foreground colour.
#[macro_export]
macro_rules! e_red {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Red), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Red), &format!($($arg)*), false);
    };
}

/// Macro similar to [`eprint!`](https://doc.servo.org/std/macro.eprint.html) using green as the
/// foreground colour.
#[macro_export]
macro_rules! e_green {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Green), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Green), &format!($($arg)*), false);
    };
}

/// Macro similar to [`eprint!`](https://doc.servo.org/std/macro.eprint.html) using yellow as the
/// foreground colour.
#[macro_export]
macro_rules! e_yellow {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Yellow), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Yellow), &format!($($arg)*), false);
    };
}

/// Macro similar to [`eprint!`](https://doc.servo.org/std/macro.eprint.html) using blue as the
/// foreground colour.
#[macro_export]
macro_rules! e_blue {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Blue), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Blue), &format!($($arg)*), false);
    };
}

/// Macro similar to [`eprint!`](https://doc.servo.org/std/macro.eprint.html) using magenta as the
/// foreground colour.
#[macro_export]
macro_rules! e_magenta {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Magenta), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Magenta), &format!($($arg)*), false);
    };
}

/// Macro similar to [`eprint!`](https://doc.servo.org/std/macro.eprint.html) using cyan as the
/// foreground colour.
#[macro_export]
macro_rules! e_cyan {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Cyan), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Cyan), &format!($($arg)*), false);
    };
}

/// Macro similar to [`eprint!`](https://doc.servo.org/std/macro.eprint.html) using white as the
/// foreground colour.
#[macro_export]
macro_rules! e_white {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::White), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::White), &format!($($arg)*), false);
    };
}

/// Macro similar to [`eprint!`](https://doc.servo.org/std/macro.eprint.html) using black as the
/// foreground colour.
#[macro_export]
macro_rules! e_black {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Black), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::Black), &format!($($arg)*), false);
    };
}

/// Macro similar to [`eprint!`](https://doc.servo.org/std/macro.eprint.html) using dark grey as the
/// foreground colour.
#[macro_export]
macro_rules! e_dark_grey {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkGrey), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkGrey), &format!($($arg)*), false);
    };
}

/// Macro similar to [`eprint!`](https://doc.servo.org/std/macro.eprint.html) using dark red as the
/// foreground colour.
#[macro_export]
macro_rules! e_dark_red {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkRed), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkRed), &format!($($arg)*), false);
    };
}

/// Macro similar to [`eprint!`](https://doc.servo.org/std/macro.eprint.html) using dark green as
/// the foreground colour.
#[macro_export]
macro_rules! e_dark_green {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkGreen), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkGreen), &format!($($arg)*), false);
    };
}

/// Macro similar to [`eprint!`](https://doc.servo.org/std/macro.eprint.html) using dark yellow as
/// the foreground colour.
#[macro_export]
macro_rules! e_dark_yellow {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkYellow), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkYellow), &format!($($arg)*), false);
    };
}

/// Macro similar to [`eprint!`](https://doc.servo.org/std/macro.eprint.html) using dark blue as the
/// foreground colour.
#[macro_export]
macro_rules! e_dark_blue {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkBlue), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkBlue), &format!($($arg)*), false);
    };
}

/// Macro similar to [`eprint!`](https://doc.servo.org/std/macro.eprint.html) using dark magenta as
/// the foreground colour.
#[macro_export]
macro_rules! e_dark_magenta {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkMagenta), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkMagenta), &format!($($arg)*), false);
    };
}

/// Macro similar to [`eprint!`](https://doc.servo.org/std/macro.eprint.html) using dark cyan as the
/// foreground colour.
#[macro_export]
macro_rules! e_dark_cyan {
    ($arg:tt) => {
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkCyan), $arg, false);
    };
    ($($arg:tt)*) => {
        #[allow(clippy::useless_format)]
        $crate::unnamed::ewrite(Some($crate::unnamed::Colour::DarkCyan), &format!($($arg)*), false);
    };
}
