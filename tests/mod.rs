use colour::*;

#[test]
fn print_stdout_with_newline() {
    grey_ln!("This is {}.", "grey");
    grey_ln!("So is this.");
    grey_ln!();
    dark_grey_ln!("This is {}.", "dark_grey");
    dark_grey_ln!("So is this.");
    dark_grey_ln!();
    red_ln!("This is {}.", "red");
    red_ln!("So is this.");
    red_ln!();
    dark_red_ln!("This is {}.", "dark_red");
    dark_red_ln!("So is this.");
    dark_red_ln!();
    green_ln!("This is {}.", "green");
    green_ln!("So is this.");
    green_ln!();
    dark_green_ln!("This is {}.", "dark_green");
    dark_green_ln!("So is this.");
    dark_green_ln!();
    yellow_ln!("This is {}.", "yellow");
    yellow_ln!("So is this.");
    yellow_ln!();
    dark_yellow_ln!("This is {}.", "dark_yellow");
    dark_yellow_ln!("So is this.");
    dark_yellow_ln!();
    blue_ln!("This is {}.", "blue");
    blue_ln!("So is this.");
    blue_ln!();
    dark_blue_ln!("This is {}.", "dark_blue");
    dark_blue_ln!("So is this.");
    dark_blue_ln!();
    magenta_ln!("This is {}.", "magenta");
    magenta_ln!("So is this.");
    magenta_ln!();
    dark_magenta_ln!("This is {}.", "dark_magenta");
    dark_magenta_ln!("So is this.");
    dark_magenta_ln!();
    cyan_ln!("This is {}.", "cyan");
    cyan_ln!("So is this.");
    cyan_ln!();
    dark_cyan_ln!("This is {}.", "dark_cyan");
    dark_cyan_ln!("So is this.");
    dark_cyan_ln!();
    black_ln!("This is {}.", "black");
    black_ln!("So is this.");
    black_ln!();
    white_ln!("This is {}.", "white");
    white_ln!("So is this.");
    white_ln!();
    prnt_ln!("This is {}.", "the default colour");
    prnt_ln!("So is this.");
    prnt_ln!();
}

#[test]
fn print_stdout_no_newline() {
    grey!("g");
    grey!();
    grey!("{}, ", "rey");
    dark_grey!("d");
    dark_grey!();
    dark_grey!("{}, ", "ark_grey");
    red!("r");
    red!();
    red!("{}, ", "ed");
    dark_red!("d");
    dark_red!();
    dark_red!("{}, ", "ark_red");
    green!("g");
    green!();
    green!("{}, ", "reen");
    dark_green!("d");
    dark_green!();
    dark_green!("{}, ", "ark_green");
    yellow!("y");
    yellow!();
    yellow!("{}, ", "ellow");
    dark_yellow!("d");
    dark_yellow!();
    dark_yellow!("{}, ", "ark_yellow");
    blue!("b");
    blue!();
    blue!("{}, ", "lue");
    dark_blue!("d");
    dark_blue!();
    dark_blue!("{}, ", "ark_blue");
    magenta!("m");
    magenta!();
    magenta!("{}, ", "agenta");
    dark_magenta!("d");
    dark_magenta!();
    dark_magenta!("{}, ", "ark_magenta");
    cyan!("c");
    cyan!();
    cyan!("{}, ", "yan");
    dark_cyan!("d");
    dark_cyan!();
    dark_cyan!("{}, ", "ark_cyan");
    black!("b");
    black!();
    black!("{}, ", "lack");
    white!("w");
    white!();
    white!("{}, ", "hite");
    prnt!("d");
    prnt!("efault {}", "colour\n\n");
}

