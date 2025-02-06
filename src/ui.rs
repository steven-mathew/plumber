use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Paragraph, Widget},
};

use ratatui::{symbols::border, widgets::Borders};
use tui_textarea::TextArea;

use crate::app::App;

#[derive(Debug, Default)]
pub struct UI<'a> {
    pub textarea: TextArea<'a>,
    output: String,
}

impl<'a> UI<'a> {
    pub fn new(app: &App) -> Self {
        let mut ui = Self::default();

        let mut textarea = TextArea::default();
        textarea.set_cursor_line_style(Style::default());
        textarea.set_block(Block::default().borders(Borders::ALL));

        ui.textarea = textarea;

        ui.update(app);
        ui
    }

    pub fn update(&mut self, app: &App) {
        self.textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::Blue),
        );

        self.output = app.output.clone().unwrap_or_default();
    }

    pub fn command_failed(&mut self) {
        self.textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::Red),
        );
    }

    pub fn textarea(&self) -> &TextArea<'a> {
        &self.textarea
    }

    pub fn mut_textarea(&mut self) -> &mut TextArea<'a> {
        &mut self.textarea
    }

    pub fn output(&self) -> &str {
        &self.output
    }

    pub fn command(&self) -> String {
        self.textarea.lines()[0].clone()
    }

    pub fn render(&mut self, area: Rect, buffer: &mut Buffer) {
        let layout = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(area);

        self.render_textarea(layout[0], buffer);
        self.render_layout(layout[1], buffer);
    }

    pub fn render_textarea(&mut self, area: Rect, buffer: &mut Buffer) {
        self.textarea.render(area, buffer);
    }

    fn render_layout(&mut self, area: Rect, buffer: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
            .border_set(border::PLAIN);

        let text = Text::raw(self.output.clone());

        Paragraph::new(text)
            .style(Style::default().fg(Color::White))
            .block(block)
            .render(area, buffer);
    }
}
