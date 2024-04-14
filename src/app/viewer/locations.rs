use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Constraint, Direction, Layout, Widget, StatefulWidget};
use ratatui::widgets::{List, ListState};

use crate::app::{AppKeyHandler, AppViewBorderDetails, ControlGuide};
use crate::data::{Database, Location};
use crate::style;

pub struct Locations<'life> {
    data: &'life Database,
    active_locations: Vec<Location>,
    list_state: ListState,
}

impl Locations<'_> {
    pub fn new(data: &Database) -> Locations {
        let data_reader = data.read().expect("Must be able to read database");
        let active_locations = data_reader.get_all_locations().clone();
        let mut list_state = ListState::default();
        if active_locations.len() > 0 {
            list_state.select(Some(0));
        }

        Locations {
            data,
            active_locations,
            list_state,
        }
    }
}

impl Widget for &mut Locations<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]).split(area);
        let list = style::standard_list(
            List::new(self.active_locations.iter().map(|location| location.get_name()))
        );
        StatefulWidget::render(list, layout[0], buf, &mut self.list_state);
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
                self.list_state.select(self.list_state.selected().map(|i| {
                    if i > 0 {
                        i - 1
                    } else {
                        self.active_locations.len() - 1
                    }
                }));
            }
            KeyCode::Down | KeyCode::Char('s') => {
                self.list_state.select(self.list_state.selected().map(|i| {
                    if i < self.active_locations.len() - 1 {
                        i + 1
                    } else {
                        0
                    }
                }));
            }
            _ => {}
        }
    }
}