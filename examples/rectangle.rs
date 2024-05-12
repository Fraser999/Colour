use std::fmt::{Display, Formatter, Result};

use colour::*;

struct Rectangle {
    length: u64,
    width: u64,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "rectangle {{ ")?;
        write_blue_bold!(f, "length")?;
        write!(f, ": ")?;
        write_bold!(f, "{}", self.length)?;
        write!(f, "mm, ")?;
        write_blue_bold!(f, "width")?;
        write!(f, ": ")?;
        write_bold!(f, "{}", self.width)?;
        write!(f, "mm }}")
    }
}

fn main() {
    let rectangle = Rectangle {
        length: 1234,
        width: 567,
    };
    println!("{rectangle}");
}
