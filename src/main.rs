#![feature(extern_prelude)]
extern crate lazy_static;
extern crate termion;

mod editor;
mod view;

fn main() {
    let mut editor = editor::Editor::new();
    editor.process_keypresses();
}
