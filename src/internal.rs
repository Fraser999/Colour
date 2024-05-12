use std::{
    env,
    error::Error,
    fmt::{self, Display, Formatter},
    io::{self, IsTerminal},
    sync::Once,
};

static mut SHOULD_COLOUR: bool = false;
static ONCE: Once = Once::new();

pub const DEFAULT_COLOUR: &str = "\x1b[22;39m";

pub const BLACK: &str = "\x1b[30m"; // (a.k.a black)
pub const DARK_RED: &str = "\x1b[31m"; // (a.k.a red)
pub const DARK_GREEN: &str = "\x1b[32m"; // (a.k.a green)
pub const DARK_YELLOW: &str = "\x1b[33m"; // (a.k.a yellow)
pub const DARK_BLUE: &str = "\x1b[34m"; // (a.k.a blue)
pub const DARK_MAGENTA: &str = "\x1b[35m"; // (a.k.a magenta)
pub const DARK_CYAN: &str = "\x1b[36m"; // (a.k.a cyan)
pub const GREY: &str = "\x1b[37m"; // (a.k.a white)

pub const DARK_GREY: &str = "\x1b[90m"; // (a.k.a. bright black)
pub const RED: &str = "\x1b[91m"; // (a.k.a. bright red)
pub const GREEN: &str = "\x1b[92m"; // (a.k.a. bright green)
pub const YELLOW: &str = "\x1b[93m"; // (a.k.a. bright yellow)
pub const BLUE: &str = "\x1b[94m"; // (a.k.a. bright blue)
pub const MAGENTA: &str = "\x1b[95m"; // (a.k.a. bright magenta)
pub const CYAN: &str = "\x1b[96m"; // (a.k.a. bright cyan)
pub const WHITE: &str = "\x1b[97m"; // (a.k.a. bright white)

pub const BOLD_DEFAULT_COLOUR: &str = "\x1b[1;39m";

pub const BOLD_BLACK: &str = "\x1b[1;30m"; // (a.k.a black)
pub const BOLD_DARK_RED: &str = "\x1b[1;31m"; // (a.k.a red)
pub const BOLD_DARK_GREEN: &str = "\x1b[1;32m"; // (a.k.a green)
pub const BOLD_DARK_YELLOW: &str = "\x1b[1;33m"; // (a.k.a yellow)
pub const BOLD_DARK_BLUE: &str = "\x1b[1;34m"; // (a.k.a blue)
pub const BOLD_DARK_MAGENTA: &str = "\x1b[1;35m"; // (a.k.a magenta)
pub const BOLD_DARK_CYAN: &str = "\x1b[1;36m"; // (a.k.a cyan)
pub const BOLD_GREY: &str = "\x1b[1;37m"; // (a.k.a white)

pub const BOLD_DARK_GREY: &str = "\x1b[1;90m"; // (a.k.a. bright black)
pub const BOLD_RED: &str = "\x1b[1;91m"; // (a.k.a bright red)
pub const BOLD_GREEN: &str = "\x1b[1;92m"; // (a.k.a bright green)
pub const BOLD_YELLOW: &str = "\x1b[1;93m"; // (a.k.a bright yellow)
pub const BOLD_BLUE: &str = "\x1b[1;94m"; // (a.k.a bright blue)
pub const BOLD_MAGENTA: &str = "\x1b[1;95m"; // (a.k.a bright magenta)
pub const BOLD_CYAN: &str = "\x1b[1;96m"; // (a.k.a bright cyan)
pub const BOLD_WHITE: &str = "\x1b[1;97m"; // (a.k.a. bright white)

/// Error returned when forcing colour on or off, if it has already been initialised to a different
/// value.
#[derive(Clone, Copy, Debug)]
pub struct AlreadySet;

impl Display for AlreadySet {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "colour has already been set")
    }
}

impl Error for AlreadySet {}

pub(super) fn force(value: bool) -> Result<(), AlreadySet> {
    unsafe {
        ONCE.call_once(|| {
            SHOULD_COLOUR = value;
        });
        if SHOULD_COLOUR == value {
            Ok(())
        } else {
            Err(AlreadySet)
        }
    }
}

pub fn should_colour() -> bool {
    unsafe {
        ONCE.call_once(|| {
            SHOULD_COLOUR = initialise_from_env();
        });
        SHOULD_COLOUR
    }
}

fn initialise_from_env() -> bool {
    if env_var_is_set_and_not_empty_string("NO_COLOR") {
        false
    } else if env_var_is_set_and_not_empty_string("CLICOLOR_FORCE") {
        true
    } else {
        io::stdout().is_terminal()
    }
}

fn env_var_is_set_and_not_empty_string(var: &str) -> bool {
    let Ok(value) = env::var(var) else {
        return false;
    };
    !value.is_empty()
}
