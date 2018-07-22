#![feature(extern_prelude)]
extern crate termion;
extern crate mio;

mod editor;
mod view;

fn main() {
    let mut editor = editor::Editor::new(Some("test.in"));
    editor.process_keypresses();
}
