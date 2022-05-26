use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    //read the command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!(
            "Usage: {} <Path> <Word to Search> <Tag to add, without #>",
            args[0]
        );
        return Ok(());
    }

    let path_arg: &String = &args[1];
    let word_to_search: &String = &args[2];
    let tag_to_add: &String = &args[3];

    let all_files = get_all_files_recursive(&path_arg);
    let all_tasks = filter_tasks(&all_files, &word_to_search);

    for task_path in all_tasks {
        //print the filename of the task
        let task_file_content = fs::read_to_string(&task_path)?;
        //prepend the desired tag to the task content
        let task_file_content = format!("#{tag_to_add}\n\n{task_file_content}");
        //write the task content to the task file
        fs::write(&task_path, task_file_content)?;
    }

    Ok(())
}

fn get_all_files_recursive(path: &str) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let path_files = get_all_files_recursive(path.display().to_string().as_str());
            files.extend(path_files);
        } else {
            files.push(path);
        }
    }

    return files;
}

fn filter_tasks(file_list: &Vec<PathBuf>, word_to_search: &str) -> Vec<PathBuf> {
    let mut tasks: Vec<PathBuf> = Vec::new();
    for file in file_list {
        //add to vector all files that contains the string "task" in the file name
        if file.to_str().unwrap().contains(word_to_search) {
            tasks.push(file.to_owned());
        }
    }
    return tasks;
}
