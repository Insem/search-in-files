use core::fmt::Error;

use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
    pin::Pin,
};

use tokio::task::{JoinHandle, JoinSet};
pub async fn search_in_files(dir: &Path, word: &str) -> io::Result<Vec<String>> {
    let mut set = JoinSet::new();
    let mut match_arr: Vec<String> = Vec::new();
    for ft in fold_files(dir, word)? {
        set.spawn(ft);
    }
    while let Some(Ok(Ok(res))) = set.join_next().await {
        match res {
            Ok(file_path) => {
                if file_path.is_some() {
                    match_arr.push(file_path.unwrap())
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }

    Ok(match_arr)
}
fn fold_files(dir: &Path, word: &str) -> io::Result<Vec<JoinHandle<io::Result<Option<String>>>>> {
    let reg = regex::Regex::new(&format!(r"{}(\W|$)", word)).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to build regex {:?}", format!(r"{}(\W|$)", word)),
        )
    })?;
    let mut match_arr: Vec<JoinHandle<io::Result<Option<String>>>> = Vec::new();
    if !dir.exists() {
        return std::io::Result::Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory doesn't exist {:?}", dir),
        ));
    }
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                match_arr.append(fold_files(&path, word)?.as_mut());
            } else {
                match_arr.push(tokio::spawn(do_files(Box::new(path), reg.clone())));
            }
        }
    }

    Ok(match_arr)
}
async fn do_files(path: Box<PathBuf>, reg: regex::Regex) -> io::Result<Option<String>> {
    if match_file(&path, &reg)? {
        return Ok(Some(
            path.as_os_str()
                .to_str()
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to get file path string {:?}", path),
                ))?
                .to_string(),
        ));
    }
    Ok(None)
}
fn match_file(filepath: &Path, word: &regex::Regex) -> io::Result<bool> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    for _line in reader.lines() {
        let line = if _line.is_ok() {
            _line.unwrap()
        } else {
            continue;
        };

        if word.is_match(&line) {
            return Ok(true);
        }
    }

    Ok(false)
}

#[tokio::test]
async fn test_search_in_files() {
    assert_eq!(
        vec![format!(
            "{}/src/file_search/tests/test_files/test1.txt",
            std::env::current_dir()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap()
        )],
        search_in_files(
            Path::new(&format!(
                "{}/src/file_search/tests",
                std::env::current_dir()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap()
            )),
            "эта"
        )
        .await
        .unwrap()
    );
}
#[tokio::test]
async fn test_recursive_search_in_files() {
    assert_eq!(
        vec![
            format!(
                "{}/src/file_search/tests/test_files/test2.txt",
                std::env::current_dir()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap()
            ),
            format!(
                "{}/src/file_search/tests/test_files/test_files/test2.txt",
                std::env::current_dir()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap()
            )
        ],
        search_in_files(
            Path::new(&format!(
                "{}/src/file_search/tests",
                std::env::current_dir()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap()
            )),
            "трясла"
        )
        .await
        .unwrap()
    );
}
#[tokio::test]
async fn test_search_with_punctiation() {
    assert_eq!(
        vec![format!(
            "{}/src/file_search/tests/test_files/test1.txt",
            std::env::current_dir()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap()
        )],
        search_in_files(
            Path::new(&format!(
                "{}/src/file_search/tests",
                std::env::current_dir()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap()
            )),
            "каналам"
        )
        .await
        .unwrap()
    );
}
#[tokio::test]
async fn test_find_multiple_files() {
    assert_eq!(
        vec![
            format!(
                "{}/src/file_search/tests/test_files/test2.txt",
                std::env::current_dir()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap()
            ),
            format!(
                "{}/src/file_search/tests/test_files/test1.txt",
                std::env::current_dir()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap()
            ),
            format!(
                "{}/src/file_search/tests/test_files/test_files/test2.txt",
                std::env::current_dir()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap()
            )
        ],
        search_in_files(
            Path::new(&format!(
                "{}/src/file_search/tests",
                std::env::current_dir()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap()
            )),
            "он"
        )
        .await
        .unwrap()
    );
}
