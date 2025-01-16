use clap::Parser;
use std::time::{Duration, Instant};

use anyhow::Result;

pub struct App {
    pub running: bool,
    pub args: Args,
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

impl App {
    pub fn new(args: Args) -> Result<Self> {
        Ok(Self {
            args,
            running: true,
            last_update: Instant::now(),
        })
    }

    pub fn tick(&mut self) -> Result<bool> {
        if self.args.interval > 0 {
            self.update(self.args.interval)
        } else {
            Ok(false)
        }
    }

    pub fn update(&mut self, interval: u64) -> Result<bool> {
        let update_rate = Duration::from_secs(interval.max(1));

        if self.last_update.elapsed() >= update_rate {
            self.last_update = Instant::now();

            return Ok(true);
        }

        Ok(false)
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
