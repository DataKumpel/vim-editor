//! # ui.rs
//!
//! This module handles the rendering with ratatui. Here the layout is defined and all widgets 
//! are being rendered, like text-area, status-bar and command-line.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Main layout: Explorer (~25%) | Editor (~75%)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 5), // Explorer
            Constraint::Ratio(4, 5), // Editor
        ])
        .split(area);

    render_explorer(frame, app, main_chunks[0]);

    // Editor area layout:
    let editor_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // Text Area
            Constraint::Length(1), // Status-bar
            Constraint::Length(1), // Command-line
        ])
        .split(main_chunks[1]);

    // Render components:
    render_text_area(frame, editor_chunks[0]);
    render_status_bar(frame, editor_chunks[1]);
    render_command_line(frame, editor_chunks[2]);
}

fn render_explorer(frame: &mut Frame, app: &App, area: Rect) {
    // Render file explorer widget:
    frame.render_widget(&app.file_explorer.widget(), area);
}

fn render_text_area(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Text Editor ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let text = Paragraph::new("Hier wird Text angezeigt...\n\nDrücke 'q' zum Beenden.")
        .block(block);

    frame.render_widget(text, area);
}

fn render_status_bar(frame: &mut Frame, area: Rect) {
   let status = Line::from(vec![
       " Normal ".into(),
       " | ".into(),
       "Line 1, Col 1".into(),
   ])
   .style(Style::default().bg(Color::DarkGray).fg(Color::White));

   frame.render_widget(Paragraph::new(status), area);
}

fn render_command_line(frame: &mut Frame, area: Rect) {
    let command = Paragraph::new("Some Command...")
        .style(Style::default());

    frame.render_widget(command, area);
}
