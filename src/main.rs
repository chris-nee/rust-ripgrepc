use colored::Colorize;
use regex::Regex;
use std::{
    env, error, fs,
    io::{BufRead, BufReader},
    path::Path,
};

static mut COUNT: i32 = 0;

fn search(file_path: String, search_text: String) -> Result<(), Box<dyn error::Error>> {
    let target_dir = Path::new(&file_path);

    for entry in fs::read_dir(target_dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;

        if metadata.is_dir() {
            let _ = search(path.to_str().unwrap().to_string(), search_text.clone());
        } else {
            let file = std::fs::File::open(path.clone())?;
            let reader = BufReader::new(file);
            let regex = Regex::new(&search_text).unwrap();

            let mut matches: Vec<String> = vec![];

            for line in reader.lines() {
                let line_clone = line?;

                if regex.is_match(&line_clone.clone()) {
                    matches.push(line_clone.clone());
                }
            }

            if matches.len() > 0 {
                unsafe { COUNT += 1 };
                println!("{}", ">>>>>".yellow());

                let count_clone;
                unsafe {
                    count_clone = COUNT.clone();
                };

                println!(
                    "{}. {}",
                    count_clone.to_string().blue().bold(),
                    path.file_name().unwrap().to_string_lossy().blue().bold()
                );

                for match_str in matches {
                    let split_strs = match_str.split(&search_text);
                    let split_strs_count = split_strs.clone().count();

                    print!(">>> ");
                    for (i, split_str) in split_strs.enumerate() {
                        print!("{}", split_str.green().bold());
                        if i < split_strs_count - 1 {
                            print!("{}", search_text.yellow().bold());
                        }
                    }
                    println!("\n");
                }

                println!("{}", ">>>>>".yellow());
            }
        }
    }

    Ok(())
}

fn main() {
    println!("Welcome to ripgrepc!");
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("2 arguments required, FILE PATH, SEARCH TEXT");
        panic!()
    }

    let file_path = &args[1];
    let search_text = &args[2];
    println!("Searching file path: [{}]", file_path);
    println!("Searching for search text: [{}]", search_text);

    // search for the file
    println!("{}", "[SEARCHING]...".bold().green());
    let _ = search(file_path.clone(), search_text.clone());

    dbg!();
}
