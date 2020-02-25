use std::io::{self, Write};
use crossterm::{QueueableCommand, style::{ResetColor, Print, SetForegroundColor, Color}, Result};

pub type Colour = Color;

pub fn write(colour: Option<Colour>, message: &str, newline: bool) {
    if do_write(colour, message, newline).is_err() {
        if newline {
            println!("{}", message);
        } else {
            print!("{}", message);
        }
    }
}

fn do_write(colour: Option<Colour>, message: &str, newline: bool) -> Result<()>{
    let mut std_out = io::stdout();

    if let Some(colr) = colour {
        let _ = std_out.queue(SetForegroundColor(colr))?;
    }

    let _ = std_out.queue(Print(message))?.queue(ResetColor)?;

    if newline {
        let _ = std_out.queue(Print('\n'))?;
    }

    std_out.flush()?;

    Ok(())
}
