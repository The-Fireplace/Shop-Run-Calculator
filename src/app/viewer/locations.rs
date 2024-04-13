use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Constraint, Direction, Layout, Modifier, Style, Widget};
use ratatui::widgets::List;

use crate::app::{AppKeyHandler, AppViewBorderDetails, ControlGuide};
use crate::data::Database;

pub struct Locations<'life> {
    data: &'life Database,
}

impl Locations<'_> {
    pub fn new(data: &Database) -> Locations {
        Locations {
            data,
        }
    }
}

impl Widget for &Locations<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]).split(area);
        let data_reader = self.data.read().expect("Must be able to read database");
        let list = List::new(data_reader.get_all_locations().iter().map(|location| location.get_name()))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");
        list.render(layout[0], buf);
        //TODO location details on the right
        // Total possible profit?
        // List of shops in location?
    }
}

impl AppViewBorderDetails for &Locations<'_> {
    fn get_controls(self) -> Vec<ControlGuide> {
        vec![
            ControlGuide {
                instruction: "Up".to_string(),
                key_names: vec!["Up".to_string(), "W".to_string()],
            },
            ControlGuide {
                instruction: "Down".to_string(),
                key_names: vec!["Down".to_string(), "S".to_string()],
            },
        ]
    }
}

impl AppKeyHandler for &mut Locations<'_> {
    fn handle_key_event(self, key_code: KeyCode) {
        match key_code {
            KeyCode::Up | KeyCode::Char('w') => {
                //TODO move selection up
            }
            KeyCode::Down | KeyCode::Char('s') => {
                //TODO move selection down
            }
            _ => {}
        }
    }
}