extern crate clap;

use std::env;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::process::Command;

use clap::{App, SubCommand};
use notify::{Watcher, RecursiveMode, DebouncedEvent, watcher};

fn init () {
    println!("Initializing directory...");

    let path = env::current_dir().expect("Unknown path");
    let path = path.to_str().unwrap();

    let mut init = Command::new("git");

    init.arg("init").arg(path);
    init.output().expect("process failed to execute");
}

fn description(event: DebouncedEvent) -> String {
    match event {
        DebouncedEvent::NoticeWrite(path) => path.to_str().unwrap().to_string(),
        DebouncedEvent::NoticeRemove(path) => path.to_str().unwrap().to_string(),
        DebouncedEvent::Create(path) => path.to_str().unwrap().to_string(),
        DebouncedEvent::Write(path) => path.to_str().unwrap().to_string(),
        DebouncedEvent::Chmod(path) => path.to_str().unwrap().to_string(),
        DebouncedEvent::Remove(path) => path.to_str().unwrap().to_string(),
        _ => "other".to_string()
    }
}

fn listen() {
    let path = env::current_dir().expect("Unknown path");
    let path = path.to_str().unwrap();

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    println!("Listening changes on '{}'...", path.clone());

    loop {
        match rx.recv() {
            Ok(event) => {
                println!("Adding new stuff...");

                let mut add = Command::new("git");

                add.arg("add").arg(".");
                add.current_dir(path);
                add.output().expect("process failed to execute");

                println!("Committing new stuff...");

                let mut commit = Command::new("git");
                let message = format!("New changes on {:?}", description(event));

                commit.arg("commit").arg("-m").arg(format!("'{}'", message));
                commit.current_dir(path);
                commit.output().expect("process failed to execute");
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn main() {
    let matches = App::new("Incessant")
        .version("1.0")
        .author("Ferran Basora <fcsonline@gmail.com>")
        .about("Asynchronous backup")
        .subcommand(SubCommand::with_name("listen"))
        .subcommand(SubCommand::with_name("init"))
        .get_matches();

    match matches.subcommand_name() {
        Some("listen") => listen(),
        Some("init") => init(),
        None        => println!("No subcommand was used"),
        _           => println!("Some other subcommand was used"),
    }
}
