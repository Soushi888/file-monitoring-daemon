use std::path::Path;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use log::{info, error};
use notify::event::ModifyKind;
use crate::get_config;

pub fn watch_directory(path: &str, excludes: Vec<String>) -> Result<(), notify::Error> {
  let (tx, rx) = channel();

  let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

  watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

  for res in rx {
    match res {
      Ok(event) => {
        let path = event.paths[0].to_str().unwrap();

        let excluded = excludes.iter().any(|exclude| path.contains(exclude));
        if excluded { continue; }

        println!("File changed : {:?}", event.paths[0].to_str().unwrap());
        },
      Err(e) => error!("watch error: {:?}", e),
    }
  }

  Ok(())
}