use std::collections::HashMap;
use std::path::Path;
use std::time::SystemTime;

use chrono::{DateTime, Local};
use log::{info, trace, warn};
use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

use crate::hash;
use crate::model;

fn format_system_time(systime: SystemTime) -> String {
    let datetime: DateTime<Local> = systime.into();
    format!("{}", datetime.format("%Y-%m-%d %T"))
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn skip_folders(e: &DirEntry, opts: &model::SearchOptions) -> bool {
    !(opts.skip_hidden_folders && is_hidden(e))
        && !opts
            .skip_folders_names
            .contains(&String::from(e.file_name().to_str().unwrap()))
}

fn file_filter(e: &DirEntry) -> bool {
    !e.file_type().is_dir() && !e.path_is_symlink()
}

fn walk_folder(opts: &model::SearchOptions) -> Vec<(String, String)> {
    let mut files = vec![];

    // Grab all filepaths
    for entry in WalkDir::new(opts.folder.as_str())
        .into_iter()
        .filter_entry(|e| skip_folders(e, opts))
        .filter_map(Result::ok)
        .filter(|e| file_filter(e))
    {
        trace!("entry : {:?}", entry);

        files.push((
            String::from(entry.path().to_str().unwrap()),
            format_system_time(entry.metadata().unwrap().created().unwrap()),
        ));
    }

    files
}

fn search(opts: &model::SearchOptions) -> HashMap<String, Vec<model::FileEntry>> {
    let mut filenames = HashMap::new();

    let files = walk_folder(opts);

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

pub(crate) fn search_duplicates(opts: &model::SearchOptions) {
    let mut grouped_files = search(opts);

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
