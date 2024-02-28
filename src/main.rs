#![allow(warnings)]

use std::{fs, io};
use std::path::Path;
use crossterm::{execute};
use crossterm::terminal::{Clear, ClearType};

fn list_file_in_directory(path: &Path) -> io::Result<()> {
    let reader = fs::read_dir(path)?;

    for file in reader {
        match file {
            // if file contains a valid entry for example : ".", "../foo" we the files
            Ok(file_entry) => {
                if let Some(file_name) = file_entry.file_name().to_str() {
                    if !file_name.starts_with(".") {
                        todo!("Find another method to show/hide hidden folder/file");
                        println!("{}", file_name);
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
    let mut directory_path = Path::new(".");
    let mut input = String::new();
    
    loop {
        execute!(io::stdout(), Clear(ClearType::All))?;
        list_file_in_directory(directory_path)?;
        
        println!("Enter a new directory to list : ");
        input.clear();
        io::stdin().read_line(&mut input)?;

        input = input.trim().to_string();

        if input.is_empty() {
            break;
        }

        directory_path = Path::new(&input);
    }


    Ok(())
}
