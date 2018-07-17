#![feature(extern_prelude)]
extern crate termion;
extern crate lazy_static;

use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{Read, Write, stdout};
use std::thread;
use std::time::Duration;
use termion::event::parse_event;
use termion::event::Key;
use termion::event::Event;


mod editor;
mod view;

fn main() {
    // let mut editor = editor::Editor::new();
    // editor.process_keypresses();

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1))
            .unwrap();

    loop {
        write!(stdout, "{}", termion::clear::CurrentLine).unwrap();

        let b = stdin.next();
        // write!(stdout, "\r{:?}    <- This demonstrates the async read input char. Between each update a 100 ms. is waited, simply to demonstrate the async fashion. \n\r", b).unwrap();

        if let Some(Ok(l)) = b {
            let event = parse_event(l, &mut stdin);
            if let Ok(Event::Key(Key::Char('q'))) = event {
                return
            }

            write!(stdout, "event {:?}\r\n", event).unwrap();
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
        stdout.flush().unwrap();
    }

}
