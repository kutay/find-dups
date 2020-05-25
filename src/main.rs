extern crate walkdir;

use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::time::SystemTime;

use chrono::{DateTime, Local};
use log::{info, trace, warn};
use rayon::prelude::*;
use walkdir::WalkDir;

mod hash;
mod model;

fn walk_folder(dirpath: &str) -> HashMap<String, Vec<model::FileEntry>> {
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
    let files_with_hashes: Vec<model::FileEntry> = files
        .par_iter()
        .map(|f| {
            let hash = hash::hash_digest(Path::new(&f.0));

            model::FileEntry {
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

    let mut grouped_files = walk_folder(&args[1]);

    // Print
    for grouped in &mut grouped_files {
        if grouped.1.len() > 1 {
            info!("Found file with {} duplicates", grouped.1.len());

            grouped.1.sort_unstable();

            for dup in grouped.1 {
                info!("    - {:?}     {:?}", dup.path, dup.created);
            }
        }
    }
}
