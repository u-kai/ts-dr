use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};
#[derive(Debug)]
pub struct TSFile {
    content: String,
    dependencies: Vec<Box<TSFile>>,
    relative_path: String,
}

impl TSFile {
    pub fn new(relative_path: &str) -> TSFile {
        let content = TSFile::read_content(relative_path).unwrap();
        let dependencies = vec![];
        TSFile {
            content,
            dependencies,
            relative_path: relative_path.to_string(),
        }
    }
    fn read_content(relative_path: &str) -> Result<String> {
        let reader = BufReader::new(File::open(relative_path)?);
        let contents = reader
            .lines()
            .map(|s| s.unwrap())
            .fold("".to_string(), |acc, s| format!("{}{}\n", acc, s));
        Ok(contents)
    }
    pub fn search_dependencies(&mut self) -> Vec<String> {
        self.content
            .split("\n")
            .filter(|s| s.contains("import"))
            .map(String::from)
            .collect::<Vec<String>>()
    }
}
