extern crate walkdir;

use std::collections::HashMap;
use std::error::Error;
use walkdir::{DirEntry, WalkDir};

use blake2::{Blake2b, Digest};
use std::path::Path;
use std::{fs, io};

fn hash_digest(filepath: &Path) -> String {
    println!("Hashing    : {:?}", filepath);
    let mut file = fs::File::open(filepath).unwrap();
    let mut hasher = Blake2b::new();
    let n = io::copy(&mut file, &mut hasher).unwrap();
    let hash = hasher.result();

    // println!("Bytes processed: {}", n);

    format!("{:x}", hash)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn walk_folder(dirpath: &str) {
    let mut filenames = HashMap::new();

    for entry in WalkDir::new(dirpath)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir() && !e.path_is_symlink())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());

        let hash = hash_digest(&entry.path());
        let key = hash;

        let counter = filenames.entry(key).or_insert(vec![]);
        counter.push(String::from(entry.path().to_str().unwrap()));
    }

    for filename in filenames {
        println!("Key    : {}", filename.0);
        println!("Values : {:?}", filename.1);
    }
}

fn main() {
    walk_folder("/home/aykut/Documents");
    // walk_folder("/home/aykut/Documents/find-dupes");
}
