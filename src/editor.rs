use std::io::stdin;
use termion::event::{Event, Key};
use termion::input::TermRead;

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

    // Dirty rows that need to be re-rendered
    // This allows us to not superfluously re-render the screen
    dirty_rows: Vec<u16>,
    dirty: bool,
}

impl Editor {
    pub fn new(filename: Option<&str>) -> Editor {
        let view = View::new();
        let mut dirty_rows: Vec<u16> = Vec::with_capacity(view.term_height as usize);
        for i in 1..=view.term_height {
            dirty_rows.push(i);
        }

        Editor {
            view,
            mode: Mode::Normal,

            cursor_x: 1,
            cursor_y: 1,

            dirty_rows,
            dirty: true,
        }
    }

    pub fn refresh_screen(&mut self) {
        self.view.hide_cursor();
        self.view.render_through();

        // Start drawing from the top left
        self.view.position_cursor(1, 1);

        // Buffering rows will clear the queue of dirty rows
        if !self.dirty_rows.is_empty() {
            self.buffer_rows();
        }

        self.view.position_cursor(self.cursor_x, self.cursor_y);
        self.view.show_cursor();

        // Actually render the buffer
        self.view.render_through();
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

        self.dirty_rows.push(self.view.term_height / 3);
    }

    fn process_keypress(&mut self, key: termion::event::Key) {
        use termion::event::Key::*;
        // eprintln!("Processing keypress!");

        match key {
            Char('k') | Char('j') | Char('h') | Char('l') => {
                self.move_cursor(key);
                self.dirty = true;
            }
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
                    Event::Key(Key::Char('q')) => {
                        self.view.position_cursor(1, 1);
                        self.view.clear_screen();
                        return;
                    }
                    Event::Key(k) => self.process_keypress(k),
                    _ => unimplemented!(),
                };
            }
        }
    }

    /// Push rows to render buffer (a BufferedWriter), to be rendered in the view
    // TODO check a dirty bit to see if the row ahs changed before rerendering
    fn buffer_rows(&mut self) {
        use std::time::SystemTime;

        let welcome_str = format!("ipsum v0.1 {:?}", SystemTime::now());

        let padding_len = (self.view.term_width as usize - welcome_str.len()) / 2;
        let mut padding_str = String::with_capacity(padding_len);
        for _ in 0..padding_len {
            padding_str.push(' ');
        }

        for y in &self.dirty_rows {
            if *y == self.view.term_height / 3 {
                self.view.position_cursor(1, *y);
                self.view.clear_line();
                self.view.write("~");
                self.view.write(&padding_str);
                self.view.write(&welcome_str);
            } else {
                self.view.position_cursor(1, *y);
                self.view.clear_line();
                self.view.write("~");
            }
        }

        self.dirty_rows.clear();
    }
}
