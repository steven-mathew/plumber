use anyhow::Result;
use plumber::app::{App, Args};

use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();
    let app = App::new(args)?;

    println!("Starting app with interval: {}", app.opts.interval);

    Ok(())
}
