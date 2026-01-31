// Presentation layer - TUI

pub mod app;
pub mod components;
pub mod events;
pub mod screens;

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use std::path::PathBuf;

use crate::asset_management::application::asset_service::AssetService;
use crate::asset_management::infrastructure::persistence::{
    SqliteAssetRepository, establish_connection, initialize_database,
};

/// Run the TUI application
pub fn run() -> Result<()> {
    // Setup database
    let db_path = get_database_path()?;
    let mut conn = establish_connection(&db_path)?;
    initialize_database(&mut conn)?;

    // Create repository and service
    let repository = SqliteAssetRepository::new(conn);
    let service = AssetService::new(repository);

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let mut app = app::App::new(service);
    let res = app.run(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(e) = res {
        eprintln!("Error: {:?}", e);
    }

    Ok(())
}

/// Get database file path
fn get_database_path() -> Result<PathBuf> {
    // Use current directory for now (later: use XDG or user's home directory)
    let mut path = std::env::current_dir()?;
    path.push("lazybook.db");
    Ok(path)
}
