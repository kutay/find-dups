extern crate walkdir;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use walkdir::{DirEntry, WalkDir};

use blake2::{Blake2b, Digest};
use std::path::Path;
use std::{fs, io};

use rayon::prelude::*;

use chrono::{DateTime, Local, Utc};
use log::{info, trace, warn};
use std::time::{Instant, SystemTime};

#[derive(Debug)]
struct FileEntry {
    path: String,
    hash: String,
    created: String,
}

fn hash_digest(filepath: &Path) -> String {
    trace!("Hashing file {:?}", filepath);

    let mut file = fs::File::open(filepath).unwrap();
    let mut hasher = Blake2b::new();
    let n = io::copy(&mut file, &mut hasher).unwrap();
    trace!("Bytes processed: {}", n);

    format!("{:x}", hasher.result())
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn walk_folder(dirpath: &str) -> HashMap<String, Vec<FileEntry>> {
    let mut filenames = HashMap::new();

    let mut files = vec![];

    // Grab all filepaths
    for entry in WalkDir::new(dirpath)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir() && !e.path_is_symlink())
    {
        // files.push(String::from(entry.path().to_str().unwrap()));
        files.push((
            String::from(entry.path().to_str().unwrap()),
            format_system_time(entry.metadata().unwrap().created().unwrap()),
        ));
    }

    // Compute hashes
    let files_with_hashes: Vec<FileEntry> = files
        .par_iter()
        .map(|f| {
            let hash = hash_digest(Path::new(&f.0));

            FileEntry {
                path: f.0.to_string(),
                hash,
                created: f.1.clone(),
            }
        })
        .collect();
    trace!("Result    : {:?}", files_with_hashes);

    // Group files with same hashes
    for file_entry in files_with_hashes {
        let counter = filenames.entry(file_entry.hash.clone()).or_insert(vec![]);
        counter.push(file_entry);
    }

    filenames
}

fn format_system_time(systime: SystemTime) -> String {
    let datetime: DateTime<Local> = systime.into();
    format!("{}", datetime.format("%Y-%m-%d %T"))
}

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a path");
    }

    let grouped_files = walk_folder(&args[1]);

    // Print
    for grouped in &grouped_files {
        if grouped.1.len() > 1 {
            info!("Found file with {} duplicates", grouped.1.len());
            for dup in grouped.1 {
                info!("    - {:?}     {:?}", dup.path, dup.created);
            }
        }
    }
}