#[test]
fn print_stderr_with_newline() {
    e_grey_ln!("This is {}.", "grey");
    e_grey_ln!("So is this.");
    e_grey_ln!();
    e_dark_grey_ln!("This is {}.", "dark_grey");
    e_dark_grey_ln!("So is this.");
    e_dark_grey_ln!();
    e_red_ln!("This is {}.", "red");
    e_red_ln!("So is this.");
    e_red_ln!();
    e_dark_red_ln!("This is {}.", "dark_red");
    e_dark_red_ln!("So is this.");
    e_dark_red_ln!();
    e_green_ln!("This is {}.", "green");
    e_green_ln!("So is this.");
    e_green_ln!();
    e_dark_green_ln!("This is {}.", "dark_green");
    e_dark_green_ln!("So is this.");
    e_dark_green_ln!();
    e_yellow_ln!("This is {}.", "yellow");
    e_yellow_ln!("So is this.");
    e_yellow_ln!();
    e_dark_yellow_ln!("This is {}.", "dark_yellow");
    e_dark_yellow_ln!("So is this.");
    e_dark_yellow_ln!();
    e_blue_ln!("This is {}.", "blue");
    e_blue_ln!("So is this.");
    e_blue_ln!();
    e_dark_blue_ln!("This is {}.", "dark_blue");
    e_dark_blue_ln!("So is this.");
    e_dark_blue_ln!();
    e_magenta_ln!("This is {}.", "magenta");
    e_magenta_ln!("So is this.");
    e_magenta_ln!();
    e_dark_magenta_ln!("This is {}.", "dark_magenta");
    e_dark_magenta_ln!("So is this.");
    e_dark_magenta_ln!();
    e_cyan_ln!("This is {}.", "cyan");
    e_cyan_ln!("So is this.");
    e_cyan_ln!();
    e_dark_cyan_ln!("This is {}.", "dark_cyan");
    e_dark_cyan_ln!("So is this.");
    e_dark_cyan_ln!();
    e_black_ln!("This is {}.", "black");
    e_black_ln!("So is this.");
    e_black_ln!();
    e_white_ln!("This is {}.", "white");
    e_white_ln!("So is this.");
    e_white_ln!();
    e_prnt_ln!("This is {}.", "the default colour");
    e_prnt_ln!("So is this.");
    e_prnt_ln!();
}

#[test]
fn print_stderr_no_newline() {
    e_grey!("g");
    e_grey!();
    e_grey!("{}, ", "rey");
    e_dark_grey!("d");
    e_dark_grey!();
    e_dark_grey!("{}, ", "ark_grey");
    e_red!("r");
    e_red!();
    e_red!("{}, ", "ed");
    e_dark_red!("d");
    e_dark_red!();
    e_dark_red!("{}, ", "ark_red");
    e_green!("g");
    e_green!();
    e_green!("{}, ", "reen");
    e_dark_green!("d");
    e_dark_green!();
    e_dark_green!("{}, ", "ark_green");
    e_yellow!("y");
    e_yellow!();
    e_yellow!("{}, ", "ellow");
    e_dark_yellow!("d");
    e_dark_yellow!();
    e_dark_yellow!("{}, ", "ark_yellow");
    e_blue!("b");
    e_blue!();
    e_blue!("{}, ", "lue");
    e_dark_blue!("d");
    e_dark_blue!();
    e_dark_blue!("{}, ", "ark_blue");
    e_magenta!("m");
    e_magenta!();
    e_magenta!("{}, ", "agenta");
    e_dark_magenta!("d");
    e_dark_magenta!();
    e_dark_magenta!("{}, ", "ark_magenta");
    e_cyan!("c");
    e_cyan!();
    e_cyan!("{}, ", "yan");
    e_dark_cyan!("d");
    e_dark_cyan!();
    e_dark_cyan!("{}, ", "ark_cyan");
    e_black!("b");
    e_black!();
    e_black!("{}, ", "lack");
    e_white!("w");
    e_white!();
    e_white!("{}, ", "hite");
    e_prnt!("d");
    e_prnt!("efault {}", "colour\n\n");
}

fn readme_example() {
    let mut v = vec![];

    let err: Result<(), u8> = Err(1);
    wrte_ln!(&mut v, "Failed on {}", 9);
    write_yellow!(&mut v, "Error details: ");
    write_red_ln!(&mut v, "{:?}", err);

    wrte!(std::io::stdout(), "{}", String::from_utf8_lossy(&v))
}

