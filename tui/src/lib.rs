#[allow(dead_code)]
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

#[cfg(test)]
mod test {
    
    use crate::loading_gauge::create_title_with_filename;
    use crate::loading_gauge::LoadingGauge;
    use crate::loading_gauge::DOWNLOAD_STR;
    use super::*;
    #[test]
    fn check_terminal() {
        let _terminal = init();
        restore_terminal();
    }
    #[test]
    fn loading_gauge() {
        let tty = init();
        let name = "file.txt".to_owned();
        let status = DOWNLOAD_STR.to_owned();
        let title = create_title_with_filename(&name, &status);
        let rt = tokio::runtime::Builder::new_multi_thread()
            .build()
            .unwrap();
        let handle = rt.spawn_blocking(move || {
        
        let tx = LoadingGauge::new(title, tty);
        for i in 0..101 {
            tx.send(i as f64).unwrap();
        }
        });
        loop {
            if handle.is_finished() {
                break
            }
        }

    }
}
