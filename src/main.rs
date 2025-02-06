use std::io;

use arboard::Clipboard;
use color_eyre::eyre::Result;
use plumber::{
    app::{App, Args},
    handler::{handle_enter, handle_input},
    message::{Message, MessageHandler},
    tui::Tui,
    ui::UI,
};

use tui_textarea::{Input, Key};

use clap::Parser;
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<()> {
    let args = Args::parse();
    let mut app = App::new(args)?;
    let mut ui = UI::new(&app);
    let mut clipboard = Clipboard::new().unwrap();

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let messages = MessageHandler::new(50);
    let mut tui = Tui::new(terminal, messages);
    tui.init()?;
    tui.draw(&mut ui)?;

    while app.running {
        let _ = match tui.messages.next()? {
            Message::Tick => {
                if app.tick()? {
                    ui.update(&app);
                    true
                } else {
                    false
                }
            }
            // TODO: handle ctrl+c and ctrl+d
            Message::Input(input) => match input {
                Input {
                    key: Key::Char('c'),
                    ctrl: true,
                    ..
                } => Err(io::Error::from(io::ErrorKind::Interrupted))?,
                Input { key: Key::Esc, .. } => break,
                Input {
                    key: Key::Char('x'),
                    ctrl: true,
                    ..
                } => {
                    clipboard.set_text(ui.command()).unwrap();
                    break;
                }
                Input {
                    key: Key::Enter, ..
                } => handle_enter(&mut app, &mut ui)?,
                input => handle_input(input, &mut app, &mut ui)?,
            },
            Message::Resize(_, _) => true,
            _ => true,
        };

        // FIXME: lazy redraw
        // if redraw {
        tui.draw(&mut ui)?;
        // }
    }

    Ok(())
}
