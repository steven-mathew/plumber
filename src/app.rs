use std::time::{Duration, Instant};
use clap::{Parser};

use anyhow::Result;

#[derive(Parser)]
#[clap(version, about)]
#[clap(propagate_version = true)]
#[clap(bin_name = "plumber")]
pub struct App {
    pub running: bool,

    #[clap(flatten)]
    pub config_args: ConfigArgs,
    last_update: Instant,
}

#[derive(clap::Args)]
pub struct ConfigArgs {
    #[clap(long, action)]
    #[clap(default_value_t = 5)]
    pub interval: u64,
}

impl App {
    pub fn new(config_args: ConfigArgs) -> Result<Self> {
        Ok(Self {
            config_args,
            running: true,
            last_update: Instant::now(),
        })
    }

    pub fn tick(&mut self) -> Result<bool> {
        if self.config_args.interval > 0 {
            self.update(self.config_args.interval)
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
