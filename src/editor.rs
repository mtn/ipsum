use std::io::{stdout, Read, Write};
use termion::async_stdin;
use termion::event::parse_event;
use termion::event::Event;
use termion::event::Key;

use view::View;

enum Mode {
    Insert,
    Normal,
}

pub struct Editor {
    view: View,
    input_stream: std::io::Bytes<termion::AsyncReader>,
    mode: Mode,

    cursor_x: u16,
    cursor_y: u16,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            view: View::new(),
            input_stream: async_stdin().bytes(),
            mode: Mode::Normal,

            cursor_x: 1,
            cursor_y: 1,
        }
    }

    pub fn refresh_screen(&mut self) {
        self.view.hide_cursor();
        self.view.position_cursor(self.cursor_x, self.cursor_y);

        self.view.draw_rows();
        self.view.show_cursor();
    }

    pub fn process_keypresses(&mut self) {
        loop {
            self.refresh_screen();
            // write!(self.view.out, "{}{}", termion::cursor::Goto(self.cursor_x, self.cursor_y), termion::clear::CurrentLine).unwrap();
            // write!(self.view.out, "{}", termion::clear::CurrentLine).unwrap();

            let b = self.input_stream.next();
            // write!(stdout, "\r{:?}    <- This demonstrates the async read input char. Between each update a 100 ms. is waited, simply to demonstrate the async fashion. \n\r", b).unwrap();

            if let Some(Ok(l)) = b {
                let event = parse_event(l, &mut self.input_stream);
                if let Ok(Event::Key(Key::Char('q'))) = event {
                    return;
                }

                // write!(self.view.out, "event {:?}\r\n", event).unwrap();
            }

            // write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
            // self.view.out.flush().unwrap();
        }
    }

}
