//! # main.rs
//!
//! This is the entry point. Initialize the terminal, start the event loop and handle errors upon
//! start. Here the ratatui terminal instance is created and its mainloop is started.

pub mod app;
pub mod buffer;
pub mod commands;
pub mod config;
pub mod editor;
pub mod input;
pub mod ui;

fn main() {
    println!("Hello, world!");
}
