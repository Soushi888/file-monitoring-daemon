use std::path::Path;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;

pub fn watch_directory(path: &str) -> Result<(), notify::Error> {
  let (tx, rx) = channel();

  let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

  watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

  for res in rx {
    match res {
      Ok(event) => println!("changed: {:?}", event),
      Err(e) => println!("watch error: {:?}", e),
    }
  }

  Ok(())
}