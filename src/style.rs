use ratatui::prelude::{Modifier, Style};
use ratatui::widgets::List;

pub fn standard_list(list: List) -> List {
    list
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
}