use std::path::Path;
use notify::{Config, RecursiveMode, RecommendedWatcher, Watcher};
use notify_debouncer_mini::{DebounceEventResult, new_debouncer};
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn watch_directory(path: &str, excludes: Vec<String>) -> Result<(), notify::Error> {
  let (tx, rx) = channel();

  let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx).unwrap();

  debouncer.watcher().watch(Path::new(path), RecursiveMode::Recursive)?;

  for res in rx {
    match res {
      Ok(events) => {
        for event in events {
          let excluded = excludes.iter().any(|exclude| event.path.to_str().unwrap().contains(exclude));
          if excluded { continue; }

          println!("File changed : {:?}", event.path);
        }
      }
      Err(e) => eprintln!("watch error: {:?}", e),
    }
  }

  Ok(())
}