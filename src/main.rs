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

use ratatui::crossterm::{
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
use app::App;

fn main() -> Result<(), Box<dyn Error>>{
    //===== TERMINAL SETUP =====//
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //===== APP ERSTELLEN =====//
    let mut app = App::new()?;

    //===== EVENT-LOOP AUSFÜHREN =====//
    let res = run_app(&mut terminal, &mut app);

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


fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), Box<dyn Error>> {
    loop {
        //===== UI ZEICHNEN =====//
        terminal.draw(|frame| ui::render(frame, app))?;

        //===== EVENTS VERARBEITEN =====//
        let event = event::read()?;
        
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') => app.quit(),
                _ => {
                    // Explorer events weiterleiten
                    app.file_explorer.handle(&event)?;
                }
            }
        }

        if app.should_quit {
            return Ok(())
        }
    }
}
