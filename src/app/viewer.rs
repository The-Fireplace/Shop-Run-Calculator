use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Constraint, Direction, Layout, Stylize, Widget};
use ratatui::style::{Color, Style};
use ratatui::widgets::Tabs;

use crate::app::{AppKeyHandler, AppViewBorderDetails, ControlGuide};
use crate::app::viewer::items::Items;
use crate::app::viewer::locations::Locations;
use crate::app::viewer::shops::Shops;
use crate::data::Database;

mod locations;
mod shops;
mod items;

enum Tab<'life> {
    Locations(Locations<'life>),
    Shops(Shops<'life>),
    Items(Items<'life>),
}

impl<'life> Tab<'life> {
    fn next(&self, data: &'life Database) -> Tab<'life> {
        match self {
            Tab::Locations(_) => Tab::Shops(Shops::new(data)),
            Tab::Shops(_) => Tab::Items(Items::new(data)),
            Tab::Items(_) => Tab::Locations(Locations::new(data)),
        }
    }

    fn as_index(&self) -> usize {
        match self {
            Tab::Locations(_) => 0,
            Tab::Shops(_) => 1,
            Tab::Items(_) => 2,
        }
    }

    fn labels() -> Vec<&'static str> {
        vec!["Locations", "Shops", "Items"]
    }
}

pub struct Viewer<'life> {
    tab: Tab<'life>,
    data: &'life Database,
}

impl<'life> Viewer<'life> {
    pub fn new(data: &'life Database) -> Self {
        Viewer {
            tab: Tab::Locations(Locations::new(data)),
            data,
        }
    }
}

impl Widget for &mut Viewer<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Min(0),
            ]).split(area);
        let tabs = Tabs::new(Tab::labels())
            .highlight_style(Style::default().bg(Color::White).fg(Color::Blue).slow_blink())
            .style(Style::default().white())
            .select(self.tab.as_index());
        tabs.render(layout[0], buf);
        let child_area = layout[1];
        match &mut self.tab {
            Tab::Locations(locations) => locations.render(child_area, buf),
            Tab::Shops(shops) => shops.render(child_area, buf),
            Tab::Items(items) => items.render(child_area, buf),
        }
    }
}

impl AppViewBorderDetails for &Viewer<'_> {
    fn get_controls(self) -> Vec<ControlGuide> {
        let mut base_instructions = match &self.tab {
            Tab::Locations(locations) => {
                locations.get_controls()
            }
            Tab::Shops(shops) => {
                shops.get_controls()
            }
            Tab::Items(items) => {
                items.get_controls()
            }
        };
        base_instructions.push(ControlGuide {
            instruction: "Switch Tabs".to_owned(),
            key_names: vec!["Tab".to_owned()],
        });

        base_instructions
    }
}

impl AppKeyHandler for &mut Viewer<'_> {
    fn handle_key_event(self, key_code: KeyCode) {
        match key_code {
            KeyCode::Tab => {
                self.tab = self.tab.next(self.data);
            },
            _ => {
                match &mut self.tab {
                    Tab::Locations(locations) => locations.handle_key_event(key_code),
                    Tab::Shops(shops) => shops.handle_key_event(key_code),
                    Tab::Items(items) => items.handle_key_event(key_code),
                }
            },
        }
    }
}