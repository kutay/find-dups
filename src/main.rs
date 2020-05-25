extern crate walkdir;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use walkdir::{DirEntry, WalkDir};

use blake2::{Blake2b, Digest};
use std::path::Path;
use std::{fs, io};

use rayon::prelude::*;

use log::{info, trace, warn};

fn hash_digest(filepath: &Path) -> String {
    trace!("Hashing    : {:?}", filepath);
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

    let mut files = vec![];

    // Grab all filepaths
    for entry in WalkDir::new(dirpath)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir() && !e.path_is_symlink())
    {
        files.push(String::from(entry.path().to_str().unwrap()));
    }

    // Compute hashes
    let files_with_hashes: Vec<(String, String)> = files
        .par_iter()
        .map(|f| {
            let hash = hash_digest(Path::new(&f));
            (f.to_string(), hash)
        })
        .collect();
    trace!("Result    : {:?}", files_with_hashes);

    // Group files with same hashes
    for file_with_hash in files_with_hashes {
        let counter = filenames.entry(file_with_hash.1).or_insert(vec![]);
        counter.push(file_with_hash.0);
    }

    // Print
    for filename in filenames {
        if filename.1.len() > 1 {
            info!("Key        : {} / {}", filename.1.len(), filename.0);
            info!("Values     : {:?}", filename.1);
        }
    }
}

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a path");
    }

    walk_folder(&args[1]);
}
