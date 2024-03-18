use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

use tokio::task::{JoinHandle, JoinSet};
pub async fn search_in_files(dir: &Path, word: &str) -> io::Result<Vec<String>> {
    //создаём джоин сет токио для асинхронной обработки
    let mut set = JoinSet::new();
    let mut match_arr: Vec<String> = Vec::new();
    //Добавляем футуры обработки каждого файла в джоинсет
    for ft in fold_files(dir, word)? {
        set.spawn(ft);
    }
    //запускаем футуры из джоинсета. Они должны отрабатывать не в порядке запуска, а каждая возвращает результат сразу после отработки
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
    //создаём регулярку для поиска
    let reg = regex::Regex::new(&format!(r"{}(\W|$)", word)).map_err(|_e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to build regex {:?}", format!(r"{}(\W|$)", word)),
        )
    })?;
    let mut fut_arr: Vec<JoinHandle<io::Result<Option<String>>>> = Vec::new();

    //проверяем папку на корректность
    if !dir.exists() {
        return std::io::Result::Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory doesn't exist {:?}", dir),
        ));
    }
    if dir.is_dir() {
        //читаем содержимаое папки
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                //если есть папка внутри папки тоже её обрабатываем
                fut_arr.append(fold_files(&path, word)?.as_mut());
            } else {
                //добавляем футуры в массив для жоин сета
                fut_arr.push(tokio::spawn(do_files(Box::new(path), reg.clone())));
            }
        }
    }

    Ok(fut_arr)
}
//Если в файле есть совпадение возвращаем его путь
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
    //читем файл построчно, после первого совпадения возвращаем true
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
