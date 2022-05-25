use std::env;
use std::fs;

fn main() {
    //read the command line arguments
    let args: Vec<String> = env::args().collect();

    let path_arg = &args[1];

    //list a directory files given a path
    for directory in fs::read_dir(path_arg).unwrap() {
        let directory = directory.unwrap();
        if directory.path().is_dir() {
            recursive_print(&directory.path().to_str().unwrap());
        }
    }
}

fn recursive_print(path: &str) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            recursive_print(path.display().to_string().as_str());
        } else {
            println!("{}", path.display());
        }
    }
}
