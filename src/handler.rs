use color_eyre::eyre::Result;
use tui_textarea::Input;

use crate::{app::App, ui::UI};

pub fn handle_enter(app: &mut App, ui: &mut UI) -> Result<bool> {
    let command = ui.command();

    match app.update(200, Some(command)) {
        Ok(true) => {
            ui.update(app);
            return Ok(true);
        }
        Err(_) => {
            ui.update(app);
            ui.command_failed();
            return Ok(false);
        }
        _ => Ok(false),
    }
}

pub fn handle_input(input: Input, app: &mut App, ui: &mut UI) -> Result<bool> {
    let textarea = ui.mut_textarea();

    if textarea.input(input) {
        ui.update(app);
        return Ok(true);
    }

    Ok(false)
}
