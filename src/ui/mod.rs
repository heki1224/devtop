pub mod layout;
pub mod widgets;

use ratatui::Frame;
use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let areas = layout::build(frame.area(), app.docker_available);
    widgets::cpu::render(frame, areas.cpu, app);
    widgets::memory::render(frame, areas.memory, app);
    widgets::network::render(frame, areas.network, app);
    widgets::process::render(frame, areas.process, app);
    if let Some(docker_area) = areas.docker {
        widgets::docker::render(frame, docker_area, app);
    }
}
