use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    text::Line,
    widgets::{Block, Borders, Paragraph, Sparkline},
    Frame,
};
use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let avg = if app.cpu_history.is_empty() {
        0.0
    } else {
        let sum: f64 = app.cpu_history.iter()
            .filter_map(|h| h.last().copied())
            .sum();
        sum / app.cpu_history.len() as f64
    };

    let block = Block::default()
        .title(format!(" CPU  avg: {:.0}% ", avg))
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    if app.cpu_history.is_empty() {
        frame.render_widget(block, area);
        return;
    }

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let core_count = app.cpu_history.len();
    if core_count == 0 { return; }

    // 各コアを縦2行（ラベル行 + スパークライン行）に分割
    let col_constraints: Vec<Constraint> = (0..core_count)
        .map(|_| Constraint::Ratio(1, core_count as u32))
        .collect();
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(col_constraints)
        .split(inner);

    for (i, history) in app.cpu_history.iter().enumerate() {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(1)])
            .split(cols[i]);

        let last = history.last().copied().unwrap_or(0.0);
        let label = Paragraph::new(Line::from(format!("C{} {:.0}%", i, last)))
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(label, rows[0]);

        let data: Vec<u64> = history.iter().map(|&v| v as u64).collect();
        let sparkline = Sparkline::default()
            .data(&data)
            .style(Style::default().fg(Color::Green))
            .bar_set(symbols::bar::NINE_LEVELS);
        frame.render_widget(sparkline, rows[1]);
    }
}
