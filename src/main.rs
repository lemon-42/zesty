#![allow(warnings)]

use std::convert::Into;
use std::fs::{read_dir, metadata};
use std::thread::current;
use std::{fs, io};
use std::path::{Path, PathBuf};
use crossterm::{cursor, style};
use crossterm::{execute, event::Event};
use crossterm::event::{KeyCode, KeyEvent};
use crossterm::terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode};

#[derive(Debug, PartialEq)]
enum Zmode {
    Insertion,
    Selection,
}

fn inspect_mode(event: Zmode, current_mode: &mut Zmode) {
    match event {
        Zmode::Insertion => {
           *current_mode = Zmode::Insertion;
           println!("You are now in Insertion mode.");
        }
        Zmode::Selection => {
           *current_mode = Zmode::Selection;
           println!("You are now in Selection mode.");
        }
    }

    if *current_mode != event {
        println!("Mode changed to : {:?}", current_mode);
    }
}

fn list_file_in_directory(path: &PathBuf) -> io::Result<()> {
    let reader = fs::read_dir(path)?;

    for file in reader {
        match file {
            // if file contains a valid entry for example : ".", "../foo" we show the files
            Ok(file_entry) => {
                if let Some(file_name) = file_entry.file_name().to_str() {
                    if !file_name.starts_with(".") {
                        // TODO find another method to handle hidden file/folder
                        let metadata = fs::metadata(file_entry.path())?;
                        let size = metadata.len();
                        let file_type = if metadata.is_dir() {
                            "Directory"
                        } else {
                            "File"
                        };

                        println!(
                            "{:30} ({}) | {} bytes",
                            file_name,
                            file_type,
                            size,
                        );
                    }
                } else {
                    eprintln!("ERROR: Unable to get the file name.");
                }
            }
            Err(err) => eprintln!("ERROR: {}", err),
        }
    } 
    Ok(())
}

fn main() -> io::Result<()> {
    // TODO : 
    enable_raw_mode().expect("Failed to enter raw mode.");

    let mut directory_path = PathBuf::from(".");
    let mut input = String::new();
    let mut current_mode = Zmode::Insertion;
    let mut trimmed_input = String::new();

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        if current_mode == Zmode::Insertion {
            println!("* {}", directory_path.display());
        } else {
            println!("{}", directory_path.display());
        }

        print!("{}", cursor::MoveToColumn(0));

        println!("* {} Current mode : {:?}", directory_path.display(), current_mode);

        list_file_in_directory(&directory_path);

        let key_event = crossterm::event::read()?;
        match current_mode {
            Zmode::Insertion => {
                match key_event {
                    Event::Key(KeyEvent { code, .. }) => {
                        match code {
                            KeyCode::Char('q') => {
                                disable_raw_mode().expect("Failed to exit raw mode.");
                                break;
                            },
                            KeyCode::Char('i') => {
                                inspect_mode(Zmode::Insertion, &mut current_mode);
                            },
                            KeyCode::Char('s') => {
                                inspect_mode(Zmode::Selection, &mut current_mode);
                            },
                            KeyCode::Up => {
                                if let Some(parent) = directory_path.parent() {
                                    directory_path = parent.to_path_buf();
                                }
                            },
                            KeyCode::Down => {
                                if let Ok(entries) = fs::read_dir(&directory_path) {
                                    for entry in entries {
                                        if let Ok(entry) = entry {
                                            if let Ok(metadata) = entry.metadata() {
                                                if metadata.is_dir() {
                                                    directory_path = entry.path();
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            KeyCode::Esc => {
                                inspect_mode(Zmode::Selection, &mut current_mode);
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            },
            Zmode::Selection => {
                match key_event {
                    Event::Key(KeyEvent { code, .. }) => {
                        match code {
                            KeyCode::Char('q') => {
                                disable_raw_mode().expect("Failed to exit raw mode.");
                                break;
                            },
                            KeyCode::Char('i') => {
                                inspect_mode(Zmode::Insertion, &mut current_mode);
                            },
                            KeyCode::Char('s') => {
                                inspect_mode(Zmode::Selection, &mut current_mode);
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
