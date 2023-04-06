use std::path::Path;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use notify::event::ModifyKind;
use crate::get_config;

pub fn watch_directory(path: &str) -> Result<(), notify::Error> {
  let (tx, rx) = channel();

  let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

  watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

  for res in rx {
    match res {
      Ok(event) => {
        let path = event.paths[0].to_str().unwrap();
        let excluded = get_config().excludes.iter().any(|x| path.contains(x));

        if excluded { continue; }

        match event.kind {
            EventKind::Create(_) => println!("created: {:?}", event),
            EventKind::Modify(ref data_type) => {
              match data_type {
                ModifyKind::Name(_) => println!("renamed: {:?}", event),
                ModifyKind::Data(_) => println!("modified: {:?}", event),
                _ => println!("unknown: {:?}", event),
              }
            },
            EventKind::Remove(_) => println!("removed: {:?}", event),
            EventKind::Other => println!("other: {:?}", event),
            _ => println!("unknown: {:?}", event),
          }
        },
      Err(e) => println!("watch error: {:?}", e),
    }
  }

  Ok(())
}