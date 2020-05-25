extern crate walkdir;

use std::collections::HashMap;
use std::error::Error;
use walkdir::WalkDir;

fn walk_folder(dirpath: &str) {
    let mut filenames = HashMap::new();

    for entry in WalkDir::new(dirpath)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let counter = filenames.entry(f_name.clone()).or_insert(vec![]);
        counter.push(String::from(entry.path().to_str().unwrap()));
    }

    for filename in filenames {
        println!("Key    : {}", filename.0);
        println!("Values : {:?}", filename.1);
    }
}

fn main() {
    walk_folder("/home/aykut/Documents/find-dupes");
}
