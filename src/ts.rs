use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

use regex::Regex;
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
    pub fn search_dependencies_paths(&self) -> Vec<String> {
        vec!["./ts-src/depends/depend.ts".to_string()]
    }
    //pub fn convert_path(&self, mut path: String) -> String {}
    pub fn search_import_statements(&mut self) -> Vec<String> {
        self.content
            .split("\n")
            .filter(|s| s.contains("import"))
            .map(String::from)
            .collect::<Vec<String>>()
    }
    pub fn get_only_path(&mut self) -> Vec<String> {
        let import_lines = self.search_import_statements();
        let re = Regex::new("[\"\']{1}([.a-zA-Z/-]*)[\"\']{1}").unwrap();
        import_lines
            .iter()
            .filter(|line| re.is_match(line))
            .map(|s| re.captures(s).unwrap().get(1).unwrap().as_str().to_owned())
            .collect::<Vec<String>>()
    }
    //pub fn dependencies_resolve(&mut self){
    //let dependencies = self.search_dependencies();

    //}
}

enum RelativePath {
    Current,
    Previous,
}
