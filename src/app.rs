//! # app.rs
//!
//! Contains the App-structure, that watches over the whole state of the application. This structure
//! orchestrates all the other modules and contains references for buffers, editor state
//! and UI states.

use ratatui_explorer::FileExplorer;

pub struct App {
    pub should_quit: bool,
    pub file_explorer: FileExplorer,
}

impl App {
    pub fn new() -> Result<Self, std::io::Error> {
        let file_explorer = FileExplorer::new()?;
        
        Ok(Self {
            should_quit: false,
            file_explorer,
        })
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
