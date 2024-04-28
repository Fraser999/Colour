use std::{
    fmt::Arguments,
    io::{self, Write},
};

use crossterm::{self, style::Color};
pub use crossterm::{style::SetForegroundColor, QueueableCommand};

pub type Colour = Color;

pub fn write<T: Write, const NEWLINE: bool>(
    output: T,
    output_name: &'static str,
    colour: Colour,
    maybe_args: Option<Arguments>,
) {
    if let Err(error) = do_write::<T, NEWLINE>(output, colour, maybe_args) {
        panic!("failed writing to {output_name}: {error}");
    }
}

fn do_write<T: Write, const NEWLINE: bool>(
    mut output: T,
    colour: Colour,
    maybe_args: Option<Arguments>,
) -> io::Result<()> {
    let _ = output.queue(SetForegroundColor(colour))?;

    if let Some(args) = maybe_args {
        output.write_fmt(args)?;
    }

    if NEWLINE {
        writeln!(output)?;
    }

    let _ = output.queue(SetForegroundColor(Colour::Reset))?;
    output.flush()
}
