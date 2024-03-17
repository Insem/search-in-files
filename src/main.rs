use core::fmt::Error;

use crate::file_search::FileSearch::search_in_files;
use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::Path,
};
pub mod file_search;
fn main() {
    println!("Hello, world!");
    let arr = search_in_files(Path::new("../examples"), "Дети").unwrap();
    println!("--Arr {:?}", arr);
}
