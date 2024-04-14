mod tui;
mod app;
mod data;
mod style;

use std::sync::{Arc, RwLock};
use app::App;
use crate::data::Data;

fn main() -> std::io::Result<()> {
    let mut terminal = tui::setup()?;
    let data = Arc::new(RwLock::new(Data::new()));
    let app_result = App::new(&data).run(&mut terminal);
    tui::restore(&mut terminal)?;
    app_result
}
