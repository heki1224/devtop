use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct Areas {
    pub cpu: Rect,
    pub memory: Rect,
    pub network: Rect,
    pub process: Rect,
    pub docker: Option<Rect>,
}

pub fn build(area: Rect, docker_available: bool) -> Areas {
    if docker_available {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),
                Constraint::Length(5),
                Constraint::Length(7),
                Constraint::Min(0),
                Constraint::Length(8),
            ])
            .split(area);
        Areas {
            cpu: chunks[0],
            memory: chunks[1],
            network: chunks[2],
            process: chunks[3],
            docker: Some(chunks[4]),
        }
    } else {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),
                Constraint::Length(5),
                Constraint::Length(7),
                Constraint::Min(0),
            ])
            .split(area);
        Areas {
            cpu: chunks[0],
            memory: chunks[1],
            network: chunks[2],
            process: chunks[3],
            docker: None,
        }
    }
}
