use std::io::Write;
use std::io;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

pub struct View {
    pub out: RawTerminal<io::BufWriter<io::Stdout>>,
}

impl View {
    pub fn new() -> View {
        let mut view = View {
            out: io::BufWriter::with_capacity(1 << 14, io::stdout())
                .into_raw_mode()
                .unwrap()
        };

        // Clear the display
        write!(view.out,
               "{}{}",
               termion::clear::All,
               termion::cursor::Goto(1, 1))
                .unwrap();

        view
    }

    pub fn write(&mut self, s: String) {
        write!(self.out, "{}", s).unwrap();
        self.out.flush().unwrap();
    }
}
