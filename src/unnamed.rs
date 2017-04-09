use std::io::Write;
use std::sync::Mutex;
use term::{self, StdoutTerminal, color};

/// Wrapper for `StdoutTerminal` which resets stdout when dropped, even if panicking.
struct StdOut(pub Box<StdoutTerminal>);

impl Drop for StdOut {
    fn drop(&mut self) { self.reset() }
}

impl StdOut {
    pub fn reset(&mut self) {
        let _ = self.0.reset();
        self.flush();
    }

    pub fn flush(&mut self) { let _ = self.0.get_mut().flush(); }
}

lazy_static! {
    static ref STD_OUT: Mutex<StdOut> = Mutex::new(StdOut(term::stdout().unwrap()));
}

pub type Colour = color::Color;

pub const BLACK: Colour = color::BRIGHT_BLACK;
pub const RED: Colour = color::BRIGHT_RED;
pub const GREEN: Colour = color::BRIGHT_GREEN;
pub const YELLOW: Colour = color::BRIGHT_YELLOW;
pub const BLUE: Colour = color::BRIGHT_BLUE;
pub const MAGENTA: Colour = color::BRIGHT_MAGENTA;
pub const CYAN: Colour = color::BRIGHT_CYAN;
pub const WHITE: Colour = color::BRIGHT_WHITE;

pub const DARK_BLACK: Colour = color::BLACK;
pub const DARK_RED: Colour = 1;
pub const DARK_GREEN: Colour = color::GREEN;
pub const DARK_YELLOW: Colour = color::YELLOW;
pub const DARK_BLUE: Colour = color::BLUE;
pub const DARK_MAGENTA: Colour = 5;
pub const DARK_CYAN: Colour = color::CYAN;
pub const DARK_WHITE: Colour = color::WHITE;

pub fn write(colour: Option<Colour>, message: &str, newline: bool) {
    if let Ok(mut std_out) = STD_OUT.lock() {
        let lf = if newline { "\n" } else { "" };
        if let Some(colr) = colour {
            let _ = std_out.0.fg(colr);
            let _ = write!(std_out.0, "{}{}", message, lf);
            std_out.reset();
        } else {
            let _ = write!(std_out.0, "{}{}", message, lf);
            std_out.flush()
        }
    }
}
