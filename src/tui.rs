use std::io;

use color_eyre::eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::Backend, Terminal};

use crate::{message::MessageHandler, ui::UI};

pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
    pub messages: MessageHandler,
}

impl<B: Backend> Tui<B> {
    pub fn new(terminal: Terminal<B>, messages: MessageHandler) -> Self {
        Self { terminal, messages }
    }

    pub fn init(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        // self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw(&mut self, ui: &mut UI) -> Result<()> {
        self.terminal
            .draw(|frame| ui.render(frame.area(), frame.buffer_mut()))?;

        Ok(())
    }

    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        // self.terminal.show_cursor()?;
        Ok(())
    }
}
