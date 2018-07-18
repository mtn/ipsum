#![feature(extern_prelude)]
extern crate termion;
extern crate lazy_static;

// use termion::raw::IntoRawMode;
// use termion::async_stdin;
// use std::io::{Read, Write, stdout};
// use termion::event::parse_event;
// use termion::event::Key;
// use termion::event::Event;


mod editor;
mod view;

fn main() {
    let mut editor = editor::Editor::new();
    editor.process_keypresses();
}
