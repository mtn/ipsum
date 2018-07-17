use termion::event::Key;
use termion;
use termion::input::TermRead;
use std::io::{stdin, Stdin};

use ::view::View;


enum Mode {
    Insert,
    Normal,
}

pub struct Editor {
    view: View,
    inp: Stdin,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            view: View::new(),
            inp: stdin(),
        }
    }

    pub fn refresh_screen() {
        unimplemented!();
    }

    pub fn process_keypresses(&mut self) {
        // Continually read keys from stdin
        // for c in self.inp.events() {
        //     Self::refresh_screen();

        //     // match c.unwrap() {
        //     //     termion::Event::Key(Key::Char('q')) => {
        //     //         println!("{}", termion::cursor::Show);
        //     //     },
        //     //     Key::Char(c) => println!("{}", c),
        //     //     Key::Alt(c) => println!("Alt-{}", c),
        //     //     Key::Ctrl(c) => println!("Ctrl-{}", c),
        //     //     Key::Left => println!("<left>"),
        //     //     Key::Right => println!("<right>"),
        //     //     Key::Up => println!("<up>"),
        //     //     Key::Down => println!("<down>"),
        //     //     _ => println!("Other"),
        //     // }
        // }
    }
}
