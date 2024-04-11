mod tui;
mod app;

use app::App;

fn main() -> std::io::Result<()> {
    let mut terminal = tui::setup()?;
    let app_result = App::new().run(&mut terminal);
    tui::restore(&mut terminal)?;
    app_result
}