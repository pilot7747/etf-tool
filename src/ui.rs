use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::etf::ETF;

pub fn render(frame: &mut Frame, etfs: &[ETF], selected_index: usize) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(0),     // Content
        ])
        .split(frame.area());

    render_title(frame, main_layout[0]);
    render_etf_table(frame, main_layout[1], etfs, selected_index);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new("ETF Explorer")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, area);
}

fn render_etf_table(frame: &mut Frame, area: Rect, etfs: &[ETF], selected_index: usize) {
    let header_cells = ["Name", "ISIN", "Asset Class", "TER", "Currency", "AUM", "1Y Perf", "YTD Perf"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));

    let rows: Vec<Row> = etfs.iter().enumerate().map(|(idx, etf)| {
        let style = if idx == selected_index {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        };

        let perf_1y = etf.performance_1y.map_or("N/A".to_string(), |p| format!("{:.2}%", p));
        let perf_ytd = etf.performance_ytd.map_or("N/A".to_string(), |p| format!("{:.2}%", p));

        Row::new(vec![
            Cell::from(etf.name.clone()),
            Cell::from(etf.isin.clone()),
            Cell::from(etf.asset_class.clone()),
            Cell::from(format!("{:.2}%", etf.ter)),
            Cell::from(etf.currency.clone()),
            Cell::from(etf.aum.clone()),
            Cell::from(perf_1y),
            Cell::from(perf_ytd),
        ]).style(style)
    }).collect();

    let widths = [
        Constraint::Percentage(25),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(8),
        Constraint::Percentage(8),
        Constraint::Percentage(12),
        Constraint::Percentage(8),
        Constraint::Percentage(9),
    ];

    let table = Table::new(rows, widths)
        .header(Row::new(header_cells))
        .block(Block::default().borders(Borders::ALL).title("ETF List"))
        .column_spacing(1)
        .row_highlight_style(Style::default().bg(Color::DarkGray));

    frame.render_widget(table, area);
} 