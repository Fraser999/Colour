use std::io::{self, Stderr, Stdout, Write};

use crossterm::{
    self,
    style::{Color, Print, ResetColor, SetForegroundColor},
    QueueableCommand,
};

pub type Colour = Color;

enum Terminal {
    StdOut(Stdout),
    StdErr(Stderr),
}

impl Write for Terminal {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Terminal::StdOut(stdout) => stdout.write(buf),
            Terminal::StdErr(stderr) => stderr.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Terminal::StdOut(stdout) => stdout.flush(),
            Terminal::StdErr(stderr) => stderr.flush(),
        }
    }
}

pub fn write(colour: Option<Colour>, message: &str, newline: bool) {
    if do_write(colour, message, newline, Terminal::StdOut(io::stdout())).is_err() {
        if newline {
            println!("{}", message);
        } else {
            print!("{}", message);
        }
    }
}

pub fn ewrite(colour: Option<Colour>, message: &str, newline: bool) {
    if do_write(colour, message, newline, Terminal::StdErr(io::stderr())).is_err() {
        if newline {
            eprintln!("{}", message);
        } else {
            eprint!("{}", message);
        }
    }
}

fn do_write(
    colour: Option<Colour>,
    message: &str,
    newline: bool,
    mut terminal: Terminal,
) -> crossterm::Result<()> {
    if let Some(colr) = colour {
        let _ = terminal.queue(SetForegroundColor(colr))?;
    }

    let _ = terminal.queue(Print(message))?.queue(ResetColor)?;

    if newline {
        let _ = terminal.queue(Print('\n'))?;
    }

    terminal.flush()?;

    Ok(())
}
