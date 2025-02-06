use clap::Parser;
use color_eyre::eyre::Result;
use std::{
    process::Command,
    time::{Duration, Instant},
};

pub struct App {
    pub running: bool,
    pub args: Args,
    pub output: Option<String>,
    last_update: Instant,
}

#[derive(Parser)]
#[clap(version, about)]
#[clap(propagate_version = true)]
#[clap(bin_name = "plumber")]
pub struct Args {
    #[arg(long, action)]
    #[arg(default_value_t = 5)]
    pub interval: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("The last update has not yet elapsed the update interval")]
    LastUpdateNotYetElapsed,
    #[error("Command failed: {0}")]
    CommandFailed(String),
}

impl App {
    pub fn new(args: Args) -> Result<Self> {
        Ok(Self {
            args,
            running: true,
            last_update: Instant::now(),
            output: None,
        })
    }

    pub fn tick(&mut self) -> Result<bool> {
        if self.args.interval > 0 {
            Ok(self.update(self.args.interval, None)?)
        } else {
            Ok(false)
        }
    }

    pub fn update(&mut self, interval: u64, command: Option<String>) -> Result<bool> {
        let update_rate = Duration::from_millis(interval.max(300));

        if self.last_update.elapsed() >= update_rate {
            if let Some(command) = command {
                self.output = Some(App::collect(command)?);
                self.last_update = Instant::now();
                return Ok(true);
            }
        }

        return Ok(false);
    }

    fn collect(command: String) -> Result<String> {
        let output = Command::new("bash").arg("-c").arg(command).output()?;

        if !output.status.success() {
            return Err(AppError::CommandFailed(
                String::from_utf8(output.stderr).unwrap_or_default(),
            )
            .into());
        }

        String::from_utf8(output.stdout).map_err(Into::into)
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
