use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};
use crate::app::App;

pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let filtered = app.filtered_processes();

    let header = Row::new(vec![
        Cell::from("Icon"),
        Cell::from("PID"),
        Cell::from("Name"),
        Cell::from("CPU%"),
        Cell::from("MEM(MB)"),
    ]).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = filtered.iter().map(|p| {
        Row::new(vec![
            Cell::from(p.process_type.icon()),
            Cell::from(p.pid.to_string()),
            Cell::from(p.name.clone()),
            Cell::from(format!("{:.1}", p.cpu_usage)),
            Cell::from(format!("{}", p.memory_kb / 1024)),
        ])
    }).collect();

    let filter_hint = if app.filter.is_empty() {
        format!(" Processes [sort: {}] (s=sort, /=filter, q=quit) ", app.sort_mode.label())
    } else {
        format!(" Processes [filter: {}] ", app.filter)
    };

    let table = Table::new(rows, [
        Constraint::Length(4),
        Constraint::Length(7),
        Constraint::Min(20),
        Constraint::Length(7),
        Constraint::Length(9),
    ])
    .header(header)
    .block(Block::default().title(filter_hint).borders(Borders::ALL))
    .row_highlight_style(Style::default().bg(Color::DarkGray))
    .highlight_symbol(">> ");

    let mut state = TableState::default();
    state.select(Some(app.selected.min(filtered.len().saturating_sub(1))));
    frame.render_stateful_widget(table, area, &mut state);
}
