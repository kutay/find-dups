extern crate walkdir;

mod cli;
mod finder;
mod hash;
mod model;

fn main() {
    env_logger::init();

    let opts = cli::init();

    finder::search_duplicates(&opts);
}
