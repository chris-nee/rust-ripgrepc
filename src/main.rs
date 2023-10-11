use regex::Regex;
use std::{
    env, error, fs,
    io::{BufRead, BufReader},
    path::Path,
};

fn search(file_path: String, search_text: String) -> Result<(), Box<dyn error::Error>> {
    // let current_dir = env::current_dir()?;
    let target_dir = Path::new(&file_path);

    println!("[SEARCHING]...");

    for entry in fs::read_dir(target_dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();

        if metadata.is_dir() {
            println!("Found a dir -> {}", path.to_str().unwrap().to_string());
            let _ = search(path.to_str().unwrap().to_string(), search_text.clone());
        } else {
            println!(
                "Last modified: {:?} seconds, filename: {:?}",
                last_modified,
                path.file_name().ok_or("No filename")?
            );
            let file = std::fs::File::open(path).unwrap();
            let reader = BufReader::new(file);

            let regex = Regex::new(&search_text).unwrap();

            for line in reader.lines() {
                let line_clone = line.unwrap();
                if regex.is_match(&line_clone.clone()) {
                    println!(">>>>>");
                    println!("FOUNDOUNFOUDNOUN -> {}", search_text);
                    println!("FULL SENTENCE -> {:?}", line_clone.clone());
                    println!(">>>>>");
                }
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
    let _ = search(file_path.clone(), search_text.clone());

    dbg!();
}
