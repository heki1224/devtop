use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge},
    Frame,
};
use crate::app::App;

fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0 * 1024.0)
}

pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let block = Block::default()
        .title(" Memory ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Magenta));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(inner);

    let mem = &app.memory;

    let ram_ratio = if mem.total > 0 { mem.used as f64 / mem.total as f64 } else { 0.0 };
    let ram_gauge = Gauge::default()
        .label(format!("RAM  {:.1}/{:.1} GB", bytes_to_gb(mem.used), bytes_to_gb(mem.total)))
        .ratio(ram_ratio.clamp(0.0, 1.0))
        .style(Style::default().fg(Color::Yellow));
    frame.render_widget(ram_gauge, chunks[0]);

    let swap_ratio = if mem.swap_total > 0 { mem.swap_used as f64 / mem.swap_total as f64 } else { 0.0 };
    let swap_gauge = Gauge::default()
        .label(format!("Swap {:.1}/{:.1} GB", bytes_to_gb(mem.swap_used), bytes_to_gb(mem.swap_total)))
        .ratio(swap_ratio.clamp(0.0, 1.0))
        .style(Style::default().fg(Color::Blue));
    frame.render_widget(swap_gauge, chunks[1]);
}
