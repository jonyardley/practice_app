mod core;

use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use crossbeam_channel::unbounded;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use shared::{Effect, Event};

#[derive(Parser, Clone)]
enum Command {
    Get,
    Inc,
    Dec,
}

impl From<Command> for Event {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Get => Event::Get,
            Command::Inc => Event::Increment,
            Command::Dec => Event::Decrement,
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Command,
}

#[tokio::main]
async fn main() -> Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());
    let format = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(filter)
        .with(format)
        .init();

    let command = Args::parse().cmd;

    let core = core::new();
    let event = command.into();
    let (tx, _rx) = unbounded::<Effect>();

    core::update(&core, event, &Arc::new(tx))?;

    let view = core.view();

    if view.confirmed {
        println!("{text}", text = view.text);
    }

    Ok(())
}
