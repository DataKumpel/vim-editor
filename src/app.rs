//! # app.rs
//!
//! Contains the App-structure, that watches over the whole state of the application. This structure
//! orchestrates all the other modules and contains references for buffers, editor state
//! and UI states.

use ratatui::{prelude::Color, style::Style};
use ratatui_explorer::{FileExplorer, Theme};

pub struct App {
    pub should_quit: bool,
    pub file_explorer: FileExplorer,
}

impl App {
    pub fn new() -> Result<Self, std::io::Error> {
        let theme = Theme::default()
            .with_title_top(|_fe| format!("[ Explorer ]").into())
            .with_style(Style::default().fg(Color::Cyan))
            .with_highlight_symbol("> ".into());
        
        let file_explorer = FileExplorer::with_theme(theme)?;
        
        Ok(Self {
            should_quit: false,
            file_explorer,
        })
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
