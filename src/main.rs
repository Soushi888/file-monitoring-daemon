mod cli;
mod daemon;
mod monitor;

use monitor::watch_directory;
use daemon::start_daemon;

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

fn main() -> Result<(), Error> {
  start_cli()
}

pub fn get_config() -> Config {
  let config_file = fs::read_to_string("./config.toml").expect("Unable to read config file");
  toml::from_str::<Config>(&config_file).expect("Unable to parse config file")
}