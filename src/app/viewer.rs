use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Line, Span, Stylize, Widget};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders};
use ratatui::widgets::block::{Position, Title};

use crate::app::KeyEventHandler;

struct ControlGuide {
    instruction: String,
    key_names: Vec<String>,
}

pub struct Viewer {
    
}

impl Viewer {
    pub fn new() -> Viewer {
        Viewer {
            
        }
    }
}

impl Widget for &Viewer {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let title = Title::from(" Shop Run Overview ".bold());
        let mut base_instructions = vec![];
        //TODO per-tab controls
        base_instructions.push(ControlGuide {
            instruction: "Switch Tabs".to_owned(),
            key_names: vec!["Tab".to_owned()],
        });
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
        block.render(area, buf)
    }
}

impl KeyEventHandler for &mut Viewer {
    fn handle_key_event(self, key_code: KeyCode) {
        
        match key_code {
            
            _ => {},
        }
    }
}