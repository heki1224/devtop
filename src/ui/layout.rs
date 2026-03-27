use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct Areas {
    pub cpu: Rect,
    pub memory: Rect,
    pub process: Rect,
}

pub fn build(area: Rect) -> Areas {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(area);

    Areas {
        cpu: chunks[0],
        memory: chunks[1],
        process: chunks[2],
    }
}
