use std::fs;

pub fn get_count() -> usize {
    fs::read_dir("./database").unwrap().count()
}