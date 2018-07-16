extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}q to exit. Type stuff, use alt, and so on.{}",
        // Clear the screen.
        termion::clear::All,
        // Goto (1,1).
        termion::cursor::Goto(1, 1),
        // Hide the cursor.
        termion::cursor::Hide
    ).unwrap();
    // Flush stdout (i.e. make the output appear).
    stdout.flush().unwrap();

    for c in stdin.keys() {
        // Clear the current line.
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        ).unwrap();

        // Print the key we type...
        match c.unwrap() {
            // Exit.
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("Alt-{}", c),
            Key::Ctrl(c) => println!("Ctrl-{}", c),
            Key::Left => println!("<left>"),
            Key::Right => println!("<right>"),
            Key::Up => println!("<up>"),
            Key::Down => println!("<down>"),
            _ => println!("Other"),
        }

        // Flush again.
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
