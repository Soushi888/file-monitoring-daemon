use std::path::Path;
use std::fs::File;
use std::io::Write;
use serde_json;
use serde::Deserialize;
use walkdir::WalkDir;
use crate::get_config;

#[derive(Debug, Deserialize)]
pub struct FileData {
  pub path: String,
  pub content: String,
}

#[derive(Debug)]
pub enum Error {
  Io(std::io::Error),
  Json(serde_json::Error),
}

#[allow(dead_code)]
impl FileData {
  pub fn new(path: String, content: String) -> Self {
    Self {
      path,
      content,
    }
  }

  pub fn write_on_fs(&self) -> Result<(), Error> {
    let root = get_config().location;
    let path = Path::new(&root).join(&self.path);

    if let Some(parent) = path.parent() {
      if !parent.exists() {
        std::fs::create_dir_all(parent).map_err(Error::Io)?;
      }
    }

    let mut file = match File::create(path) {
      Ok(file) => file,
      Err(err) => {
        eprintln!("Error creating file: {}", err);
        return Err(Error::Io(err));
      }
    };

    file.write_all(self.content.as_bytes()).map_err(Error::Io)
  }

  pub fn write_on_dht(&self) -> Result<(), Error> {
    unimplemented!("Function that will write a file on the DHT")
  }
}

pub fn get_all_files_from_mock() -> Result<Vec<FileData>, Error> {
  let path = Path::new("./files.json");
  let file = match File::open(path) {
    Ok(file) => file,
    Err(err) => {
      eprintln!("Error opening file: {}", err);
      return Ok(vec![]);
    }
  };
  let files: Vec<FileData> = serde_json::from_reader(file).map_err(Error::Json)?;

  for file in files.iter() {
    file.write_on_fs()?;
  }

  println!("Files synced from mock to FS: {:#?}", files);

  Ok(files)
}

pub fn get_all_files_from_fs(path: &str) -> Vec<String> {
  let mut file_paths = vec![];

  let files = WalkDir::new(path).into_iter().filter_map(Result::ok).filter(|e| e.file_type().is_file());

  for entry in files {
    let path_str = entry.path().to_str().unwrap().to_string();
    file_paths.push(path_str);
  }

  file_paths
}


// Tests
#[test]
fn test_get_all_files() {
  let files = get_all_files().unwrap();
  assert_eq!(files.len(), 3);
}

#[test]
fn test_write_on_fs() {
  let file = FileData::new("test.txt".to_string(), "Hello World!".to_string());

  assert!(file.write_on_fs().is_ok());

  let location = get_config().location;
  let path = Path::new(&location).join("test.txt");

  if path.exists() {
    std::fs::remove_file(path).unwrap();
  }
}
