mod viewer;

use std::error::Error;
use std::io;
use std::time::Duration;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::prelude::Stylize;
use ratatui::widgets::Paragraph;
use crate::app::viewer::Viewer;
use crate::tui::Tui;

enum Screen {
    Viewer(Viewer),
}

pub struct App {
    screen: Screen,
    exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            screen: Screen::Viewer(Viewer::new()),
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| {
                let area = frame.size();
                frame.render_widget(
                    Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                        .white()
                        .on_blue(),
                    area,
                );
            })?;

            if event::poll(Duration::from_millis(10))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press
                        && key.code == KeyCode::Char('q')
                    {
                        self.exit = true;
                    }
                }
            }
        }

        Ok(())
    }
}