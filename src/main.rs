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

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend, prelude::Backend, widgets::Paragraph, Terminal
};
use std::{
    error::Error,
    io,
};

fn main() -> Result<(), Box<dyn Error>>{
    //===== TERMINAL SETUP =====//
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //===== EVENT-LOOP AUSFÜHREN =====//
    let res = run_app(&mut terminal);

    //===== TERMINAL CLEANUP =====//
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    //===== SHOW ERRORS =====//
    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}


fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    loop {
        //===== UI ZEICHNEN =====//
        terminal.draw(|frame| {
            let area = frame.area();
            let text = Paragraph::new("Hello VIM Editor! Press 'q' to quit.");
            frame.render_widget(text, area);
        })?;

        //===== EVENTS VERARBEITEN =====//
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
