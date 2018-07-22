use std::io::{stdin, Stdin};
use termion::input::TermRead;
use termion::event::{Event, Key};

use view::View;

enum Mode {
    Insert,
    Normal,
}

pub struct Editor {
    view: View,

    mode: Mode,

    cursor_x: u16,
    cursor_y: u16,

    // Indicates if the buffer should be re-rendered
    dirty: bool,
}

impl Editor {
    pub fn new(filename: Option<&str>) -> Editor {
        Editor {
            view: View::new(),
            mode: Mode::Normal,

            cursor_x: 1,
            cursor_y: 1,

            dirty: true,
        }
    }

    pub fn refresh_screen(&mut self) {
        self.view.hide_cursor();
        self.view.render_through();

        // Start drawing from the top left
        self.view.position_cursor(1, 1);

        // self.buffer_rows();

        self.view.position_cursor(self.cursor_x, self.cursor_y);
        self.view.show_cursor();

        // Actually render the buffer
        self.view.render_through();
        self.dirty = false;
    }

    fn move_cursor(&mut self, key: termion::event::Key) {
        use termion::event::Key::*;

        // eprintln!("{} {}", self.cursor_x, self.cursor_y);
        match key {
            Char('k') => self.cursor_y -= 1,
            Char('j') => self.cursor_y += 1,
            Char('h') => self.cursor_x -= 1,
            Char('l') => self.cursor_x += 1,
            _ => panic!("Unexpected character type {:?} in move_cursor", key),
        }
    }

    fn process_keypress(&mut self, key: termion::event::Key) {
        use termion::event::Key::*;
        // eprintln!("Processing keypress!");

        match key {
            Char('k') | Char('j') | Char('h') | Char('l') => {
                self.move_cursor(key);
                self.dirty = true;
            },
            _ => unimplemented!(),
        }
    }

    pub fn process_keypresses(&mut self) {
        let stdin = stdin();
        let mut events = stdin.events();

        loop {
            if self.dirty {
                self.refresh_screen();
            }

            if let Some(event) = events.next() {
                match event.unwrap() {
                    Event::Key(Key::Char('q')) => return,
                    Event::Key(k) => self.process_keypress(k),
                    _ => unimplemented!(),
                };
            }
        }
    }

    /// Push rows to render buffer (a BufferedWriter), to be rendered in the view
    // TODO check a dirty bit to see if the row ahs changed before rerendering
    fn buffer_rows(&mut self) {
        for y in 0..(self.view.term_height-1) {
            // u16 division is closed, so rounding isn't ever an issue
            use std::time::SystemTime;

            // let welcome_str = "ipsum v0.1";
            let welcome_str = format!("ipsum v0.1 {:?}", SystemTime::now());

            let padding_len = (self.view.term_width as usize - welcome_str.len()) / 2;
            let mut padding_str = String::with_capacity(padding_len);
            for _ in 0..padding_len {
                padding_str.push(' ');
            }

            if y == self.view.term_height / 3 {
                self.view.write(&format!("{}~{}{}\r\n", termion::clear::CurrentLine, padding_str, welcome_str));
            } else {
                self.view.write(&format!("{}~\r\n", termion::clear::CurrentLine));
            }

        }
        self.view.write(&format!("{}~", termion::clear::CurrentLine));
    }
}
