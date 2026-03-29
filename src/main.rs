mod app;
mod collector;
mod types;
mod ui;

use anyhow::Result;
use app::App;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    // パニック時のターミナル復元
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel(100);

    tokio::spawn(collector::cpu::start(tx.clone()));
    tokio::spawn(collector::memory::start(tx.clone()));
    tokio::spawn(collector::process::start(tx.clone()));
    tokio::spawn(collector::network::start(tx.clone()));
    tokio::spawn(collector::disk::start(tx.clone()));
    tokio::spawn(collector::docker::start(tx.clone()));

    let mut app = App::new(rx);

    loop {
        app.tick();
        terminal.draw(|frame| ui::draw(frame, &app))?;

        if let Some(key) = app.poll_event()? {
            app.handle_key(key);
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
