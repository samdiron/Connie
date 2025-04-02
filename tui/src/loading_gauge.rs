use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use ratatui::buffer::Buffer;
use ratatui::layout::Constraint::Ratio;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::style::palette::tailwind;
use ratatui::style::Color;
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui::widgets::Gauge;
use ratatui::widgets::Widget;
use ratatui::DefaultTerminal;
use ratatui::Frame;

 pub const DOWNLOAD: Color = tailwind::LIME.c600;
 pub const UPLOAD: Color = tailwind::VIOLET.c600;
 pub const DOWNLOAD_STR: &str = "Downloading";
 pub const UPLOAD_STR: &str = "Uploading";


pub fn create_title_with_filename(
    filename: &String,
    status: &String
) -> String {
    format!(" {status}: {filename} ... ")
}


pub struct LoadingGauge {
    title: String,
    is_finished: bool,
    percent:Arc<Mutex<Receiver<f64>>>,
}

impl LoadingGauge {
    pub fn new(
        title: String,
        tty: DefaultTerminal
    ) -> Sender<f64> {
        let (tx, rx) = channel::<f64>();
        let bind = Mutex::new(rx);
        let rx = Arc::new(bind);
        let gauge = LoadingGauge {
            title,
            is_finished: false,
            percent: rx
        };
        let _handle = tokio::task::spawn(async move {
            let mut tty = tty;
            while !gauge.is_finished {
                tty.draw(|frame| gauge.draw(frame))
                    .expect("could not draw LoadingGauge");
            }
            
        });
        tx
    }
    fn draw(self: &Self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    } 
}


impl Widget for &LoadingGauge {
    fn render(
        self,
        area: Rect,
        buf: &mut Buffer ) {
        let inner_title = self.title.as_str();
        let color_accent = if inner_title.contains(DOWNLOAD_STR) {
            DOWNLOAD
        }else {
            UPLOAD
        };
        let title = Line::from(inner_title);
        let block = Block::bordered()
            .title(title.left_aligned())
            .border_set(border::DOUBLE);
        let bind_to_rx = self.percent.clone();
        let rx = bind_to_rx.lock().unwrap();
        let recving = rx.recv();
        let layout = Layout::vertical([Ratio(1, 4); 1]);
        let [gauge_area] = layout.areas(area);
        if recving.is_ok() {
            let recived = recving.unwrap();
            let percent = recived.round() as u16;
            if percent < 100 {
                let gauge = Gauge::default()
                    .percent(percent)
                    .block(block)
                    .style(color_accent);
                gauge.render(gauge_area, buf)
            };
        };
    }
}
