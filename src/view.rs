use std::io;
use std::io::Write;
use termion;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

pub struct View {
    // TODO make out private
    pub out: RawTerminal<io::BufWriter<io::Stdout>>,
    pub term_width: u16,
    pub term_height: u16,
    pub render_buffer: String,
}

impl View {
    /// Create a view object, and run through basic initialization
    pub fn new() -> View {
        let term_size = termion::terminal_size().unwrap();

        let mut view = View {
            out: io::BufWriter::with_capacity(1 << 14, io::stdout())
                .into_raw_mode()
                .unwrap(),
            term_width: term_size.0,
            term_height: term_size.1,
            render_buffer: String::with_capacity(100),
        };

        // Clear the display
        write!(
            view.out,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Show,
        ).unwrap();

        view
    }

    pub fn render(&mut self) {
        write!(self.out, "{}", self.render_buffer).unwrap();
        write!(self.out, "rendered").unwrap();
        // Truncate the render buffer, but don't change the capacity
        self.render_buffer.truncate(0);
    }

    /// Clear screen
    pub fn clear_screen(&mut self) {
        write!(self.out, "{}", termion::clear::All).unwrap();
    }

    /// Render the cursor at its current position
    pub fn position_cursor(&mut self, cursor_x: u16, cursor_y: u16) {
        write!(self.out, "{}", termion::cursor::Goto(cursor_x, cursor_y)).unwrap();
    }

    /// Hide the cursor (useful while rerendering to avoid flashing)
    pub fn hide_cursor(&mut self) {
        write!(self.out, "{}", termion::cursor::Hide).unwrap();
    }

    /// Show the cursor
    pub fn show_cursor(&mut self) {
        write!(self.out, "{}", termion::cursor::Show).unwrap();
    }
}
