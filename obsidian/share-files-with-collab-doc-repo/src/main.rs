use std::fs::read_dir;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use clap::Parser;
use extract_frontmatter::config::Splitter;
use extract_frontmatter::Extractor;

#[derive(Parser)]
struct Cli {
    #[clap(short = 's', long = "source-dir")]
    source_dir: String,
    #[clap(short = 'd', long = "dest-dir")]
    dest_dir: String,
    #[clap(short = 'k', long = "key", default_value = "shared")]
    key: String,
    #[clap(
        short = 'i',
        long,
        multiple_values = true,
        value_delimiter = ':',
        default_value = "assets:dailynotes:templates"
    )]
    ignore_folders: Vec<String>,
}

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
    let args: Cli = Cli::parse();

    let ignored_folders: Vec<String> = args
        .ignore_folders
        .iter()
        .map(|x| args.source_dir.to_string() + "/" + x.as_str())
        .collect();

    let files = get_files(&args.source_dir, &args.key, &ignored_folders);

    for file in files {
        let dest_path =
            PathBuf::from(&args.dest_dir).join(file.strip_prefix(&args.source_dir).unwrap());
        std::fs::create_dir_all(dest_path.parent().unwrap()).unwrap();
        std::fs::copy(file, dest_path).unwrap();
    }
}
