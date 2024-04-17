use ratatui::prelude::{Color, Modifier, Style, Text};
use ratatui::widgets::List;

pub fn standard_list(list: List) -> List {
    list
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
}

pub fn selected_filter(text: Text) -> Text {
    text.patch_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan))
}