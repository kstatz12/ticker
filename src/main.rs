pub mod config;
pub mod finance;

use crate::config::Config;

use cursive::traits::*;
use cursive::Vec2;
use cursive::{Cursive, Printer};
use cursive::theme::{Color, PaletteColor, Theme};
use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::env;

fn main() {
    let config_path_key = "TICKER_CONFIG";
    let config_path = match env::var(config_path_key) {
        Ok(val) => val,
        Err(e) => panic!(e)
    };

    let config = Config::new(&config_path);
    let buff_size = config.symbols.len();

    let mut siv = cursive::default();
    let theme = custom_theme_from_cursive(&siv);
    siv.set_theme(theme);
    let cb_sink = siv.cb_sink().clone();

    siv.add_global_callback('q', |s| s.quit());

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        update(&tx, cb_sink, config);
    });

    siv.add_layer(BufferView::new(buff_size, rx).full_screen());
    siv.run();
}

fn custom_theme_from_cursive(siv: &Cursive) -> Theme {
    // We'll return the current theme with a small modification.
    let mut theme = siv.current_theme().clone();

    theme.palette[PaletteColor::Background] = Color::TerminalDefault;

    theme
}


fn update(tx: &mpsc::Sender<String>, cb_sink: cursive::CbSink, config: Config) {
    loop {
        let stocks = finance::refresh_symbols(&config);

        for s in stocks.iter() {
            if tx.send(s.to_string()).is_err() {
                return;
            }
        }
        cb_sink.send(Box::new(Cursive::noop)).unwrap();
        thread::sleep(Duration::from_secs(config.interval * 60));
    }
}

struct BufferView {
    buffer: VecDeque<String>,
    rx: mpsc::Receiver<String>
}

impl BufferView {
    fn new(size: usize, rx: mpsc::Receiver<String>) -> Self {
        let mut buffer = VecDeque::<String>::new();
        buffer.resize(size, String::new());
        BufferView { rx, buffer }
    }

    fn update(&mut self) {
        while let Ok(stock) = self.rx.try_recv() {
            self.buffer.push_back(stock);
            self.buffer.pop_front();
        }
    }
}

impl View for BufferView {
    fn layout(&mut self, _: Vec2) {
        self.update();
    }

    fn draw(&self, printer: &Printer) {
        for (i, stock) in
            self.buffer.iter().rev().take(printer.size.y).enumerate()
        {
            printer.print((0, printer.size.y - 1 - i), stock);
        }
    }
}
