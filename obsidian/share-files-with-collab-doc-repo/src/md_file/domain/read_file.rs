use std::fs::File;
use std::io::Read;

pub fn execute(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_read_the_content_of_a_file() {
        let path = "./test/fixtures/valid.md".to_string();
        let contents = execute(&path).unwrap();
        assert!(!contents.is_empty());
    }
}
