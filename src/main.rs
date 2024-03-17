use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() {
    println!("Hello, world!");
    search_in_files(Path::new("./examples"), "вавпаавпвпвпвапвапв");
}

fn search_in_files(dir: &Path, word: &str) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                search_in_files(&path, word)?;
            } else {
                read_file(&path, word);
                //println!("File: {:?}", path);
            }
        }
    }
    Ok(())
}

fn read_file(filepath: &Path, word: &str) -> io::Result<()> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    println!("str:{:?}", &format!(r"{}(\W|$)", word));
    let re = regex::Regex::new(&format!(r"{}(\W|$)", word)).unwrap();
    for line in reader.lines() {
        //println!("Liine {:?}", line);
        let l = line.unwrap();
        /*match l.find(word) {
            Some(w) => {
                let end_of_the_word = w + word.to_string().len() + 1;
                println!(
                    "Line:  {:?} {:?} {:?} {:?} {:?}",
                    l.len(),
                    end_of_the_word,
                    end_of_the_word + 1,
                    l.get(end_of_the_word..), //char::from_u32(l.bytes().nth(w)),
                    w
                );
                let symbol_after_word = l.get(end_of_the_word..end_of_the_word + 1).unwrap();

                if symbol_after_word.chars().nth(0).unwrap().is_alphabetic() {
                    println!("Line: {:?}", w);
                }
            }

            None => println!(""),
        };*/
        if re.is_match(&l) {
            println!("Found {:?}", filepath);
        }
    }

    Ok(())
}
