use super::md_file::MDFile;
use std::{fs::read_dir, path::PathBuf};

fn get_files(path: &str, ignored_folders: &Vec<String>) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = vec![];

    for entry in read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if ignored_folders.contains(&path.display().to_string()) {
            continue;
        }

        if path.is_dir() {
            let path_files = get_files(path.display().to_string().as_str(), ignored_folders);
            files.extend(path_files);
        } else {
            files.push(path);
        }
    }

    files
}

pub fn all(path: &str, ignored_folders: &Vec<String>) -> Vec<MDFile> {
    return get_files(path, ignored_folders)
        .iter()
        .filter(|x| x.extension().unwrap().to_str().unwrap() == "md")
        .map(|x| MDFile::from_path(x.display().to_string().as_str()).unwrap())
        .filter(|x| !x.frontmatter.is_string())
        .collect();
}

pub fn sync_with_dest(files: &Vec<&MDFile>, target_dir: &str) {
    for file in files {
        let file_name = file.path.split('/').last().unwrap();
        let target_path = format!("{}/{}", target_dir, file_name);

        let dest_path: PathBuf = PathBuf::from(target_path);

        dbg!(&dest_path);
        dbg!(&target_dir);
        dbg!(&file_name);

        std::fs::create_dir_all(dest_path.parent().unwrap()).unwrap();
        std::fs::copy(file.path.as_str(), dest_path).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_get_a_list_of_files_recursively_not_present_in_ignored_folders() {
        let ignored_folders: Vec<String> = vec!["./target".to_string()];
        let files = get_files("./test/fixtures", &ignored_folders);
        dbg!(&files);
        assert!(!files.is_empty());
        //assert if the file dont contains any path pointing to any ignored folder
        assert!(!files.iter().all(|x| ignored_folders.iter().any(|y| x
            .display()
            .to_string()
            .matches(y)
            .count()
            > 0)));
    }

    #[test]
    fn it_should_get_a_list_of_md_files() {
        let path = "./test/fixtures".to_string();
        let ignored_folders = vec![];
        let md_files = all(&path, &ignored_folders);
        dbg!(&md_files);
        assert!(md_files.len() == 1);
    }

    #[test]
    fn it_should_copy_md_files_to_target_dir() {
        let source_dir = "./test/fixtures".to_string();
        let target_dir = "./test/fixtures/target".to_string();
        let ignored_folders = vec![];

        let md_files = all(&source_dir, &ignored_folders);
        let sharing_files: Vec<&MDFile> = md_files
            .iter()
            .filter(|x| x.frontmatter.get("test_bool").unwrap().as_bool().unwrap())
            .collect();

        sync_with_dest(sharing_files, &target_dir);

        let copied_files = get_files(&target_dir, &ignored_folders);
        assert!(copied_files.len() == 1);
    }
}
