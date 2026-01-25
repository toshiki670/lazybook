// TUI Application state machine

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    Frame, Terminal,
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::asset_management::application::asset_service::AssetService;
use crate::asset_management::domain::asset::{Asset, AssetRepository};

/// Application state
pub enum AppState {
    AssetList,
    AssetForm,
    Exit,
}

/// TUI Application
pub struct App<R: AssetRepository> {
    state: AppState,
    asset_service: AssetService<R>,
    // Asset list state
    assets: Vec<Asset>,
    selected_index: usize,
    // Asset form state
    form_name_input: String,
}

impl<R: AssetRepository> App<R> {
    /// Create new App
    pub fn new(asset_service: AssetService<R>) -> Self {
        Self {
            state: AppState::AssetList,
            asset_service,
            assets: Vec::new(),
            selected_index: 0,
            form_name_input: String::new(),
        }
    }

    /// Run the application
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()>
    where
        <B as Backend>::Error: Send + Sync + 'static,
    {
        // Load initial data
        self.refresh_assets()?;

        loop {
            terminal.draw(|f| self.render(f))?;

            if let Event::Key(key) = event::read()? {
                self.handle_key(key)?;

                if matches!(self.state, AppState::Exit) {
                    break;
                }
            }
        }

        Ok(())
    }

    /// Refresh asset list from database
    fn refresh_assets(&mut self) -> Result<()> {
        self.assets = self.asset_service.list_assets()?;
        Ok(())
    }

    /// Render the UI
    fn render(&self, f: &mut Frame) {
        match self.state {
            AppState::AssetList => self.render_asset_list(f),
            AppState::AssetForm => self.render_asset_form(f),
            AppState::Exit => {}
        }
    }

    /// Render asset list screen
    fn render_asset_list(&self, f: &mut Frame) {
        let size = f.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(size);

        // Title
        let title = Paragraph::new("lazybook - 資産管理")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Asset list
        let items: Vec<ListItem> = self
            .assets
            .iter()
            .enumerate()
            .map(|(i, asset)| {
                let content = format!("{}: {}", i + 1, asset.name);
                let style = if i == self.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(content).style(style)
            })
            .collect();

        let title_text = format!("資産一覧 (全 {} 件)", self.assets.len());
        let list = List::new(items).block(Block::default().borders(Borders::ALL).title(title_text));
        f.render_widget(list, chunks[1]);

        // Help
        let help = Paragraph::new("n: 新規登録 | q: 終了")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(help, chunks[2]);
    }

    /// Render asset form screen
    fn render_asset_form(&self, f: &mut Frame) {
        let size = f.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(size);

        // Title
        let title = Paragraph::new("資産登録")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Form
        let input = Paragraph::new(self.form_name_input.as_str())
            .style(Style::default())
            .block(Block::default().borders(Borders::ALL).title("名前"));
        f.render_widget(input, chunks[1]);

        // Help
        let help = Paragraph::new("Enter: 保存 | Esc: キャンセル")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(help, chunks[2]);
    }

    /// Handle keyboard input
    fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        match self.state {
            AppState::AssetList => self.handle_key_asset_list(key)?,
            AppState::AssetForm => self.handle_key_asset_form(key)?,
            AppState::Exit => {}
        }
        Ok(())
    }

    /// Handle keys in asset list screen
    fn handle_key_asset_list(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') => {
                self.state = AppState::Exit;
            }
            KeyCode::Char('n') => {
                self.form_name_input.clear();
                self.state = AppState::AssetForm;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if !self.assets.is_empty() && self.selected_index < self.assets.len() - 1 {
                    self.selected_index += 1;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Handle keys in asset form screen
    fn handle_key_asset_form(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.state = AppState::AssetList;
            }
            KeyCode::Enter => {
                if !self.form_name_input.trim().is_empty() {
                    // Create asset
                    self.asset_service.create_asset(
                        self.form_name_input.clone(),
                        None,
                        None,
                        None,
                        vec![],
                    )?;

                    // Refresh and return to list
                    self.refresh_assets()?;
                    self.selected_index = self.assets.len().saturating_sub(1);
                    self.state = AppState::AssetList;
                }
            }
            KeyCode::Char(c) => {
                self.form_name_input.push(c);
            }
            KeyCode::Backspace => {
                self.form_name_input.pop();
            }
            _ => {}
        }
        Ok(())
    }
}
