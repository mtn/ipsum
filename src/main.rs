#![feature(extern_prelude)]
extern crate termion;

mod editor;
mod view;

fn main() {
    let mut editor = editor::Editor::new(Some("test.in"));
    editor.process_keypresses();
}
