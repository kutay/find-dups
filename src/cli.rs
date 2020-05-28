use clap::Clap;

use crate::model;
use crate::model::SearchOptions;

use log::trace;

#[derive(Clap)]
struct Opts {
    folder: String,

    #[clap(long, about("Skip hidden folders"))]
    skip_hidden_folders: bool,

    #[clap(long, about("Comma-separated list of folder names to skip"))]
    skip_folders: Option<String>,
}

pub(crate) fn init() -> SearchOptions {
    let opts: Opts = Opts::parse();

    let skip_folder_names = opts
        .skip_folders
        .unwrap()
        .split(",")
        .map(String::from)
        .collect();

    trace!("Opts - folder               : {}", opts.folder);
    trace!("Opts - skip_hidden_folders  : {}", opts.skip_hidden_folders);
    trace!("Opts - skip_folder_names    : {:?}", skip_folder_names);

    let search_options = model::SearchOptions {
        folder: opts.folder,
        skip_hidden_folders: opts.skip_hidden_folders,
        skip_folders_names: skip_folder_names,
    };

    search_options
}
