//! Macros for creating coloured console output.
//!
//! The macros are based on the following colours:
//!
//! * black
//! * red
//! * green
//! * yellow
//! * blue
//! * magenta
//! * cyan
//! * white
//!
//! There are dark versions of each colour, and for all colours there is version with suffix `_ln`
//! which appends a newline (similar to [`print!`](https://doc.servo.org/std/macro.print.html) and
//! [`println!`](https://doc.servo.org/std/macro.println.html)).
//!
//! There are also `prnt!` and `prnt_ln!` available which print using the current default foreground
//! colour.
//!
//! Internally, the macros use a global static handle to stdout which is locked on each call.  At
//! the end of each call, stdout is flushed.  This results in slower performance compared to
//! `print!` or `println!`, but avoids interleaving of the output in a multi-threaded environment.

#![forbid(warnings)]
#![warn(missing_copy_implementations, trivial_casts, trivial_numeric_casts, unsafe_code,
        unused_extern_crates, unused_import_braces, unused_qualifications, unused_results,
        variant_size_differences)]
#![cfg_attr(feature="cargo-clippy", deny(clippy, clippy_pedantic))]

#[macro_use]
extern crate lazy_static;
extern crate term;

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
        $crate::unnamed::write(None, &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using black as the
/// foreground colour.
#[macro_export]
macro_rules! black_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::BLACK), $arg, true);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::BLACK), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using red as the
/// foreground colour.
#[macro_export]
macro_rules! red_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::RED), $arg, true);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::RED), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using green as the
/// foreground colour.
#[macro_export]
macro_rules! green_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::GREEN), $arg, true);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::GREEN), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using yellow as the
/// foreground colour.
#[macro_export]
macro_rules! yellow_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::YELLOW), $arg, true);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::YELLOW), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using blue as the
/// foreground colour.
#[macro_export]
macro_rules! blue_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::BLUE), $arg, true);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::BLUE), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using magenta as the
/// foreground colour.
#[macro_export]
macro_rules! magenta_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::MAGENTA), $arg, true);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::MAGENTA), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using cyan as the
/// foreground colour.
#[macro_export]
macro_rules! cyan_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::CYAN), $arg, true);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::CYAN), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using white as the
/// foreground colour.
#[macro_export]
macro_rules! white_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::WHITE), $arg, true);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::WHITE), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using dark black as
/// the foreground colour.
#[macro_export]
macro_rules! dark_black_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_BLACK), $arg, true);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_BLACK), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using dark red as
/// the foreground colour.
#[macro_export]
macro_rules! dark_red_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_RED), $arg, true);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_RED), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using dark green as
/// the foreground colour.
#[macro_export]
macro_rules! dark_green_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_GREEN), $arg, true);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_GREEN), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using dark yellow as
/// the foreground colour.
#[macro_export]
macro_rules! dark_yellow_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_YELLOW), $arg, true);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_YELLOW), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using dark blue as
/// the foreground colour.
#[macro_export]
macro_rules! dark_blue_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_BLUE), $arg, true);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_BLUE), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using dark magenta
/// as the foreground colour.
#[macro_export]
macro_rules! dark_magenta_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_MAGENTA), $arg, true);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_MAGENTA), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using dark cyan as
/// the foreground colour.
#[macro_export]
macro_rules! dark_cyan_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_CYAN), $arg, true);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_CYAN), &format!($($arg)*), true);
    };
}

/// Macro similar to [`println!`](https://doc.servo.org/std/macro.println.html) using dark white as
/// the foreground colour.
#[macro_export]
macro_rules! dark_white_ln {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_WHITE), $arg, true);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_WHITE), &format!($($arg)*), true);
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
        $crate::unnamed::write(None, &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using black as the
/// foreground colour.
#[macro_export]
macro_rules! black {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::BLACK), $arg, false);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::BLACK), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using red as the
/// foreground colour.
#[macro_export]
macro_rules! red {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::RED), $arg, false);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::RED), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using green as the
/// foreground colour.
#[macro_export]
macro_rules! green {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::GREEN), $arg, false);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::GREEN), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using yellow as the
/// foreground colour.
#[macro_export]
macro_rules! yellow {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::YELLOW), $arg, false);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::YELLOW), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using blue as the
/// foreground colour.
#[macro_export]
macro_rules! blue {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::BLUE), $arg, false);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::BLUE), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using magenta as the
/// foreground colour.
#[macro_export]
macro_rules! magenta {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::MAGENTA), $arg, false);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::MAGENTA), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using cyan as the
/// foreground colour.
#[macro_export]
macro_rules! cyan {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::CYAN), $arg, false);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::CYAN), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using white as the
/// foreground colour.
#[macro_export]
macro_rules! white {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::WHITE), $arg, false);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::WHITE), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using dark black as the
/// foreground colour.
#[macro_export]
macro_rules! dark_black {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_BLACK), $arg, false);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_BLACK), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using dark red as the
/// foreground colour.
#[macro_export]
macro_rules! dark_red {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_RED), $arg, false);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_RED), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using dark green as the
/// foreground colour.
#[macro_export]
macro_rules! dark_green {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_GREEN), $arg, false);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_GREEN), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using dark yellow as the
/// foreground colour.
#[macro_export]
macro_rules! dark_yellow {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_YELLOW), $arg, false);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_YELLOW), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using dark blue as the
/// foreground colour.
#[macro_export]
macro_rules! dark_blue {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_BLUE), $arg, false);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_BLUE), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using dark magenta as
/// the foreground colour.
#[macro_export]
macro_rules! dark_magenta {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_MAGENTA), $arg, false);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_MAGENTA), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using dark cyan as the
/// foreground colour.
#[macro_export]
macro_rules! dark_cyan {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_CYAN), $arg, false);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_CYAN), &format!($($arg)*), false);
    };
}

/// Macro similar to [`print!`](https://doc.servo.org/std/macro.print.html) using dark white as the
/// foreground colour.
#[macro_export]
macro_rules! dark_white {
    ($arg:tt) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_WHITE), $arg, false);
    };
    ($($arg:tt)*) => {
        $crate::unnamed::write(Some($crate::unnamed::DARK_WHITE), &format!($($arg)*), false);
    };
}
