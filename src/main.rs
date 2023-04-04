mod cli;
mod daemon;
mod monitor;

// use monitor::watch_directory;

use std::fs;
use toml;
use serde::Deserialize;
use notify::{RecursiveMode, Watcher};
use std::path::Path;
use crate::monitor::watch_directory;

#[derive(Debug, Deserialize)]
struct Config {
  location: String,
}

fn main() -> Result<(), notify::Error> {
  let location = get_config().location;
  watch_directory(&location)
}

fn get_config() -> Config {
  let config_file = fs::read_to_string("config.toml").expect("Unable to read config file");
  toml::from_str::<Config>(&config_file).expect("Unable to parse config file")
}