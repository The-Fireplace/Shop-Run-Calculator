use std::error::Error;
use std::io;
use std::time::Duration;

use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::{Stylize, Widget};

use crate::app::viewer::Viewer;
use crate::tui::Tui;

mod viewer;

pub struct App {
    screen: Screen,
    exit: bool,
}

enum Screen {
    Viewer(Viewer),
}

trait KeyEventHandler {
    fn handle_key_event(self, key_code: KeyCode);
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
                match &self.screen {
                    Screen::Viewer(viewer) => {
                        frame.render_widget(viewer, frame.size())
                    }
                }
            })?;

            if event::poll(Duration::from_millis(10))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press
                    {
                        let key_code = key.code;
                        if key_code == KeyCode::Esc {
                            self.exit = true;
                        } else {
                            match &mut self.screen {
                                Screen::Viewer(viewer) => viewer.handle_key_event(key_code)
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}