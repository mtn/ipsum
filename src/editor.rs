use termion::event::Key;
use termion::async_stdin;
use std::io::{Read, Write, stdout};
use termion::event::Event;
use termion::event::parse_event;

use ::view::View;


enum Mode {
    Insert,
    Normal,
}

pub struct Editor {
    view: View,
    input_stream: std::io::Bytes<termion::AsyncReader>,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            view: View::new(),
            input_stream: async_stdin().bytes(),
        }
    }

    pub fn refresh_screen() {
        unimplemented!();
    }

    pub fn process_keypresses(&mut self) {
        loop {
            write!(self.view.out, "{}", termion::clear::CurrentLine).unwrap();

            let b = self.input_stream.next();
            // write!(stdout, "\r{:?}    <- This demonstrates the async read input char. Between each update a 100 ms. is waited, simply to demonstrate the async fashion. \n\r", b).unwrap();

            if let Some(Ok(l)) = b {
                let event = parse_event(l, &mut self.input_stream);
                if let Ok(Event::Key(Key::Char('q'))) = event {
                    return
                }

                write!(self.view.out, "event {:?}\r\n", event).unwrap();
            }
            // if let Some(Ok(b'q')) = b {
            //     break;
            // }

            // stdout.flush().unwrap();

            // thread::sleep(Duration::from_millis(50));
            // stdout.write_all(b"# ").unwrap();
            // stdout.flush().unwrap();
            // thread::sleep(Duration::from_millis(50));
            // stdout.write_all(b"\r #").unwrap();
            // write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
            self.view.out.flush().unwrap();
        }

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
