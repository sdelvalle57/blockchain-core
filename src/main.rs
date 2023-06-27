
mod common;
mod server;

extern crate walkdir;

use std::fs;
use crate::common::block;
use server::start_server;
use walkdir::WalkDir;



fn main() {

    let database =  fs::read_dir("./database");

    match database {
        Err(..) => fs::create_dir("./database").expect("Could not create directory"),
        _ => ()
    }

    start_server();

    let count = fs::read_dir("./database").unwrap().count();
    
    if count == 0 {
        //TODO: init blockchain
    }

    for file in WalkDir::new("./database").into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            println!("{:?}", file.file_name());
        }
    }

}
