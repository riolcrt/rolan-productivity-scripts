mod md_file;

use clap::Parser;
use md_file::domain::md_file::MDFile;
use md_file::domain::md_local_repository;

#[derive(Parser)]
pub struct Cli {
    #[clap(short = 's', long = "source-dir")]
    source_dir: String,
    #[clap(short = 'd', long = "dest-dir")]
    dest_dir: String,
    #[clap(short = 'k', long = "key", default_value = "shared")]
    key: String,
    #[clap(
        short = 'i',
        long,
        value_delimiter = ':',
        default_value = "assets:dailynotes:templates"
    )]
    ignore_folders: Vec<String>,
}

fn main() {
    let args = Cli::parse();

    let ignored_folders: Vec<String> = args
        .ignore_folders
        .iter()
        .map(|x| args.source_dir.to_string() + "/" + x.as_str())
        .collect();

    let md_files = md_local_repository::all(&args.source_dir, &ignored_folders);

    let sharing_files: Vec<&MDFile> = md_files
        .iter()
        .filter(|x| x.frontmatter.get(&args.key).is_some())
        .filter(|x| x.frontmatter.get(&args.key).unwrap().as_bool().unwrap())
        .collect();

    md_local_repository::sync_with_dest(&sharing_files, &args.dest_dir);

    println!("Synced [{}] files", sharing_files.len())
}
