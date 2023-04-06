use std::fs::File;
use std::io::{Read};
use std::process::Command;
use daemonize::Daemonize;

const PID_FILE: &str = "/tmp/file_watcher.pid";

pub fn start_daemon() {
  let stdout = File::create("/tmp/file_watcher.stdout.log").expect("Unable to create stdout file");
  let stderr = File::create("/tmp/file_watcher.stderr.log").expect("Unable to create stderr file");

  println!("Starting daemon");

  let daemonize = Daemonize::new()
    .pid_file(PID_FILE) // Path to the PID file
    .working_directory(".") // The working directory for the daemon process
    .stdout(stdout) // Redirect stdout to a log file
    .stderr(stderr); // Redirect stderr to a log file

  match daemonize.start() {
    Ok(_) => println!("Success, daemonized"),
    Err(e) => eprintln!("Error, {}", e),
  }
}

pub fn stop_daemon() {
  let mut file = match File::open(PID_FILE) {
    Ok(file) => file,
    Err(_) => {
      eprintln!("No daemon running");
      return;
    }
  };

  let mut pid = String::new();

  if let Err(_) = file.read_to_string(&mut pid) {
    eprintln!("Unable to read pid file");
    return;
  }

  // if let Err(e) = &pid.parse::<u32>() {
  //   eprintln!("{pid}");
  //   eprintln!("Unable to parse pid: {}", e);
  //   return;
  // }

  let pid = match pid.trim().parse::<u32>() {
    Ok(pid) => pid,
    Err(_) => {
      eprintln!("Unable to parse pid file");
      return;
    }
  };

  let output = Command::new("kill")
    .arg("-9")
    .arg(&pid.to_string())
    .output()
    .unwrap();

  match std::fs::remove_file(PID_FILE) {
    Ok(_) => (),
    Err(_) => eprintln!("Unable to remove pid file"),
  }

  if !output.status.success() {
    eprintln!("Unable to kill daemon");
    return;
  }

  println!("Stopped daemon");
}