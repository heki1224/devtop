pub mod layout;
pub mod widgets;

use ratatui::Frame;
use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let areas = layout::build(frame.area());
    widgets::cpu::render(frame, areas.cpu, app);
    widgets::memory::render(frame, areas.memory, app);
    widgets::process::render(frame, areas.process, app);
}
