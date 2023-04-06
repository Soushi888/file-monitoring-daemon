use clap::{Parser, Subcommand};
use notify::Error;
use crate::{start_daemon, watch_directory, get_config};
use crate::daemon::stop_daemon;

#[derive(Parser, Debug)]
#[command(name = "file-watcher", about = "A file watcher daemon", version, author)]
pub struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  #[command(name = "start", about = "Start the daemon")]
  Start,
  #[command(name = "stop", about = "Stop the daemon")]
  Stop,
}

pub fn start_cli() -> Result<(), Error> {
  let cli = Cli::parse();

  match &cli.command {
    Commands::Start => {
      start_daemon();
      let location = get_config().location;
      watch_directory(&location)
    }
    Commands::Stop => {
      stop_daemon();
      Ok(())
    }
  }
}