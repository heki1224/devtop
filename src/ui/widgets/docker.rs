use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};
use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let header = Row::new(vec![
        Cell::from("NAME"),
        Cell::from("STATUS"),
        Cell::from("CPU%"),
        Cell::from("MEMORY"),
    ])
    .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = app.containers.iter().map(|c| {
        let mem_display = if c.memory_limit > 0 {
            format!(
                "{} / {}",
                format_bytes(c.memory_bytes),
                format_bytes(c.memory_limit)
            )
        } else {
            "N/A".to_string()
        };
        let status_style = if c.status.contains("running") {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        Row::new(vec![
            Cell::from(c.name.as_str()),
            Cell::from(c.status.as_str()).style(status_style),
            Cell::from(format!("{:.1}%", c.cpu_percent)),
            Cell::from(mem_display),
        ])
    }).collect();

    let title = format!(" Docker Containers ({}) ", app.containers.len());
    let table = Table::new(rows, [
        Constraint::Min(20),
        Constraint::Length(10),
        Constraint::Length(7),
        Constraint::Min(20),
    ])
    .header(header)
    .block(Block::default().title(title).borders(Borders::ALL));

    frame.render_widget(table, area);
}

const KB: u64 = 1024;
const MB: u64 = 1024 * 1024;
const GB: u64 = 1024 * 1024 * 1024;

fn format_bytes(bytes: u64) -> String {
    if bytes >= GB {
        format!("{:.1}GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.0}MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.0}KB", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}
