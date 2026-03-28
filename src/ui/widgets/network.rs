use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};
use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let header = Row::new(vec![
        Cell::from("INTERFACE"),
        Cell::from("RX/s"),
        Cell::from("TX/s"),
    ])
    .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = app.network_stats.iter().map(|n| {
        Row::new(vec![
            Cell::from(n.name.as_str()),
            Cell::from(format_bps(n.rx_bps)).style(Style::default().fg(Color::Green)),
            Cell::from(format_bps(n.tx_bps)).style(Style::default().fg(Color::Yellow)),
        ])
    }).collect();

    let table = Table::new(rows, [
        Constraint::Min(15),
        Constraint::Length(12),
        Constraint::Length(12),
    ])
    .header(header)
    .block(Block::default().title(" Network ").borders(Borders::ALL));

    frame.render_widget(table, area);
}

const KB: u64 = 1024;
const MB: u64 = 1024 * 1024;
const GB: u64 = 1024 * 1024 * 1024;

fn format_bps(bps: u64) -> String {
    if bps >= GB {
        format!("{:.1} GB/s", bps as f64 / GB as f64)
    } else if bps >= MB {
        format!("{:.1} MB/s", bps as f64 / MB as f64)
    } else if bps >= KB {
        format!("{:.0} KB/s", bps as f64 / KB as f64)
    } else {
        format!("{} B/s", bps)
    }
}
