use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
};
mod ts;
fn main() {
    let mut index_ts = ts::TSFile::new("index.ts");
    println!("{:?}", index_ts);
    println!("{:?}", index_ts.search_dependencies())
}
