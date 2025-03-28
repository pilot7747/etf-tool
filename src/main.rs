use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;

mod etf;
mod ui;
mod utils;
mod xtrackers;
mod ishares;
mod invesco;

use etf::ETF;

struct App {
    etfs: Vec<ETF>,
    selected_index: usize,
}

impl App {
    fn new() -> Result<Self> {
        let mut xtrackers_etfs = xtrackers::get_xtrackers_etfs()?;
        let ishares_etfs = ishares::get_ishares_etfs()?;
        let invesco_etfs = invesco::get_invesco_etfs()?;
        
        // Combine all ETF lists
        let mut all_etfs = Vec::new();
        all_etfs.extend(xtrackers_etfs);
        all_etfs.extend(ishares_etfs);
        all_etfs.extend(invesco_etfs);
        
        Ok(Self {
            etfs: all_etfs,
            selected_index: 0,
        })
    }

    fn next(&mut self) {
        self.selected_index = (self.selected_index + 1).min(self.etfs.len() - 1);
    }

    fn previous(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = App::new()?;

    loop {
        terminal.draw(|frame| ui::render(frame, &app.etfs, app.selected_index))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                KeyCode::Down | KeyCode::Char('j') => app.next(),
                KeyCode::Up | KeyCode::Char('k') => app.previous(),
                _ => {}
            }
        }
    }
}
