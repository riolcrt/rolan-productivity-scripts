use serde_yaml::Value;

#[derive(Debug)]
pub struct MDFile {
    pub path: String,
    pub content: String,
    pub frontmatter: Value,
}

impl MDFile {
    pub fn from_path(path: &str) -> Result<MDFile, String> {
        let content = super::read_file::execute(path).unwrap();
        let frontmatter = crate::md_file::domain::parse_md_file::execute(&content);

        Ok(MDFile {
            path: path.to_string(),
            content,
            frontmatter,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_a_md_file_from_a_path_buffer() {
        let path = "./test/fixtures/valid.md".to_string();
        let md_file = MDFile::from_path(&path);

        match md_file {
            Ok(md_file) => {
                assert_eq!(md_file.path, path);
                assert_eq!(
                    md_file.content,
                    "---\ntest_bool: true\ntest_str: \"test\"\ntest_num: 1\n---\n\nContent"
                );
                assert!(md_file
                    .frontmatter
                    .get("test_bool")
                    .unwrap()
                    .as_bool()
                    .unwrap());
            }
            Err(_) => panic!("Should not have failed"),
        }
    }
}
