mod common;
mod server;
mod db_handler;


// extern crate walkdir;

// use crate::common::block;
use server::start_server;
use std::fs;
// use walkdir::WalkDir;

fn main() {
    

    if let Err(err) = fs::read_dir("./database") {
        println!("{}", err);
        fs::create_dir("./database").unwrap();
    }

    if let Err(err) =  start_server() {
        println!("{}", &err);
        std::process::exit(1);
    }

    // let count = fs::read_dir("./database").unwrap().count();

    // if count == 0 {
    //     //TODO: init blockchain
    // }

    // for file in WalkDir::new("./database")
    //     .into_iter()
    //     .filter_map(|file| file.ok())
    // {
    //     if file.metadata().unwrap().is_file() {
    //         println!("{:?}", file.file_name());
    //     }
    // }
}
