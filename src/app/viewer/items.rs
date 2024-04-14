use crossterm::event::KeyCode;
use ratatui::layout::Rect;
use ratatui::prelude::{Buffer, Constraint, Direction, Layout, StatefulWidget, Widget};
use ratatui::widgets::{List, ListState};
use crate::app::{AppKeyHandler, AppViewBorderDetails, ControlGuide};
use crate::data::{Database, Item};
use crate::style;

pub struct Items<'life> {
    data: &'life Database,
    active_items: Vec<Item>,
    list_state: ListState,
}

impl Items<'_> {
    pub fn new(data: &Database) -> Items {
        let data_reader = data.read().expect("Must be able to read database");
        let active_items = data_reader.get_all_items().clone();
        let mut list_state = ListState::default();
        if active_items.len() > 0 {
            list_state.select(Some(0));
        }

        Items {
            data,
            active_items,
            list_state,
        }
    }
}

impl Widget for &mut Items<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]).split(area);
        let list = style::standard_list(
            List::new(self.active_items.iter().map(|item| item.get_name()))
        );
        StatefulWidget::render(list, layout[0], buf, &mut self.list_state);
        //TODO item details on the right
        // Total possible profit?
        // List of shops with item?
    }
}

impl AppViewBorderDetails for &Items<'_> {
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

impl AppKeyHandler for &mut Items<'_> {
    fn handle_key_event(self, key: KeyCode) {
        match key {
            KeyCode::Up | KeyCode::Char('w') => {
                let selected = self.list_state.selected();
                if let Some(selected) = selected {
                    if selected > 0 {
                        self.list_state.select(Some(selected - 1));
                    }
                }
            }
            KeyCode::Down | KeyCode::Char('s') => {
                let selected = self.list_state.selected();
                if let Some(selected) = selected {
                    if selected < self.active_items.len() - 1 {
                        self.list_state.select(Some(selected + 1));
                    }
                }
            }
            _ => {}
        }
    }
}