#[test]
fn write_stdout_with_newline() {
    readme_example();
    write_grey_ln!(std::io::stdout(), "This is {}.", "grey");
    write_grey_ln!(std::io::stdout(), "So is this.\n");
    write_dark_grey_ln!(std::io::stdout(), "This is {}.", "dark_grey");
    write_dark_grey_ln!(std::io::stdout(), "So is this.\n");
    write_red_ln!(std::io::stdout(), "This is {}.", "red");
    write_red_ln!(std::io::stdout(), "So is this.\n");
    write_dark_red_ln!(std::io::stdout(), "This is {}.", "dark_red");
    write_dark_red_ln!(std::io::stdout(), "So is this.\n");
    write_green_ln!(std::io::stdout(), "This is {}.", "green");
    write_green_ln!(std::io::stdout(), "So is this.\n");
    write_dark_green_ln!(std::io::stdout(), "This is {}.", "dark_green");
    write_dark_green_ln!(std::io::stdout(), "So is this.\n");
    write_yellow_ln!(std::io::stdout(), "This is {}.", "yellow");
    write_yellow_ln!(std::io::stdout(), "So is this.\n");
    write_dark_yellow_ln!(std::io::stdout(), "This is {}.", "dark_yellow");
    write_dark_yellow_ln!(std::io::stdout(), "So is this.\n");
    write_blue_ln!(std::io::stdout(), "This is {}.", "blue");
    write_blue_ln!(std::io::stdout(), "So is this.\n");
    write_dark_blue_ln!(std::io::stdout(), "This is {}.", "dark_blue");
    write_dark_blue_ln!(std::io::stdout(), "So is this.\n");
    write_magenta_ln!(std::io::stdout(), "This is {}.", "magenta");
    write_magenta_ln!(std::io::stdout(), "So is this.\n");
    write_dark_magenta_ln!(std::io::stdout(), "This is {}.", "dark_magenta");
    write_dark_magenta_ln!(std::io::stdout(), "So is this.\n");
    write_cyan_ln!(std::io::stdout(), "This is {}.", "cyan");
    write_cyan_ln!(std::io::stdout(), "So is this.\n");
    write_dark_cyan_ln!(std::io::stdout(), "This is {}.", "dark_cyan");
    write_dark_cyan_ln!(std::io::stdout(), "So is this.\n");
    write_black_ln!(std::io::stdout(), "This is {}.", "black");
    write_black_ln!(std::io::stdout(), "So is this.\n");
    write_white_ln!(std::io::stdout(), "This is {}.", "white");
    write_white_ln!(std::io::stdout(), "So is this.\n");
    wrte_ln!(std::io::stdout(), "This is {}.", "the default colour");
    wrte_ln!(std::io::stdout(), "So is this.\n");
}

#[test]
fn write_stdout_no_newline() {
    write_grey!(std::io::stdout(), "g");
    write_grey!(std::io::stdout(), "{}, ", "rey");
    write_dark_grey!(std::io::stdout(), "d");
    write_dark_grey!(std::io::stdout(), "{}, ", "ark_grey");
    write_red!(std::io::stdout(), "r");
    write_red!(std::io::stdout(), "{}, ", "ed");
    write_dark_red!(std::io::stdout(), "d");
    write_dark_red!(std::io::stdout(), "{}, ", "ark_red");
    write_green!(std::io::stdout(), "g");
    write_green!(std::io::stdout(), "{}, ", "reen");
    write_dark_green!(std::io::stdout(), "d");
    write_dark_green!(std::io::stdout(), "{}, ", "ark_green");
    write_yellow!(std::io::stdout(), "y");
    write_yellow!(std::io::stdout(), "{}, ", "ellow");
    write_dark_yellow!(std::io::stdout(), "d");
    write_dark_yellow!(std::io::stdout(), "{}, ", "ark_yellow");
    write_blue!(std::io::stdout(), "b");
    write_blue!(std::io::stdout(), "{}, ", "lue");
    write_dark_blue!(std::io::stdout(), "d");
    write_dark_blue!(std::io::stdout(), "{}, ", "ark_blue");
    write_magenta!(std::io::stdout(), "m");
    write_magenta!(std::io::stdout(), "{}, ", "agenta");
    write_dark_magenta!(std::io::stdout(), "d");
    write_dark_magenta!(std::io::stdout(), "{}, ", "ark_magenta");
    write_cyan!(std::io::stdout(), "c");
    write_cyan!(std::io::stdout(), "{}, ", "yan");
    write_dark_cyan!(std::io::stdout(), "d");
    write_dark_cyan!(std::io::stdout(), "{}, ", "ark_cyan");
    write_black!(std::io::stdout(), "b");
    write_black!(std::io::stdout(), "{}, ", "lack");
    write_white!(std::io::stdout(), "w");
    write_white!(std::io::stdout(), "{}, ", "hite");
    wrte!(std::io::stdout(), "d");
    wrte!(std::io::stdout(), "efault {}", "colour\n\n");
}
