use crossterm::event::KeyCode;
use ratatui::prelude::{Buffer, Constraint, Direction, Layout, Rect, StatefulWidget, Widget};
use ratatui::widgets::{List, ListState};
use crate::app::{AppKeyHandler, AppViewBorderDetails};
use crate::data::{Database, Shop};
use crate::style;

pub struct Shops<'life> {
    data: &'life Database,
    active_shops: Vec<Shop>,
    list_state: ListState,
}

impl Shops<'_> {
    pub fn new(data: &Database) -> Shops {
        let data_reader = data.read().expect("Must be able to read database");
        let active_shops = data_reader.get_all_shops().clone();
        let mut list_state = ListState::default();
        if active_shops.len() > 0 {
            list_state.select(Some(0));
        }

        Shops {
            data,
            active_shops,
            list_state,
        }
    }
}

impl Widget for &mut Shops<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]).split(area);
        let list = style::standard_list(
            List::new(self.active_shops.iter().map(|shop| shop.get_name()))
        );
        StatefulWidget::render(list, layout[0], buf, &mut self.list_state);
        //TODO shop details on the right
        // Total possible profit?
        // List of items in shop?
    }
}

impl AppViewBorderDetails for &Shops<'_> {
    fn get_controls(self) -> Vec<crate::app::ControlGuide> {
        vec![
            crate::app::ControlGuide {
                instruction: "Up".to_string(),
                key_names: vec!["Up".to_string(), "W".to_string()],
            },
            crate::app::ControlGuide {
                instruction: "Down".to_string(),
                key_names: vec!["Down".to_string(), "S".to_string()],
            },
        ]
    }
}

impl AppKeyHandler for &mut Shops<'_> {
    fn handle_key_event(self, key_code: KeyCode) {
        match key_code {
            KeyCode::Up | KeyCode::Char('w') => {
                self.list_state.select(self.list_state.selected().map(|i| {
                    if i > 0 {
                        i - 1
                    } else {
                        self.active_shops.len() - 1
                    }
                }));
            }
            KeyCode::Down | KeyCode::Char('s') => {
                self.list_state.select(self.list_state.selected().map(|i| {
                    if i < self.active_shops.len() - 1 {
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