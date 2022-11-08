use extract_frontmatter::config::Splitter;
use extract_frontmatter::Extractor;
use serde_yaml::from_str;
use serde_yaml::Value;

pub fn execute(content: &str) -> Value {
    let (frontmatter, _) = Extractor::new(Splitter::EnclosingLines("---")).extract(content);

    match from_str(&frontmatter) {
        Ok(frontmatter) => frontmatter,
        Err(_) => Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_convert_yaml_like_strings_to_an_object() {
        let content = "---\nkey: value\n---\n\n# Hello World";
        let frontmatter = execute(content);
        assert_eq!(frontmatter.get("key").unwrap().as_str().unwrap(), "value");

        let content = "# Hello World";
        let frontmatter = execute(content);
        assert!(frontmatter.is_null());

        let content = "---\nkey: true\n---\n\n# Hello World";
        let frontmatter = execute(content);
        assert!(frontmatter.get("key").unwrap().as_bool().unwrap());

        let content = "---\nkey: True\n---\n\n# Hello World";
        let frontmatter = execute(content);
        assert!(frontmatter.get("key").unwrap().as_bool().unwrap());
    }
}
