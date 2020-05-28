use clap::Clap;

use crate::model;
use crate::model::SearchOptions;

use log::trace;

#[derive(Clap)]
struct Opts {
    folder: String,

    #[clap(long)]
    skip_hidden_folders: bool,

    #[clap(long)]
    skip_folders: Option<String>,
}

pub(crate) fn init() -> SearchOptions {
    let opts: Opts = Opts::parse();

    trace!("Using input file: {}", opts.folder);
    trace!("Using input file: {}", opts.skip_hidden_folders);

    let search_options = model::SearchOptions {
        folder: opts.folder,
        skip_hidden_folders: opts.skip_hidden_folders,
    };

    search_options
}
