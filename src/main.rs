mod ts;
#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::ts::TSFile;
    #[test]
    fn ts_struct_test() {
        let mut ts = TSFile::new("./ts-src/index.ts");
        assert_eq!(
            ts.search_dependencies_paths(),
            vec![
                "./ts-src/depends/depend.ts".to_string(),
                "./ts-src/depends/deep-depends/depend.ts".to_string()
            ]
        );
    }
    #[test]
    fn search_dependencies_test() {
        let mut ts = TSFile::new("./ts-src/index.ts");
        assert_eq!(
            ts.search_import_statements(),
            vec![
                r#"import { DeepDepends } from "./depends/deep-depends/depend";"#.to_string(),
                r#"import { Depends } from "./depends/depend";"#.to_string(),
            ]
        );
    }
    #[test]
    fn get_only_path_test() {
        let mut ts = TSFile::new("./ts-src/index.ts");
        assert_eq!(
            ts.get_only_path(),
            vec![
                r#"./depends/deep-depends/depend"#.to_string(),
                r#"./depends/depend"#.to_string(),
            ]
        );
    }
}
fn main() {
    let mut index_ts = ts::TSFile::new("index.ts");
    println!("{:?}", index_ts);
    //println!("{:?}", index_ts.search_dependencies())
}
