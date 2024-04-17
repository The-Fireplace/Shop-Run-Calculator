use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Constraint, Direction, Layout, StatefulWidget, Text, Widget};
use ratatui::widgets::{List, ListItem, ListState};

use crate::app::{AppKeyHandler, AppViewBorderDetails, ControlGuide};
use crate::data::{Database, Location};
use crate::style;

pub struct Locations<'life> {
    data: &'life Database,
    active_locations: Vec<Location>,
    list_state: ListState,
}

impl<'life> Locations<'life> {
    pub fn new(data: &'life Database) -> Locations<'life> {
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

    fn is_active_filter_selected(&self) -> bool {
        match self.list_state.selected() {
            Some(selected) => match self.data.read() {
                Ok(data_reader) => match data_reader.get_filter().get_location() {
                    Some(location) => location == self.active_locations[selected].get_name(),
                    None => false,
                }
                Err(_) => false,
            }
            None => false,
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
        let mut active_filter = None;
        if let Ok(data_reader) = self.data.read() {
            active_filter = data_reader.get_filter().get_location().map(|location| location.to_string());
        }
        let list = style::standard_list(
            List::new(self.active_locations.iter().map(|location| {
                let mut text = Text::from(location.get_name());
                if let Some(active_filter) = &active_filter {
                    if active_filter == location.get_name() {
                        text = style::selected_filter(text);
                    }
                }
                ListItem::new(text)
            }))
        );
        StatefulWidget::render(list, layout[0], buf, &mut self.list_state);
        //TODO location details on the right
        // Total possible profit?
        // List of shops in location?
    }
}

impl AppViewBorderDetails for &Locations<'_> {
    fn get_controls(self) -> Vec<ControlGuide> {
        let select_location_text = match self.is_active_filter_selected() {
            true => "Clear Location",
            false => "Select Location",
        };
        vec![
            ControlGuide {
                instruction: "Up".to_string(),
                key_names: vec!["Up".to_string(), "W".to_string()],
            },
            ControlGuide {
                instruction: "Down".to_string(),
                key_names: vec!["Down".to_string(), "S".to_string()],
            },
            ControlGuide {
                instruction: select_location_text.to_string(),
                key_names: vec!["Enter".to_string()],
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
            KeyCode::Enter => {
                if let Some(selected) = self.list_state.selected() {
                    let is_active_filter_selected = self.is_active_filter_selected();
                    match self.data.write() {
                        Ok(mut data_writer) => {
                            if is_active_filter_selected {
                                data_writer.get_mut_filter().clear_location();
                            } else {
                                data_writer.get_mut_filter().set_location(&self.active_locations[selected]);
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
            _ => {}
        }
    }
}