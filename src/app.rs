use std::io;
use std::time::Duration;

use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Line, Span, Stylize, Widget};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders};
use ratatui::widgets::block::{Position, Title};

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

trait AppViewBorderDetails {
    fn get_controls(self) -> Vec<ControlGuide>;
}

trait AppKeyHandler {
    fn handle_key_event(self, key_code: KeyCode);
}

struct ControlGuide {
    instruction: String,
    key_names: Vec<String>,
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
                self.render_frame(frame);
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
    
    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let title = Title::from(" Shop Run Overview ".bold());
        let mut base_instructions = match &self.screen {
            Screen::Viewer(viewer) => viewer.get_controls(),
        };
        base_instructions.push(ControlGuide {
            instruction: "Quit".to_owned(),
            key_names: vec!["Esc".to_owned()],
        });
        let instructions = base_instructions.iter().flat_map(|control| {
            let mut name = String::new();
            name.push(' ');
            name.push_str(&control.instruction);
            name.push(' ');
            let mut controls = String::new();
            controls.push('<');
            controls.push_str(&control.key_names.join("|"));
            controls.push('>');
            if &control.instruction == &base_instructions.last().expect("We're inside an iter over this vec, so last must be defined.").instruction {
                controls.push(' ');
            }
            vec![
                name.into(),
                controls.blue().bold(),
            ]
        }).collect::<Vec<Span>>();

        let instructions = Title::from(Line::from(instructions));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);
        let screen_bounds = block.inner(area);
        block.render(area, buf);
        match &self.screen {
            Screen::Viewer(viewer) => viewer.render(screen_bounds, buf),
        }
    }
}