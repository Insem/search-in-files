use core::fmt::Error;

use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::Path,
};
fn main() {
    println!("Hello, world!");
    let arr = search_in_files(Path::new("./examples"), "Дети").unwrap();
    println!("--Arr {:?}", arr);
}

fn search_in_files(dir: &Path, word: &str) -> io::Result<Vec<String>> {
    println!("str:{:?}", &format!(r"{}(\W|$)", word));
    let reg = regex::Regex::new(&format!(r"{}(\W|$)", word)).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to build regex {:?}", format!(r"{}(\W|$)", word)),
        )
    })?;
    let mut match_arr: Vec<String> = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                match_arr.append(search_in_files(&path, word)?.as_mut());
            } else {
                if match_file(&path, &reg)? {
                    match_arr.push(path.into_os_string().into_string().map_err(|e| {
                        std::io::Error::new(std::io::ErrorKind::Other, "Failed to get os file path")
                    })?);
                }
            }
        }
    }

    Ok(match_arr)
}

fn match_file(filepath: &Path, word: &regex::Regex) -> io::Result<bool> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    for _line in reader.lines() {
        let line = if _line.is_ok() {
            _line.unwrap()
        } else {
            println!("Err: failed to read line in {:?}", filepath);
            continue;
        };

        if word.is_match(&line) {
            println!("Found {:?}", filepath);
            return Ok(true);
        }
    }

    Ok(false)
}
