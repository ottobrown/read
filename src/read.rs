use std::fs;
use std::io::{Result, stdin, stdout, Write};

use crate::config;

pub fn read(cf: config::Config) -> Result<()> {
    let lines = get_lines(cf.clone())?;

    let mut line = 0;

    let mut printed = 0; 

    for l in &lines {
        if printed == cf.max_lines && cf.read_style != config::ReadStyle::LineByLine {
            println!();
            println!("Max Lines Exceeded!!");
            println!("{} more lines left in file", lines.len() - printed as usize);
            return Ok(())
        }

        if cf.numbered_lines {print!("{}  |  ", line+1); line += 1};

        match cf.read_style {
            config::ReadStyle::Default => {
                println!("{}", l)
            },

            config::ReadStyle::LineByLine => {
                print!("{}", l);
                stdout().flush()?;
        
                //Wait for input
                stdin().read_line(&mut String::new())?;
            },
            config::ReadStyle::Top => {
                unimplemented!()
            }

        }

        printed += 1
    }

    Ok(())
}

pub fn get_lines(cf: config::Config) -> Result<Vec<String>> {
    let mut content = String::new();

    let borrowed_path = &cf.path;

    let md = fs::metadata(borrowed_path)?;

    if md.is_dir() {
        let dir = match fs::read_dir(borrowed_path) {
            Ok(p) => p,
            Err(e) => {return Err(e)}
        };
        
        for i in dir {
            let path = i.unwrap().path();
            content += &path.display().to_string();
            content += "\n";
        }
    }
    if md.is_file() {
        content = fs::read_to_string(borrowed_path)?;
    }

    let split: Vec<&str> = content.split("\n").collect();

    // convert every &str in split to a String 
    let v2: Vec<String> = split.iter().map(|s| String::from(*s)).collect();


    Ok(v2)
}