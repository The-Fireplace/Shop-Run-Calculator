use std::io;

use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

pub type Tui = Terminal<CrosstermBackend<io::Stdout>>;

pub fn setup() -> io::Result<Tui> {
    execute!(io::stdout(), EnterAlternateScreen,)?;
    enable_raw_mode()?;
    Ok(Terminal::new(CrosstermBackend::new(io::stdout()))?)
}

pub fn restore(
    terminal: &mut Tui,
) -> io::Result<()> {
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    disable_raw_mode()?;
    Ok(())
}