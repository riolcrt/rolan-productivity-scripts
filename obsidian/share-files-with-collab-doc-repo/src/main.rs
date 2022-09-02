use clap::Parser;

use extract_frontmatter::config::Splitter;
use extract_frontmatter::Extractor;
use std::fs::read_dir;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    source_dir: String,
    dest_dir: String,
    key: String,
    branch_name: String,
    #[clap(short, long, default_value = "Automatic sychronization")]
    commit_message: String,
    #[clap(
        short,
        long,
        multiple_values = true,
        value_delimiter = ':',
        default_value = "assets:dailynotes:templates"
    )]
    ignore_folders: Vec<String>,
}

//return the content of a file as a string or a empty string if an error occurs
fn read_file(path: &PathBuf) -> Result<String, std::io::Error> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_files(path: &str, key: &str, ignored_folders: &Vec<String>) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();

    for entry in read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let comparsion = path.is_dir() && !ignored_folders.contains(&path.display().to_string());

        if path.is_dir() && !ignored_folders.contains(&path.display().to_string()) {
            let path_files = get_files(path.display().to_string().as_str(), key, ignored_folders);
            files.extend(path_files);
        } else {
            let contents = match read_file(&path) {
                Ok(contents) => contents,
                Err(_) => "".to_string(),
            };

            let (frontmatter, data) =
                Extractor::new(Splitter::EnclosingLines("---")).extract(&contents);

            //if the length of the data field is greater than 0, then the file has frontmatter
            if !data.is_empty() && frontmatter.contains(key) {
                files.push(path);
            }
        }
    }

    files
}

fn main() {
    //parse args --source-dir <path> --dest-dir <path> --key <key> --branch <branch>
    let args: Cli = Cli::parse();

    // get all the files from the source directory recursively
    let files = get_files(&args.source_dir, &args.key, &args.ignore_folders);

    //print the files
    for file in files {
        println!("{}", file.display());
    }
}
