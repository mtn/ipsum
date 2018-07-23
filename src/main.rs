#![feature(extern_prelude)]
extern crate ropey;
extern crate termion;

mod editor;
mod buffer;
mod view;

fn main() {
    let mut editor = editor::Editor::new(Some("test.in"));
    // let mut editor = editor::Editor::new(None);
    editor.process_keypresses();
}
