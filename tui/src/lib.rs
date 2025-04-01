pub mod loading_gauge;





use std::io::Stdout;

use ratatui::{self, prelude::CrosstermBackend, Terminal};


pub fn init() -> Terminal<CrosstermBackend<Stdout>> {
    let terminal = ratatui::init();
    terminal
}

pub fn restore_terminal() {
    ratatui::restore();
}
