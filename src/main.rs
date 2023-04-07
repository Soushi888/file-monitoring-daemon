mod daemon;
mod monitor;

use daemon::{start_daemon, stop_daemon, status};
use crate::monitor::watch_directory;

use clap::{Parser, Subcommand};
use std::fs;
use notify::Error;
use toml;
use serde::Deserialize;
use crate::cli::start_cli;

#[derive(Debug, Deserialize)]
pub struct Config {
  location: String,
  excludes: Vec<String>,
}

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
  #[command(name = "status", about = "Get the daemon status")]
  Status,
  #[command(name = "restart", about = "Restart the daemon")]
  Restart,
}

pub fn main() -> Result<(), Error> {
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
    Commands::Status => {
      let result = if status() { "running" } else { "stopped" };
      println!("Daemon is {}", result);
      info!("Daemon is {}", result);
      Ok(())
    }
    Commands::Restart => {
      stop_daemon();
      start_daemon();
      let location = get_config().location;
      watch_directory(&location)
    }
  }
}

pub fn get_config() -> Config {
  let config_file = fs::read_to_string("./config.toml").expect("Unable to read config file");
  toml::from_str::<Config>(&config_file).expect("Unable to parse config file")
}