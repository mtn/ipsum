use std::io;
use std::io::Write;
use termion;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

pub struct View {
    out: RawTerminal<io::BufWriter<io::Stdout>>,
    term_width: u16,
    term_height: u16,
}

impl View {
    pub fn new() -> View {
        let term_size = termion::terminal_size().unwrap();

        let mut view = View {
            out: io::BufWriter::with_capacity(1 << 14, io::stdout())
                .into_raw_mode()
                .unwrap(),
            term_width: term_size.0,
            term_height: term_size.1,
        };

        // Clear the display
        write!(
            view.out,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        ).unwrap();

        view
    }

    /// Render the cursor at its current position
    pub fn position_cursor(&mut self, cursor_x: u16, cursor_y: u16) {
        write!(self.out, "{}", termion::cursor::Goto(cursor_x, cursor_y)).unwrap();
    }

    pub fn draw_rows(&mut self) {
        for y in 1..self.term_height {
            write!(self.out, "~\r\n").unwrap();
        }
        // write!(self.out, "{}", termion::cursor::Goto(1, 1)).unwrap();
    }

    pub fn hide_cursor(&mut self) {
        write!(self.out, "{}", termion::cursor::Hide).unwrap();
    }

    pub fn show_cursor(&mut self) {
        write!(self.out, "{}", termion::cursor::Show).unwrap();
    }
}
