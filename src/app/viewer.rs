mod locations;
mod shops;
mod items;

use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::{Rect};
use ratatui::prelude::{Stylize, Widget};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Tabs};

use crate::app::{AppKeyHandler, AppViewBorderDetails, ControlGuide};

enum Tab {
    Locations,
    Shops,
    Items,
}

pub struct Viewer {
    tab: Tab,
}

impl Viewer {
    pub fn new() -> Viewer {
        Viewer {
            tab: Tab::Locations,
        }
    }
}

impl Widget for &Viewer {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let tabs = Tabs::new(vec!["Locations", "Shops", "Items"])
            .highlight_style(Style::default().bg(Color::White).fg(Color::Blue).slow_blink())
            .style(Style::default().white())
            .select(0);
        //TODO render tab contents
        tabs.render(area, buf)
    }
}

impl AppViewBorderDetails for &Viewer {
    fn get_controls(self) -> Vec<ControlGuide> {
        let mut base_instructions = vec![];
        //TODO add tab instructions
        base_instructions.push(ControlGuide {
            instruction: "Switch Tabs".to_owned(),
            key_names: vec!["Tab".to_owned()],
        });

        base_instructions
    }
}

impl AppKeyHandler for &mut Viewer {
    fn handle_key_event(self, key_code: KeyCode) {
        match key_code {
            //TODO handle tab switching
            _ => {},
        }
    }
}