use std::time::{Duration, Instant};
use std::{sync::mpsc, thread};

use color_eyre::eyre::Result;
use crossterm::event::MouseEvent;
use tui_textarea::Input;

#[derive(Clone, Debug)]
pub enum Message {
    Tick,
    Input(Input),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MessageHandler {
    tx: mpsc::Sender<Message>,
    rx: mpsc::Receiver<Message>,
    thread: thread::JoinHandle<()>,
}

impl MessageHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (tx, rx) = mpsc::channel();
        let thread = {
            let tx = tx.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if crossterm::event::poll(timeout).unwrap() {
                        match crossterm::event::read().unwrap().into() {
                            input => tx.send(Message::Input(input)),
                            // TODO: handle other messages
                        }
                        .expect("failed to send message");
                    }

                    if last_tick.elapsed() >= tick_rate {
                        tx.send(Message::Tick).expect("failed to send tick");
                        last_tick = Instant::now();
                    }
                }
            })
        };

        Self { tx, rx, thread }
    }

    pub fn next(&self) -> Result<Message> {
        Ok(self.rx.recv()?)
    }
}
